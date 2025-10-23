use derive_more::{AsMut, AsRef, Deref, DerefMut};
use std::io::{Read, Write};

pub trait Type: Sized {
    fn write(&self, writer: impl Write) -> std::io::Result<()>;

    fn read(reader: impl Read) -> std::io::Result<Self>;
}

impl Type for bool {
    fn write(&self, mut writer: impl Write) -> std::io::Result<()> {
        writer.write_all(&[*self as u8])
    }

    fn read(mut reader: impl Read) -> std::io::Result<Self> {
        let mut buffer = [0; 1];
        reader.read_exact(&mut buffer)?;

        Ok(buffer[0] != 0)
    }
}

impl Type for i8 {
    fn write(&self, mut writer: impl Write) -> std::io::Result<()> {
        writer.write_all(&[*self as u8])
    }

    fn read(mut reader: impl Read) -> std::io::Result<Self> {
        let mut buffer = [0; 1];
        reader.read_exact(&mut buffer)?;

        Ok(buffer[0] as i8)
    }
}

impl Type for u8 {
    fn write(&self, mut writer: impl Write) -> std::io::Result<()> {
        writer.write_all(&[*self])
    }

    fn read(mut reader: impl Read) -> std::io::Result<Self> {
        let mut buffer = [0; 1];
        reader.read_exact(&mut buffer)?;

        Ok(buffer[0])
    }
}

macro_rules! impl_type {
    ($($t:ty),* $(,)?) => {
        $(impl Type for $t {
            fn write(&self, mut writer: impl Write) -> std::io::Result<()> {
                writer.write_all(&self.to_be_bytes())
            }

            fn read(mut reader: impl Read) -> std::io::Result<Self> {
                let mut buffer = [0; std::mem::size_of::<$t>()];
                reader.read_exact(&mut buffer)?;

                Ok(Self::from_be_bytes(buffer))
            }
        })*
    };
}

impl_type!(i16, u16, i32, i64, f32, f64,);

#[repr(transparent)]
#[derive(Deref, DerefMut, AsRef, AsMut, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VarInt(i32);

impl std::fmt::Debug for VarInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::fmt::Display for VarInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl VarInt {
    pub fn new(value: i32) -> Self {
        Self(value)
    }
}

#[repr(transparent)]
#[derive(Deref, DerefMut, AsRef, AsMut, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VarLong(i64);

impl std::fmt::Debug for VarLong {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::fmt::Display for VarLong {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl VarLong {
    pub fn new(value: i64) -> Self {
        Self(value)
    }
}

const SEGMENT_BITS: u8 = 0x7f;
const CONTINUE_BIT: u8 = 0x80;

macro_rules! impl_var_int {
    ($($struct:ty, $primitive:ty, $inner:ty);* $(;)?) => {
        $(impl Type for $struct {
            fn write(&self, mut writer: impl Write) -> std::io::Result<()> {
                let mut value = **self as $primitive;

                loop {
                    if value & !(SEGMENT_BITS as $primitive) == 0 {
                        writer.write_all(&[value as u8])?;
                        return Ok(());
                    }

                    writer.write_all(&[(value as u8 & SEGMENT_BITS) | CONTINUE_BIT])?;

                    value >>= 7;
                }
            }

            fn read(mut reader: impl Read) -> std::io::Result<Self> {
                const NAME: &str = if std::mem::size_of::<$primitive>() == 4 {"VarInt"} else {"VarLong"};

                let mut value = 0;
                let mut shift = 0;
                let mut current_byte = [0; 1];

                loop {
                    reader.read_exact(&mut current_byte)?;
                    value |= (current_byte[0] & SEGMENT_BITS) as $primitive << shift;

                    if current_byte[0] & CONTINUE_BIT == 0 {break;}

                    shift += 7;

                    if shift >= std::mem::size_of::<$primitive>() * 8 {
                        return Err(std::io::Error::new(std::io::ErrorKind::FileTooLarge, format!("{NAME} is too large")));
                    }
                }

                Ok(Self(value as $inner))
            }
        })*
    }
}

impl_var_int! {
    VarInt, u32, i32;
    VarLong, u64, i64
}

impl Type for String {
    fn write(&self, mut writer: impl Write) -> std::io::Result<()> {
        let bytes = self.as_bytes();
        VarInt(
            bytes
                .len()
                .try_into()
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::FileTooLarge, e))?,
        )
        .write(&mut writer)?;
        writer.write_all(bytes)
    }

    fn read(mut reader: impl Read) -> std::io::Result<Self> {
        let length = VarInt::read(&mut reader)?;
        let mut buffer = vec![0; length.0 as usize];
        reader.read_exact(&mut buffer)?;

        String::from_utf8(buffer)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }
}
