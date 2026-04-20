use std::fmt::Display;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, uniffi::Error)]
pub enum Sv2MessageError {
    FailedToConvertProtocol,
    FailedToSerializeString,
    ByteArrayLengthMismatch,
    ByteArrayTooLong,
    SequenceTooLong,
    SequenceItemLengthMismatch,
    SequenceItemTooLong,
    FailedToSerializeByteArray,
}

impl Display for Sv2MessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Sv2MessageError {}
