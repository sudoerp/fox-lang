#![allow(unused)]

use std::fmt::{self, write};
use std::convert::From;

type Value = f32;

#[repr(u8)]
pub enum Operation{
    Return = 0,
    Constant = 1
}


pub struct Chunk{
    pub code: Vec<u8>,
    /*
    A way to store const that will result in a value at runtime
    1 + 2; -> 3[But at runtime] Interpreter needs to store 1, 2
    sort of instructions that 'Produce a value'
    */
    constants: Vec<Value>
}

impl Chunk{
    pub fn init() -> Self{
        Self{
            code: vec![],
            constants: vec![]
        }
    }

    pub fn write_chunk(&mut self, op: Operation){
        self.code.push(op as u8);
    }

    pub fn add_constant(&mut self, value: Value) -> i32{
        self.constants.push(value);
        get_index(&self.constants, value)
    }

}


fn get_index<T: PartialEq>(vec: &Vec<T>, x: T) -> i32{
    let index = vec.iter().position(|element| element == &x).unwrap();
    index as i32
}

pub fn u8_to_operation(byte_code: u8) -> Operation{
    match byte_code{
        0 => Operation::Return,
        1 => Operation::Constant,
        _ => todo!()
    }
}