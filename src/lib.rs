pub fn solve(amount: usize, roof_size: u32, pos: &[u32]) -> usize {
    let mut max: usize = 0;

    let mut front_pos = 0;
    for i in 0..amount {
        // de-queue front chickens that move outside of the roof
        while pos[i] - pos[front_pos] >= roof_size {
            front_pos = front_pos + 1;
        }

        // check max chicken
        if i - front_pos > max {
            max = i - front_pos;
        }
    }

    max + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(5, 5, &[2, 5, 10, 12, 15]), 2);
    }

    #[test]
    fn test_table_solve() {
        let cases = [
            (1, 1, &vec![20], 1),
            (2, 1, &vec![2, 3], 1),
            (3, 2, &vec![3, 4, 5], 2),
            (5, 5, &vec![2, 5, 10, 12, 15], 2),
            (6, 10, &vec![1, 11, 30, 34, 35, 37], 4),
        ];
        for &(amount, roof_size, pos, expected) in &cases {
            let result = solve(amount,roof_size,pos);
            assert_eq!(result, expected, "input: {}&{:?}, wanted: {}, got: {}", roof_size, pos, expected, result);
        }
    }
}
