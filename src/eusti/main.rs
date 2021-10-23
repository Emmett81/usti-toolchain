/* Quick and dirty emulator */

extern crate customasm;

use std::env;
use std::collections::HashMap;
use std::convert::TryInto;

const MEM_SIZE: usize = 512*1024; // 512Kword of memory

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

    let ram = alloc_ram(output.0);   
    let (sym2addr, addr2sym) = parse_symbols(output.1); 
 
    emulate(ram, sym2addr, addr2sym);

}

fn invalid_mem(ram: Vec<i32>, pc: usize) -> bool
{
    //println!("{}, {}, {}, {}", ram[pc] as u32, ram[pc+1] as u32, ram[pc+2] as u32, ram[pc+3] as u32);
    if  (pc < ram.len() || (pc >= 0x80000000 && pc <= 0x80000003)) && 
        ((ram[pc] as u32) < (ram.len() as u32) || ((ram[pc] as u32) >= 0x80000000 && (ram[pc] as u32) <= 0x80000003)) &&
        ((ram[pc+1] as u32) < (ram.len() as u32) || ((ram[pc+1] as u32) >= 0x80000000 && (ram[pc+1] as u32) <= 0x80000003)) &&
        ((ram[pc+2] as u32) < (ram.len() as u32) || ((ram[pc+2] as u32) >= 0x80000000 && (ram[pc+2] as u32) <= 0x80000003)) &&
        ((ram[pc+3] as u32) < (ram.len() as u32) || ((ram[pc+3] as u32) >= 0x80000000 && (ram[pc+3] as u32) <= 0x80000003))
    {
        return false
    }
    return true;
}

fn emulate(mut ram: Vec<i32>, sym2addr: HashMap<String, usize>, addr2sym: HashMap<usize, String>)
{
    let mut pc = 0;

    loop
    {        
        // PC = suspend
        if (ram[pc+2] as u32) == 0x80000000
        {
            if addr2sym.contains_key(&pc)
            {
                println!("Suspended @ {:x} [{}]", pc, addr2sym[&pc]);
                break;
            }
            println!("Suspended @ {:x}", pc);
            break;
        } 

        // CN = reset
        else if (ram[pc+2] as u32) == 0x80000002
        {
            if addr2sym.contains_key(&pc)
            {
                println!("Reset @ {:x} [{}]", pc, addr2sym[&pc]);
                break;
            }
            println!("Reset @ {:x}", pc);
            break;
        }
        
        // Invalid memory
        else if invalid_mem(ram.clone(), pc) 
        {
            println!("Invalid memory access @ {:x}", pc);
            break;
        }

        // Execute
        else
        {
            let r = ram[pc] - ram[pc+1];
            ram[pc+2] = r;
            if r < 0 
            {
                // jump
                let j = ram[pc+3] as u32;
                pc = j as usize;
            } else {
                pc+=4;
            }
        }
    }
}

fn alloc_ram(bytes: Vec<u8>) -> Vec<i32>
{
    // usti has 4Mword of memory 
    let mut ram: Vec<i32> = Vec::with_capacity(MEM_SIZE); 
    ram.resize(MEM_SIZE, 0);

    for lp in 0..bytes.len()/std::mem::size_of::<i32>()
    {
        let start = lp*std::mem::size_of::<i32>();
        let slice = &bytes[start..start+std::mem::size_of::<i32>()];
        let value = i32::from_be_bytes(slice.try_into().expect("Not a 32bit program!"));
        ram[lp] = value;
    }

    return ram;
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
