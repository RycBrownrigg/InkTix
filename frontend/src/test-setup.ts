import "@testing-library/jest-dom/vitest";

// Polyfill localStorage for jsdom environment if not available or broken
const store: Record<string, string> = {};
const localStoragePolyfill: Storage = {
  getItem: (key: string) => store[key] ?? null,
  setItem: (key: string, value: string) => { store[key] = String(value); },
  removeItem: (key: string) => { delete store[key]; },
  clear: () => { Object.keys(store).forEach((k) => delete store[k]); },
  key: (index: number) => Object.keys(store)[index] ?? null,
  get length() { return Object.keys(store).length; },
};

try {
  // Test if localStorage works
  window.localStorage.setItem("__test__", "1");
  window.localStorage.removeItem("__test__");
} catch {
  Object.defineProperty(window, "localStorage", {
    value: localStoragePolyfill,
    writable: true,
  });
}
