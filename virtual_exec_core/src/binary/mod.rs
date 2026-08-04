#![cfg(feature = "binary_inst")]

use bytes::{Buf, Bytes, BytesMut};
use crate::sequential::instructions::Instruction;

pub fn export(instructions: Vec<Instruction>) -> Result<Bytes, std::io::Error> {
    let result = borsh::to_vec(&instructions)?;
   Ok(Bytes::from(result))
}

pub fn export_one(instruction: Instruction) -> Result<Bytes, std::io::Error> {
    let result = borsh::to_vec(&instruction)?;
    Ok(Bytes::from(result))
}

pub fn import(bytes: &Bytes) -> Result<Vec<Instruction>, std::io::Error> {
    let result = borsh::from_slice(&bytes)?;
    Ok(result)
}

// pub fn import_one(byte_stream: &mut BytesMut) -> Result<Instruction, std::io::Error> {
//     use borsh::BorshDeserialize;
//     let mut slice: &[u8] = byte_stream.as_ref();
//     let init = slice.len();
//     let inst = Instruction::deserialize(&mut slice)?;
//     byte_stream.advance(init - slice.len());
//     Ok(inst)
// }

pub fn import_one(byte_stream: &mut BytesMut) -> Result<Instruction, std::io::Error> {
    use borsh::BorshDeserialize;
    Instruction::deserialize_reader(&mut byte_stream.reader())
}
