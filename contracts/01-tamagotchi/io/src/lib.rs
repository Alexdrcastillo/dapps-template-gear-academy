#![no_std]

use codec::{Decode, Encode};
use gmeta::Metadata;
use gstd::prelude::*;
use scale_info::TypeInfo;

#[derive(Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Tamagotchi {
    pub name: String,  // 1️⃣ Added `name` field
    pub age: u64,      // 1️⃣ Added `age` field
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TmgAction {
    Name(String),  // 2️⃣ Added `Name` action that sets the name
    Age(u64),      // 2️⃣ Added `Age` action that sets the age
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TmgEvent {
    Name(String),  // 3️⃣ Added `Name` event that returns the name
    Age(u64),      // 3️⃣ Added `Age` event that returns the age
}

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = In<()>;
    type Handle = InOut<TmgAction, TmgEvent>;
    type State = Out<Tamagotchi>;
    type Reply = ();
    type Others = ();
    type Signal = ();
}
