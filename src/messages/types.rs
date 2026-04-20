use crate::messages::error::Sv2MessageError;
use std::convert::{TryFrom, TryInto};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sv2U256(Vec<u8>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sv2B032(Vec<u8>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sv2B0255(Vec<u8>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sv2B064K(Vec<u8>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sv2B016M(Vec<u8>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sv2Seq0255U256(Vec<Vec<u8>>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sv2Seq064KU256(Vec<Vec<u8>>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sv2Seq064KB016M(Vec<Vec<u8>>);

uniffi::custom_type!(Sv2U256, Vec<u8>);
uniffi::custom_type!(Sv2B032, Vec<u8>);
uniffi::custom_type!(Sv2B0255, Vec<u8>);
uniffi::custom_type!(Sv2B064K, Vec<u8>);
uniffi::custom_type!(Sv2B016M, Vec<u8>);
uniffi::custom_type!(Sv2Seq0255U256, Vec<Vec<u8>>);
uniffi::custom_type!(Sv2Seq064KU256, Vec<Vec<u8>>);
uniffi::custom_type!(Sv2Seq064KB016M, Vec<Vec<u8>>);

impl Sv2U256 {
    pub(crate) fn into_bytes(self) -> Vec<u8> {
        self.0
    }
}

impl Sv2B032 {
    pub(crate) fn into_bytes(self) -> Vec<u8> {
        self.0
    }
}

impl Sv2B0255 {
    pub(crate) fn into_bytes(self) -> Vec<u8> {
        self.0
    }
}

impl Sv2B064K {
    pub(crate) fn into_bytes(self) -> Vec<u8> {
        self.0
    }
}

impl Sv2Seq0255U256 {
    pub(crate) fn into_inner(self) -> Vec<Vec<u8>> {
        self.0
    }
}

impl Sv2Seq064KU256 {
    pub(crate) fn into_inner(self) -> Vec<Vec<u8>> {
        self.0
    }
}

impl Sv2Seq064KB016M {
    pub(crate) fn into_inner(self) -> Vec<Vec<u8>> {
        self.0
    }
}

impl From<Sv2U256> for Vec<u8> {
    fn from(value: Sv2U256) -> Self {
        value.0
    }
}

impl From<Sv2B032> for Vec<u8> {
    fn from(value: Sv2B032) -> Self {
        value.0
    }
}

impl From<Sv2B0255> for Vec<u8> {
    fn from(value: Sv2B0255) -> Self {
        value.0
    }
}

impl From<Sv2B064K> for Vec<u8> {
    fn from(value: Sv2B064K) -> Self {
        value.0
    }
}

impl From<Sv2B016M> for Vec<u8> {
    fn from(value: Sv2B016M) -> Self {
        value.0
    }
}

impl From<Sv2Seq0255U256> for Vec<Vec<u8>> {
    fn from(value: Sv2Seq0255U256) -> Self {
        value.0
    }
}

impl From<Sv2Seq064KU256> for Vec<Vec<u8>> {
    fn from(value: Sv2Seq064KU256) -> Self {
        value.0
    }
}

impl From<Sv2Seq064KB016M> for Vec<Vec<u8>> {
    fn from(value: Sv2Seq064KB016M) -> Self {
        value.0
    }
}

impl TryFrom<Vec<u8>> for Sv2U256 {
    type Error = Sv2MessageError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        validate_exact(&value, 32)?;
        Ok(Self(value))
    }
}

impl TryFrom<Vec<u8>> for Sv2B032 {
    type Error = Sv2MessageError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        validate_max(&value, 32)?;
        Ok(Self(value))
    }
}

impl TryFrom<Vec<u8>> for Sv2B0255 {
    type Error = Sv2MessageError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        validate_max(&value, 255)?;
        Ok(Self(value))
    }
}

impl TryFrom<Vec<u8>> for Sv2B064K {
    type Error = Sv2MessageError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        validate_max(&value, u16::MAX as usize)?;
        Ok(Self(value))
    }
}

impl TryFrom<Vec<u8>> for Sv2B016M {
    type Error = Sv2MessageError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        validate_max(&value, 2_usize.pow(24) - 1)?;
        Ok(Self(value))
    }
}

impl TryFrom<Vec<Vec<u8>>> for Sv2Seq0255U256 {
    type Error = Sv2MessageError;

    fn try_from(value: Vec<Vec<u8>>) -> Result<Self, Self::Error> {
        validate_sequence_max(&value, 255)?;
        validate_items_exact(&value, 32)?;
        Ok(Self(value))
    }
}

impl TryFrom<Vec<Vec<u8>>> for Sv2Seq064KU256 {
    type Error = Sv2MessageError;

    fn try_from(value: Vec<Vec<u8>>) -> Result<Self, Self::Error> {
        validate_sequence_max(&value, u16::MAX as usize)?;
        validate_items_exact(&value, 32)?;
        Ok(Self(value))
    }
}

impl TryFrom<Vec<Vec<u8>>> for Sv2Seq064KB016M {
    type Error = Sv2MessageError;

    fn try_from(value: Vec<Vec<u8>>) -> Result<Self, Self::Error> {
        validate_sequence_max(&value, u16::MAX as usize)?;
        validate_items_max(&value, 2_usize.pow(24) - 1)?;
        Ok(Self(value))
    }
}

fn validate_exact(value: &[u8], expected: usize) -> Result<(), Sv2MessageError> {
    if value.len() == expected {
        Ok(())
    } else {
        Err(Sv2MessageError::ByteArrayLengthMismatch)
    }
}

fn validate_max(value: &[u8], max: usize) -> Result<(), Sv2MessageError> {
    if value.len() <= max {
        Ok(())
    } else {
        Err(Sv2MessageError::ByteArrayTooLong)
    }
}

fn validate_sequence_max<T>(value: &[T], max: usize) -> Result<(), Sv2MessageError> {
    if value.len() <= max {
        Ok(())
    } else {
        Err(Sv2MessageError::SequenceTooLong)
    }
}

fn validate_items_exact(value: &[Vec<u8>], expected: usize) -> Result<(), Sv2MessageError> {
    if value.iter().all(|item| item.len() == expected) {
        Ok(())
    } else {
        Err(Sv2MessageError::SequenceItemLengthMismatch)
    }
}

fn validate_items_max(value: &[Vec<u8>], max: usize) -> Result<(), Sv2MessageError> {
    if value.iter().all(|item| item.len() <= max) {
        Ok(())
    } else {
        Err(Sv2MessageError::SequenceItemTooLong)
    }
}
