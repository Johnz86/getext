mod names;
use names::*;
use rand::Rng;
use rand::seq::SliceRandom;

pub fn female_first_name<R: Rng + ?Sized>(rng: &mut R) -> &str {
    return FEMALE_FIRST_NAME.choose(rng).unwrap()
}