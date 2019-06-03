const POLY: u16 = 0x1021;
const INIT: u16 = 0xffff;

/*
Zwei vergleichende Implementierungen für CRC16 mit CCITT-FALSE polynominal.

First one is a simple bit by bit implementation. The 0x00 0x00 augmentation
has to be added manually to the input array.

The second one is from nRF51 SDK. No manual augmentation needed.
*/

fn crc(bytes: &[u8]) -> u16 {
    bytes.iter()
        .fold(INIT,
              |acc, &byte| {
                  (0..8).fold((acc, byte),
                              |(acc, byte), _| {
                                  let next_bit = (byte >> 7) as u16;
                                  if acc & 0x8000 == 0x8000 {
                                      (acc << 1 ^ POLY ^ next_bit, byte << 1)
                                  } else {
                                      (acc << 1        ^ next_bit, byte << 1)
                                  }
                              }).0
              })
}

fn bl_crc(bytes: &[u8]) -> u16 {
    bytes.iter()
        .fold(0xFFFF,
              |crc, byte| {
                  let crc1 = crc >> 8 | crc << 8;
                  let crc2 = crc1 ^ (*byte as u16);
                  let crc3 = crc2 ^ ((crc2 & 0xFF) >> 4);
                  let crc4 = crc3 ^ ((crc3 << 8) << 4);
                  crc4 ^ (((crc4 & 0xFF) << 4) << 1)
              })
}

fn main() {
    let data = vec![0x11, 0x22, 0x00, 0x00];
    println!("CRC: {:x}", crc(&data[0 .. 4]));
    println!("BL-CRC: {:x}", bl_crc(&data[0 .. 2]));
}
