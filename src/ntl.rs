use anyhow::Result;
use byteorder::LittleEndian;
use byteorder::{ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

const SIGNATURE: u32 = 0x2C8D14F1;

fn read_string<T: ReadBytesExt>(reader: &mut BufReader<T>) -> Result<String> {
    let len = reader.read_u8()?;
    let mut text_buf = vec![0; len as usize];
    reader.read_exact(&mut text_buf)?;
    let result = String::from_utf8(text_buf)?;
    Ok(result)
}

fn write_string<T: WriteBytesExt>(writer: &mut BufWriter<T>, text: &str) -> Result<()> {
    writer.write_u8(text.len() as u8)?;
    writer.write_all(text.as_bytes())?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
enum FunctionType {
    Function,
    Operator,
    PreOperator,
    PostOperator,
}

impl FunctionType {
    fn from(n: u8) -> Self {
        match n {
            1 => FunctionType::Function,
            2 => FunctionType::Operator,
            3 => FunctionType::PreOperator,
            4 => FunctionType::PostOperator,
            _ => panic!("invalid function type: {}", n),
        }
    }

    fn to_u8(&self) -> u8 {
        match self {
            FunctionType::Function => 1,
            FunctionType::Operator => 2,
            FunctionType::PreOperator => 3,
            FunctionType::PostOperator => 4,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NativeTableItem {
    pub opcode: u32,
    name: String,
    oper_precedence: u8,
    typ: FunctionType,
}

pub fn read_ntl(path: &PathBuf) -> Result<Vec<NativeTableItem>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let sig = reader.read_u32::<LittleEndian>()?;
    let num = reader.read_u32::<LittleEndian>()?;
    assert_eq!(sig, SIGNATURE);

    let mut items = Vec::new();
    for _ in 0..num {
        let name = read_string(&mut reader)?;
        let oper_precedence = reader.read_u8()?;
        let typ = FunctionType::from(reader.read_u8()?);
        let opcode = reader.read_u32::<LittleEndian>()?;
        items.push(NativeTableItem {
            name,
            oper_precedence,
            typ,
            opcode,
        });
    }

    Ok(items)
}

pub fn write_ntl(items: &Vec<NativeTableItem>, path: &PathBuf) -> Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    writer.write_u32::<LittleEndian>(SIGNATURE)?;
    writer.write_u32::<LittleEndian>(items.len() as u32)?;

    for item in items {
        write_string(&mut writer, &item.name)?;
        writer.write_u8(item.oper_precedence)?;
        writer.write_u8(item.typ.to_u8())?;
        writer.write_u32::<LittleEndian>(item.opcode)?;
    }
    Ok(())
}
