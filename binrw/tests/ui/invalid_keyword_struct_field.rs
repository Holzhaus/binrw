#![feature(generic_associated_types)]
use binrw::BinRead;

#[derive(BinRead)]
struct Struct {
    #[br(invalid_struct_field_keyword)]
    field: i32,
}

fn main() {}
