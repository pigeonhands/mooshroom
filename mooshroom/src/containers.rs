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
    fn read(reader: impl std::io::Read) -> mooshroom_core::error::Result<Self> {
        let s = <String as MooshroomReadable<PV>>::read(reader)?;
        serde_json::from_str(&s)
            .map_err(|e| mooshroom_core::error::MoshroomError::InvalidJson(e.to_string()))
    }
}

impl<const PV: usize, T> MooshroomWritable<PV> for Json<T>
where
    T: Serialize,
{
    fn write(&self, writer: impl std::io::Write) -> mooshroom_core::error::Result<()> {
        let s = serde_json::to_string(&self)
            .map_err(|e| mooshroom_core::error::MoshroomError::InvalidJson(e.to_string()))?;
        <String as MooshroomWritable<PV>>::write(&s, writer)
    }
}

// Tagged option
#[derive(Debug, Clone, Default)]
pub struct TOption<T>(Option<T>);

impl<T> From<Option<T>> for TOption<T> {
    fn from(i: Option<T>) -> Self {
        Self(i)
    }
}

impl<T> Into<Option<T>> for TOption<T> {
    fn into(self) -> Option<T> {
        self.0
    }
}

impl<T> AsRef<Option<T>> for TOption<T> {
    fn as_ref(&self) -> &Option<T> {
        &self.0
    }
}

impl<T> AsMut<Option<T>> for TOption<T> {
    fn as_mut(&mut self) -> &mut Option<T> {
        &mut self.0
    }
}

impl<T> Deref for TOption<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for TOption<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const PV: usize, T> MooshroomReadable<PV> for TOption<T>
where
    T: MooshroomReadable<PV>,
{
    fn read(mut reader: impl std::io::Read) -> mooshroom_core::error::Result<Self> {
        if <bool as MooshroomReadable<PV>>::read(&mut reader)? {
            Ok(Self(Some(T::read(reader)?)))
        } else {
            Ok(Self(None))
        }
    }
}

impl<const PV: usize, T> MooshroomWritable<PV> for TOption<T>
where
    T: MooshroomWritable<PV>,
{
    fn write(&self, mut writer: impl std::io::Write) -> mooshroom_core::error::Result<()> {
        if let Some(t) = &self.0 {
            <bool as MooshroomWritable<PV>>::write(&true, &mut writer)?;
            t.write(writer)?;
        } else {
            <bool as MooshroomWritable<PV>>::write(&false, &mut writer)?;
        }
        Ok(())
    }
}
