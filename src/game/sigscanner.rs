use std::collections::BTreeMap;

use crate::game::{sig, sig::CPointer };
use crate::game::Process;

pub struct SigScanner {
    pub cached_sigs: BTreeMap<String, usize>
}

impl SigScanner {
    fn parse_pattern_string(s: &str) -> Vec<i16> {
        s.split_whitespace()
            .map(|byte_str|
                 if byte_str == "??" || byte_str == "?" {
                     -1 as i16
                 } else {
                     i16::from_str_radix(byte_str, 16)
                         .expect("Invalid hex byte")
                 }
            )
            .collect()
    }
    pub fn new() -> Self {
        Self {
            cached_sigs: BTreeMap::new(),
        }
    }
    pub fn cache_sigs(&mut self, process: &Process) {
        for signature in sig::SIGNATURES {
            let pattern = Self::parse_pattern_string(signature.sig);
            let module = process.modules.get(signature.module).unwrap();
            let base_address = module.lpBaseOfDll as usize;

            // read buffer of module into memory
            let bytes = process.read_buffer(
                base_address,
                module.SizeOfImage as usize
            ).unwrap();

            for i in 0..=bytes.len() - pattern.len() {

                let mut found = true;

                for (j , &pattern_byte) in pattern.iter().enumerate() {
                    if pattern_byte != -1 && pattern_byte != bytes[i + j].into() {
                        found = false;
                        break;
                    }
                }

                if found {
                    let pattern_address = base_address + i;
                    let address: usize = match signature.pointer {
                        CPointer::Absolute => {
                            let jmp_offset: u32 = process.read(pattern_address + signature.offset).unwrap();
                            pattern_address + jmp_offset as usize + 4
                        },
                        CPointer::Relative => {
                            let rip = pattern_address + 7;
                            let offset: u32 = process.read(pattern_address + signature.offset).unwrap();
                            rip + offset as usize
                        },
                        _ => {
                            pattern_address + signature.offset
                        }
                    };
                    self.cached_sigs.insert(String::from(signature.name), address);
                }
            }
        }
    }
}
