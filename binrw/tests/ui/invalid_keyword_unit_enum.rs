#![feature(generic_associated_types)]
use binrw::BinRead;

#[derive(BinRead)]
#[br(invalid_unit_enum_keyword)]
enum UnitEnum {
    #[br(magic = 0u8)]
    A,
}

fn main() {}
