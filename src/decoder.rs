use crate::DecoderError;

use bytes::Buf;

/*
https://www.rfc-editor.org/rfc/rfc9204.html#section-4.1.1
https://www.rfc-editor.org/rfc/rfc7541#section-5.1

The implementation works on these premises.

* The given prefix length is not greater than 8.
* The given prefix length and start byte can work together,
  i.e. prefix doesn't overlap with the data in start byte.
*/
fn decode_int<B: Buf>(inp: &mut B, prefix_len: usize) -> Result<u64, DecoderError> {
    let start = inp.get_u8();
    let prefix = 1 << prefix_len - 1;
    let mut i = (start & prefix) as u64;

    if i < prefix as u64 {
        return Ok(i);
    }

    let mut byte = inp.get_u8();
    let mut shift = 0u8;

    while byte & 0x80 == 0x80 {
        i += ((byte & 0x7f) as u64) << shift;

        byte = inp.get_u8();
        shift += 7;
    }

    i += ((byte & 0x7f) as u64) << shift;

    Ok(i)
}
