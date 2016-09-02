#[macro_use]
extern crate nom;
use std::str;
use std::vec::*;

use nom::*;

fn make_quoted_string (contents:Vec<&[u8]>) -> String {
    let to_vectors : Vec< Vec<u8> > = contents.iter().map(|c| c.to_vec()).collect();
    let smushed : Vec<u8> = to_vectors.concat();
    let mut quoted : Vec<u8> = Vec::new();
    quoted.push('\"' as u8);
    quoted.extend(smushed);
    quoted.push('\"' as u8);
    let result : String = String::from_utf8(quoted).unwrap();
    result
}

named!(klstringinnards< &[u8] >,
       escaped!(none_of!("\"\\"), '\\', one_of!("\"n\\"))
);

named!(klstring<String>,
       chain!(
           char!('\"') ~
           contents:  many0!(klstringinnards) ~
           char!('\"'),
           || make_quoted_string(contents)
       )
);

fn print_it(it: &[u8]) -> () {
    match klstring(it){
        IResult::Done(_, out) => println!("{:?}", out),
        IResult::Incomplete(x) => panic!("incomplete: {:?}", x),
        IResult::Error(e) => panic!("error: {:?}", e),
    }
}

fn main () {
    let f = b"\"ab\\\"cd\"";
    let g = b"\"hello world\"";
    print_it(f);
    print_it(g);
}
