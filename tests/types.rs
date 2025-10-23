use mc_server_rs::types::{Type, VarInt, VarLong};
use rstest::{fixture, rstest};

#[fixture]
fn buffer() -> Vec<u8> {
    Vec::new()
}

struct SampleData<T> {
    value: T,
    bytes: Vec<u8>,
}

impl<T> SampleData<T> {
    fn new(value: T, bytes: Vec<u8>) -> Self {
        Self { value, bytes }
    }
}

// Sample VarInts and VarLongs from Mincecraft Wiki (https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Protocol?oldid=2938097#VarInt_and_VarLong)

#[fixture]
fn sample_var_int_data() -> Vec<SampleData<VarInt>> {
    vec![
        SampleData::new(VarInt::new(0), vec![0x00]),
        SampleData::new(VarInt::new(1), vec![0x01]),
        SampleData::new(VarInt::new(2), vec![0x02]),
        SampleData::new(VarInt::new(127), vec![0x7f]),
        SampleData::new(VarInt::new(128), vec![0x80, 0x01]),
        SampleData::new(VarInt::new(255), vec![0xff, 0x01]),
        SampleData::new(VarInt::new(25565), vec![0xdd, 0xc7, 0x01]),
        SampleData::new(VarInt::new(2097151), vec![0xff, 0xff, 0x7f]),
        SampleData::new(VarInt::new(2147483647), vec![0xff, 0xff, 0xff, 0xff, 0x07]),
        SampleData::new(VarInt::new(-1), vec![0xff, 0xff, 0xff, 0xff, 0x0f]),
        SampleData::new(VarInt::new(-2147483648), vec![0x80, 0x80, 0x80, 0x80, 0x08]),
    ]
}

#[fixture]
fn sample_var_long_data() -> Vec<SampleData<VarLong>> {
    vec![
        SampleData::new(VarLong::new(0), vec![0x00]),
        SampleData::new(VarLong::new(1), vec![0x01]),
        SampleData::new(VarLong::new(2), vec![0x02]),
        SampleData::new(VarLong::new(127), vec![0x7f]),
        SampleData::new(VarLong::new(128), vec![0x80, 0x01]),
        SampleData::new(VarLong::new(255), vec![0xff, 0x01]),
        SampleData::new(VarLong::new(2147483647), vec![0xff, 0xff, 0xff, 0xff, 0x07]),
        SampleData::new(
            VarLong::new(9223372036854775807),
            vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f],
        ),
        SampleData::new(
            VarLong::new(-1),
            vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01],
        ),
        SampleData::new(
            VarLong::new(-2147483648),
            vec![0x80, 0x80, 0x80, 0x80, 0xf8, 0xff, 0xff, 0xff, 0xff, 0x01],
        ),
        SampleData::new(
            VarLong::new(-9223372036854775808),
            vec![0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01],
        ),
    ]
}

#[rstest]
fn write_var_int(mut buffer: Vec<u8>, sample_var_int_data: Vec<SampleData<VarInt>>) {
    for data in sample_var_int_data {
        buffer.clear();
        dbg!(data.value);
        data.value.write(&mut buffer).unwrap();
        assert_eq!(data.bytes, buffer);
    }
}

#[rstest]
fn read_var_int(sample_var_int_data: Vec<SampleData<VarInt>>) {
    for data in sample_var_int_data {
        let value = VarInt::read(data.bytes.as_slice()).unwrap();
        assert_eq!(data.value, value);
    }
}

#[rstest]
fn write_var_long(mut buffer: Vec<u8>, sample_var_long_data: Vec<SampleData<VarLong>>) {
    for data in sample_var_long_data {
        buffer.clear();
        dbg!(data.value);
        data.value.write(&mut buffer).unwrap();
        assert_eq!(data.bytes, buffer);
    }
}

#[rstest]
fn read_var_long(sample_var_long_data: Vec<SampleData<VarLong>>) {
    for data in sample_var_long_data {
        let value = VarLong::read(data.bytes.as_slice()).unwrap();
        assert_eq!(data.value, value);
    }
}

#[test]
fn string() {
    let value = "Hello, world!".to_string();
    let mut buffer = Vec::new();
    value.write(&mut buffer).unwrap();
    let read = String::read(buffer.as_slice()).unwrap();
    assert_eq!(value, read);
}
