/* Quick and dirty emulator */

extern crate customasm;

use std::env;
use std::collections::HashMap;
use std::convert::TryInto;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ram 
{
    contents: Vec<i32>,
    size: usize,
}

impl Ram 
{
    pub fn new() -> Ram
    {
        Ram 
        {
            contents: Vec::new(),
            size: 0,
        }
    }

    pub fn load(&mut self, bytes: Vec<u8>)
    {
        for lp in 0..bytes.len()/std::mem::size_of::<i32>()
        {
            let start = lp*std::mem::size_of::<i32>();
            let slice = &bytes[start..start+std::mem::size_of::<i32>()];
            let value = i32::from_be_bytes(slice.try_into().expect("Not a 32bit program!"));
            self.contents[lp] = value;
        }
    }

    pub fn set_size(&mut self, size: usize)
    {
        self.contents.resize(size, 0);
        self.size = size;
    }

    pub fn set(&mut self, index: usize, value: i32) 
    {
        if index < self.size { 
            self.contents[index] = value;
        }
    }

    pub fn get(&self, index: usize) -> i32
    {
        if index < self.size {
            return self.contents[index];
        } 
        return 0;
    }

    pub fn get_as_u32(&self, index: usize) -> u32
    {
        return self.get(index) as u32;
    }

    pub fn get_as_usize(&self, index: usize) -> usize
    {
        return (self.get(index) as u32) as usize;
    }

    pub fn is_invalid_memory(&self, pc: usize) -> bool
    {
        if pc >= 0x80000000 && pc <= 0x80000003 { return false };
        if pc+1 >= 0x80000000 && pc+1 <= 0x80000003 { return false };
        if pc+2 >= 0x80000000 && pc+2 <= 0x80000003 { return false };
        if pc+3 >= 0x80000000 && pc+3 <= 0x80000003 { return false };

        if self.contents[pc] as u32 >= 0x80000000 && self.contents[pc] as u32 <= 0x80000003 { return false };
        if self.contents[pc+1] as u32 >= 0x80000000 && self.contents[pc+1] as u32 <= 0x80000003 { return false };
        if self.contents[pc+2] as u32 >= 0x80000000 && self.contents[pc+2] as u32 <= 0x80000003 { return false };
        if self.contents[pc+3] as u32 >= 0x80000000 && self.contents[pc+3] as u32 <= 0x80000003 { return false };

        if pc >= self.size { return true };
        if pc+1 >= self.size { return true };
        if pc+2 >= self.size { return true };
        if pc+3 >= self.size { return true };


        if (self.contents[pc] as u32) as usize >= self.size { return true };
        if (self.contents[pc] as u32) as usize >= self.size { return true };
        if (self.contents[pc] as u32) as usize >= self.size { return true };
        if (self.contents[pc] as u32) as usize >= self.size { return true };

        return false;
    }

}

const MEM_SIZE: usize = 4096*1024; 

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() == 1
    {
        println!("no input file!");
        std::process::exit(1);
    }

    if args.len() != 2 
    {
        println!("to many parameters.");
        println!("usage: eusti <filename>");
        std::process::exit(1);
    }   

    let mut fileserver = customasm::util::FileServerReal::new();
    let report = customasm::diagn::RcReport::new();
    let mut assembler = customasm::asm::Assembler::new();
    
    assembler.register_file(&args[1]);

    customasm::util::enable_windows_ansi_support();

    let output = match assemble(report.clone(), &mut fileserver, assembler)
    {
        Ok(output) => output,
        Err(_) =>
        {
            println!("");    
            report.print_all(&mut std::io::stderr(), &fileserver);
            std::process::exit(1);
        }
    };

    if report.has_messages()
    { println!(""); }
  
    report.print_all(&mut std::io::stderr(), &fileserver);
 
    emulate(output);
}

fn emulate(output: (std::vec::Vec<u8>, std::string::String))
{
    let mut memory = Ram::new();
    memory.set_size(MEM_SIZE);
    memory.load(output.0);
    
    let mut pc = 0;

    let (sym2addr, addr2sym) = parse_symbols(output.1); 

    loop
    {

       // println!("PC(0x{:x}) = {:x}, {:x}, {:x}, {:x}", pc, ram[pc], ram[pc+1], ram[pc+2], ram[pc+3]);        

        //println!("RAM[r0] = {}", ram[sym2addr["r0"]]);

        // PC = suspend
        if memory.get_as_u32(pc+2) == 0x80000000
        {
            if addr2sym.contains_key(&pc)
            {
                println!("Suspended @ 0x{:x} [{}]", pc, addr2sym[&pc]);
                break;
            }
            println!("Suspended @ 0x{:x}", pc);
            break;
        } 

        // CN = reset
        else if memory.get_as_u32(pc+2) == 0x80000002
        {
            if addr2sym.contains_key(&pc)
            {
                println!("Reset @ 0x{:x} [{}]", pc, addr2sym[&pc]);
                break;
            }
            println!("Reset @ 0x{:x}", pc);
            break;
        }
        
        // Invalid memory
        else if memory.is_invalid_memory(pc) 
        {
            println!("Invalid memory access @ 0x{:x}", pc);
            break;
        }

        // Execute
        else
        {

            //println!("PC+2 = {:x}", pc+2);

            let a = memory.get(memory.get_as_usize(pc));
            let b = memory.get(memory.get_as_usize(pc+1));
            let r = memory.get_as_usize(pc+2);
            
            let v = a - b;

            //println!("{:x}, {:x}, {:x}, {:x}", a, b, r, v);

            memory.set(r, v);

            if v < 0 
            {
                pc = memory.get_as_usize(pc+3); 
            } else {
                pc += 4;
            }
        }
        //println!("");
    }

    println!();    
    println!("  rA: 0x{:<width$x} rB: 0x{:<width$x} rR: 0x{:<width$x} rJ: 0x{:<width$x} rV: 0x{:<width$x}", 
        memory.get_as_u32(sym2addr["ra"]), memory.get_as_u32(sym2addr["rb"]), memory.get_as_u32(sym2addr["rr"]), memory.get_as_u32(sym2addr["rj"]), memory.get_as_u32(sym2addr["rv"]), width=14);
    
    println!("HEAP: 0x{:<width$x} SP: 0x{:<width$x}", 
        memory.get_as_u32(sym2addr["HEAP"]), memory.get_as_u32(sym2addr["SP"]), width=14);
    
        println!();    
    
    println!("R0: 0x{:<width$x} R1: 0x{:<width$x} R2: 0x{:<width$x} R3: 0x{:<width$x}", 
        memory.get_as_u32(sym2addr["r0"]), memory.get_as_u32(sym2addr["r1"]), memory.get_as_u32(sym2addr["r2"]), memory.get_as_u32(sym2addr["r3"]), width=14);
    
    println!("R4: 0x{:<width$x} R5: 0x{:<width$x} R6: 0x{:<width$x} R7: 0x{:<width$x}", 
        memory.get_as_u32(sym2addr["r4"]), memory.get_as_u32(sym2addr["r5"]), memory.get_as_u32(sym2addr["r6"]), memory.get_as_u32(sym2addr["r7"]), width=14);
  
}

fn parse_symbols(symbols: String) -> (HashMap<String, usize>, HashMap<usize, String>)
{

    let mut name2addr: HashMap<String, usize> = HashMap::new();
    let mut addr2name: HashMap<usize, String> = HashMap::new();
    
    let lines: Vec<_> = symbols.lines().collect();

    for l in lines {
        let p: Vec<_> = l.split('=').collect();
        let name: String = p[0].chars().filter(|c| !c.is_whitespace()).collect();
        let hexstr: String = p[1].chars().filter(|c| !c.is_whitespace()).collect();
        let hex = usize::from_str_radix(&hexstr.trim_start_matches("0x"), 16).unwrap();
        name2addr.insert(name.clone(), hex.clone());
        addr2name.insert(hex.clone(), name.clone());
    }
    
    name2addr.insert("PC".to_string(), 0x80000000);
    addr2name.insert(0x80000000, "PC".to_string());
    
    addr2name.insert(0x80000001, "RV".to_string());
    name2addr.insert("RV".to_string(), 0x80000001);
 
    addr2name.insert(0x80000002, "CN".to_string());
    name2addr.insert("CN".to_string(), 0x80000002);
 
    addr2name.insert(0x80000003, "IO".to_string());
    name2addr.insert("IO".to_string(), 0x80000003);
 
    return (name2addr, addr2name);
}

fn assemble(
    report: customasm::diagn::RcReport, 
    fileserver: &mut dyn customasm::util::FileServer, 
    assembler: customasm::asm::Assembler) -> Result<(std::vec::Vec<u8>, String), bool>
{
	let output = assembler.assemble(
		report.clone(),
		fileserver,
		10)
		.map_err(|_| false)?;

    let binary = output.binary.format_binary();
    let symbols = output.state.symbols.format_default();

    return Ok((binary, symbols));
}
