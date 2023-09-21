use base64::Engine;
use chrono::Utc;
use rand::Rng;

#[cfg(feature = "serde")]
mod serde;

#[cfg(feature = "sqlx")]
mod sqlx;

pub(crate) type Inner = u128;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(Inner);

impl Id {
    pub const TIMESTAMP_BITS: usize = 42;
    pub const OTHER_BITS: usize = 8 * std::mem::size_of::<Inner>() - Self::TIMESTAMP_BITS;

    pub fn new() -> Self {
        let timestamp_ms = Utc::now().timestamp_millis() as u128;

        let random_bits: u128 = rand::thread_rng().gen();

        Self(timestamp_ms << Self::OTHER_BITS | random_bits >> Self::TIMESTAMP_BITS)
    }

    fn to_base64(self) -> String {
        base64::prelude::BASE64_URL_SAFE_NO_PAD.encode(&self.0.to_be_bytes())
    }

    fn from_base64(s: &str) -> anyhow::Result<Self> {
        Ok(Self(Inner::from_be_bytes(
            base64::prelude::BASE64_URL_SAFE_NO_PAD
                .decode(s)?
                .try_into()
                .map_err(|_| anyhow::anyhow!("failed to decode \"{s}\" as base64 u128"))?,
        )))
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_base64())
    }
}

impl std::str::FromStr for Id {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_base64(s)
    }
}
