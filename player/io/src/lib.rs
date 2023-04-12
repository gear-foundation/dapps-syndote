#![no_std]
use gmeta::{InOut, Metadata};
use syndote_io::YourTurn;
use gstd::prelude::*;
pub struct PlayerMetadata;

impl Metadata for PlayerMetadata {
    type Init = ();
    type Handle = InOut<YourTurn, String>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = ();
}
