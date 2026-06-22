const WS_URL = 'ws://localhost:7171';

let ws = null;
const handlers = {};

export function connect() {
  return new Promise((resolve, reject) => {
    ws = new WebSocket(WS_URL);
    ws.addEventListener('open', () => resolve());
    ws.addEventListener('error', () => reject(new Error('Não foi possível conectar ao servidor.')));
    ws.addEventListener('message', (e) => {
      const msg = JSON.parse(e.data);
      const handler = handlers[msg.op];
      if (handler) handler(msg.data);
    });
    ws.addEventListener('close', () => {
      const handler = handlers['_disconnect'];
      if (handler) handler();
    });
  });
}

export function send(op, data = {}) {
  if (ws && ws.readyState === WebSocket.OPEN) {
    ws.send(JSON.stringify({ op, data }));
  }
}

export function on(op, fn) {
  handlers[op] = fn;
}

export function off(op) {
  delete handlers[op];
}

export function disconnect() {
  if (ws) { ws.close(); ws = null; }
}
