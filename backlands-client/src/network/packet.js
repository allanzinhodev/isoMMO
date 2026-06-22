const enc = new TextEncoder();
const dec = new TextDecoder();

export class PacketWriter {
  constructor() { this._buf = []; }

  writeU8(v)  { this._buf.push(v & 0xFF); }
  writeU16(v) { this._buf.push(v & 0xFF, (v >> 8) & 0xFF); }
  writeU32(v) { this._buf.push(v & 0xFF, (v >> 8) & 0xFF, (v >> 16) & 0xFF, (v >>> 24) & 0xFF); }

  writeString(s) {
    const bytes = enc.encode(s);
    this.writeU16(bytes.length);
    bytes.forEach(b => this._buf.push(b));
  }

  // Returns an ArrayBuffer: [length u16 LE][opcode u8][payload]
  build(opcode) {
    const length = 1 + this._buf.length;
    const out = new Uint8Array(2 + length);
    out[0] = length & 0xFF;
    out[1] = (length >> 8) & 0xFF;
    out[2] = opcode;
    this._buf.forEach((b, i) => { out[3 + i] = b; });
    return out.buffer;
  }
}

export class PacketReader {
  constructor(buffer) {
    this._view = new DataView(buffer);
    this._pos  = 3; // skip [length u16][opcode u8]
  }

  readU8()  { return this._view.getUint8(this._pos++); }
  readU16() { const v = this._view.getUint16(this._pos, true); this._pos += 2; return v; }
  readU32() { const v = this._view.getUint32(this._pos, true); this._pos += 4; return v; }

  readString() {
    const len   = this.readU16();
    const bytes = new Uint8Array(this._view.buffer, this._pos, len);
    this._pos  += len;
    return dec.decode(bytes);
  }
}
