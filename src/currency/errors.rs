use exonum::blockchain::ExecutionError;

#[derive(Debug, Fail)]
#[repr(u8)]
pub enum Error {
    #[fail(display = "Wallet already exists")]
    WalletAlreadyExists = 0,

    #[fail(display = "Sender doesn't exist")]
    SenderNotFound = 1,

    #[fail(display = "Receiver doesn't exist")]
    ReceiverNotFound = 2,

    #[fail(display = "Insufficient currency amount")]
    InsufficientCurrencyAmount = 3,
}

// Conversion between service-specific errors and the standard error type
// that can be emitted by transactions.
impl From<Error> for ExecutionError {
    fn from(value: Error) -> ExecutionError {
        let description = format!("{}", value);
        ExecutionError::with_description(value as u8, description)
    }
}
