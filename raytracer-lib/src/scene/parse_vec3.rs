use std::{fmt, marker::PhantomData, str::FromStr};

use crate::prelude::*;

use nalgebra::{coordinates::XYZ, Scalar, Vector3};
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

impl<'de, T> Deserialize<'de> for W<Vector3<T>>
where
    T: Scalar + FromStr + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TVisitor<T>(PhantomData<T>);

        impl<'de, T> Visitor<'de> for TVisitor<T>
        where
            T: Scalar + FromStr + Deserialize<'de>,
        {
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str(
                    "string like \"1.0 2.0 -3\" or array of numbers like [1.0, 2.0, -3.0]",
                )
            }

            fn visit_string<E: serde::de::Error>(self, value: String) -> Result<W<Vector3<T>>, E> {
                let numbers: Result<Vec<T>, _> =
                    value.split_whitespace().map(T::from_str).collect();

                match numbers {
                    Ok(nums) if nums.len() == 3 => Ok(W(Vector3::new(
                        nums[0].clone(),
                        nums[1].clone(),
                        nums[2].clone(),
                    ))),
                    Ok(_) => Err(de::Error::custom(
                        "expected exactly 3 space-separated numbers",
                    )),
                    Err(_) => Err(de::Error::custom("failed to parse number in vector string")),
                }
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                self.visit_string(v.to_string())
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let mut numbers: Vec<T> = Vec::new();

                while let Some(value) = seq.next_element()? {
                    numbers.push(value);
                }

                if numbers.len() != 3 {
                    return Err(de::Error::custom(
                        "array must have expected exactly 3 numbers",
                    ));
                }

                Ok(W(Vector3::new(
                    numbers[0].clone(),
                    numbers[1].clone(),
                    numbers[2].clone(),
                )))
            }

            type Value = W<Vector3<T>>;
        }

        let visitor = TVisitor::<T>(PhantomData);

        deserializer.deserialize_any(visitor)
    }
}

impl<T> Serialize for W<Vector3<T>>
where
    T: Scalar + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let vec = &self.0 as &XYZ<T>;
        let array = [vec.x.clone(), vec.y.clone(), vec.z.clone()];
        array.serialize(serializer)
    }
}
