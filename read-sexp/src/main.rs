#[macro_use]
extern crate nom;
use std::str;
use std::vec::*;

use nom::*;

#[derive(Debug)]
pub enum Sexp {
    SexpString(Vec<u8>),
    Inner(Vec<Sexp>)
}

fn not_separator(c: u8) -> bool {
    ! (c as char == '\t' || c as char == '\n' || c as char == '\r' || c as char == ' ' || c as char == ')' || c as char == '(')
}

#[macro_export]
macro_rules! many0_until (
    ($input:expr, $stopmac:ident!( $($args:tt)* ), $submac:ident!( $($args2:tt)* )) => (
        {
            let mut res = Vec::new();
            let mut input = $input;
            let mut loop_result = Ok(());

            while input.input_len() != 0 {
                match $stopmac!(input, $($args)*) {
                    IResult::Error(_) => {
                        match $submac!(input, $($args2)*) {
                            IResult::Error(_) => {
                                break;
                            },
                            IResult::Incomplete(Needed::Unknown) => {
                                loop_result = Err(IResult::Incomplete(Needed::Unknown));
                                break;
                            },
                            IResult::Incomplete(Needed::Size(i)) => {
                                let size = i + ($input).input_len() - input.input_len();
                                loop_result = Err(IResult::Incomplete(Needed::Size(size)));
                                break;
                            },
                            IResult::Done(i, o) => {
                                res.push(o);
                                input = i;
                            }
                        }
                    },
                    IResult::Done(_,_) => {
                        break;
                    }
                    IResult::Incomplete(Needed::Unknown) => {
                        loop_result = Err(IResult::Incomplete(Needed::Unknown));
                        break;
                    },
                    IResult::Incomplete(Needed::Size(i)) => {
                        let size = i + ($input).input_len() - input.input_len();
                        loop_result = Err(IResult::Incomplete(Needed::Size(size)));
                        break;
                    },
                }
            }
            match loop_result {
                Ok(()) => IResult::Done(input,res),
                Err(e) => e
            }
        }
    );
    ($i:expr, $stopmac:ident!( $($args:tt)* ), $p:expr) => (
        many0_until!($i, $stopmac!($($args)*), call!($p));
    );
);

named!(klsexpinnards<Sexp>,
       alt_complete!(
           chain!(
               opt!(multispace) ~
                   inner: klsexp,
               || Sexp::Inner(inner)
           ) |
           chain!(
               opt!(multispace) ~
                   inner: take_while!(not_separator) ~
                   opt!(multispace),
               || Sexp::SexpString(inner.to_vec())
           )
       )
);

named!(klsexp< Vec<Sexp> >,
       chain!(
           char!('(') ~
           inner: many0_until!(char!(')'),klsexpinnards) ~
           char!(')'),
           || inner
       )
);

named!(klsexps< Vec< Vec<Sexp> > >,
       many0!(
           chain!(
               opt!(multispace) ~
               sexp: klsexp ~
               opt!(multispace),
               || sexp
           )
       )
);

fn print_it(it: &[u8]) -> () {
    match klsexps(it){
        IResult::Done(_, out) => println!("{:?}", out),
        IResult::Incomplete(x) => panic!("incomplete: {:?}", x),
        IResult::Error(e) => panic!("error: {:?}", e),
    }
}

fn main () {
    let f = b"(a hello \n c) \n (\n a b \"hello\")";
    let g = b"(a (he()l()(lo)) c)";
    let h = b"()";
    print_it(f);
    print_it(g);
    print_it(h);
}
