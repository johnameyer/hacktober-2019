use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Invalid use:\n\tExpected: {} [num]", &args[0])
    }
    let mut num = 102;
    if args.len() == 2 {
        num = args[1].parse::<i32>().expect("Invalid use:\n\tExpected number") + 1;
    }
    for i in 1..num {
        let result: &dyn std::fmt::Display = match (i%3, i%5) {
            (0, 0) => &"FizzBuzz",
            (0, _) => &"Fizz",
            (_, 0) => &"Buzz",
            (_, _) => &i
        };
        println!("{}", result);
    }
}