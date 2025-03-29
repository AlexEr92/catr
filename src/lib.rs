use std::error::Error;

type GenError = Box<dyn Error>;
type GenResult<T> = Result<T, GenError>;

pub fn run() -> GenResult<()> {
    println!("Start application");
    Ok(())
}
