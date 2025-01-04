struct Solution;

impl Solution {
    // A single digit
    // I -> I -> #I up to 3 times? because IIII is IV
    // I -> V -> 4
    // I -> X -> 9
    // V -> V -> impossible? because VV should be written as X
    // X -> X -> #X*10 up to 3 times? because XXXX should be written as XL
    // X -> L -> 40
    // X -> C -> 90
    // L -> L -> impossible? because LL should be written as C
    // C -> C -> #C*100 up to 3 times? because CCCC should be written as CD
    // C -> D -> 400
    // C -> M -> 900
    // D -> D -> impossible? because DD should be written as M
    // M -> M -> #M*1000 up to 4 times?

    pub fn roman_to_int(s: String) -> i32 {}
}

fn f(s: &str) -> () {
    let i = Solution::roman_to_int(String::from(s));
    print!("{} -> {}", s, i)
}

fn main() {
    f("I");
    f("V");
    f("X");
    f("L");
    f("C");
    f("D");
    f("M");
    f("III");
    f("LVIII");
    f("MCMXCIV");
}
