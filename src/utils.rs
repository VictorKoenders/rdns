#[cfg(test)]
extern crate std;

pub fn to_u16(bytes: &[u8]) -> Option<u16> {
    let high = bytes.get(0)?;
    let low = bytes.get(1)?;
    Some((*high as u16) << 8 | (*low as u16))
}

pub fn from_u16(val: u16) -> [u8; 2] {
    [(val >> 8) as u8, val as u8]
}

pub fn from_u32(val: u32) -> [u8; 4] {
    [
        (val >> 24) as u8,
        (val >> 16) as u8,
        (val >> 8) as u8,
        val as u8,
    ]
}

#[test]
fn test_conversion() {
    for i in 0..=u16::max_value() {
        let buff = from_u16(i);
        let result = to_u16(&buff).unwrap();
        std::println!("{} -> {:?} -> {}", i, buff, result);
        assert_eq!(i, result);
    }
}
