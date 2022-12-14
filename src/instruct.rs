///This file will hold all the necessary OPCode instruction code that will be called elsewhere
use std::io::Write;
use std::io::Read;

use crate::read_bin::UmState;

/// This sets register A's value to be the same as register b, so long as register C's value is 0
/// It then incraments the instance coutner
///
/// # Arguments
/// * active_state: A struct containing the registers, memory, isntance counter, and memeory tracker
/// * rega: Index of register A
/// * regb: Index of register B
/// * rebc: Index of register C
pub fn con_move(active_state: &mut UmState, rega:usize, regb:usize, regc:usize) {
    if active_state.registers[regc] != 0 {
        active_state.registers[rega] = active_state.registers[regb];
    }
    
    active_state.inst_count += 1;
}

/// This sets register A to the value at Memory[Value in Register B][Value in Register C]
/// It then incraments the instance coutner
///
/// # Arguments
/// * active_state: A struct containing the registers, memory, isntance counter, and memeory tracker
/// * rega: Index of register A
/// * regb: Index of register B
/// * rebc: Index of register C
pub fn seg_load(active_state: &mut UmState, rega:usize, regb:usize, regc:usize) {
    active_state.registers[rega] = active_state.memory[active_state.registers[regb] as usize][active_state.registers[regc] as usize];
    
    active_state.inst_count += 1;
}

/// This stores the value at Memory[Value in Register A][Value in Register B] in register C
/// It then incraments the instance coutner
///
/// # Arguments
/// * active_state: A struct containing the registers, memory, isntance counter, and memeory tracker
/// * rega: Index of register A
/// * regb: Index of register B
/// * rebc: Index of register C
pub fn seg_store(active_state: &mut UmState, rega:usize, regb:usize, regc:usize) {
    //println!("{}, {}, {}", active_state.registers[rega], active_state.registers[regb], active_state.registers[regc]);
    // println!("[DEBUG] seg_store start");
    // println!("[DEBUG] memory length is {}", active_state.memory.len());
    // println!("[DEBUG] accessing memory segment {}", active_state.registers[rega]);
    // println!("[DEBUG] memory segment {} has length {}", active_state.registers[rega], active_state.memory[active_state.registers[rega] as usize].len());
    // println!();
    active_state.memory[active_state.registers[rega] as usize][active_state.registers[regb] as usize] = active_state.registers[regc];

    active_state.inst_count += 1;
}

/// This stores the sum of the values in register's B and C in register A
/// It then incraments the instance coutner
///
/// # Arguments
/// * active_state: A struct containing the registers, memory, isntance counter, and memeory tracker
/// * rega: Index of register A
/// * regb: Index of register B
/// * rebc: Index of register C
pub fn add(active_state: &mut UmState, rega:usize, regb:usize, regc:usize) {
    active_state.registers[rega] = active_state.registers[regb].wrapping_add(active_state.registers[regc]);

    active_state.inst_count += 1;
}

/// This stores the product of the values in register's B and C in register A
/// It then incraments the instance coutner
///
/// # Arguments
/// * active_state: A struct containing the registers, memory, isntance counter, and memeory tracker
/// * rega: Index of register A
/// * regb: Index of register B
/// * rebc: Index of register C
pub fn mult(active_state: &mut UmState, rega:usize, regb:usize, regc:usize) {
    active_state.registers[rega] = active_state.registers[regb].wrapping_mul(active_state.registers[regc]);

    active_state.inst_count += 1;
}

/// This stores the value from the value of register B after it's divided by the value in register C in register A
/// It then incraments the instance coutner
///
/// # Arguments
/// * active_state: A struct containing the registers, memory, isntance counter, and memeory tracker
/// * rega: Index of register A
/// * regb: Index of register B
/// * rebc: Index of register C
pub fn div(active_state: &mut UmState, rega:usize, regb:usize, regc:usize) {
    active_state.registers[rega] = active_state.registers[regb] / active_state.registers[regc];

    active_state.inst_count += 1;
}

/// This has the values in register's B and C bitwised anded together and then is stored in register A
/// It then incraments the instance coutner
///
/// # Arguments
/// * active_state: A struct containing the registers, memory, isntance counter, and memeory tracker
/// * rega: Index of register A
/// * regb: Index of register B
/// * rebc: Index of register C
pub fn bnand(active_state: &mut UmState, rega:usize, regb:usize, regc:usize) {
    active_state.registers[rega] = !(active_state.registers[regb] & active_state.registers[regc]);

    active_state.inst_count += 1;
}

/// This created a new segment with a number of words equal to the value in register C, that segment is placed in Memory[Value of Reister B]
/// It then incraments the instance coutner
///
/// # Arguments
/// * active_state: A struct containing the registers, memory, isntance counter, and memeory tracker
/// * regb: Index of register B
/// * rebc: Index of register C
pub fn map_seg(active_state: &mut UmState, regb:usize, regc:usize) {
    // println!("[DEBUG] Mapping a new segment");
    // println!("[DEBUG] Current number of memory segments is {}", active_state.memory.len());
    // println!("[DEBUG] b = {regb}, c = {regc}");
    // println!("[DEBUG] $r[b] = {}, $r[c] = {}", active_state.registers[regb], active_state.registers[regc]);
   // println!("[DEBUG] memory_tracker: {:?}", active_state.memory_tracker);
   
    if active_state.memory_tracker.len() == 0 {
        let init = active_state.registers[regc] as usize;
        let mem_seg: Vec<u32> = vec![0; init];
        active_state.memory.push(mem_seg);
        active_state.registers[regb] = (active_state.memory.len() - 1) as u32;

    }
    else{
        let init = active_state.registers[regc] as usize;
        let mem_seg: Vec<u32> = vec![0; init];
        let mem_pos = active_state.memory_tracker.remove(0);
        active_state.memory[mem_pos as usize]= mem_seg; 
        active_state.registers[regb] = mem_pos;
    }

 

    active_state.inst_count += 1;
    
    
    // println!("[DEBUG] Mapped a new segment");
    // println!("[DEBUG] Current number of memory segments is {}", active_state.memory.len());
    // println!();
}

/// This unmaps the segment at Memeory[Value in register C]
///
/// # Arguments
/// * active_state: A struct containing the registers, memory, isntance counter, and memeory tracker
/// * rebc: Index of register C
pub fn unmap_seg(active_state: &mut UmState, regc:usize) {
    //active_state.memory_tracker[active_state.registers[regc] as usize] = false;
    active_state.memory_tracker.push(active_state.registers[regc]);

    active_state.inst_count += 1;
}

/// Displays on the I/O device the value stored in register C so long as the value is between 0 and 255
///
/// # Arguments
/// * active_state: A struct containing the registers, memory, isntance counter, and memeory tracker
/// * rebc: Index of register C
pub fn output(active_state: &mut UmState, regc: usize) {
    //println!("Got into intruct::output");
    // if active_state.registers[regc] > 255 || active_state.registers[regc] < 0 {
    //     panic!("NO CAN DO BUCKAROO");
    // }
    //else {

        let out_word: [u8; 1] = [active_state.registers[regc] as u8];
        //println!("outword {}", out_word[0]);
        std::io::stdout().write(&out_word).ok();

        active_state.inst_count += 1;
   // }
}

/// Takes an input from the I/O device and then places it in register C, so long as teh value is between 0 and 255
///
/// # Arguments
/// * active_state: A struct containing the registers, memory, isntance counter, and memeory tracker
/// * rebc: Index of register C
pub fn input(active_state: &mut UmState, regc: usize) {

    let mut take_in: [u8; 1] = [0; 1];
    match std::io::stdin().read(&mut take_in) {
        Ok(0) => {
            // set reg to u32::MAX
            active_state.registers[regc] = u32::MAX;


        }
        Ok(_) => {
            // set reg to take_in
            active_state.registers[regc] = take_in[0] as u32;
        }
        Err(_) => {
            panic!();
        }
    }

    active_state.inst_count += 1;

}

/// Replaces Memory[0] which should be the isntance counter with Memory[Value in register B]
/// It then sets the program coutner to point to Memeory[0][Vlaue in register C]
///
/// # Arguments
/// * active_state: A struct containing the registers, memory, isntance counter, and memeory tracker
/// * regb: Index of register B
/// * rebc: Index of register C
pub fn load_prog(active_state: &mut UmState, regb: usize, regc: usize) {
    if active_state.registers[regb] == 0 {
        active_state.inst_count = active_state.registers[regc];
        //println!("new inst count: {:b}", active_state.memory[0][active_state.registers[regc] as usize]);
    }
    else {
        active_state.memory[0] = active_state.memory[active_state.registers[regb] as usize].clone();
        //println!("new inst count: {}", active_state.memory[0][active_state.registers[regc] as usize]);
        active_state.inst_count = active_state.registers[regc];
    }

}

/// This stores value in register A
///
/// # Arguments
/// * active_state: A struct containing the registers, memory, isntance counter, and memeory tracker
/// * rega: Index of register A
/// * Vale: A 25-bit value
pub fn load_val(active_state: &mut UmState, rega: usize, value: usize) {
    active_state.registers[rega] = value as u32;
    active_state.inst_count += 1;
}