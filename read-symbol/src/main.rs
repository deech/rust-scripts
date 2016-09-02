#[macro_use]
extern crate nom;
use std::str;

use nom::*;

const CHARACTERS: &'static str = "abcdefghijklmnopqrstuvwxzABCDEFGHIJKLMNOPQRSTUVWXZ=-*/+_?$!@~.><&%\\#`;:";
const DIGITS: &'static str = "0123456789";

named!(klsymbol<String>,
       chain!(
           initial: one_of!(CHARACTERS) ~
           remainder: many0!(
               alt_complete!(
                   one_of!(DIGITS) |
                   one_of!(CHARACTERS)
               )
           ),
           || {
               let mut res : Vec <char> = vec![initial];
               res.extend(remainder);
               res.into_iter().collect()
           }
       )
);

fn main () {
    let f = b"abc123=";
    match klsymbol(f){
        IResult::Done(_, out) => println!("{:?}", out),
        IResult::Incomplete(x) => panic!("incomplete: {:?}", x),
        IResult::Error(e) => panic!("error: {:?}", e),
    }
}
