#![feature(generic_associated_types)]
use binrw::BinRead;

#[derive(BinRead)]
struct Foo {
    #[br(map = does_not_exist)]
    a: i32,
}

fn main() {}
