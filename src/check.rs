use test_result::TestResult;
use test_result::Status::{Pass, Fail};
use std::fmt::Debug;

pub fn eq<T>(a: T, b: T) -> TestResult where T : Debug + PartialEq {
    if a == b {
        TestResult {
            status : Pass,
            description: format!("{:?} was equal to {:?}", a, b)
        }
    } else {
        TestResult {
            status : Fail,
            description: format!("{:?} was not equal to {:?}", a, b),
        }
    }
}

pub fn neq<T>(a: T, b: T) -> TestResult where T : Debug + PartialEq {
    if a != b {
        TestResult {
            status : Pass,
            description: format!("{:?} was not equal to {:?}", a, b),
        }
    } else {
        TestResult {
            status : Fail,
            description: format!("{:?} was equal to {:?}", a, b)
        }
    }
}