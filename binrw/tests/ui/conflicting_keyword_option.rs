#![feature(generic_associated_types)]
use binrw::BinRead;

#[derive(BinRead)]
#[br(magic = 0u8, magic = 0u8)]
struct Foo;

fn main() {}
