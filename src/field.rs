// https://www.rfc-editor.org/rfc/rfc9204.html#section-1.1
pub trait FieldLine {
    fn name(&self) -> &[u8];

    fn value(&self) -> &[u8];
}

impl FieldLine for (&str, &str) {
    fn name(&self) -> &[u8] {
        self.0.as_bytes()
    }

    fn value(&self) -> &[u8] {
        self.1.as_bytes()
    }
}

impl FieldLine for (&[u8], &[u8]) {
    fn name(&self) -> &[u8] {
        self.0
    }

    fn value(&self) -> &[u8] {
        self.1
    }
}
