mod table;

use table::{DECODE_IS_SYM, DECODE_MAY_FINISH, DECODE_TABLE, ENCODE_TABLE};

use crate::DecoderError;

use bytes::{BufMut, BytesMut};

pub fn encode(src: &[u8], out: &mut BytesMut) {
    let mut buf: u64 = 0;

    // longest code spans 30 bits
    // min multiple of 8 larger than 30 + 7
    let mut buf_avail = 40;

    for &sym in src {
        let (len, code) = ENCODE_TABLE[sym as usize];

        buf_avail -= len;
        buf |= code << buf_avail;

        // have complete bytes
        while buf_avail <= 32 {
            out.put_u8((buf >> 32) as u8);

            buf <<= 8;
            buf_avail += 8;
        }
    }

    // if there are unwritten bits
    if buf_avail < 40 {
        buf |= (1 << buf_avail) - 1;
        out.put_u8((buf >> 32) as u8);
    }
}

pub fn decode(code_stream: &[u8], out: &mut BytesMut) -> Result<(), DecoderError> {
    let mut decoder = Decoder::new();

    // little endian
    for code in code_stream {
        if let Some(sym) = decoder.decode(code >> 4) {
            out.put_u8(sym);
        }

        if let Some(sym) = decoder.decode(code & 0xf) {
            out.put_u8(sym);
        }
    }

    if decoder.may_finish() {
        Ok(())
    } else {
        Err(DecoderError::HuffmanInvalidCode)
    }
}

#[derive(Default)]
struct Decoder {
    state: usize,
    may_finish: bool,
}

impl Decoder {
    fn new() -> Self {
        Self::default()
    }

    fn decode(&mut self, code: u8) -> Option<u8> {
        let (state, sym) = DECODE_TABLE[self.state][code as usize];

        self.state = state & 0xff;
        self.may_finish = state & DECODE_MAY_FINISH == DECODE_MAY_FINISH;

        if state & DECODE_IS_SYM == DECODE_IS_SYM {
            Some(sym)
        } else {
            None
        }
    }

    fn may_finish(&self) -> bool {
        self.may_finish || self.state == 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn encode_decode() {
        const SAMPLES: &[&[u8]] = &[
            b"0",
            b"/",
            b"301",
            b":status",
            b":path",
            b"alt-svc",
            b"cache-control",
            b"content-type",
            b"early-data",
            b"google.com",
            b"text/html,application/xhtml+xml,*/*;q=0.8",
        ];

        for s in SAMPLES {
            let mut enc_out = BytesMut::with_capacity(s.len());
            encode(s, &mut enc_out);

            let mut dec_out = BytesMut::with_capacity(s.len());
            decode(&enc_out, &mut dec_out).unwrap();

            assert_eq!(&dec_out[..], &s[..]);
        }
    }
}
