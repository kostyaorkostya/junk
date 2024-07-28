use std::io::Read;

struct RotDecoder<R: Read> {
    input: R,
    rot: u8,
}

fn decode(c: u8, rot: u8) -> u8 {
    let decode = |a, z, c, rot| {
        if c >= a + rot {
            c - rot
        } else {
            z + 1 - (rot - (c - a))
        }
    };
    if 'a' as u8 <= c && c <= 'z' as u8 {
        decode('a' as u8, 'z' as u8, c, rot)
    } else if 'A' as u8 <= c && c <= 'Z' as u8 {
        decode('A' as u8, 'Z' as u8, c, rot)
    } else {
        c
    }
}

impl<R> Read for RotDecoder<R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        match self.input.read(buf) {
            e @ Result::Err(_) => e,
            Ok(len) => {
                for i in 0..len {
                    buf[i] = decode(buf[i], self.rot);
                }
                Ok(len)
            }
        }
    }
}

// Implement the `Read` trait for `RotDecoder`.

fn main() {
    let mut rot = RotDecoder {
        input: "Gb trg gb gur bgure fvqr!".as_bytes(),
        rot: 13,
    };
    let mut result = String::new();
    rot.read_to_string(&mut result).unwrap();
    println!("{}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn joke() {
        let mut rot = RotDecoder {
            input: "Gb trg gb gur bgure fvqr!".as_bytes(),
            rot: 13,
        };
        let mut result = String::new();
        rot.read_to_string(&mut result).unwrap();
        assert_eq!(&result, "To get to the other side!");
    }

    #[test]
    fn binary() {
        let input: Vec<u8> = (0..=255u8).collect();
        let mut rot = RotDecoder::<&[u8]> {
            input: input.as_ref(),
            rot: 13,
        };
        let mut buf = [0u8; 256];
        assert_eq!(rot.read(&mut buf).unwrap(), 256);
        for i in 0..=255 {
            if input[i] != buf[i] {
                assert!(input[i].is_ascii_alphabetic());
                assert!(buf[i].is_ascii_alphabetic());
            }
        }
    }
}
