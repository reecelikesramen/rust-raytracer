use std::{fmt::Display, str::FromStr};

use crate::prelude::*;

use nalgebra::{Scalar, Vector3};
use serde::{de, Deserialize, Deserializer};

impl<'de, T> Deserialize<'de> for W<Vector3<T>>
where
    T: Scalar + Copy + FromStr + Display,
    <T as FromStr>::Err: Display,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let numbers: Result<Vec<T>, _> = String::deserialize(deserializer)?
            .split_whitespace()
            .map(T::from_str)
            .collect();

        match numbers {
            Ok(nums) if nums.len() == 3 => Ok(W(Vector3::new(nums[0], nums[1], nums[2]))),
            Ok(_) => Err(de::Error::custom(
                "expected exactly 3 space-separated numbers",
            )),
            Err(e) => Err(de::Error::custom(format!("failed to parse number: {}", e))),
        }
    }
}
