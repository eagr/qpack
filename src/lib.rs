mod decoder;
mod encoder;
mod field;
mod huffman;
mod static_table;

#[derive(Debug, Clone, Copy)]
pub enum DecoderError {
    HuffmanInvalidCode,
}
