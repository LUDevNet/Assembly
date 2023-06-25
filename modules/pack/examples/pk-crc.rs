use std::env;

use assembly_pack::crc::CRC;

#[derive(Debug)]
enum MainError {}

fn print_usage(program: &str) {
    println!("Usage: {} PATH", program);
}

fn main() -> Result<(), MainError> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    if args.len() <= 1 {
        print_usage(&program);
        Ok(())
    } else {
        let filename = args[1].clone();
        let crc = CRC::from_path(&filename);
        println!("{:10} {}", crc, filename);
        Ok(())
    }
}
