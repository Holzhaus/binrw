#![feature(generic_associated_types)]
use binrw::BinRead;

#[derive(BinRead)]
#[br(assert(false, String::from("message"), "too", "many", "arguments"))]
struct Foo;

fn main() {}
