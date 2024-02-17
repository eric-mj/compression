
fn xor_float_encode(data: &[f64]) -> Vec<u64> {
    let mut encoded_data: Vec<u64> = Vec::new();
    let first_value = float_to_bytes(data[0]);
    println!("First Value: {:x?}", first_value);
    return encoded_data;
}

fn float_to_bytes(f: f64) -> u64 {
    return u64::from_be_bytes(f.to_be_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor() {
        let test_data = vec![1.0, 2.0];
        let encoded = xor_float_encode(&test_data);
        println!("Encoded: {:x?}", encoded);
        assert!(false);
    }
}
