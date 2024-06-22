extern crate serde_dep as serde;

use crate::SemVer;
use serde::{de::Visitor, Deserialize, Serialize};

impl Serialize for SemVer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{self}"))
    }
}

struct SemVerVisitor;

impl<'de> Visitor<'de> for SemVerVisitor {
    type Value = SemVer;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a semantic version number (e.g. 11.45.14)")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let s: Vec<&str> = v.split('.').collect();
        let len = s.len();
        if len != 3 {
            return Err(E::invalid_length(len, &self));
        }
        let mut err = false;
        let r: Vec<i16> = s
            .into_iter()
            .map(|s| match s.parse::<i16>() {
                Ok(x) => x,
                Err(_) => {
                    err = true;
                    0
                }
            })
            .collect();
        if err {
            return Err(E::invalid_value(serde::de::Unexpected::Str(v), &self));
        }
        Ok(SemVer(r[0], r[1], r[2]))
    }
}

impl<'de> Deserialize<'de> for SemVer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(SemVerVisitor)
    }
}
