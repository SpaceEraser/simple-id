use sqlx::{
    sqlite::{SqliteArgumentValue, SqliteTypeInfo, SqliteValueRef},
    Decode, Encode, Sqlite, Type,
};

use super::{Id, Inner};

impl Encode<'_, Sqlite> for Id {
    fn encode(
        self,
        buf: &mut <Sqlite as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        <Vec<u8> as Encode<Sqlite>>::encode(self.0.to_be_bytes().to_vec(), buf)
    }

    fn encode_by_ref(&self, buf: &mut Vec<SqliteArgumentValue<'_>>) -> sqlx::encode::IsNull {
        <Vec<u8> as Encode<Sqlite>>::encode_by_ref(&self.0.to_be_bytes().to_vec(), buf)
    }
}

impl Decode<'_, Sqlite> for Id {
    fn decode(value: SqliteValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        let num_bytes_required = std::mem::size_of::<Inner>();
        match <&[u8] as Decode<Sqlite>>::decode(value) {
            Ok(slice) => {
                if slice.len() != num_bytes_required {
                    Err(format!("Id requires {num_bytes_required} but got {}", slice.len()).into())
                } else {
                    Ok(Self(Inner::from_be_bytes(slice.try_into().unwrap())))
                }
            }
            Err(e) => Err(e),
        }
    }
}

impl Type<Sqlite> for Id {
    fn type_info() -> SqliteTypeInfo {
        <&[u8] as Type<Sqlite>>::type_info()
    }
}
