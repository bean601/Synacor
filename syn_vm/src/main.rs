use std::fs;
use std::fs::File;
use std::io::{self, prelude::*};
use std::str;

struct Vm {
    mem: [u16; 32767],
    index: usize,
    ax: u8, //reg 0
    bx: u8, //reg 1
    cx: u8, //reg 2
    dx: u8, //reg 3
    ex: u8, //reg 4
    fx: u8, //reg 5
    gx: u8, //reg 6
    hx: u8, //reg 7
}

fn main() -> io::Result<()> {
    let mut vm = Vm { 
        index: 0,
        ax: 0,
        bx: 0,
        cx: 0,
        dx: 0,
        ex: 0,
        fx: 0,
        gx: 0,
        hx: 0,
        mem: [0; 32767]
    };

    //let mut file = File::open("../../challenge.bin")?;
    //let reader = BufReader::new(file);
    let data = fs::read("../../challenge.bin").expect("Can't find challenge.bin file");

    loop {
        
        //println!("{:?}", data[vm.index].to_string());
        
        let op = data[vm.index];
        match op {
            0 => {
                panic!("Exiting from opcode 0");
            }
            19 => {
                vm.index += 2;
                let c = char::from_u32(data[vm.index] as u32).unwrap();
                print!("{}", c);
                //print!("{:?}", data[vm.index].to_string());
            },
            21 => {
                //println!("noop");
                
            },
            _ => {
                //println!("{}", data[vm.index]);
            }
        }
        //println!("t");
        if vm.index > 1000 {
            break;
        }
        
        vm.index += 1;
    }

    //println!("The bytes: {:?}", &buffer[..bytes_read]);
    Ok(())
}
