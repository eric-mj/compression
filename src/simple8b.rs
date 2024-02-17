use itertools::izip;
use std::{collections::HashMap, mem::size_of};

use num::{self, PrimInt};

const SELECTOR_VALUE: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
const INTEGERS_CODED: [u8; 16] = [240, 120, 60, 30, 20, 15, 12, 10, 8, 7, 6, 5, 4, 3, 2, 1];
const BPI: [u8; 16] = [0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 10, 12, 15, 20, 30, 60];
const BIT_WIDTH: u8 = size_of::<u64>() as u8;

pub fn simple8b_encode(data: &[u64]) -> Vec<u64> {
    let mut encoded_data: Vec<u64> = Vec::new();
    let mut buff = data;
    while buff.len() > 0 {
        for (selector, int_count, num_bits) in izip!(SELECTOR_VALUE, INTEGERS_CODED, BPI) {
            let can_pack = can_pack(buff, int_count, num_bits);
            if can_pack {
                let packed = pack(buff, selector, int_count, num_bits);
                buff = &buff[int_count as usize..];
                encoded_data.push(packed);
                break;
            }
        }
    }
    return encoded_data;
}

pub fn simple8b_decode(data: &[u64]) -> Vec<u64> {
    let mut decoded: Vec<u64> = Vec::new();
    for word in 0..data.len() {
        let unpacked = unpack(data[word as usize]);
    }
}

fn unpack(word: u64) -> Vec<u64> {
    let mut decoded_words: Vec<u64> = Vec::new();
    let selector = word >> 60;
    let bits_per_int = BPI[word as usize];
    let encoded_words = INTEGERS_CODED[word as usize];
    for w in 0..encoded_words {
       let decoded_word =  
    }
}

fn pack(data: &[u64], selector: u8, integers_coded: u8, num_bits: u8) -> u64 {
    let mut packed: u64 = u64::from(selector) << 60;
    for num in 0..integers_coded {
        packed = packed | data[num as usize] << (60 - (num_bits * (num + 1)));
    }
    return packed;
}

//Can you pack `num_elements` integers from `data` using `bits` per integer
fn can_pack(data: &[u64], num_elements: u8, bits: u8) -> bool {
    if data.len() < num_elements.into() {
        return false;
    }
    let mut end = data.len();
    if num_elements < end as u8 {
        end = num_elements as usize;
    }
    if bits == 0 {
        for item in data[..end].iter() {
            if item != &1 {
                return false;
            }
        }
    }
    let max_value: u64 = (1 << bits) - 1;
    for item in data[..end].iter() {
        if item > &max_value {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pack() {
        let test_data = vec![1, 1];
        let packed = pack(&test_data, 14, 2, 30);
        let correct: u64 = 0xE000000040000001;
        assert_eq!(packed, correct);
    }

    #[test]
    fn test_pack_all_ones() {
        let test_data: Vec<u64> = vec![1; 60];
        let packed = pack(&test_data, 2, 60, 1);
        let correct: u64 = 0x2FFFFFFFFFFFFFFF;
        assert_eq!(packed, correct);
    }

    #[test]
    fn test_pack_all_bytes() {
        let test_data: Vec<u64> = vec![255; 7];
        let packed = pack(&test_data, 9, 7, 8);
        println!("Packed: {:x}", packed);
        let correct: u64 = 0x9FFFFFFFFFFFFFF0;
        assert_eq!(packed, correct);
    }


    #[test]
    fn test_can_pack_ones() {
        let test_data: Vec<u64> = vec![1; 60];
        assert_eq!(can_pack(&test_data, 60, 1), true);
        assert_eq!(can_pack(&test_data, 120, 0), false);
        assert_eq!(can_pack(&test_data, 240, 0), false);
    }

    #[test]
    fn test_can_pack_twos() {
        let test_data: Vec<u64> = vec![2; 30];
        assert_eq!(can_pack(&test_data, 30, 1), false);
        assert_eq!(can_pack(&test_data, 60, 1), false);
        assert_eq!(can_pack(&test_data, 30, 2), true);
        assert_eq!(can_pack(&test_data, 240, 0), false);
    }

    #[test]
    fn test_can_pack_byte() {
        let test_data: Vec<u64> = vec![255; 7];
        assert_eq!(can_pack(&test_data, 30, 1), false);
        assert_eq!(can_pack(&test_data, 60, 1), false);
        assert_eq!(can_pack(&test_data, 30, 2), false);
        assert_eq!(can_pack(&test_data, 7, 8), true);
        assert_eq!(can_pack(&test_data, 240, 0), false);
    }

    #[test]
    fn test_simple8b() {
        let test_data = vec![1; 60];
        let encoded = simple8b_encode(&test_data);
        let correct: Vec<u64> = vec![0x2FFFFFFFFFFFFFFF];
        assert_eq!(encoded, correct);
    }

    #[test]
    fn test_simple8b_2_words() {
        let test_data = vec![1; 60 * 2];
        let encoded = simple8b_encode(&test_data);
        let correct: Vec<u64> = vec![0x2FFFFFFFFFFFFFFF, 0x2FFFFFFFFFFFFFFF];
        assert_eq!(encoded, correct);
    }

    #[test]
    fn test_simple8b_2_different_words() {
        let mut test_data = vec![1; 60 * 2];
        let mut test_data_2 = vec![255; 7 * 2];
        test_data.append(&mut test_data_2);
        let encoded = simple8b_encode(&test_data);
        println!("Encoded: {:x?}", encoded);
        let correct: Vec<u64> = vec![0x2FFFFFFFFFFFFFFF, 0x2FFFFFFFFFFFFFFF, 0x9FFFFFFFFFFFFFF0, 0x9FFFFFFFFFFFFFF0];
        assert_eq!(encoded, correct);
    }
}
