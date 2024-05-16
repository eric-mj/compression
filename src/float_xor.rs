use itertools::Itertools;

fn xor_float_encode(data: &[f64]) -> Vec<u64> {
    let mut encoded_data: Vec<u64> = Vec::new();
    let first_value = data.first().unwrap().clone().to_bits();
    let tuples: Vec<(&f64, &f64)> = data.iter().tuple_windows().collect();

    let xored: Vec<u64> = tuples
        .iter()
        .map(|(a, b)| a.to_bits() ^ b.to_bits())
        .collect();
    println!("xored {:#^016x?}", xored);
    encoded_data.push(first_value);
    for word in xored {
        if word == 0 {
            encoded_data.push(word);
        } else {
        }
    }

    let mut last_xor: u64 = 0;

    for i in 0..data.len() - 1 {
        let first = data[i];
        let second = data[i + 1];
        let xored = first.to_bits() ^ second.to_bits();

        println!("comparing {} and {}", first.to_bits(), second.to_bits());

        let mut last_xored_leading_zeros = 0;
        let mut last_xored_trailing_zeros = 0;

        // First iteration need to just compare with the actual value
        if i == 0 {
            last_xored_leading_zeros = data[0].to_bits().leading_zeros() as u64;
            last_xored_trailing_zeros = data[0].to_bits().trailing_zeros() as u64;
        } else {
            last_xored_leading_zeros = last_xor.leading_zeros() as u64;
            last_xored_trailing_zeros = last_xor.trailing_zeros() as u64;
        }

        // If the xored value is zero (the values are equal) then we just store 0
        if xored == 0 {
            encoded_data.push(0);
        } else {
            // Count the leading and trailing zeros so we know if all meaningful bits overlap
            let xored_leading_zeros = xored.leading_zeros() as u64;
            let xored_trailing_zeros = xored.trailing_zeros() as u64;

            // If the meaningful bits overlap then we don't have to do anything special
            if xored_leading_zeros >= last_xored_leading_zeros
                && xored_trailing_zeros >= last_xored_trailing_zeros
            {
                let meaningful_data = second.to_bits() >> xored_trailing_zeros;
                let meaningful_len = 64 - xored_trailing_zeros - xored_leading_zeros;
                let encoded_word = (2 << meaningful_len) | meaningful_data;
                encoded_data.push(encoded_word);
            } else {
                // Extra data needs to be encoded when the meaningful bits don't overlap
                let meaningful_data = second.to_bits() >> xored_trailing_zeros;
                let meaningful_len = 64 - xored_trailing_zeros - xored_leading_zeros;
                let mut encoded_word = (xored_leading_zeros << (6 + meaningful_len))
                    | (meaningful_len << meaningful_len)
                    | meaningful_data;
                encoded_word = encoded_word | (3 << (64 - encoded_word.leading_zeros()));
                encoded_data.push(encoded_word);
            }
        }

        last_xor = xored;
    }
    return encoded_data;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor() {
        let test_data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let encoded = xor_float_encode(&test_data);
        println!("Test data");
        for d in test_data {
            println!("{:016x}", d.to_bits());
        }
        println!("Encoded: {:016x?}", encoded);
        assert!(false);
    }
}
