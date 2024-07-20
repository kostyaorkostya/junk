/// Determine the length of the collatz sequence beginning at `n`.
/// https://en.wikipedia.org/wiki/Collatz_conjecture
fn collatz_length(mut n: i32) -> u32 {
    let mut res = 1;
    while n != 1 {
        res += 1;
        n = dbg!(if n % 2 == 0 { n / 2 } else { 3 * n + 1 })
    }
    res
}

#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length(3), 8);
    assert_eq!(collatz_length(11), 15);
}

fn main() {
    for n in [1, 3] {
        print!("n={} -> collatz_length(n)={}", n, collatz_length(n));
    }
}
