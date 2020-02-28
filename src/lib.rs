extern crate rand;
pub mod locales;

#[cfg(test)]
mod tests {

    use crate::locales::en::female_first_name;

    #[test]
    fn ranom_female_name() {
        let mut rng = rand::thread_rng();
        println!("name: {}", female_first_name(&mut rng));
        assert_eq!(female_first_name(&mut rng).len() > 4, true);
    }
}
