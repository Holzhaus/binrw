#![feature(generic_associated_types)]
use binrw::{BinRead, derive_binread};

#[derive(BinRead)]
enum Foo {}

#[derive_binread]
enum Bar {}

fn main() {}
