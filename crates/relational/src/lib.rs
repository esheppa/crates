// struct Correlate<T, U> {

// }

// struct Lookup<T, U> {

// }

trait Relational {

}

trait Key: Clone + Send + Eq + Ord + serde::Serialize + serde::de::DeserializeOwned {

}

trait ForeignKey {
    type References: Key;
}

//trait SurrogateRelational: Relational {
trait SurrogateRelational: Clone + Send + serde::Serialize + serde::de::DeserializeOwned {
        type PrimaryKey: Copy + Key;
}

trait GenericTuple: Send + Clone  + serde::Serialize + serde::de::DeserializeOwned { }

impl<A> GenericTuple for (A,)
where 
    A: Send + Clone + serde::Serialize + serde::de::DeserializeOwned,
{}

impl<A, B> GenericTuple for (A, B,) 
where 
    A: Send + Clone + serde::Serialize + serde::de::DeserializeOwned,
    B:Send + Clone + serde::Serialize + serde::de::DeserializeOwned,
{ }
impl<A, B, C> GenericTuple for (A, B, C,) 
where 
    A: Send + Clone + serde::Serialize + serde::de::DeserializeOwned,
    B:Send + Clone + serde::Serialize + serde::de::DeserializeOwned,
    C:Send + Clone + serde::Serialize + serde::de::DeserializeOwned,
{ }
impl<A, B, C, D> GenericTuple for (A, B, C, D,) 
where 
    A: Send + Clone + serde::Serialize + serde::de::DeserializeOwned,
    B:Send + Clone + serde::Serialize + serde::de::DeserializeOwned,
    C:Send + Clone + serde::Serialize + serde::de::DeserializeOwned,
    D:Send + Clone + serde::Serialize + serde::de::DeserializeOwned,
{ }

impl<A, B, C, D, E> GenericTuple for (A, B, C, D, E,) 
where 
    A: Send + Clone + serde::Serialize + serde::de::DeserializeOwned,
    B:Send + Clone + serde::Serialize + serde::de::DeserializeOwned,
    C:Send + Clone + serde::Serialize + serde::de::DeserializeOwned,
    D:Send + Clone + serde::Serialize + serde::de::DeserializeOwned,
    E:Send + Clone + serde::Serialize + serde::de::DeserializeOwned,
{ }

trait KeyTuple: Ord + Eq + Send + Clone + serde::Serialize + serde::de::DeserializeOwned { }

impl<A> KeyTuple for (A,) 
where A: Ord + Eq + Send + Clone + serde::Serialize + serde::de::DeserializeOwned,
{}

impl<A, B> KeyTuple for (A, B,) 
where 
A: Ord + Eq + Send + Clone + serde::Serialize + serde::de::DeserializeOwned,
B: Ord + Eq + Send + Clone + serde::Serialize + serde::de::DeserializeOwned,
{}

impl<A, B, C> KeyTuple for (A, B, C) 
where 
A: Ord + Eq + Send + Clone + serde::Serialize + serde::de::DeserializeOwned,
B: Ord + Eq + Send + Clone + serde::Serialize + serde::de::DeserializeOwned,
C: Ord + Eq + Send + Clone + serde::Serialize + serde::de::DeserializeOwned,
{}

impl<A, B, C, D> KeyTuple for (A, B, C, D) 
where 
A: Ord + Eq + Send + Clone + serde::Serialize + serde::de::DeserializeOwned,
B: Ord + Eq + Send + Clone + serde::Serialize + serde::de::DeserializeOwned,
C: Ord + Eq + Send + Clone + serde::Serialize + serde::de::DeserializeOwned,
D: Ord + Eq + Send + Clone + serde::Serialize + serde::de::DeserializeOwned,
{}

impl<A, B, C, D, E> KeyTuple for (A, B, C, D, E) 
where 
A: Ord + Eq + Send + Clone + serde::Serialize + serde::de::DeserializeOwned,
B: Ord + Eq + Send + Clone + serde::Serialize + serde::de::DeserializeOwned,
C: Ord + Eq + Send + Clone + serde::Serialize + serde::de::DeserializeOwned,
D: Ord + Eq + Send + Clone + serde::Serialize + serde::de::DeserializeOwned,
E: Ord + Eq + Send + Clone + serde::Serialize + serde::de::DeserializeOwned,
{}

impl<T, U> SurrogateRelational for (T, U) 
where 
    T: SurrogateRelational,
    U: Clone + Send + serde::Serialize + serde::de::DeserializeOwned,
{
    type PrimaryKey = T::PrimaryKey;
}

// trait Lookup<S: SurrogateRelational>: SurrogateRelational + ForeignKey<References=<S as SurrogateRelational>::PrimaryKey> {
//     fn lookup(self, s: RelationalTable<S>) -> RelationalTable<(Self, S)> 
//     {
//         todo!()
//     }
// }

// impl<S: SurrogateRelational, FK> Lookup<S> for FK 
// where 
//     FK: ForeignKey<References=S::PrimaryKey> + SurrogateRelational,
// {}

// trait Correlate<S: SurrogateRelational>: SurrogateRelational + ForeignKey<References=<S as SurrogateRelational>::PrimaryKey> {
//     fn correlate(self, s: RelationalTable<S>) -> RelationalTable<(Self, RelationalTable<S>)> 
//     {
//         todo!()
//     }
// }

// impl<S: SurrogateRelational, FK> Correlate<S> for FK 
// where 
//     FK: ForeignKey<References=S::PrimaryKey> + SurrogateRelational,
// {}

fn lookup<S, F>(s: RelationalTable<S>, f: RelationalTable<F>) -> RelationalTable<(S, F)> 
where 
    S: SurrogateRelational + ForeignKey<References=<F as SurrogateRelational>::PrimaryKey>,
    F: SurrogateRelational,
{
    todo!()
}

fn correlate<S, F>(f: RelationalTable<F>, s: RelationalTable<S>) -> RelationalTable<(F, RelationalTable<S>)> 
where 
    F: SurrogateRelational,
    S: SurrogateRelational + ForeignKey<References=<F as SurrogateRelational>::PrimaryKey>,
{
    todo!()
}

fn group_by<S: SurrogateRelational, K: KeyTuple, V: GenericTuple>(
    s: RelationalTable<S>, 
    group: impl Fn(S::PrimaryKey, &S) -> K, 
    agg: impl FnOnce(S::PrimaryKey, &S, V) -> V, 
) -> Table_<K, V> {
    todo!()
}

//trait NaturalRelational: Relational {
trait NaturalRelational {
        type Key: Clone + Key;
}

type RelationalTable<T> = Table_<<T as SurrogateRelational>::PrimaryKey, T>;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
#[serde(bound(deserialize = "V: serde::de::DeserializeOwned"))]
struct Table_<K, V>(collections::BTreeMap<K, V>)
where
    K: Ord + Eq + Clone + Send + serde::Serialize + serde::de::DeserializeOwned,
    V: Clone + Send + serde::Serialize + serde::de::DeserializeOwned;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(bound(deserialize = "T: serde::de::DeserializeOwned"))]
pub struct Table<T>(collections::BTreeMap<ulid::Ulid, T>)
where
    T: Send + serde::Serialize + serde::de::DeserializeOwned;

impl<T> Table<T>
where
    T: Send + serde::Serialize + serde::de::DeserializeOwned,
{
    // ??
    fn new(d: collections::BTreeMap<ulid::Ulid, T>) -> Table<T> {
        Table(d)
    }
}

impl<T> convert::AsRef<collections::BTreeMap<ulid::Ulid, T>> for Table<T>
where
    T: Send + serde::Serialize + serde::de::DeserializeOwned,
{
    fn as_ref(&self) -> &collections::BTreeMap<ulid::Ulid, T> {
        &self.0
    }
}

impl<T> convert::AsMut<collections::BTreeMap<ulid::Ulid, T>> for Table<T>
where
    T: Send + serde::Serialize + serde::de::DeserializeOwned,
{
    fn as_mut(&mut self) -> &mut collections::BTreeMap<ulid::Ulid, T> {
        &mut self.0
    }
}

impl<T> warp::Reply for Table<T>
where
    T: Send + serde::Serialize + serde::de::DeserializeOwned,
{
    fn into_response(self) -> warp::reply::Response {
        warp::http::Response::new(bincode::serialize(&self).unwrap().into())
    }
}
