const M: u64 = 1_000_000_007;

#[derive(Clone, Copy, Debug)]
pub struct WrapU64(u64);

impl std::str::FromStr for WrapU64 {
    type Err = <u64 as std::str::FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = u64::from_str(s)?;
        Ok(x.into())
    }
}

impl From<u64> for WrapU64 {
    fn from(x: u64) -> Self {
        Self(x % M)
    }
}

impl Into<u64> for WrapU64 {
    fn into(self) -> u64 {
        self.0
    }
}

impl std::ops::Add for WrapU64 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self((self.0 + rhs.0) % M)
    }
}

impl std::iter::Sum for WrapU64 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut res = 0.into();

        for x in iter {
            res = res + x;
        }

        res
    }
}

impl std::ops::Sub for WrapU64 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self((self.0 + M - rhs.0) % M)
    }
}

impl std::ops::Mul for WrapU64 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self((self.0 * rhs.0) % M)
    }
}
