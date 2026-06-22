/// Little-Endian binary buffer for TFS-style packet encoding/decoding.
/// Packet wire format: [Length: u16 LE][Opcode: u8][Payload: N bytes]
/// where Length = 1 (opcode) + payload_len.

pub struct ByteBuffer {
    buf: Vec<u8>,
    pos: usize,
}

impl ByteBuffer {
    pub fn new() -> Self {
        Self { buf: Vec::new(), pos: 0 }
    }

    pub fn from_bytes(data: Vec<u8>) -> Self {
        Self { buf: data, pos: 0 }
    }

    // --- Writers ---

    pub fn write_u8(&mut self, v: u8) {
        self.buf.push(v);
    }

    pub fn write_u16(&mut self, v: u16) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    pub fn write_u32(&mut self, v: u32) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    pub fn write_string(&mut self, s: &str) {
        let bytes = s.as_bytes();
        self.write_u16(bytes.len() as u16);
        self.buf.extend_from_slice(bytes);
    }

    /// Wraps the buffer in a full packet: [length u16][opcode u8][payload].
    pub fn into_packet(self, opcode: u8) -> Vec<u8> {
        let length = (1u16).saturating_add(self.buf.len() as u16);
        let mut packet = Vec::with_capacity(3 + self.buf.len());
        packet.extend_from_slice(&length.to_le_bytes());
        packet.push(opcode);
        packet.extend_from_slice(&self.buf);
        packet
    }

    // --- Readers ---

    pub fn read_u8(&mut self) -> Option<u8> {
        if self.pos < self.buf.len() {
            let v = self.buf[self.pos];
            self.pos += 1;
            Some(v)
        } else {
            None
        }
    }

    pub fn read_u16(&mut self) -> Option<u16> {
        if self.pos + 2 <= self.buf.len() {
            let v = u16::from_le_bytes([self.buf[self.pos], self.buf[self.pos + 1]]);
            self.pos += 2;
            Some(v)
        } else {
            None
        }
    }

    pub fn read_u32(&mut self) -> Option<u32> {
        if self.pos + 4 <= self.buf.len() {
            let v = u32::from_le_bytes([
                self.buf[self.pos],
                self.buf[self.pos + 1],
                self.buf[self.pos + 2],
                self.buf[self.pos + 3],
            ]);
            self.pos += 4;
            Some(v)
        } else {
            None
        }
    }

    pub fn read_string(&mut self) -> Option<String> {
        let len = self.read_u16()? as usize;
        if self.pos + len <= self.buf.len() {
            let s = String::from_utf8_lossy(&self.buf[self.pos..self.pos + len]).into_owned();
            self.pos += len;
            Some(s)
        } else {
            None
        }
    }
}

/// Parse a raw WebSocket binary frame → (opcode, payload buffer).
/// Expects wire format: [length: u16 LE][opcode: u8][payload…]
pub fn parse_packet(data: &[u8]) -> Option<(u8, ByteBuffer)> {
    if data.len() < 3 { return None; }
    let opcode  = data[2];
    let payload = ByteBuffer::from_bytes(data[3..].to_vec());
    Some((opcode, payload))
}
