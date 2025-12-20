use super::{
    ids,
    simple::{self, Simple},
};

pub fn new(run: bool) -> Simple {
    simple::new(ids::CHECK_THE_FIT_OF_EARBUDS, run as u8)
}
