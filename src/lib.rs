// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2022 Oxide Computer Company

mod de;
mod error;
mod ser;

pub use de::{from_bytes, from_bytes_le, Deserializer};
pub use error::{Error, Result};
pub use ser::{to_bytes, to_bytes_le, Serializer};

pub struct LittleEndian {}

pub mod str_lv8 {
    use serde::ser::SerializeTuple;

    pub fn serialize<S>(v: &str, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut t = s.serialize_tuple(std::mem::size_of::<u8>() + v.len())?;
        t.serialize_element(&(v.len() as u8))?;
        t.serialize_element(v.as_bytes())?;
        t.end()
    }

    pub fn deserialize<'de, D>(d: D) -> Result<String, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        d.deserialize_tuple_struct("string8", 2, crate::de::TlvStringVisitor)
    }
}

pub mod str_lv16 {
    use serde::ser::SerializeTuple;

    pub fn serialize<S>(v: &str, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut t = s.serialize_tuple(std::mem::size_of::<u16>() + v.len())?;
        t.serialize_element(&(v.len() as u16))?;
        t.serialize_element(v.as_bytes())?;
        t.end()
    }

    pub fn deserialize<'de, D>(d: D) -> Result<String, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        d.deserialize_tuple_struct("string16", 2, crate::de::TlvStringVisitor)
    }
}

pub mod str_lv32 {
    use serde::ser::SerializeTuple;

    pub fn serialize<S>(v: &str, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut t = s.serialize_tuple(std::mem::size_of::<u32>() + v.len())?;
        t.serialize_element(&(v.len() as u32))?;
        t.serialize_element(v.as_bytes())?;
        t.end()
    }

    pub fn deserialize<'de, D>(d: D) -> Result<String, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        d.deserialize_tuple_struct("string32", 2, crate::de::TlvStringVisitor)
    }
}

pub mod str_lv64 {
    use serde::ser::SerializeTuple;

    pub fn serialize<S>(v: &str, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut t = s.serialize_tuple(std::mem::size_of::<u64>() + v.len())?;
        t.serialize_element(&(v.len() as u64))?;
        t.serialize_element(v.as_bytes())?;
        t.end()
    }

    pub fn deserialize<'de, D>(d: D) -> Result<String, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        d.deserialize_tuple_struct("string64", 2, crate::de::TlvStringVisitor)
    }
}

pub mod vec_lv8 {
    use serde::ser::SerializeTuple;

    pub fn serialize<S, T>(v: &Vec<T>, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
        T: serde::Serialize,
    {
        let mut t = s.serialize_tuple(std::mem::size_of::<u8>() + v.len())?;
        t.serialize_element(&(v.len() as u8))?;
        t.serialize_element(&v)?;
        t.end()
    }

    pub fn deserialize<'de, D, T>(d: D) -> Result<Vec<T>, D::Error>
    where
        D: serde::Deserializer<'de>,
        T: serde::Deserialize<'de>,
    {
        d.deserialize_tuple_struct("vec8", 2, crate::de::TlvVecVisitor::new())
    }
}

pub mod vec_lv16 {
    use serde::ser::SerializeTuple;

    pub fn serialize<S, T>(v: &Vec<T>, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
        T: serde::Serialize,
    {
        let mut t = s.serialize_tuple(std::mem::size_of::<u16>() + v.len())?;
        t.serialize_element(&(v.len() as u16))?;
        t.serialize_element(&v)?;
        t.end()
    }

    pub fn deserialize<'de, D, T>(d: D) -> Result<Vec<T>, D::Error>
    where
        D: serde::Deserializer<'de>,
        T: serde::Deserialize<'de>,
    {
        d.deserialize_tuple_struct("vec16", 2, crate::de::TlvVecVisitor::new())
    }
}

pub mod vec_lv32 {
    use serde::ser::SerializeTuple;

    pub fn serialize<S, T>(v: &Vec<T>, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
        T: serde::Serialize,
    {
        let mut t = s.serialize_tuple(std::mem::size_of::<u32>() + v.len())?;
        t.serialize_element(&(v.len() as u32))?;
        t.serialize_element(&v)?;
        t.end()
    }

    pub fn deserialize<'de, D, T>(d: D) -> Result<Vec<T>, D::Error>
    where
        D: serde::Deserializer<'de>,
        T: serde::Deserialize<'de>,
    {
        d.deserialize_tuple_struct("vec32", 2, crate::de::TlvVecVisitor::new())
    }
}

pub mod vec_lv64 {
    use serde::ser::SerializeTuple;

    pub fn serialize<S, T>(v: &Vec<T>, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
        T: serde::Serialize,
    {
        let mut t = s.serialize_tuple(std::mem::size_of::<u64>() + v.len())?;
        t.serialize_element(&(v.len() as u64))?;
        t.serialize_element(&v)?;
        t.end()
    }

    pub fn deserialize<'de, D, T>(d: D) -> Result<Vec<T>, D::Error>
    where
        D: serde::Deserializer<'de>,
        T: serde::Deserialize<'de>,
    {
        d.deserialize_tuple_struct("vec64", 2, crate::de::TlvVecVisitor::new())
    }
}

pub trait WireSize {
    fn wire_size(&self) -> usize;
}

pub mod vec_lv8b {
    use serde::ser::SerializeTuple;

    pub fn serialize<S, T>(v: &Vec<T>, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
        T: serde::Serialize + crate::WireSize,
    {
        let mut sz = 0usize;
        for e in v {
            sz += e.wire_size();
        }
        let mut t = s.serialize_tuple(std::mem::size_of::<u8>() + v.len())?;
        t.serialize_element(&(sz as u8))?;
        t.serialize_element(&v)?;
        t.end()
    }

    pub fn deserialize<'de, D, T>(d: D) -> Result<Vec<T>, D::Error>
    where
        D: serde::Deserializer<'de>,
        T: serde::Deserialize<'de>,
    {
        d.deserialize_tuple_struct("vec8b", 2, crate::de::TlvVecVisitor::new())
    }
}

pub mod vec_lv16b {
    use serde::ser::SerializeTuple;

    pub fn serialize<S, T>(v: &Vec<T>, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
        T: serde::Serialize + crate::WireSize,
    {
        let mut sz = 0usize;
        for e in v {
            sz += e.wire_size();
        }
        let mut t = s.serialize_tuple(std::mem::size_of::<u16>() + v.len())?;
        t.serialize_element(&(sz as u16))?;
        t.serialize_element(&v)?;
        t.end()
    }

    pub fn deserialize<'de, D, T>(d: D) -> Result<Vec<T>, D::Error>
    where
        D: serde::Deserializer<'de>,
        T: serde::Deserialize<'de>,
    {
        d.deserialize_tuple_struct("vec16b", 2, crate::de::TlvVecVisitor::new())
    }
}

pub mod vec_lv32b {
    use serde::ser::SerializeTuple;

    pub fn serialize<S, T>(v: &Vec<T>, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
        T: serde::Serialize + crate::WireSize,
    {
        let mut sz = 0usize;
        for e in v {
            sz += e.wire_size();
        }
        let mut t = s.serialize_tuple(std::mem::size_of::<u32>() + v.len())?;
        t.serialize_element(&(sz as u32))?;
        t.serialize_element(&v)?;
        t.end()
    }

    pub fn deserialize<'de, D, T>(d: D) -> Result<Vec<T>, D::Error>
    where
        D: serde::Deserializer<'de>,
        T: serde::Deserialize<'de>,
    {
        d.deserialize_tuple_struct("vec32b", 2, crate::de::TlvVecVisitor::new())
    }
}

pub mod vec_lv64b {
    use serde::ser::SerializeTuple;

    pub fn serialize<S, T>(v: &Vec<T>, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
        T: serde::Serialize + crate::WireSize,
    {
        let mut sz = 0usize;
        for e in v {
            sz += e.wire_size();
        }
        let mut t = s.serialize_tuple(std::mem::size_of::<u64>() + v.len())?;
        t.serialize_element(&(sz as u64))?;
        t.serialize_element(&v)?;
        t.end()
    }

    pub fn deserialize<'de, D, T>(d: D) -> Result<Vec<T>, D::Error>
    where
        D: serde::Deserializer<'de>,
        T: serde::Deserialize<'de>,
    {
        d.deserialize_tuple_struct("vec64b", 2, crate::de::TlvVecVisitor::new())
    }
}
