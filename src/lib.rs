pub mod delta_encode;
pub mod dict_encode;
pub mod float_xor;
pub mod rle;
pub mod simple8b;

#[cfg(test)]
mod tests {
    use self::{
        rle::{rle_decode, rle_encode},
        simple8b::{simple8b_decode, simple8b_encode},
    };

    use super::*;

    #[test]
    fn test_rle_with_pack() {
        let test_data = vec![1, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 4, 5];
        let rle: Vec<u64> = rle_encode(&test_data)
            .iter()
            .flat_map(|x| x.as_bytes())
            .map(|x| x as u64)
            .collect();
        let packed = simple8b_encode(&rle);
        // should be [1, 5, 2, 4, 3, 2, 4, 1, 5, 1]
        let correct: u64 = 0b111000001000101000010000100000011000010000100000001000101000001;
        assert_eq!(packed[0], correct);
    }

    #[test]
    fn test_rle_with_pack_round_trip() {
        let test_data = vec![1, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 4, 5];
        let rle: Vec<u64> = rle_encode(&test_data)
            .iter()
            .flat_map(|x| x.as_bytes())
            .map(|x| x as u64)
            .collect();
        let packed = simple8b_encode(&rle);
        let unpacked = simple8b_decode(&packed);
        let decoded = rle_decode(&unpacked);
        assert_eq!(decoded, test_data);
    }
}
