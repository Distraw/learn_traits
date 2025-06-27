use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SaturatingU16 {
    value: u16
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        SaturatingU16 {value: value.into()}
    }
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        SaturatingU16 {value: value.into()}
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        SaturatingU16 {value: value.clone().into()}
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        SaturatingU16 {value: value.clone().into()}
    }
}

impl Add for SaturatingU16 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            value: self.value.saturating_add(rhs.value)
        }
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self::Output {
        Self {
            value: self.value.saturating_add(rhs.value),
        }
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = Self;
    fn add(self, rhs: u16) -> Self::Output {
        Self {
            value: self.value.saturating_add(rhs),
        }
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = Self;
    fn add(self, rhs: &u16) -> Self::Output {
        Self {
            value: self.value.saturating_add(*rhs),
        }
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}