use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::Id;

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.0)
    }
}

impl<'a> Deserialize<'a> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'a>,
    {
        Ok(Self(<u64 as Deserialize>::deserialize(deserializer)?))
    }
}
