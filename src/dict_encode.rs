use std::{collections::HashMap, hash::Hash};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CodingError {
    #[error("Missing code in codebook")]
    MissingCode,
}
pub struct EncodingDict<T> {
    encoding_map: HashMap<T, u64>,
    decoding_map: HashMap<u64, T>,
    num_items: u64,
}

impl<T> EncodingDict<T>
where
    T: Hash + Eq + Clone,
{
    fn new() -> Self {
        return Self {
            encoding_map: HashMap::new(),
            decoding_map: HashMap::new(),
            num_items: 0,
        };
    }

    fn insert(&mut self, k: T) -> u64 {
        self.encoding_map.insert(k.clone(), self.num_items);
        self.decoding_map.insert(self.num_items, k.clone());
        self.num_items += 1;
        return self.num_items - 1;
    }

    fn get_encode(&self, k: &T) -> Option<u64> {
        return self.encoding_map.get(k).copied();
    }

    fn get_decode(&self, k: &u64) -> Option<T> {
        return Some(self.decoding_map.get(k)?.clone());
    }
}

pub fn dict_encode<T>(data: &[T]) -> (Vec<u64>, EncodingDict<T>)
where
    T: Hash + Eq + Clone,
{
    let mut encoded_map = EncodingDict::<T>::new();
    let mut encoded_data = Vec::<u64>::new();
    for item in data {
        if let Some(val) = encoded_map.get_encode(item) {
            encoded_data.push(val.clone());
        } else {
            let val = encoded_map.insert(item.clone());
            encoded_data.push(val);
        }
    }

    return (encoded_data, encoded_map);
}

pub fn encode_with_dict<T>(data: &[T], dict: &mut EncodingDict<T>) -> Vec<u64>
where
    T: Hash + Eq + Clone,
{
    let mut encoded_data = Vec::<u64>::new();
    for item in data {
        if let Some(val) = dict.get_encode(item) {
            encoded_data.push(val.clone());
        } else {
            let val = dict.insert(item.clone());
            encoded_data.push(val);
        }
    }
    return encoded_data;
}

pub fn decode_with_dict<T>(data: &[u64], dict: &EncodingDict<T>) -> Result<Vec<T>, CodingError>
where
    T: Hash + Eq + Clone,
{
    let mut decoded_data = Vec::<T>::new();
    for item in data {
        decoded_data.push(
            dict.get_decode(item)
                .ok_or_else(|| CodingError::MissingCode)?,
        );
    }
    return Ok(decoded_data);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dict_encode() {
        let raw_vec = vec![
            "test".to_string(),
            "test".to_string(),
            "other".to_string(),
            "test".to_string(),
            "other".to_string(),
        ];
        let (encoded, dict) = dict_encode(&raw_vec);
        let expected_encoding =
            HashMap::<String, u64>::from([("test".to_string(), 0u64), ("other".to_string(), 1u64)]);

        assert_eq!(encoded, vec![0, 0, 1, 0, 1]);
        assert_eq!(dict.encoding_map, expected_encoding);
    }

    #[test]
    fn test_encode_with_existing_dict() {
        let raw_vec = vec![
            "test".to_string(),
            "test".to_string(),
            "other".to_string(),
            "test".to_string(),
            "other".to_string(),
        ];
        let (_, mut dict) = dict_encode(&raw_vec);
        let new_vec = vec![
            "test".to_string(),
            "test".to_string(),
            "other".to_string(),
            "test".to_string(),
            "other".to_string(),
        ];
        let encoded = encode_with_dict(&new_vec, &mut dict);
        assert_eq!(encoded, vec![0, 0, 1, 0, 1]);
    }

    #[test]
    fn test_encode_add_to_existing_dict() {
        let raw_vec = vec![
            "test".to_string(),
            "test".to_string(),
            "other".to_string(),
            "test".to_string(),
            "other".to_string(),
        ];
        let (_, mut dict) = dict_encode(&raw_vec);
        let new_vec = vec![
            "test".to_string(),
            "test".to_string(),
            "other".to_string(),
            "test".to_string(),
            "new".to_string(),
        ];
        let encoded = encode_with_dict(&new_vec, &mut dict);
        let expected_encoding = HashMap::<String, u64>::from([
            ("test".to_string(), 0u64),
            ("other".to_string(), 1u64),
            ("new".to_string(), 2u64),
        ]);
        assert_eq!(encoded, vec![0, 0, 1, 0, 2]);
        assert_eq!(dict.encoding_map, expected_encoding)
    }
    #[test]
    fn test_round_trip() {
        let raw_vec = vec![
            "test".to_string(),
            "test".to_string(),
            "other".to_string(),
            "test".to_string(),
            "other".to_string(),
        ];
        let (encoded, dict) = dict_encode(&raw_vec);
        println!("{:?}", dict.decoding_map);
        let decoded = decode_with_dict(&encoded, &dict).expect("Should not have error in test");
        assert_eq!(raw_vec, decoded);
    }
}
