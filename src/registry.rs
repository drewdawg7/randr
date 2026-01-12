pub trait SpawnFromSpec<K> {
    type Output;
    fn spawn_from_spec(kind: K, spec: &Self) -> Self::Output;
}

pub trait RegistryDefaults<K> {
    fn defaults() -> impl IntoIterator<Item = (K, Self)>;
}









