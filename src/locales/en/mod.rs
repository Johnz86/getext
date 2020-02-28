mod names;
use names::*;
use rand::Rng;
use rand::seq::SliceRandom;

pub fn female_first_name<R: Rng + ?Sized>(rng: &mut R) -> &str {
    return FEMALE_FIRST_NAME.choose(rng).unwrap();
}

pub fn male_first_name<R: Rng + ?Sized>(rng: &mut R) -> &str {
    return MALE_FIRST_NAME.choose(rng).unwrap();
}

pub fn last_name<R: Rng + ?Sized>(rng: &mut R) -> &str {
    return LAST_NAME.choose(rng).unwrap();
}

pub fn female_prefix<R: Rng + ?Sized>(rng: &mut R) -> &str {
    return FEMALE_PREFIX.choose(rng).unwrap();
}

pub fn male_prefix<R: Rng + ?Sized>(rng: &mut R) -> &str {
    return MALE_PREFIX.choose(rng).unwrap();
}

pub fn job_prefix<R: Rng + ?Sized>(rng: &mut R) -> &str {
    return JOB_PREFIX.choose(rng).unwrap();
}

pub fn suffix<R: Rng + ?Sized>(rng: &mut R) -> &str {
    return SUFFIX.choose(rng).unwrap();
}

pub fn title_description<R: Rng + ?Sized>(rng: &mut R) -> &str {
    return TITLE_DESCRIPTOR.choose(rng).unwrap();
}

pub fn title_job<R: Rng + ?Sized>(rng: &mut R) -> &str {
    return TITLE_JOB.choose(rng).unwrap();
}

pub fn title_level<R: Rng + ?Sized>(rng: &mut R) -> &str {
    return TITLE_LEVEL.choose(rng).unwrap();
}

pub fn gender<R: Rng + ?Sized>(rng: &mut R) -> &str {
    return GENDER.choose(rng).unwrap();
}