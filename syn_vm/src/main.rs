use std::fs::File;
use std::io::{self, prelude::*};
use std::str;

struct Vm {
    mem: [u16, 32768],
    index: usize,
    data: Vec<u8> 
}

fn main() -> io::Result<()> {

    


    // let mut file = File::open("../../challenge.bin")?;
    // //let reader = BufReader::new(file);
    // let mut buffer = [0; 20];

    // // read up to 10 bytes
    // let bytes_read = file.read(&mut buffer[..])?;

    // let mut last_op = 0_u8;

    // for mut n in 0..bytes_read - 1 {
    //     let byte = &buffer[n];
    //     //println!("Current bytes: {:?}", &buffer[n]);

    //     if n % 2 == 0 {
    //         if last_op == 19_u8 {
    //             if *byte as char == 'u' {
    //                 //carriage return
    //                 println!();
    //             } else {
    //                 print!("{:?}", *byte as char);
    //             }
    //         } else {
    //             if byte == &21_u8 {
    //                 // noop
    //             } else if byte == &19_u8 {
    //                 // print
    //                 last_op = 19_u8;
    //             }
    //         }
    //     }
    // }

    //println!("The bytes: {:?}", &buffer[..bytes_read]);
    Ok(())
}
