#![feature(generic_associated_types)]
use binrw::BinRead;

#[derive(BinRead)]
#[br(invalid_enum_keyword)]
enum Enum {
    A(i32),
}

fn main() {}
