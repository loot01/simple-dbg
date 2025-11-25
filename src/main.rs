use std::env;
use std::process;
use std::path::Path;
use std::fs;

fn main() -> process::ExitCode {
    let args: Vec<String> = env::args().collect();
    let mut flags: [bool; 10] = [false; 10];
    let mut file_name = String::new();
    let file_buf : Vec<u8>;

    if args.len() < 2 {
        print_usage();
        return process::ExitCode::FAILURE;
    }

    check_args(&args, &mut flags, &mut file_name);

    if file_name.is_empty() {
        print_usage();
        return process::ExitCode::FAILURE;
    }

    file_buf = match fs::read(&Path::new(&file_name)) {
            Err(why) => panic!("Couldn't open {file_name}: {why}"),
            Ok(file_buf) => file_buf,
    };
    hexdump(&file_buf);
    return process::ExitCode::SUCCESS;
}

fn check_args(args: &Vec<String>, _flags: &mut [bool; 10], file_name: &mut String) {
    let mut file_name_found: bool = false;

    for arg in args[1..].iter() {
        if arg.chars().nth(0).unwrap() == '-'
        {
            match arg.chars().nth(1).unwrap() {
                'd' => println!("-d ENTERED!"),
                _ => println!("inexistant flag ENTERED!"),
            }
        }

        else {
            if !file_name_found {
                *file_name = arg.clone();
                file_name_found = true;
            }
        }
    }
}

fn print_usage() {
    println!("Usage: ./simple-dbg [FLAGS] [BINARY]");
    println!("FLAGS: <TODO>");
}

fn hexdump(buffer: &Vec<u8>)
{
    for (i, byte_pair) in buffer.chunks_exact(2).enumerate() {
        let short = u16::from_le_bytes([byte_pair[0], byte_pair[1]]);
        print!("{short:0>4x} ");
        if (i + 1) % 8 == 0  {
            print!("\n");
        }
    }
}
