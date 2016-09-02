use std::str;

fn factorial(accum: i64, n:i64) -> i64 {
    if n == 1 {
        return accum
    }
    else {
        return factorial(accum + 1, n - 1)
    }
}

fn main() {
    println!("factorial: {:?}",  factorial(0,10000000000000));
}
