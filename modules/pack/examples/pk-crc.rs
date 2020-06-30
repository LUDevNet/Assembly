use std::env;

#[derive(Debug)]
enum MainError {}

const CRC_POLY: u32 = 0x04C11DB7;
const CRC_INIT: u32 = 0xFFFFFFFF;
const CRC_FXOR: u32 = 0x00000000;

fn update_crc(crc: &mut u32, b: u8) {
    *crc ^= u32::from(b) << 24; /* Move byte to MSB */
    for _i in 0..8 {
        if (*crc & 0x80000000) == 0 {
            *crc <<= 1;
        } else {
            *crc = (*crc << 1) ^ CRC_POLY;
        }
    }
}

fn calculate_crc(path: &[u8]) -> u32 {
    let mut crc: u32 = CRC_INIT;
    /* Process the actual string */
    for bp in path {
        let mut b = *bp;
        /* Perform some cleanup on the input */
        if b == '/' as u8 {
            b = '\\' as u8;
        }
        if 'A' as u8 <= b && b <= 'Z' as u8 {
            b += 'a' as u8 - 'A' as u8;
        }

        update_crc(&mut crc, b);
    }
    /* I have no clue why this was added */
    for _i in 0..4 {
        update_crc(&mut crc, 0);
    }
    crc ^= CRC_FXOR;
    return crc;
}

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
        let crc = calculate_crc(filename.as_str().as_bytes());
        println!("{:10} {}", crc, filename);
        Ok(())
    }
}
