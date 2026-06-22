const store = new Map();

export const state = {
  get: (key) => store.get(key),
  set: (key, value) => store.set(key, value),
  has: (key) => store.has(key),
  delete: (key) => store.delete(key),
};
