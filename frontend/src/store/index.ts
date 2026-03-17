/**
 * Combined Zustand store for all InkTix client-side state.
 *
 * Composes four domain slices (connection, wallet, contract, data) into a
 * single store with `zustand/persist` middleware for localStorage
 * persistence of key fields across page reloads.
 *
 * @module store/index
 *
 * Exported types:
 * - {@link InkTixStore} - Union of all slice interfaces plus initialization
 *
 * Exported hooks:
 * - {@link useInkTixStore} - Zustand hook providing the combined store
 */
"use client";

import { create } from "zustand";
import { persist, createJSONStorage } from "zustand/middleware";
import { BlockchainService } from "../services/blockchain";
import {
  ConnectionSlice,
  createConnectionSlice,
} from "./slices/connectionSlice";
import { WalletSlice, createWalletSlice } from "./slices/walletSlice";
import { ContractSlice, createContractSlice } from "./slices/contractSlice";
import { DataSlice, createDataSlice } from "./slices/dataSlice";
import { migrateFromOldKeys } from "./persistence";

// Combined store type
export type InkTixStore = ConnectionSlice &
  WalletSlice &
  ContractSlice &
  DataSlice & {
    _service: BlockchainService | null;
    _initialized: boolean;
    initialize: () => Promise<void>;
  };

// Only persist these keys to localStorage
const PERSISTED_KEYS: (keyof InkTixStore)[] = [
  "isConnected",
  "isWalletConnected",
  "accounts",
  "selectedAccount",
  "endpoint",
];

export const useInkTixStore = create<InkTixStore>()(
  persist(
    (set, get, api) => ({
      // Internal: blockchain service singleton
      _service: null,
      _initialized: false,

      // Compose all slices
      ...createConnectionSlice(set, get, api),
      ...createWalletSlice(set, get, api),
      ...createContractSlice(set, get, api),
      ...createDataSlice(set, get, api),

      // Initialize: create service, restore connection, load mock data
      initialize: async () => {
        if (get()._initialized) return;
        if (typeof window === "undefined") return;

        // Migrate from old localStorage keys if present
        const migrated = migrateFromOldKeys();
        if (migrated) {
          console.log("Migrated from old localStorage keys to inktix-store");
          const updates: Partial<InkTixStore> = {};
          if (migrated.isConnected !== undefined) updates.isConnected = migrated.isConnected;
          if (migrated.isWalletConnected !== undefined) updates.isWalletConnected = migrated.isWalletConnected;
          if (migrated.accounts) updates.accounts = migrated.accounts;
          if (migrated.selectedAccount !== undefined) updates.selectedAccount = migrated.selectedAccount;
          if (migrated.endpoint) updates.endpoint = migrated.endpoint;
          set(updates);
        }

        const service = BlockchainService.getInstance();
        set({ _service: service, _initialized: true });

        // Load mock data immediately
        await get().loadMockData();

        // Try to restore connection if previously connected
        const state = get();
        if (state.isConnected || state.isWalletConnected) {
          try {
            const restored = await service.restoreConnection();
            if (restored) {
              set({ isConnected: true });

              // Restore wallet state
              if (state.selectedAccount) {
                await service.selectAccount(state.selectedAccount);
              }

              // Load network info
              const networkResult = await service.getNetworkInfo();
              if (networkResult.success) {
                set({ networkInfo: networkResult.data });
              }

              // Load balance
              if (state.selectedAccount) {
                await get().loadBalance();
              }
            } else {
              // Connection restoration failed - reset connection state
              set({ isConnected: false });
            }
          } catch {
            set({ isConnected: false });
          }
        }

        // Check contract deployment status
        try {
          const address = await service.getContractAddress();
          const deployed = await service.isContractDeployed();
          set({ contractAddress: address, isContractDeployed: deployed });
        } catch {
          // Ignore - contract may not be deployed
        }
      },
    }),
    {
      name: "inktix-store",
      storage: createJSONStorage(() => localStorage),
      partialize: (state) => {
        const persisted: Record<string, any> = {};
        PERSISTED_KEYS.forEach((key) => {
          persisted[key] = state[key];
        });
        return persisted as InkTixStore;
      },
    }
  )
);
