#![feature(generic_associated_types)]
use binrw::BinRead;

#[derive(BinRead)]
struct Foo {
    #[br(deref_now, offset_after(1))]
    a: u8,
}

fn main() {}
