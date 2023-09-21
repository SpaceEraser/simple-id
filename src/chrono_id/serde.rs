use super::Id;

impl serde::Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_base64())
    }
}

impl<'a> serde::Deserialize<'a> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        Ok(
            Self::from_base64(<&str as serde::Deserialize>::deserialize(deserializer)?)
                .map_err(|e| serde::de::Error::custom(e.to_string()))?,
        )
    }
}
