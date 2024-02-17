use std::ops::Sub;

pub fn delta<T>(data: &[T]) -> Vec<T>
where
    for<'a> &'a T: Sub<&'a T, Output = T>,
    T: Clone,
    T: Default,
{
    if data.len() == 0 {
        return Vec::<T>::new();
    }
    let first = data.first().unwrap().clone();
    let mut delta_encoded = vec![first];

    let mut prev = &T::default();
    for (idx, item) in data.iter().enumerate() {
        if idx == 0 {
            prev = item;
            continue;
        }
        let curr = item;
        let delta = curr - prev;
        prev = curr;
        delta_encoded.push(delta.clone());
    }
    
    return delta_encoded;
}

pub fn delta_delta<T>(data: &[T]) -> Vec<T>
where
    for<'a> &'a T: Sub<&'a T, Output = T>,
    T: Clone,
    T: Default,
{
    let delta_encoded = delta(&data);
    let mut delta_delta = vec![delta_encoded.first().unwrap().clone()];
    delta_delta.append(&mut delta(&delta_encoded[1..]));
    return delta_delta;
}
