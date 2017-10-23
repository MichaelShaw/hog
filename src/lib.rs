extern crate rand;
extern crate num_traits;
extern crate num_integer;
extern crate colored;

pub mod random;
pub mod gen;
pub mod property;
pub mod test_result;
pub mod check;


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
