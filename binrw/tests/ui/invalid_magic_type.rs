#![feature(generic_associated_types)]
use binrw::BinRead;

#[derive(BinRead)]
#[br(magic = "invalid_type")]
struct Foo;

fn main() {}
