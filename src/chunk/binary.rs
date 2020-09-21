use nom;
use nom::number::streaming::{le_f64, le_i64};
use nom_derive::Nom;
use std::rc::Rc;

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

#[derive(Debug, Nom)]
pub struct Chunk {
    pub header: Header,
    pub size_upvalues: u8,
    pub main: Prototype,
}

#[derive(Debug, Nom)]
pub struct Header {
    #[nom(Count = "4")]
    signature: Vec<u8>,
    pub version: u8,
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

#[derive(Debug)]
// #[nom(DebugDerive)]
// #[nom(LittleEndian)]
pub struct Prototype {
    pub source: Option<String>,
    pub line_defined: u32,
    pub last_line_defined: u32,
    pub num_params: u8,
    pub is_vararg: u8,
    pub max_stack_size: u8,
    pub code: Vec<u32>,
    pub constants: Vec<Constant>,
    pub upvalues: Vec<UpValue>,
    pub prototypes: Vec<Rc<Prototype>>,
    pub line_info: Vec<u32>,
    pub loc_vars: Vec<LocVar>,
    pub upvalue_names: Vec<UpValueName>,
}

impl Prototype {
    pub fn parse(orig_i: &[u8]) -> nom::IResult<&[u8], Prototype> {
        let i = orig_i;
        let (mut i, len) = nom::number::streaming::le_u8(i)?;
        let mut source: Option<String> = None;
        if len > 0 {
            let (chunk, name) =
                nom::multi::count(nom::number::streaming::le_u8, { len - 1 } as usize)(i)?;
            let (chunk, src) = nom::combinator::cond(len > 0, {
                |__i__| Ok((__i__, String::from_utf8(name.clone()).unwrap()))
            })(chunk)?;
            source = src;
            i = chunk;
        }
        let (i, line_defined) = nom::number::streaming::le_u32(i)?;
        let (i, last_line_defined) = nom::number::streaming::le_u32(i)?;
        let (i, num_params) = nom::number::streaming::le_u8(i)?;
        let (i, is_vararg) = nom::number::streaming::le_u8(i)?;
        let (i, max_stack_size) = nom::number::streaming::le_u8(i)?;
        let (i, code_length) = nom::number::streaming::le_u32(i)?;
        let (i, code) =
            nom::multi::count(nom::number::streaming::le_u32, { code_length } as usize)(i)?;
        let (i, const_length) = nom::number::streaming::le_u32(i)?;
        let (i, constants) = nom::multi::count(Constant::parse, { const_length } as usize)(i)?;
        let (i, upvalue_length) = nom::number::streaming::le_u32(i)?;
        let (i, upvalues) = nom::multi::count(UpValue::parse, { upvalue_length } as usize)(i)?;
        let (i, proto_length) = nom::number::streaming::le_u32(i)?;
        let (i, prototypes) = nom::multi::count(
            |x| {
                let (data, proto) = Prototype::parse(x)?;
                Ok((data, Rc::new(proto)))
            },
            { proto_length } as usize,
        )(i)?;
        let (i, line_length) = nom::number::streaming::le_u32(i)?;
        let (i, line_info) =
            nom::multi::count(nom::number::streaming::le_u32, { line_length } as usize)(i)?;
        let (i, loc_length) = nom::number::streaming::le_u32(i)?;
        let (i, loc_vars) = nom::multi::count(LocVar::parse, { loc_length } as usize)(i)?;
        let (i, up_length) = nom::number::streaming::le_u32(i)?;
        let (i, upvalue_names) = nom::multi::count(UpValueName::parse, { up_length } as usize)(i)?;
        let struct_def = Prototype {
            // len,
            // name,
            source,
            line_defined,
            last_line_defined,
            num_params,
            is_vararg,
            max_stack_size,
            // code_length,
            code,
            // const_length,
            constants,
            // upvalue_length,
            upvalues,
            // proto_length,
            prototypes,
            // line_length,
            line_info,
            // loc_length,
            loc_vars,
            // up_length,
            upvalue_names,
        };
        Ok((i, struct_def))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Nom)]
pub struct ShortString {
    len: u8,
    #[nom(Count = "len-1")]
    content: Vec<u8>,
    #[nom(Value = "String::from_utf8(content.clone()).unwrap()")]
    pub value: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Nom)]
pub struct ConstantType(pub u8);

#[derive(Clone, Debug, PartialEq, Nom)]
#[nom(Selector = "ConstantType")]
// #[nom(DebugDerive)]
#[nom(LittleEndian)]
pub enum ConstantValue {
    #[nom(Selector = "ConstantType(0)")]
    Nil,
    #[nom(Selector = "ConstantType(1)")]
    Boolean(u8),
    #[nom(Selector = "ConstantType(3)")]
    Number(#[nom(Parse = "le_f64")] f64),
    #[nom(Selector = "ConstantType(19)")]
    Integer(i64),
    #[nom(Selector = "ConstantType(4)")]
    ShortStr(ShortString),
}

#[derive(Clone, Debug, PartialEq, Nom)]
pub struct Constant {
    pub const_type: ConstantType,
    #[nom(Parse = "{ |i| ConstantValue::parse(i, const_type) }")]
    pub const_value: ConstantValue,
}

#[derive(Debug, PartialEq, Eq, Clone, Nom)]
#[nom(LittleEndian)]
pub struct LocVar {
    pub var_name: VariableName,
    pub start_pc: u32,
    pub end_pc: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Nom)]
pub struct VariableName {
    len: u8,
    #[nom(Count = "len-1")]
    content: Vec<u8>,
    #[nom(Value = "String::from_utf8(content.clone()).unwrap()")]
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Nom)]
pub struct UpValue {
    pub instack: u8,
    pub idx: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Nom)]
pub struct UpValueName {
    len: u8,
    #[nom(Count = "len-1")]
    content: Vec<u8>,
    #[nom(Value = "String::from_utf8(content.clone()).unwrap()")]
    pub value: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::bytes::complete::tag;
    use nom::IResult;
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
        let name = "foo.out";
        let content = read(name).unwrap();
        let parse_result = Chunk::parse(content.as_slice());
        assert!(parse_result.is_ok());
        let chunk: Chunk = parse_result.unwrap().1;
        let head: Header = chunk.header;
        assert_eq!(head.signature.as_slice(), LUA_SIGNATURE);
        assert_eq!(head.version, 83);
        assert_eq!(head.format, 0);
        assert_eq!(head.luac_data.as_slice(), LUAC_DATA);
        assert_eq!(head.c_int_size, 4);
        assert_eq!(head.c_size_t_size, 8);
        assert_eq!(head.instruction_size, 4);
        assert_eq!(head.lua_integer_size, 8);
        assert_eq!(head.lua_number_size, 8);
        assert_eq!(head.luac_int, LUAC_INT);
        assert_eq!(head.luac_num, LUAC_NUM);

        assert_eq!(chunk.size_upvalues, 1);
        let main = chunk.main;
        assert_eq!(main.source.clone().unwrap().len(), 6);
        assert_eq!(main.source.unwrap().as_str(), "=stdin");
        assert_eq!(main.line_defined, 0);
        assert_eq!(main.last_line_defined, 0);
        assert_eq!(main.num_params, 0);
        assert_eq!(main.is_vararg, 1);
        assert_eq!(main.max_stack_size, 2);
        assert_eq!(main.code.len(), 3);
        // Constants
        assert_eq!(main.constants.len(), 1);
        let lua_const = main.constants[0].clone();
        assert_eq!(lua_const.const_type.0, 4);
        assert_eq!(
            lua_const.const_value,
            ConstantValue::ShortStr(ShortString {
                len: 4,
                content: vec![102, 111, 111],
                value: "foo".to_string(),
            })
        );
        // Upvalue
        assert_eq!(main.upvalues.len(), 1);
        assert_eq!(main.upvalues[0], UpValue { instack: 1, idx: 0 });
        // Prototype
        assert_eq!(main.prototypes.len(), 1);
        assert_eq!(main.line_info.len(), 3);
        assert_eq!(main.loc_vars.len(), 0);
        assert_eq!(main.upvalue_names.len(), 1);
        assert_eq!(main.upvalue_names[0].value, "_ENV".to_string());
    }
}
