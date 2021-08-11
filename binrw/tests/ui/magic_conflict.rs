#![feature(generic_associated_types)]
use binrw::BinRead;

#[derive(BinRead)]
enum Foo {
    #[br(magic = 0u8)] A,
    #[br(magic = 1i16)] B,
}

fn main() {}
