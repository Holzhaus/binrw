#![feature(generic_associated_types)]
use binrw::BinRead;

#[derive(BinRead)]
struct Foo {
    #[br(if("wrong type"))]
    a: i32,
}

fn main() {}
