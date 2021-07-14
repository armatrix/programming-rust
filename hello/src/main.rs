use std::io::Write;
use std::str::FromStr;

fn main() {
    println!("Hello, world!");
    let gcd48 = gcd(4, 8);
    println!("GCD 4 and 8 is {}", gcd48);

    // let overflow_number = test_overflow();
    // println!("max u32 after plus 1 is : {}", overflow_number);

    // let after_add = test_plus_overflow(u32::MAX, 1);
    // println!("max u32 after plus 1 is : {}", after_add);

    // 在最开始的时候我们可以将这里的Vec理解为python中的列表或者JS中的数组
    // 尽管平时我们理解的数组本身是可变长的，和这里的变量一样，要对其发生可能引起长度的改变操作，我们还是要加上mut关键字

    // 注意当我们没有写后续的代码的时候，这里会报错
    // let mut numbers: Vec<{unknown}> = Vec::new();
    // type annotations needed for `Vec<T>`
    // cannot infer type for type parameter `T`rustcE0282
    let mut numbers = Vec::new();
    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error while parsing arguments"))
    }
    // println!("numbers: {:?}", numbers)

    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "usage: xxxx...").unwrap();
        std::process::exit(1);
    }

    let mut d = numbers[0];
    // &借用 和 *解引用, 这里和其他语言类似
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }
    println!("the greatest common divisor of {:?} is {}.", numbers, d);
}

// 这里有个rust的典型的地方，rust中有return语句，这里将n作为返回值的时候并且不是以分号结尾的则不需要，这提供了一种简写的方式
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let temp = m;
            m = n;
            n = temp;
        }
        m = m % n;
    }
    n
}

#[allow(dead_code)]
fn test_overflow() -> u32 {
    //   can't compile
    //          max_u32 + 1
    //    |     ^^^^^^^^^^^ attempt to compute `u32::MAX + 1_u32`, which would overflow
    let max_u32 = u32::MAX;
    max_u32 + 1
}

#[allow(dead_code)]
fn test_plus_overflow(m: u32, n: u32) -> u32 {
    m + n
    //     thread 'main' panicked at 'attempt to add with overflow', src/main.rs:36:5
    // stack backtrace:
    //    0: rust_begin_unwind
    //              at /rustc/9bc8c42bb2f19e745a63f3445f1ac248fb015e53/library/std/src/panicking.rs:493:5
    //    1: core::panicking::panic_fmt
    //              at /rustc/9bc8c42bb2f19e745a63f3445f1ac248fb015e53/library/core/src/panicking.rs:92:14
    //    2: core::panicking::panic
    //              at /rustc/9bc8c42bb2f19e745a63f3445f1ac248fb015e53/library/core/src/panicking.rs:50:5
    //    3: hello::test_plus_overflow
    //              at ./src/main.rs:36:5
    //    4: hello::main
    //              at ./src/main.rs:9:21
    //    5: core::ops::function::FnOnce::call_once
    //              at /Users/matrix/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/ops/function.rs:227:5
    // note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
    // The terminal process "cargo 'run', '--package', 'hello', '--bin', 'hello'" terminated with exit code: 101.
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1)
}

// 你可能会用到的命令行工具

// cargo run 799459 28823 27347
// output: xxxx 41
