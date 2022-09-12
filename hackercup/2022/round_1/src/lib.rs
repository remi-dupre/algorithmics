#[derive(Clone, Copy, Debug)]
pub struct WrapU64<const M: u64>(u64);

impl<const M: u64> std::str::FromStr for WrapU64<M> {
    type Err = <u64 as std::str::FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = u64::from_str(s)?;
        Ok(x.into())
    }
}

impl<const M: u64> From<u64> for WrapU64<M> {
    fn from(x: u64) -> Self {
        Self(x % M)
    }
}

impl<const M: u64> Into<u64> for WrapU64<M> {
    fn into(self) -> u64 {
        self.0
    }
}

impl<const M: u64> std::ops::Add for WrapU64<M> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self((self.0 + rhs.0) % M)
    }
}

impl<const M: u64> std::iter::Sum for WrapU64<M> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut res = 0.into();

        for x in iter {
            res = res + x;
        }

        res
    }
}

impl<const M: u64> std::ops::Sub for WrapU64<M> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self((self.0 + M - rhs.0) % M)
    }
}

impl<const M: u64> std::ops::Mul for WrapU64<M> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self((self.0 * rhs.0) % M)
    }
}
