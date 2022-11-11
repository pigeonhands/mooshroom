use std::ops::{Deref, DerefMut};

use mooshroom_core::io::{MooshroomReadable, MooshroomWritable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Json<T> {
    #[serde(flatten)]
    data: T,
}

impl<T> Default for Json<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

impl<T> AsRef<T> for Json<T> {
    fn as_ref(&self) -> &T {
        &self.data
    }
}

impl<T> AsMut<T> for Json<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

impl<T> Deref for Json<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for Json<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T> PartialEq<T> for Json<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        self.data == *other
    }
}

impl<T> PartialOrd<T> for Json<T>
where
    T: PartialOrd<T>,
{
    fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> {
        self.data.partial_cmp(other)
    }
}

impl<T> Json<T> {
    pub fn new(v: T) -> Self {
        Self { data: v }
    }

    pub fn into_inner(self) -> T {
        self.data
    }
}

impl<const PV: usize, T> MooshroomReadable<PV> for Json<T>
where
    T: for<'de> Deserialize<'de>,
{
    fn read(reader: &mut impl std::io::Read) -> mooshroom_core::error::Result<Self> {
        let s = <String as MooshroomReadable<PV>>::read(reader)?;
        serde_json::from_str(&s)
            .map_err(|e| mooshroom_core::error::MoshroomError::InvalidJson(e.to_string()))
    }
}

impl<const PV: usize, T> MooshroomWritable<PV> for Json<T>
where
    T: Serialize,
{
    fn write(&self, writer: &mut impl std::io::Write) -> mooshroom_core::error::Result<()> {
        let s = serde_json::to_string(&self)
            .map_err(|e| mooshroom_core::error::MoshroomError::InvalidJson(e.to_string()))?;
        <String as MooshroomWritable<PV>>::write(&s, writer)
    }
}
