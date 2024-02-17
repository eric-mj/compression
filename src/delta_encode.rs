use std::{fmt::{Debug, Display}, ops::{Add, Sub}};

pub fn delta_encode<T>(data: &[T]) -> Vec<T>
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

pub fn delta_delta_encode<T>(data: &[T]) -> Vec<T>
where
    for<'a> &'a T: Sub<&'a T, Output = T>,
    T: Clone,
    T: Default,
{
    let delta_encoded = delta_encode(&data);
    let mut delta_delta = vec![delta_encoded.first().unwrap().clone()];
    delta_delta.append(&mut delta_encode(&delta_encoded[1..]));
    return delta_delta;
}

pub fn delta_decode<T>(data: &[T]) -> Vec<T>
where
    for<'a> &'a T: Add<&'a T, Output = T>,
    T: Clone,
    T: Default,
{
    let mut output = vec![data[0].clone()];
    let mut prev = data[0].clone();
    for val in data[1..].iter() {
        let s  = &prev + val; 
        output.push(s.clone());
        prev = s; 
    }
    return output
}

pub fn delta_delta_decode<T>(data: &[T]) -> Vec<T> 
where
    for<'a> &'a T: Add<&'a T, Output = T>,
    T: Clone,
    T: Default,
    T: Display,
    T: Debug,
{
    let mut output = vec![data[0].clone()];
    let mut cur_change = data[1].clone();
    let mut cur_val = data[0].clone();
    output.push(&cur_val + &cur_change);
    cur_val = &cur_val + &cur_change;
    for val in data[2..].iter() {
        println!("val {:}", val);
        println!("output {:?}", output);
        println!("cur_val {:}", cur_val);
        cur_val = &(&cur_val + &cur_change) + val;
        output.push(cur_val.clone());
        cur_change = &cur_change + val;
    }
    return output;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delta() {
        let test_data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let encoded_data = delta_encode(&test_data);
        let truth = vec![1; 10];
        assert_eq!(truth, encoded_data)
    }

    #[test]
    fn test_delta_delta() {
        let test_data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let encoded_data = delta_delta_encode(&test_data);
        let truth = vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0];

        assert_eq!(truth, encoded_data);
    }

    #[test]
    fn test_delta_decode() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(input, delta_decode(&delta_encode(&input)));
    }

    #[test]
    fn test_delta_delta_decode() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(input, delta_delta_decode(&delta_delta_encode(&input)));
    }
}
