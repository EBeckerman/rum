use std::convert::TryInto;

/// Retrieve an unsigned value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
///
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn getu(word: u64, width: u64, lsb: u64) -> u64 {
    let place_holder = (1 << width) - 1;
    let part = (word >> lsb) & place_holder;

    return part.try_into().unwrap();
}

/*
// println!("[DEBUG] Mapping a new segment");
    // println!("[DEBUG] Current number of memory segments is {}", active_state.memory.len());
    // println!("[DEBUG] b = {regb}, c = {regc}");
    // println!("[DEBUG] $r[b] = {}, $r[c] = {}", active_state.registers[regb], active_state.registers[regc]);
   // println!("[DEBUG] memory_tracker: {:?}", active_state.memory_tracker);
    if active_state.memory_tracker.len() == 0 {
        //println!("[DEBUG] Entered first branch");
        let init = active_state.registers[regc] as usize;
        let mem_seg: Vec<u32> = vec![0; init];
        active_state.memory.push(mem_seg);
        let mem_index = active_state.memory.len();
        active_state.registers[regb] = (mem_index - 1) as u32;
        active_state.memory_tracker.push(true);

        active_state.inst_count += 1;
    }
    else {
        //println!("[DEBUG] Entered the second branch");
        let mut counter = 0;
        let mut found = false;
        for i in active_state.memory_tracker.clone() {
            if !i {        
                let init = active_state.registers[regc] as usize;
                let mem_seg: Vec<u32> = vec![0; init];
                active_state.memory[counter] = mem_seg;
                active_state.registers[regb] = counter as u32;
                active_state.memory_tracker.push(true);
                found = true;
            }
            else {
                counter += 1;
            }

            if !found {
                let init = active_state.registers[regc] as usize;
                let mem_seg: Vec<u32> = vec![0; init];
                //println!("Size of new mem_seg {init}");
                active_state.memory.push(mem_seg);
                active_state.registers[regb] = (active_state.memory.len() - 1) as u32;
                active_state.memory_tracker.push(true);
            }

        }


        active_state.inst_count += 1;
    }
    
    // println!("[DEBUG] Mapped a new segment");
    // println!("[DEBUG] Current number of memory segments is {}", active_state.memory.len());
    // println!();
*/