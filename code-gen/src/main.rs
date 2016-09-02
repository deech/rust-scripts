#![cfg_attr(feature = "nightly", feature(rustc_private))]

extern crate aster;

#[cfg(feature = "nightly")]
extern crate syntax;

#[cfg(not(feature = "nightly"))]
extern crate syntex_syntax as syntax;

fn main() {
    let builder = aster::AstBuilder::new();

    let expr = builder.expr()
        .add().u32(1).u32(2);

    prints `1 + 2`.
    // println!("{}", syntax::print::pprust::expr_to_string(&expr));
}
