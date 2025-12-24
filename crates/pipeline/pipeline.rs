use std::any::Any;
use std::{future::Future, marker::PhantomData, num::NonZeroUsize};

use anyhow::bail;
use frunk::HList;
use frunk::{HCons, HNil, hlist::HList};
use futures::FutureExt;
use futures::future::BoxFuture;
use futures::task::Spawn;
use tokio::task::JoinSet;

// struct NonEmptyVec<T> {
//     head: T,
//     tail: Vec<T>,
// }

// impl<T> From<T> for NonEmptyVec<T> {
//     fn from(value: T) -> Self {
//         NonEmptyVec {
//             head: value,
//             tail: Vec::new(),
//         }
//     }
// }

// impl<T> NonEmptyVec<T> {
//     fn new(head: T) -> NonEmptyVec<T> {
//         NonEmptyVec {
//             head,
//             tail: Vec::new(),
//         }
//     }

// }

// sync + async concurrency managed globally
// but each stage has its own capped output queue

struct Stage<In, Out> {
    output_cap: Option<NonZeroUsize>,
    transformer: Transformer<In, Out>,
}

enum Transformer<In, Out> {
    Init(Vec<Out>),
    SyncMany(Box<dyn Fn(In) -> anyhow::Result<Vec<Out>> + Sync + Send + 'static>),
    SyncOne(Box<dyn Fn(In) -> anyhow::Result<Out> + Sync + Send + 'static>),
    AsyncMany(Box<dyn Fn(In) -> BoxFuture<'static, anyhow::Result<Vec<Out>>>>),
    AsyncOne(Box<dyn Fn(In) -> BoxFuture<'static, anyhow::Result<Out>>>),
}

impl<In, Out, Func> From<Func> for Transformer<In, Out>
where
    Func: Fn(In) -> anyhow::Result<Out> + 'static,
{
    fn from(value: Func) -> Self {
        Transformer::SyncOne(Box::new(value))
    }
}

pub struct Pipeline<Inner> {
    inner: Inner,
}

enum StageOutput<A, B, C, D, E, F> {
    A(A),
    B(B),
    C(C),
    D(D),
    E(E),
    F(F),
}

struct Stage1<T>(T)
where
    T: Any + Sync + Send + 'static;
struct Stage2<T>(T)
where
    T: Any + Sync + Send + 'static;
struct Stage3<T>(T)
where
    T: Any + Sync + Send + 'static;
struct Stage4<T>(T)
where
    T: Any + Sync + Send + 'static;

struct Spawner<A, B, C, D, E, F> {
    async_tasks: JoinSet<anyhow::Result<Vec<StageOutput<A, B, C, D, E, F>>>>,
    async_concurrency: usize,
    sync_tasks: JoinSet<anyhow::Result<Vec<StageOutput<A, B, C, D, E, F>>>>,
    sync_concurrency: usize,
}

trait SpawnerT {
    type Output: Sync + Send + 'static;

    fn spawn_async<I, T, Fut>(
        &mut self,
        i: I,
        f: impl FnOnce(I) -> Fut + Sync + Send + 'static,
        mapper: impl Fn(T) -> Self::Output + Sync + Send + 'static,
    ) -> Option<I>
    where
        I: Sync + Send + 'static,
        Fut: Future<Output = anyhow::Result<Vec<T>>> + Sync + Send + 'static;
    fn spawn_sync<I, T>(
        &mut self,
        i: I,
        f: impl FnOnce(I) -> anyhow::Result<Vec<T>> + Sync + Send + 'static,
        mapper: impl Fn(T) -> Self::Output + Sync + Send + 'static,
    ) -> Option<I>
    where
        I: Sync + Send + 'static;
}

impl<A, B, C, D, E, F> SpawnerT for Spawner<A, B, C, D, E, F>
where
    A: Sync + Send + 'static,
    B: Sync + Send + 'static,
    C: Sync + Send + 'static,
    D: Sync + Send + 'static,
    E: Sync + Send + 'static,
    F: Sync + Send + 'static,
{
    type Output = StageOutput<A, B, C, D, E, F>;

    fn spawn_async<I, T, Fut>(
        &mut self,
        i: I,
        f: impl FnOnce(I) -> Fut + Sync + Send + 'static,
        mapper: impl Fn(T) -> Self::Output + Sync + Send + 'static,
    ) -> Option<I>
    where
        Fut: Future<Output = anyhow::Result<Vec<T>>> + Sync + Send + 'static,
    {
        if self.async_tasks.len() >= self.async_concurrency {
            return Some(i);
        }

        self.async_tasks
            .spawn(f(i).map(|v| v.map(|x| x.into_iter().map(mapper).collect())));

        None
    }

    fn spawn_sync<I, T>(
        &mut self,
        i: I,
        f: impl FnOnce(I) -> anyhow::Result<Vec<T>> + Sync + Send + 'static,
        mapper: impl Fn(T) -> Self::Output + Sync + Send + 'static,
    ) -> Option<I>
    where
        I: Sync + Send + 'static,
    {
        if self.sync_tasks.len() >= self.sync_concurrency {
            return Some(i);
        }

        self.sync_tasks
            .spawn_blocking(move || f(i).map(|v| v.into_iter().map(mapper).collect::<Vec<_>>()));

        None
    }
}

impl<A, B, C, D, E, F> Spawner<A, B, C, D, E, F>
where
    A: Sync + Send + 'static,
    B: Sync + Send + 'static,
    C: Sync + Send + 'static,
    D: Sync + Send + 'static,
    E: Sync + Send + 'static,
    F: Sync + Send + 'static,
{
    async fn next(&mut self) -> Option<anyhow::Result<Vec<StageOutput<A, B, C, D, E, F>>>> {
        tokio::select! {
            x = self.async_tasks.join_next() => {
                let x = x?;
                match x {
                    Err(e) => {
                        return Some(Err(e.into()));
                    }
                    Ok(x) => {
                        return Some(x);
                    }
                }
            }
            x = self.sync_tasks.join_next() => {
                let x = x?;
                match x {
                    Err(e) => {
                        return Some(Err(e.into()));
                    }
                    Ok(x) => {
                        return Some(x);
                    }
                }
            }
        }
    }

    fn try_spawn1(&mut self, inputs: &mut Vec<A>, tf: Stage<A, B>, count: &mut usize) {
        if let Some(max) = tf.output_cap
            && *count >= max.get()
        {
            return;
        }

        let Some(proposed) = inputs.pop() else {
            return;
        };

        match tf.transformer {
            Transformer::SyncMany(f) if self.sync_tasks.len() < self.sync_concurrency => {
                self.sync_tasks.spawn_blocking(move || {
                    f(proposed).map(|v| v.into_iter().map(StageOutput::B).collect::<Vec<_>>())
                });
            }
            Transformer::SyncOne(f) if self.sync_tasks.len() < self.sync_concurrency => {
                self.sync_tasks
                    .spawn_blocking(move || f(proposed).map(|v| [StageOutput::B(v)].into()));
            }
            Transformer::AsyncMany(f) if self.async_tasks.len() < self.async_concurrency => {
                self.async_tasks.spawn(
                    f(proposed).map(|v| v.map(|x| x.into_iter().map(StageOutput::B).collect())),
                );
            }
            Transformer::AsyncOne(f) if self.async_tasks.len() < self.async_concurrency => {
                self.async_tasks
                    .spawn(f(proposed).map(|v| v.map(|x| [StageOutput::B(x)].into())));
            }
            _ => {
                inputs.push(proposed);
            }
        }
    }
}

fn try_spawn<A, B, T, U>(
    spawner: &mut T,
    inputs: &mut Vec<A>,
    tf: Stage<A, B>,
    count: &mut usize,
    wrapper: impl Fn(B) -> U + Sync + Send + 'static,
) where
    A: Sync + Send + 'static,
    B: Sync + Send + 'static,
    T: SpawnerT<Output = U>,
{
    if let Some(max) = tf.output_cap
        && *count >= max.get()
    {
        return;
    }

    let Some(proposed) = inputs.pop() else {
        return;
    };

    if let Some(got_back) = match tf.transformer {
        Transformer::SyncMany(f) => {
            spawner.spawn_sync(proposed, f, wrapper);
            None
        }
        Transformer::SyncOne(f) => {
            spawner.spawn_sync(proposed, move |i| Ok([f(i)?].into()), wrapper);
            None
        }
        Transformer::AsyncMany(f) => {
            spawner.spawn_async(proposed, f, wrapper);
            None
        }
        Transformer::AsyncOne(f) => {
            spawner.spawn_async(proposed, f, wrapper);
            None
        }
        _ => Some(proposed),
    } {
        inputs.push(got_back);
    }
}

impl<B, C>
    Pipeline<
        HList![
            Stage<B, C>,
            Stage<(), B>,
        ],
    >
{
    pub async fn run2(self, rt: tokio::runtime::Runtime) -> anyhow::Result<Vec<C>> {
        // let sync = tokio::runtime::JoinSet::new();

        // self.inner.foldl(folder, acc);

        let list = self.inner;
        let (s1, list) = list.pluck::<Stage<(), B>, _>();
        let (s2, list) = list.pluck::<Stage<B, C>, _>();

        todo!()
    }
}

impl<B, C, D>
    Pipeline<
        HList![
            Stage<C, D>,
            Stage<B, C>,
            Stage<(), B>,
        ],
    >
{
    pub async fn run3(self, rt: tokio::runtime::Runtime) -> anyhow::Result<Vec<D>> {
        // let sync = tokio::runtime::JoinSet::new();

        // self.inner.foldl(folder, acc);
        let list = self.inner;
        let (s1, list) = list.pluck::<Stage<(), B>, _>();
        let (s2, list) = list.pluck::<Stage<B, C>, _>();
        let (s3, list) = list.pluck::<Stage<C, D>, _>();

        let Transformer::Init(x) = s1.transformer else {
            bail!("nope");
        };

        let spawner = Spawner {
            async_tasks: JoinSet::new(),
            async_concurrency: 10,
            sync_tasks: JoinSet::new(),
            sync_concurrency: 2,
        };

        let s2out = Vec::new();
        let s2count = 0;
        let s3out = Vec::new();

        match s2.transformer {
            Transformer::Init(items) => todo!(),
            Transformer::SyncMany(_) => todo!(),
            Transformer::SyncOne(_) => todo!(),
            Transformer::AsyncMany(_) => todo!(),
            Transformer::AsyncOne(_) => todo!(),
        }

        todo!()
    }
}

impl<B, C, D, E>
    Pipeline<
        HList![
            Stage<D, E>,
            Stage<C, D>,
            Stage<B, C>,
            Stage<(), B>,
        ],
    >
{
    pub async fn run4(self, rt: tokio::runtime::Runtime) -> anyhow::Result<Vec<E>> {
        // let sync = tokio::runtime::JoinSet::new();

        // self.inner.foldl(folder, acc);
        let list = self.inner;
        let (s1, list) = list.pluck::<Stage<(), B>, _>();
        let (s2, list) = list.pluck::<Stage<B, C>, _>();
        let (s3, list) = list.pluck::<Stage<C, D>, _>();
        let (s4, list) = list.pluck::<Stage<D, E>, _>();

        todo!()
    }
}

trait Run {
    type Output;
    async fn run(self, rt: tokio::runtime::Runtime) -> anyhow::Result<Vec<Self::Output>>;
}

impl<B, C> Run
    for Pipeline<
        HList![
            Stage<B, C>,
            Stage<(), B>,
        ],
    >
{
    type Output = C;

    async fn run(self, rt: tokio::runtime::Runtime) -> anyhow::Result<Vec<Self::Output>> {
        self.run2(rt).await
    }
}

impl<B, C, D> Run
    for Pipeline<
        HList![
            Stage<C, D>,
            Stage<B, C>,
            Stage<(), B>,
        ],
    >
{
    type Output = D;

    async fn run(self, rt: tokio::runtime::Runtime) -> anyhow::Result<Vec<Self::Output>> {
        self.run3(rt).await
    }
}

impl<B, C, D, E> Run
    for Pipeline<
        HList![
            Stage<D, E>,
            Stage<C, D>,
            Stage<B, C>,
            Stage<(), B>,
        ],
    >
{
    type Output = E;

    async fn run(self, rt: tokio::runtime::Runtime) -> anyhow::Result<Vec<Self::Output>> {
        self.run4(rt).await
    }
}

impl Pipeline<HNil> {
    pub fn new<Initial>(inputs: Vec<Initial>) -> Pipeline<HCons<Stage<(), Initial>, HNil>>
    where
        Initial: 'static + Sync + Send,
    {
        Pipeline {
            inner: HNil.prepend(Stage {
                output_cap: None,
                transformer: Transformer::Init(inputs),
            }),
        }
    }
}

impl<Prev, In, Tail> Pipeline<HCons<Stage<Prev, In>, Tail>>
where
    Tail: HList,
{
    pub fn sync_one<Out, Func>(
        self,
        f: Func,
        output_cap: Option<NonZeroUsize>,
    ) -> Pipeline<HCons<Stage<In, Out>, HCons<Stage<Prev, In>, Tail>>>
    where
        Func: Fn(In) -> anyhow::Result<Out> + 'static,
    {
        Pipeline {
            inner: self.inner.prepend(Stage {
                output_cap,
                transformer: Transformer::SyncOne(Box::new(f)),
            }),
        }
    }

    pub fn sync_many<Out, Func>(
        self,
        f: Func,
        output_cap: Option<NonZeroUsize>,
    ) -> Pipeline<HCons<Stage<In, Out>, HCons<Stage<Prev, In>, Tail>>>
    where
        Func: Fn(In) -> anyhow::Result<Vec<Out>> + 'static,
    {
        Pipeline {
            inner: self.inner.prepend(Stage {
                output_cap,
                transformer: Transformer::SyncMany(Box::new(f)),
            }),
        }
    }

    pub fn async_one<Out, Func, Fut>(
        self,
        f: Func,
        output_cap: Option<NonZeroUsize>,
    ) -> Pipeline<HCons<Stage<In, Out>, HCons<Stage<Prev, In>, Tail>>>
    where
        Func: Fn(In) -> Fut + 'static + Sync + Send,
        Fut: Future<Output = anyhow::Result<Out>> + 'static + Sync + Send,
    {
        Pipeline {
            inner: self.inner.prepend(Stage {
                output_cap,
                transformer: Transformer::AsyncOne(Box::new(move |i| f(i).boxed())),
            }),
        }
    }

    pub fn async_many<Out, Func, Fut>(
        self,
        f: Func,
        output_cap: Option<NonZeroUsize>,
    ) -> Pipeline<HCons<Stage<In, Out>, HCons<Stage<Prev, In>, Tail>>>
    where
        Func: Fn(In) -> Fut + 'static + Sync + Send,
        Fut: Future<Output = anyhow::Result<Vec<Out>>> + 'static + Sync + Send,
    {
        Pipeline {
            inner: self.inner.prepend(Stage {
                output_cap,
                transformer: Transformer::AsyncMany(Box::new(move |i| f(i).boxed())),
            }),
        }
    }

    fn then<Out>(
        self,
        stage: Stage<In, Out>,
    ) -> Pipeline<HCons<Stage<In, Out>, HCons<Stage<Prev, In>, Tail>>> {
        Pipeline {
            inner: self.inner.prepend(stage),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use crate::{Pipeline, Stage, Transformer};

    #[test]
    fn check() {
        Pipeline::new([0i32, 1, 2, 3].into())
            .then(Stage::<i32, i64> {
                output_cap: None,
                transformer: Transformer::SyncOne(Box::new(|_| Ok(0i64))),
            })
            .sync_one(|x| Ok(x + 5), None)
            // .sync_many(|x| Ok([x + 0, x + 1, x + 2, x + 3].into()), None)
            .run3(tokio::runtime::Runtime::new().unwrap())
            // .async_one(|x| async { Ok(5) }, None)
            // .async_one(async |x| Ok(5), None)
            ;
    }
}
