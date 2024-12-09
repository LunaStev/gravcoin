use byteorder::{LittleEndian, BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Read, Write};

pub fn read_le16(ptr: &[u8]) -> u16 {
    let mut rdr = Cursor::new(ptr);
    rdr.read_u16::<LittleEndian>().unwrap()
}

pub fn read_le32(ptr: &[u8]) -> u32 {
    let mut rdr = Cursor::new(ptr);
    rdr.read_u32::<LittleEndian>().unwrap()
}

pub fn read_le64(ptr: &[u8]) -> u64 {
    let mut rdr = Cursor::new(ptr);
    rdr.read_u64::<LittleEndian>().unwrap()
}

pub fn write_le16(ptr: &mut [u8], x: u16) {
    let mut wtr = Cursor::new(ptr);
    wtr.write_u16::<LittleEndian>(x).unwrap();
}

pub fn write_le32(ptr: &mut [u8], x: u32) {
    let mut wtr = Cursor::new(ptr);
    wtr.write_u32::<LittleEndian>(x).unwrap();
}

pub fn write_le64(ptr: &mut [u8], x: u64) {
    let mut wtr = Cursor::new(ptr);
    wtr.write_u64::<LittleEndian>(x).unwrap();
}

pub fn read_be16(ptr: &[u8]) -> u16 {
    let mut rdr = Cursor::new(ptr);
    rdr.read_u16::<BigEndian>().unwrap()
}

pub fn read_be32(ptr: &[u8]) -> u32 {
    let mut rdr = Cursor::new(ptr);
    rdr.read_u32::<BigEndian>().unwrap()
}

pub fn read_be64(ptr: &[u8]) -> u64 {
    let mut rdr = Cursor::new(ptr);
    rdr.read_u64::<BigEndian>().unwrap()
}

pub fn write_be32(ptr: &mut [u8], x: u32) {
    let mut wtr = Cursor::new(ptr);
    wtr.write_u32::<BigEndian>(x).unwrap();
}

pub fn write_be64(ptr: &mut [u8], x: u64) {
    let mut wtr = Cursor::new(ptr);
    wtr.write_u64::<BigEndian>(x).unwrap();
}
