#[macro_use]
extern crate nom;
use std::str::*;

use nom::*;

const Digits: &'static str = "0123456789";

fn make_digit_string(c: char, v:Vec<char>) -> String {
    let s2 : String = v.into_iter().collect();
    c.to_string() + &s2
}

named!(oneof_test<String>,
       alt!(
           chain!(d: one_of!(Digits) ~ rest: many0!(one_of!(Digits)), || make_digit_string(d, rest)) |
           chain!(d: one_of!(Digits), || d.to_string())
       ));

fn main () {
    let f = b"12";
    match oneof_test(f){
        IResult::Done(in_, out) => println!("{:?}", out),
        IResult::Incomplete(x) => panic!("incomplete: {:?}", x),
        IResult::Error(e) => panic!("error: {:?}", e),
    }
}
