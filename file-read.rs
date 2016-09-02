fn foo() -> std::io::Result<()> {
    use std::io::prelude::*;
    use std::fs::File;

    let mut f = try!(File::create("foo.txt"));
    try!(f.write_all(b"Hello, world!"));

    let mut f = try!(File::open("foo.txt"));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    assert_eq!(s, "Hello, world!");
    Ok(())
}

fn main() {
    match foo() {
        Ok(()) => println!("Fine!"),
        Err(e) => println!("{:?}", e),
    }
}
