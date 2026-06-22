import { PacketReader } from './packet.js';

const WS_URL = 'ws://localhost:7171';

let ws = null;
const handlers = {};

export function connect() {
  return new Promise((resolve, reject) => {
    ws = new WebSocket(WS_URL);
    ws.binaryType = 'arraybuffer';

    ws.addEventListener('open',  () => resolve());
    ws.addEventListener('error', () => reject(new Error('Não foi possível conectar ao servidor.')));

    ws.addEventListener('message', (e) => {
      if (!(e.data instanceof ArrayBuffer) || e.data.byteLength < 3) return;
      const opcode  = new DataView(e.data).getUint8(2);
      const handler = handlers[opcode];
      if (handler) handler(new PacketReader(e.data));
    });

    ws.addEventListener('close', () => {
      const handler = handlers['_disconnect'];
      if (handler) handler();
    });
  });
}

// Send a pre-built ArrayBuffer packet
export function send(buffer) {
  if (ws && ws.readyState === WebSocket.OPEN) ws.send(buffer);
}

export function on(opcode, fn)  { handlers[opcode] = fn; }
export function off(opcode)     { delete handlers[opcode]; }
export function disconnect()    { if (ws) { ws.close(); ws = null; } }
