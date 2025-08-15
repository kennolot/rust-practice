#[allow(unused_imports)]
use std::collections::HashMap;

fn main() {}

#[allow(dead_code)]
fn determinant(matrix: &[Vec<i64>]) -> i64 {
    let n = matrix.len();

    if n == 0 || matrix.iter().any(|row| row.len() != n) {
        return 0;
    }
    if n == 1 {
        return matrix[0][0];
    }
    if n == 2 {
        return matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0];
    }

    let mut result = 0i64;

    for j in 0..n {
        let sub_matrix: Vec<Vec<i64>> = matrix[1..]
            .iter()
            .map(|row| {
                row.iter()
                    .enumerate()
                    .filter(|&(col, _)| col != j)
                    .map(|(_, &val)| val)
                    .collect()
            })
            .collect();
        let cofactor = if j % 2 == 0 { 1 } else { -1 };
        let minor_det = determinant(&sub_matrix);

        result += matrix[0][j] * cofactor * minor_det;
    }
    result
}

#[allow(dead_code)]
fn is_perfect_power(n: u64) -> Option<(u64, u32)> {
    let max_k = (n as f64).log2() as u32;
    for k in 2..=max_k {
        let root_est = (n as f64).powf(1.0 / k as f64);
        let max_m = root_est.floor() as u64;

        for m in (max_m.saturating_sub(1))..=(max_m + 1) {
            if m >= 2 && m.pow(k) == n {
                return Some((m, k));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_powers() {
        assert_eq!(is_perfect_power(4), Some((2, 2))); // 2^2
        assert_eq!(is_perfect_power(27), Some((3, 3))); // 3^3
        assert_eq!(is_perfect_power(81), Some((9, 2))); // 9^2
        assert_eq!(is_perfect_power(5), None);
    }
}
