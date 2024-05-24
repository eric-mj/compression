use itertools::Itertools;

#[derive(PartialEq, Debug)]
pub struct RunPair {
    value: isize,
    run: usize,
}

impl RunPair {
    // There's gotta be a better return type, but tuples are iterable
    pub fn as_bytes(&self) -> Vec<usize> {
        return vec![self.value as usize, self.run];
    }
}

pub fn rle_encode(data: &[isize]) -> Vec<RunPair> {
    let mut encoded_data = Vec::<RunPair>::new();
    let mut current_run_value = data.iter().next().unwrap();
    let mut current_run_len: usize = 0;
    for val in data.iter() {
        if val == current_run_value {
            current_run_len += 1;
        } else {
            encoded_data.push(RunPair {
                value: current_run_value.clone(),
                run: current_run_len,
            });
            current_run_value = val;
            current_run_len = 1;
        }
    }
    encoded_data.push(RunPair {
        value: current_run_value.clone(),
        run: current_run_len,
    });
    return encoded_data;
}

// Need to do error checking
pub fn rle_decode(data: &[u64]) -> Vec<isize> {
    let mut decoded = Vec::<isize>::new();
    for (value, run) in data.iter().tuples() {
        decoded.extend(vec![value.clone() as isize; run.clone() as usize]);
    }
    return decoded;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rle() {
        let test_data = vec![1, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 4, 5];
        let encoded_data = rle_encode(&test_data);
        let truth = vec![
            RunPair { value: 1, run: 5 },
            RunPair { value: 2, run: 4 },
            RunPair { value: 3, run: 2 },
            RunPair { value: 4, run: 1 },
            RunPair { value: 5, run: 1 },
        ];
        assert_eq!(truth, encoded_data)
    }

    #[test]
    fn test_rle_decode() {
        let test_data = vec![1, 5, 2, 4, 3, 2, 4, 1, 5, 1];

        let correct = vec![1, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 4, 5];
        let decoded = rle_decode(&test_data);
        assert_eq!(correct, decoded);
    }
}
