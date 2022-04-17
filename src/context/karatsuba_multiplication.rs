use std::cmp::max;

const TEN: i128 = 10;
pub fn multiply(num1: i128, num2: i128) -> i128 {
    karatsuba_algo(num1, num2)
}

fn karatsuba_algo(num1: i128, num2: i128) -> i128 {
    if num1 < 10 || num2 < 10 {
        return num1 * num2;
    }
    let mut num1_str = num1.to_string();
    let mut num2_str = num2.to_string();

    let n = max(num1_str.len(), num2_str.len());
    num1_str = normalize(num1_str, n);
    num2_str = normalize(num2_str, n);

    let a = &num1_str[0..n / 2];
    let b = &num1_str[n / 2..];
    let c = &num2_str[0..n / 2];
    let d = &num2_str[n / 2..];

    let ac = karatsuba_algo(a.parse().unwrap(), c.parse().unwrap());
    let bd = karatsuba_algo(b.parse().unwrap(), d.parse().unwrap());
    let a_b: i128 = a.parse::<i128>().unwrap() + b.parse::<i128>().unwrap();
    let c_d: i128 = c.parse::<i128>().unwrap() + d.parse::<i128>().unwrap();
    let ad_bc = karatsuba_algo(a_b, c_d) - (ac + bd);

    let m = n / 2 + n % 2;
    (TEN.pow(2 * m as u32) * ac) + (TEN.pow(m as u32) * ad_bc) + (bd)
    
}

fn normalize(mut a: String, n: usize) -> String {
    let mut counter = 0;
    for _ in a.len()..n {
        a.insert(counter, '0');
        counter += 1;
    }
    a
}
#[cfg(test)]
mod tests {
    use crate::context::karatsuba_multiplication::multiply;

    #[test]
    fn test_1() {
        let n1: i128 = 314159265;
        let n2: i128 = 314159265;
        let ans = multiply(n1, n2);
        assert_eq!(ans, n1 * n2);
    }

    #[test]
    fn test_2() {
        let n1: i128 = 3141592653589793232;
        let n2: i128 = 2718281828459045233;
        let ans = multiply(n1, n2);
        assert_eq!(ans, n1 * n2);
    }
    
    #[test]
    fn test_3() {
        let n1: i128 = 123456789;
        let n2: i128 = 101112131415;
        let ans = multiply(n1, n2);
        assert_eq!(ans, n1 * n2);
    }
}
