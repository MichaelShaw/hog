extern crate rand;
extern crate num_traits;
extern crate num_integer;
extern crate colored;

pub mod random;
pub mod gen;
pub mod property;
pub mod test_result;
pub mod check;

use std::collections::{HashMap, HashSet};


#[macro_export]
macro_rules! hashset {
    ($($val: expr ),*) => {{
         let mut set = HashSet::default();
         $( set.insert( $val); )*
         set
    }}
}

#[macro_export]
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = HashMap::default();
         $( map.insert($key, $val); )*
         map
    }}
}