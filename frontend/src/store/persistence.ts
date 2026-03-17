/**
 * Persistence and migration utilities for the Zustand store.
 *
 * When upgrading from the legacy BlockchainContext approach, this module
 * detects old `inktix_*` and `blockchain_*` localStorage keys, reads their
 * values into a unified shape, and removes the originals so the new
 * `inktix-store` key becomes the single source of truth.
 *
 * @module store/persistence
 *
 * Exported interfaces:
 * - {@link MigratedState} - Shape of migrated localStorage values
 *
 * Exported functions:
 * - {@link migrateFromOldKeys} - One-time migration from legacy keys
 */

// Old keys used by BlockchainContext and BlockchainService
const OLD_KEYS = [
  "inktix_isConnected",
  "inktix_isWalletConnected",
  "inktix_accounts",
  "inktix_selectedAccount",
  "blockchain_connected",
  "blockchain_endpoint",
] as const;

export interface MigratedState {
  isConnected: boolean;
  isWalletConnected: boolean;
  accounts: any[];
  selectedAccount: any | null;
  endpoint: string;
}

/**
 * Reads old localStorage keys and returns migrated state.
 * Deletes old keys after reading.
 */
export function migrateFromOldKeys(): Partial<MigratedState> | null {
  if (typeof window === "undefined") return null;

  const hasOldKeys = OLD_KEYS.some((key) => localStorage.getItem(key) !== null);
  if (!hasOldKeys) return null;

  const migrated: Partial<MigratedState> = {};

  // Read old values
  const isConnected = localStorage.getItem("inktix_isConnected");
  const isWalletConnected = localStorage.getItem("inktix_isWalletConnected");
  const accounts = localStorage.getItem("inktix_accounts");
  const selectedAccount = localStorage.getItem("inktix_selectedAccount");
  const endpoint = localStorage.getItem("blockchain_endpoint");

  if (isConnected !== null) {
    migrated.isConnected = isConnected === "true";
  }
  if (isWalletConnected !== null) {
    migrated.isWalletConnected = isWalletConnected === "true";
  }
  if (accounts !== null) {
    try {
      migrated.accounts = JSON.parse(accounts);
    } catch {
      migrated.accounts = [];
    }
  }
  if (selectedAccount !== null) {
    try {
      migrated.selectedAccount = JSON.parse(selectedAccount);
    } catch {
      migrated.selectedAccount = null;
    }
  }
  if (endpoint !== null) {
    migrated.endpoint = endpoint;
  }

  // Delete old keys
  OLD_KEYS.forEach((key) => localStorage.removeItem(key));

  return migrated;
}
