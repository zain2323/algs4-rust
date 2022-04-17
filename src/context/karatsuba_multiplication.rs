use std::cmp::max;

const TEN: isize = 10;
pub fn multiply(num1: isize, num2: isize) -> isize {
    karatsuba_algo(num1, num2)
}

fn karatsuba_algo(num1: isize, num2: isize) -> isize {
    if num1 < 10 || num2 < 10 {
        return num1 * num2;
    }
    let mut num1_str = num1.to_string();
    let mut num2_str = num2.to_string();

    let n = max(num1_str.len(), num2_str.len());
    num1_str = normalize(num1_str, n);
    num2_str = normalize(num2_str, n);

    let a = &num1_str[0..n/2];
    let b = &num1_str[n/2..];
    let c = &num2_str[0..n/2];
    let d = &num2_str[n/2..];

    let ac = karatsuba_algo(a.parse().unwrap(), c.parse().unwrap());
    let bd = karatsuba_algo(b.parse().unwrap(), d.parse().unwrap());
    let a_b: isize = a.parse::<isize>().unwrap() + b.parse::<isize>().unwrap();
    let c_d: isize = c.parse::<isize>().unwrap()  + d.parse::<isize>().unwrap();
    let mut ad_bc = karatsuba_algo( a_b, c_d);
    ad_bc = ad_bc - ac - bd;

    let m = n/2 + n%2;
    (TEN.pow(2*m as u32) * ac) + (TEN.pow(m as u32) * ad_bc) + (bd)
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
    fn test1() {
        // 3141592653589793238462643383279502884197169399375105820974944592
        // 2718281828459045235360287471352662497757247093699959574966967627
        let n1: isize = 314159265;
        let n2: isize = 314159265;
        let ans = multiply(n1, n2);
        println!("{:}", ans);
    }
}