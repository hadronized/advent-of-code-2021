const INPUT: &str = include_str!("input.txt");
const SAMPLES: [&str; 4] = [
  "8A004A801A8002F478",
  "620080001611562C8802118E34",
  "C0015000016115A2E0802F182340",
  "A0016C880162017C3686B18A3D4780",
];

#[derive(Clone, Debug, Eq, PartialEq)]
struct Header {
  version: u8,
  type_id: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Packet {
  Literal {
    header: Header,
    value: u128,
  },

  Operator {
    header: Header,
    length_type_id: u8,
    packets: Vec<Packet>,
  },
}

#[derive(Clone, Debug)]
struct Decoder {
  bytes: Vec<u8>,
  bitpos: usize, // the position in bit in the byte array
}

impl Decoder {
  fn new(input: &str) -> Self {
    let digits: Vec<_> = input.chars().flat_map(|hexa| hexa.to_digit(16)).collect();
    let bytes = digits.chunks(2).map(|d| (d[0] << 4 | d[1]) as u8).collect();

    Self { bytes, bitpos: 0 }
  }

  // Get the next N bits.
  //
  // This is limited to 16 bits.
  //
  // Note: I hate this puzzle.
  fn next_bits(&mut self, n: usize) -> u16 {
    // how many bits are still available from the current byte
    //
    // 0000000011111111
    //      ^ position is 5, so we have 3 bits still available
    let bits_avail = 8 - self.bitpos % 8;

    if bits_avail >= n {
      // we have enough in the current byte; compute the new position and right shift to align correctly
      let shift = bits_avail - n;
      let byte = (self.bytes[self.bitpos / 8] as u16 >> shift) & ((1 << n) - 1);
      self.bitpos += n;
      byte
    } else {
      // this is the annoying part; first, get all the bits from the current byte
      let byte0 = (self.bytes[self.bitpos / 8] as u16) & ((1 << bits_avail) - 1);
      self.bitpos += bits_avail;

      // then, left shift those bits as most significant bits for the final number
      let mut final_byte = byte0;

      // then, we need to get the remaining bits; compute the number of bytes we need
      let n = n - bits_avail;
      let bytes_needed = n / 8;

      // while we require more than 1 byte, the problem is trivial; just copy and left shift the bytes
      for _ in 1..=bytes_needed {
        let byte = self.bytes[self.bitpos / 8] as u16;
        self.bitpos += 8;
        final_byte = (final_byte << 8) | byte;
      }

      // the remaining bits can be extracted from the last byte
      let n = n % 8;
      let shift = 8 - n;
      let byte = (self.bytes[self.bitpos / 8] as u16 >> shift) & ((1 << n) - 1);
      self.bitpos += n;

      final_byte = (final_byte << n) | byte;

      final_byte
    }
  }

  fn decode(&mut self) -> Packet {
    let version = self.next_bits(3) as u8;
    let type_id = self.next_bits(3) as u8;
    let header = Header { version, type_id };

    match type_id {
      // literal value
      4 => {
        let mut value = 0u128;
        loop {
          let bits = self.next_bits(5) as u128;
          value = (value << 4) | (bits & 0xF);

          if (bits >> 4) & 0x1 == 0 {
            break Packet::Literal { header, value };
          }
        }
      }

      // operator
      _ => {
        let length_type_id = self.next_bits(1) as u8;
        let mut packets = Vec::new();

        if length_type_id == 0 {
          let total_length_bits = self.next_bits(15) as usize;
          let mut bits_read = 0;

          while bits_read < total_length_bits {
            let bitpos = self.bitpos;
            packets.push(self.decode());
            bits_read += self.bitpos - bitpos;
          }
        } else {
          let sub_packets_nb = self.next_bits(11);

          for _ in 0..sub_packets_nb {
            packets.push(self.decode());
          }
        }

        Packet::Operator {
          header,
          length_type_id,
          packets,
        }
      }
    }
  }
}

fn solve1(input: &str) -> u32 {
  let mut decoder = Decoder::new(input);
  checksum(&decoder.decode())
}

fn solve2(input: &str) -> u64 {
  let mut decoder = Decoder::new(input);
  eval(&decoder.decode())
}

fn checksum(packet: &Packet) -> u32 {
  match packet {
    Packet::Literal { header, .. } => header.version as _,
    Packet::Operator {
      header, packets, ..
    } => header.version as u32 + packets.iter().map(checksum).sum::<u32>(),
  }
}

fn eval(packet: &Packet) -> u64 {
  match packet {
    Packet::Literal { value, .. } => *value as _,
    Packet::Operator {
      header, packets, ..
    } => match header.type_id {
      0 => packets.iter().map(eval).sum::<u64>(),
      1 => packets.iter().map(eval).product::<u64>(),
      2 => packets.iter().map(eval).min().unwrap(),
      3 => packets.iter().map(eval).max().unwrap(),
      5 => {
        if eval(&packets[0]) > eval(&packets[1]) {
          1
        } else {
          0
        }
      }
      6 => {
        if eval(&packets[0]) < eval(&packets[1]) {
          1
        } else {
          0
        }
      }
      7 => {
        if eval(&packets[0]) == eval(&packets[1]) {
          1
        } else {
          0
        }
      }

      id => panic!("unknown type id: {}", id),
    },
  }
}

fn main() {
  println!("part1");
  for (i, sample) in SAMPLES.iter().enumerate() {
    println!("sample {}: {}", i, solve1(sample));
  }
  println!("input: {}", solve1(INPUT));

  println!("part2");
  println!("sample C200B40A82: {}", solve2("C200B40A82"));
  println!(
    "sample 9C0141080250320F1802104A08: {}",
    solve2("9C0141080250320F1802104A08")
  );
  println!("input: {}", solve2(INPUT));
}

#[cfg(test)]
mod test {
  use super::*;

  const SAMPLE_LIT: &str = "D2FE28";
  const SAMPLE_OP0: &str = "38006F45291200";
  const SAMPLE_OP1: &str = "EE00D40C823060";

  #[test]
  fn hexa_bin() {
    let decoder = Decoder::new(SAMPLE_LIT);
    assert_eq!(decoder.bytes, vec![210, 254, 40]);
  }

  #[test]
  fn next_bits() {
    let mut decoder = Decoder::new(SAMPLE_LIT);
    assert_eq!(decoder.next_bits(3), 0b110);
    assert_eq!(decoder.next_bits(3), 0b100);
    assert_eq!(decoder.next_bits(2), 0b10);
    assert_eq!(decoder.next_bits(9), 0b111111100);
    assert_eq!(decoder.next_bits(7), 0b0101000);

    let mut decoder = Decoder::new(SAMPLE_LIT);
    assert_eq!(decoder.next_bits(3), 0b110);
    assert_eq!(decoder.next_bits(3), 0b100);
    assert_eq!(decoder.next_bits(5), 0b10111);

    let mut decoder = Decoder::new(SAMPLE_LIT);
    assert_eq!(decoder.next_bits(16), 0b1101001011111110);
  }

  #[test]
  fn literal_packet() {
    let mut decoder = Decoder::new(SAMPLE_LIT);

    assert_eq!(
      decoder.decode(),
      Packet::Literal {
        header: Header {
          version: 6,
          type_id: 4
        },

        value: 2021,
      }
    );
  }

  #[test]
  fn operator0_packet() {
    let mut decoder = Decoder::new(SAMPLE_OP0);

    assert_eq!(
      decoder.decode(),
      Packet::Operator {
        header: Header {
          version: 1,
          type_id: 6
        },

        length_type_id: 0,

        packets: vec![
          Packet::Literal {
            header: Header {
              version: 6,
              type_id: 4
            },
            value: 10,
          },
          Packet::Literal {
            header: Header {
              version: 2,
              type_id: 4
            },
            value: 20
          }
        ]
      }
    );
  }

  #[test]
  fn operator1_packet() {
    let mut decoder = Decoder::new(SAMPLE_OP1);

    assert_eq!(
      decoder.decode(),
      Packet::Operator {
        header: Header {
          version: 7,
          type_id: 3
        },

        length_type_id: 1,

        packets: vec![
          Packet::Literal {
            header: Header {
              version: 2,
              type_id: 4
            },
            value: 1,
          },
          Packet::Literal {
            header: Header {
              version: 4,
              type_id: 4
            },
            value: 2
          },
          Packet::Literal {
            header: Header {
              version: 1,
              type_id: 4
            },
            value: 3
          }
        ]
      }
    );
  }
}
