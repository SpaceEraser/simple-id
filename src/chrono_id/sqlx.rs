use sqlx::{
    any::{Any, AnyTypeInfo, AnyTypeInfoKind},
    encode::IsNull,
    error::BoxDynError,
    Database, Decode, Encode, Type,
};

use super::Id;

impl<'q> Encode<'q, Any> for Id {
    fn encode_by_ref(
        &self,
        buf: &mut <Any as Database>::ArgumentBuffer<'q>,
    ) -> Result<IsNull, BoxDynError> {
        Encode::<Any>::encode(self.0 as i64, buf)
    }
}

impl<'r> Decode<'r, Any> for Id {
    fn decode(value: <Any as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        Ok(Self(<i64 as Decode<Any>>::decode(value)? as _))
    }
}

impl Type<Any> for Id {
    fn type_info() -> AnyTypeInfo {
        AnyTypeInfo {
            kind: AnyTypeInfoKind::BigInt,
        }
    }

    fn compatible(ty: &AnyTypeInfo) -> bool {
        ty.kind().is_integer()
    }
}
