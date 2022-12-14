//Reads binar
//4294967296 

use std::convert::TryInto;
use crate::instruct;
use crate::bitpack;
//use bitpack::bitpack;

pub struct UmState {
    pub inst_count: u32,
    pub registers: Vec<u32>,
    pub memory: Vec<Vec<u32>>,
    pub memory_tracker: Vec<u32>,
}

/// This takes a UM file of 8-bit binary numbers and turns it into a vector of 32-bit instructions
///
/// # Arguments
/// * Input: an option that either is empty or has a string with a file address
pub fn load(input: Option<&str>) -> Vec<u32> {
    let mut raw_reader: Box<dyn std::io::BufRead> = match input {
        None => Box::new(std::io::BufReader::new(std::io::stdin())),
        Some(filename) => Box::new(std::io::BufReader::new(std::fs::File::open(filename).unwrap(),)),
    };
    let mut buf = Vec::<u8>::new();
    raw_reader.read_to_end(&mut buf).unwrap();
    let instructions: Vec<u32> = buf
        .chunks_exact(4)
        .map(|x| u32::from_be_bytes(x.try_into().unwrap()))
        .collect();
    instructions
}

/// This takes a vector of 32-bit binary instructions and proforms the actions described in them
///
/// # Arguments
/// * A vector of 32-bit numbers
pub fn instructs(instructions: Vec<u32>) {
    let inst_count = 0;
    let registers: Vec<u32> = vec![0; 8];
    let memory: Vec<Vec<u32>> = Vec::new();
    let memory_tracker: Vec<u32> = Vec::new();  

    let mut active_state = UmState {
        inst_count,
        registers,
        memory,
        memory_tracker,
    };
    
    active_state.memory.push(instructions.clone());


    while active_state.inst_count < active_state.memory[0].len() as u32 {
        // println!("inst count: {}, instruction len: {}", active_state.inst_count, active_state.memory[0].len());
        let opcode: u32 = bitpack::getu(active_state.memory[0][active_state.inst_count as usize] as u64, 4, 28) as u32;
        // println!("instructions {:b}", instructions[active_state.inst_count as usize]);
        // println!("{}", opcode);

        let rega;
        let mut regb = 0;
        let mut regc = 0;
        let mut value = 0;

        if opcode == 13 {
            //[1-4][5-7][8-32]
            rega = bitpack::getu(active_state.memory[0][active_state.inst_count as usize] as u64, 3, 25) as u32;
            value = bitpack::getu(active_state.memory[0][active_state.inst_count as usize] as u64, 25, 0) as u32;
            
        } else {
            //[1-4][5-23]][24-26][27-29][30-32]
            rega = bitpack::getu(active_state.memory[0][active_state.inst_count as usize] as u64, 3, 6) as u32;
            regb = bitpack::getu(active_state.memory[0][active_state.inst_count as usize] as u64, 3, 3) as u32;
            regc = bitpack::getu(active_state.memory[0][active_state.inst_count as usize] as u64, 3, 0) as u32;
        }
        //println!("{}", opcode);
        match opcode {
            0 => instruct::con_move(&mut active_state, rega as usize, regb as usize, regc as usize),
            1 => instruct::seg_load(&mut active_state, rega as usize, regb as usize, regc as usize),
            2 => instruct::seg_store(&mut active_state, rega as usize, regb as usize, regc as usize),
            3 => instruct::add(&mut active_state, rega as usize, regb as usize, regc as usize),
            4 => instruct::mult(&mut active_state, rega as usize, regb as usize, regc as usize),
            5 => instruct::div(&mut active_state, rega as usize, regb as usize, regc as usize),
            6 => instruct::bnand(&mut active_state, rega as usize, regb as usize, regc as usize),
            7 => break,
            8 => instruct::map_seg(&mut active_state, regb as usize, regc as usize),
            9 => instruct::unmap_seg(&mut active_state, regc as usize),
            10 => instruct::output(&mut active_state, regc as usize),
            11 => instruct::input(&mut active_state, regc as usize),
            12 => instruct::load_prog(&mut active_state, regb as usize, regc as usize),
            13 => instruct::load_val(&mut active_state, rega as usize, value as usize),
            _ => panic!("Invalid OpCode {}", opcode)
        }
        
    }
    
}