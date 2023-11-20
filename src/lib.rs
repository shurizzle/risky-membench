pub mod new;
pub mod old;

pub mod error {
    #[derive(Debug)]
    pub enum Error {
        InvalidOpCode,
    }
}
