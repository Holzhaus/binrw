#![feature(generic_associated_types)]
use binrw::BinRead;

#[derive(BinRead)]
struct Foo {
    #[br(restore_position, restore_position)]
    a: i32,
}

fn main() {}
