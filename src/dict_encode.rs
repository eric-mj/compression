use std::{collections::HashMap, hash::Hash};

fn dict_encode<T>(data: &[T]) -> (Vec<u64>, HashMap<T, u64>)
where
    T: Hash + Eq + Clone,
{
    let mut encoded_map = HashMap::<T, u64>::new();
    let mut encoded_data = Vec::<u64>::new();
    let mut num_items: u64 = 0;
    for item in data {
        if let Some(val) = encoded_map.get(item) {
            encoded_data.push(val.clone());
        } else {
            encoded_data.push(num_items);
            encoded_map.insert(item.clone(), num_items);
            num_items += 1
        }
    }

    return (encoded_data, encoded_map);
}

fn encode_with_dict<T>(data: &[T], dict: &mut HashMap<T, u64>) -> Vec<u64>
where
    T: Hash + Eq + Clone,
{
    let mut encoded_data = Vec::<u64>::new();
    let mut num_items: u64 = dict.len() as u64;
    for item in data {
        if let Some(val) = dict.get(item) {
            encoded_data.push(val.clone());
        } else {
            encoded_data.push(num_items);
            dict.insert(item.clone(), num_items);
            num_items += 1
        }
    }
    return encoded_data;
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
        assert_eq!(encoded, vec![0, 0, 1, 0, 1]);
        assert_eq!(
            dict,
            HashMap::<String, u64>::from([("test".to_string(), 0u64), ("other".to_string(), 1u64)])
        );
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
        assert_eq!(encoded, vec![0, 0, 1, 0, 2]);
        assert_eq!(
            dict,
            HashMap::<String, u64>::from([
                ("test".to_string(), 0u64),
                ("other".to_string(), 1u64),
                ("new".to_string(), 2u64)
            ])
        )
    }
}
