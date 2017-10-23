extern crate rand;

pub mod random;
pub mod gen;
pub mod property;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn totally_works() {
        assert_eq!(2 + 6, 5);
    }
}
