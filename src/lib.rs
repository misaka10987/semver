#[cfg(feature = "serde")]
mod serde;

use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use interva::Interval;

#[derive(Clone, Copy)]
pub struct SemVer(pub i16, pub i16, pub i16);

impl PartialEq for SemVer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

impl Eq for SemVer {}

impl PartialOrd for SemVer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.0.partial_cmp(&other.0) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.1.partial_cmp(&other.1) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.2.partial_cmp(&other.2)
    }
}

impl Ord for SemVer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Display for SemVer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let SemVer(x, y, z) = self;
        write!(f, "{x}.{y}.{z}")
    }
}

impl Debug for SemVer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SemVer").field(&format!("{self}")).finish()
    }
}

impl FromStr for SemVer {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<&str> = s.split('.').collect();
        let len = s.len();
        if len > 3 || len < 1 {
            return Err(format!("invalid length of {len}, [1,3] expected"));
        }
        let mut v = vec![];
        for s in s {
            let x: i16 = s
                .parse()
                .map_err(|_| format!("invalid syntax: {s}, int.int.int expected"))?;
            v.push(x);
        }
        while v.len() < 3 {
            v.push(0);
        }
        Ok(SemVer(v[0], v[1], v[2]))
    }
}

pub type ReqVer = Interval<SemVer>;
