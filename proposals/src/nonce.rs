use core::ops::{Add, Mul};
use num_traits::{Bounded, CheckedMul, One, Saturating, Zero};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

/// Proposal Nonce (4 bytes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(
    feature = "scale",
    derive(scale_info::TypeInfo, scale_codec::Encode, scale_codec::Decode)
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Nonce(pub u32);

impl Add for Nonce {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Nonce(self.0 + rhs.0)
    }
}
impl Mul for Nonce {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Nonce(self.0 * rhs.0)
    }
}

impl CheckedMul for Nonce {
    fn checked_mul(&self, v: &Self) -> Option<Self> {
        self.0.checked_mul(v.0).map(Nonce)
    }
}

impl Zero for Nonce {
    fn zero() -> Self {
        Nonce(0)
    }

    fn is_zero(&self) -> bool {
        self.0 == 0
    }

    fn set_zero(&mut self) {
        *self = Zero::zero();
    }
}

impl One for Nonce {
    fn one() -> Self {
        Nonce(1)
    }
}

impl Saturating for Nonce {
    fn saturating_add(self, v: Self) -> Self {
        self.0.saturating_add(v.0).into()
    }

    fn saturating_sub(self, v: Self) -> Self {
        self.0.saturating_sub(v.0).into()
    }
}

impl Bounded for Nonce {
    fn min_value() -> Self {
        Nonce(u32::min_value())
    }

    fn max_value() -> Self {
        Nonce(u32::max_value())
    }
}
