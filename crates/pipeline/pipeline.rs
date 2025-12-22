use std::{future::Future, marker::PhantomData, num::NonZeroUsize};

use frunk::{hlist::HList, HCons, HNil};
use futures::future::BoxFuture;
use futures::FutureExt;

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
    SyncMany(Box<dyn Fn(In) -> anyhow::Result<Vec<Out>>>),
    SyncOne(Box<dyn Fn(In) -> anyhow::Result<Out>>),
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

impl<In, Out, X> Pipeline<HCons<Stage<In, Out>, X>> {
    pub async fn run(self, rt: tokio::runtime::Runtime) -> anyhow::Result<Vec<Out>> {

        // let sync = tokio::runtime::JoinSet::new();

        // self.inner.foldl(folder, acc);

        todo!()
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
            .sync_many(|x| Ok([x + 0, x + 1, x + 2, x + 3].into()), None)
            .async_one(|x| async { Ok(5) }, None)
            .async_one(async |x| Ok(5), None);
    }
}
