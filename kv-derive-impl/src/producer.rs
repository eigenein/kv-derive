use std::iter;

use crate::into_vec::IntoVec;

/// Responsible for producing the key-value entries based on its value.
///
/// May produce none, one or many entries, depending on a specific type.
pub trait Producer<T> {
    type Iter: Iterator<Item = (String, String)>;

    fn produce(self, key: &'static str, value: T) -> Self::Iter;
}

/// Scalar producer.
///
/// Produces exactly one key-value pair.
#[derive(Copy, Clone)]
pub struct ScalarProducer;

impl<V: ToString> Producer<V> for ScalarProducer {
    type Iter = iter::Once<(String, String)>;

    #[inline]
    fn produce(self, key: &'static str, value: V) -> Self::Iter {
        iter::once((key.to_string(), value.to_string()))
    }
}

#[derive(Copy, Clone)]
pub struct WrappedProducer<P, F>(pub P, pub F);

impl<V1, V2, P: Producer<V2>, F: Fn(V1) -> V2> Producer<V1> for WrappedProducer<P, F> {
    type Iter = P::Iter;

    #[inline]
    fn produce(self, key: &'static str, value: V1) -> Self::Iter {
        self.0.produce(key, self.1(value))
    }
}

/// Optional scaler producer.
///
/// Produces none or one key-value pair.
#[derive(Copy, Clone)]
pub struct OptionProducer<P>(pub P);

impl<V, P: Producer<V>> Producer<Option<V>> for OptionProducer<P> {
    type Iter = iter::FlatMap<
        iter::Zip<iter::Zip<std::option::IntoIter<V>, iter::Once<&'static str>>, iter::Once<P>>,
        P::Iter,
        fn(((V, &'static str), P)) -> P::Iter,
    >;

    #[inline]
    fn produce(self, key: &'static str, value: Option<V>) -> Self::Iter {
        value
            .into_iter()
            .zip(iter::once(key))
            .zip(iter::once(self.0))
            .flat_map(|((value, key), producer)| producer.produce(key, value))
    }
}

/// Collection producer.
///
/// Produces as many key-value pairs as the number of the collection elements.
#[derive(Copy, Clone)]
pub struct CollectionProducer<P>(pub P);

impl<V, P: Producer<V> + Copy + Clone> Producer<Vec<V>> for CollectionProducer<P> {
    type Iter = iter::FlatMap<
        iter::Zip<iter::Zip<iter::Repeat<&'static str>, std::vec::IntoIter<V>>, iter::Repeat<P>>,
        <P as Producer<V>>::Iter,
        fn(((&'static str, V), P)) -> <P as Producer<V>>::Iter,
    >;

    #[inline]
    fn produce(self, key: &'static str, values: Vec<V>) -> Self::Iter {
        iter::repeat(key)
            .zip(values.into_iter())
            .zip(iter::repeat(self.0))
            .into_iter()
            .flat_map(|((key, value), producer)| producer.produce(key, value))
    }
}

/// Simple flattening producer.
///
/// Forwards all the key-value pairs from the inner structure.
#[derive(Copy, Clone)]
pub struct FlatteningProducer;

impl<T: IntoVec> Producer<T> for FlatteningProducer {
    type Iter = Box<dyn Iterator<Item = (String, String)>>;

    #[inline]
    fn produce(self, _key: &'static str, value: T) -> Self::Iter {
        value.into_iter()
    }
}

/// Prefixed flattening producer.
///
/// Forwards all the key-value pairs from the inner structure,
/// but additionally prepends the keys with the prefix.
#[derive(Copy, Clone)]
pub struct PrefixedFlatteningProducer(pub &'static str);

impl<V: IntoVec> Producer<V> for PrefixedFlatteningProducer {
    type Iter = Box<dyn Iterator<Item = (String, String)>>;

    #[inline]
    fn produce(self, _key: &'static str, value: V) -> Self::Iter {
        Box::new(
            value
                .into_iter()
                .map(move |(key, value)| (format!("{}{}", self.0, key), value)),
        )
    }
}
