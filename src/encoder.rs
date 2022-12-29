use bytes::BufMut;

/*
https://www.rfc-editor.org/rfc/rfc9204.html#section-4.1.1
https://www.rfc-editor.org/rfc/rfc7541#section-5.1

An integer is represented in two parts: a prefix that fills the
current octet and an optional list of octets that are used if the
integer value does not fit within the prefix.  The number of bits of
the prefix (called N) is a parameter of the integer representation.

The implementation works on these premises.

* The given prefix length is not greater than 8.
* The given prefix length and start byte can work together,
  i.e. prefix doesn't overlap with the data in start byte.
*/
fn encode_int<B: BufMut>(mut i: u64, prefix_len: usize, start_byte: u8, out: &mut B) {
    let prefix = 1 << prefix_len - 1;

    if i < prefix {
        out.put_u8(start_byte | i as u8);
        return;
    }

    out.put_u8(start_byte | prefix as u8);

    i -= prefix;

    while i >= 0x80 {
        out.put_u8(((i % 0x80) | 0x80) as u8);
        i >>= 7;
    }

    out.put_u8(i as u8);
}
