use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Default)]
pub struct Registry<K, V> {
    specs: HashMap<K, V>,
}



impl<K, V> Registry<K, V>
where
    K: Eq + Hash,
{
    pub fn add(&mut self, kind: K, spec: V) {
        self.specs.insert(kind, spec);
    }

    pub fn get(&self, kind: &K) -> Option<&V> {
        self.specs.get(kind)
    }
}

impl <K, V> Registry<K, V>
where 
    K: Eq + Hash + Copy,
    V: RegistryDefaults<K>
{
    pub fn new() -> Self {
        let mut r = Self { specs: HashMap::new() };
        r.register_defaults();
        r
    }

}

impl<K, V> Registry<K, V>
where 
    K: Eq + Hash + Copy,
    V: SpawnFromSpec<K>
{
    pub fn spawn(&self, kind: K) -> V::Output {
        let spec = self.specs.get(&kind).expect("missing spec");
        V::spawn_from_spec(kind, spec)
    }
}

pub trait SpawnFromSpec<K> {
    type Output;
    fn spawn_from_spec(kind: K, spec: &Self) -> Self::Output;
}


pub trait RegistryDefaults<K> {
    fn defaults() -> impl IntoIterator<Item = (K, Self)>;

}


impl<K, V> Registry<K, V>
where 
    K: Eq + Hash,
    V: RegistryDefaults<K>
{
    pub fn register_defaults(&mut self) {
        for (kind, spec) in V::defaults() {
            self.add(kind, spec);
        }
    }
}









