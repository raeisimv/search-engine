pub type MyError = Box<dyn std::error::Error>;
pub type MyResult<T = (), E = MyError> = Result<T, E>;
