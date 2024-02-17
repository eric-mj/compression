#[derive(PartialEq, Debug)]
pub struct RunPair {
    value: usize,
    run: usize,
}

pub fn rle_encode<T>(data: &[T]) -> Vec<RunPair> 
where
    T: num::PrimInt + Ord + num::NumCast
{
    let mut encoded_data = Vec::<RunPair>::new();
    let mut current_run_value = data.iter().next().unwrap();
    let mut current_run_len: usize = 0;
    for val in data.iter() {
        if val == current_run_value {
            current_run_len += 1;
        } else {
            encoded_data.push(RunPair { value: current_run_value.to_usize().unwrap(), run: current_run_len });
            current_run_value = val;
            current_run_len = 1;
        }
    }
    encoded_data.push(RunPair { value: current_run_value.to_usize().unwrap(), run: current_run_len });
    return  encoded_data;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rle() {
        let test_data = vec![1, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 4, 5];
        let encoded_data = rle_encode(&test_data);
        let truth = vec![RunPair{value: 1, run: 5}, RunPair{value: 2, run: 4}, RunPair{value: 3, run: 2}, RunPair{value: 4, run: 1}, RunPair{value: 5, run: 1}];
        assert_eq!(truth, encoded_data)
    }
}