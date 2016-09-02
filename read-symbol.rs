#[macro_use]
extern crate nom;
use std::str;
use nom::*;

pub enum KlSymbol {
    KlVariable(String),
    KlNormalSymbol(String),
}

static KLSymbolCharacters: [char; 75] = [
    'a' , 'b' , 'c' , 'd' , 'e' , 'f' , 'g' , 'h' , 'i' , 'j' , 'k' , 'l' , 'm' , 'n' , 'o' , 'p' , 'q' , 'r' , 's' , 't' , 'u' , 'v' , 'w' , 'x' , 'y' , 'z',
    'A' , 'B' , 'C' , 'D' , 'E' , 'F' , 'G' , 'H' , 'I' , 'J' , 'K' , 'L' , 'M' , 'N' , 'O' , 'P' , 'Q' , 'R' , 'S' , 'T' , 'U' , 'V' , 'W' , 'X' , 'Y' , 'Z',
    '=' , '-' , '*' , '/' , '+' , '_' , '?' , '$' , '!' , '@' , '~' , '.' , '>' , '<' , '&' , '%' , '\'', '#' , '`' , ';' , ':' , '{' , '}',
];

fn is_klsymbolcharacter(candidate:u8) -> bool {
    KLSymbolCharacters.into_iter().any(|&c| c == candidate as char)
}

named!(klsymbolcharacters,
       (alt!
        (is_klsymbolcharacter ~ klsymbolcharacters) |
        (is_digit             ~ klsymbolcharacters) |
        is_klsymbolcharacter                        |
        is_digit
       )
);

named!(klsymbol<&KlSymbol>,
       (alt!
        (is_klsymbolcharacter ~ klsymbolcharacters) |
        is_klsymbolcharacter
       )
);

fn main () {
    let f = b"abc123=";
    match klsymbol(f){
        IResult::Done(in_, out) => {
            println!(out, b"nom");
            println!(in_, b",age\ncarles,30\nlaure,28\n");
        },
        IResult::Incomplete(x) => panic!("incomplete: {:?}", x),
        IResult::Error(e) => panic!("error: {:?}", e),
    }
}
