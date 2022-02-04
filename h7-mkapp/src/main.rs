use std::{env, fs};

fn main() {
    let mut args = env::args().skip(1);
    let input = args.next().expect("No input");
    let output = args.next().expect("No output");

    println!("input = {}", input);
    println!("input = {}", output);

    let input_data = fs::read(&input).unwrap();
    if input_data.len() < 4 {
        panic!("Not ennough data");
    }

    let boot_address =
        u32::from_le_bytes([input_data[0], input_data[1], input_data[2], input_data[3]]);
    println!("Boot address: 0x{:08x}", boot_address);

    let mut output_data = vec![0u8; input_data.len() + 4];
    // Convert address to big endian (the proper way...)
    output_data[..4].copy_from_slice(&boot_address.to_be_bytes());
    output_data[4..input_data.len()].copy_from_slice(&input_data[4..]);
    let crc = crc::Crc::<u32>::new(&crc::CRC_32_MPEG_2);
    let mut digest = crc.digest();
    digest.update(&output_data[..input_data.len()]);
    let crc_value = digest.finalize();
    println!("CRC: 0x{:08x}", crc_value);
    // Store CRC as big endian as well
    output_data[input_data.len()..].copy_from_slice(&crc_value.to_be_bytes());

    fs::write(&output, &output_data).unwrap();
    println!("Done");
}