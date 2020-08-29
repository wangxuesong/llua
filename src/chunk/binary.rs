use anyhow::{Result, Error};
use nom;
use nom_derive::Nom;
use nom::number::streaming::{le_f64, le_i64};

// "\x1bLua"
pub const LUA_SIGNATURE: [u8; 4] = [0x1b, 0x4c, 0x75, 0x61];
pub const LUAC_VERSION: u8 = 0x53;
pub const LUAC_FORMAT: u8 = 0;
// "\x19\x93\r\n\x1a\n"
pub const LUAC_DATA: [u8; 6] = [0x19, 0x93, 0x0d, 0x0a, 0x1a, 0x0a];
pub const CINT_SIZE: u8 = 4;
pub const CSIZET_SIZE: u8 = 8;
pub const INSTRUCTION_SIZE: u8 = 4;
pub const LUA_INTEGER_SIZE: u8 = 8;
pub const LUA_NUMBER_SIZE: u8 = 8;
pub const LUAC_INT: i64 = 0x5678;
pub const LUAC_NUM: f64 = 370.5;

pub const TAG_NIL: u8 = 0x00;
pub const TAG_BOOLEAN: u8 = 0x01;
pub const TAG_NUMBER: u8 = 0x03;
pub const TAG_INTEGER: u8 = 0x13;
pub const TAG_SHORT_STR: u8 = 0x04;
pub const TAG_LONG_STR: u8 = 0x14;

#[derive(Nom)]
struct Header {
    #[nom(Count = "4")]
    signature: Vec<u8>,
    version: u8,
    format: u8,
    #[nom(Count = "6")]
    luac_data: Vec<u8>,
    c_int_size: u8,
    c_size_t_size: u8,
    instruction_size: u8,
    lua_integer_size: u8,
    lua_number_size: u8,
    #[nom(Parse = "le_i64")]
    luac_int: i64,
    #[nom(Parse = "le_f64")]
    luac_num: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::bytes::complete::tag;
    use nom::IResult;
    use nom::Err;
    use std::fs;

    fn parser(s: &[u8]) -> IResult<&[u8], &[u8]> {
        tag(LUA_SIGNATURE)(s)
    }

    fn read(name: &str) -> std::io::Result<Vec<u8>> {
        fs::read(name)
    }

    #[test]
    fn parse_magic() {
        const MAGIC: &[u8] = &[0x65, 0x51, 0x48, 0x54, 0x52];
        let expect: Vec<u8> = vec![0x1b, 0x4c, 0x75, 0x61];
        let mut data: Vec<u8> = expect.clone();
        data.append(MAGIC.clone().to_vec().as_mut());
        let magic_parser = parser(data.as_slice());
        assert_eq!(magic_parser, Ok((MAGIC, expect.as_slice())));
    }

    #[test]
    fn parse_hello() {
        let name = "luac.out";
        let content = read(name).unwrap();
        let parse_result = Header::parse(content.as_slice());
        assert!(parse_result.is_ok());
        let head: Header = parse_result.unwrap().1;
        assert_eq!(head.signature.as_slice(), LUA_SIGNATURE);
        assert_eq!(head.version, 83);
        assert_eq!(head.format, 0);
        assert_eq!(head.luac_data.as_slice(), LUAC_DATA);
        assert_eq!(head.c_int_size, 4);
        assert_eq!(head.c_size_t_size, 8);
        assert_eq!(head.instruction_size, 4);
        assert_eq!(head.lua_integer_size, 8);
        assert_eq!(head.lua_number_size, 8);
        assert_eq!(head.luac_int, 0x5678);
        assert_eq!(head.luac_num, 370.5);
    }
}