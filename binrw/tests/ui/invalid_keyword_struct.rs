#![feature(generic_associated_types)]
use binrw::BinRead;

#[derive(BinRead)]
#[br(invalid_struct_keyword)]
struct Struct {
    field: i32,
}

fn main() {}
