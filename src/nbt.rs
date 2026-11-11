//! A stand-in for the parts of `horizon_world::nbt` the logic uses.
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JavaString(pub Vec<u16>);

impl From<&str> for JavaString {
    fn from(value: &str) -> Self {
        Self(value.encode_utf16().collect())
    }
}

impl JavaString {
    pub fn to_string_lossy(&self) -> String {
        String::from_utf16_lossy(&self.0)
    }
}

pub type Compound = BTreeMap<JavaString, Tag>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Tag {
    Byte(i8),
    Short(i16),
    Int(i32),
    String(JavaString),
    List(Vec<Tag>),
    Compound(Compound),
}

impl Tag {
    pub fn get(&self, key: &str) -> Option<&Tag> {
        match self {
            Self::Compound(map) => map.get(&JavaString::from(key)),
            _ => None,
        }
    }
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Tag> {
        match self {
            Self::Compound(map) => map.get_mut(&JavaString::from(key)),
            _ => None,
        }
    }
    pub fn compound(entries: impl IntoIterator<Item = (impl Into<JavaString>, Tag)>) -> Self {
        Self::Compound(entries.into_iter().map(|(k, v)| (k.into(), v)).collect())
    }
}
