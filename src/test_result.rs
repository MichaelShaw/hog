use self::Status::{Pass, Fail};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Status {
    Pass,
    Fail,
}

#[derive(Clone, Debug)]
pub struct TestResult {
    pub status: Status,
    pub description: String,
}

impl TestResult {
    pub fn is_pass(&self) -> bool {
        self.status == Pass
    }

    pub fn is_failure(&self) -> bool {
        self.status == Fail
    }

    pub fn from_bool(b: bool) -> TestResult {
        TestResult {
            status: if b { Pass } else { Fail },
            description: if b { "Boolean success".into() } else { "Boolean failure".into() },
        }
    }
}

pub trait Testable : Send + 'static {
    fn result(&self) -> TestResult;
}

impl Testable for bool {
    fn result(&self) -> TestResult {
        TestResult::from_bool(*self)
    }
}

impl Testable for () {
    fn result(&self) -> TestResult {
        TestResult { status: Pass, description: "Unit Pass".into() }
    }
}

impl Testable for TestResult {
    fn result(&self) -> TestResult { self.clone() }
}
//
//impl<A, E> Testable for Result<A, E>
//    where A: Testable, E: Debug + Send + 'static {
//    fn result(&self) -> TestResult {
//        match *self {
//            Ok(ref r) => r.result(g),
//            Err(ref err) => TestResult {
//                status: Fail,
//                description: format!("{:?}", err),
//            },
//        }
//    }
//}