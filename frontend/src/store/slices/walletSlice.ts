import { StateCreator } from "zustand";
import { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import {
  BlockchainService,
  ContractCallResult,
} from "../../services/blockchain";

export interface WalletSlice {
  // State
  isWalletConnected: boolean;
  accounts: InjectedAccountWithMeta[];
  selectedAccount: InjectedAccountWithMeta | null;

  // Actions
  connectWallet: () => Promise<ContractCallResult<InjectedAccountWithMeta[]>>;
  selectAccount: (account: InjectedAccountWithMeta) => Promise<void>;
  setAccounts: (accounts: InjectedAccountWithMeta[]) => void;
  setSelectedAccount: (account: InjectedAccountWithMeta | null) => void;
  setIsWalletConnected: (value: boolean) => void;
  disconnectWallet: () => void;
}

export const createWalletSlice: StateCreator<
  WalletSlice & { _service: BlockchainService | null },
  [],
  [],
  WalletSlice
> = (set, get) => ({
  isWalletConnected: false,
  accounts: [],
  selectedAccount: null,

  setAccounts: (accounts) => set({ accounts }),
  setSelectedAccount: (account) => set({ selectedAccount: account }),
  setIsWalletConnected: (value) => set({ isWalletConnected: value }),

  connectWallet: async () => {
    const service = (get() as any)._service;
    if (!service) {
      return { success: false, error: "Blockchain service not available" };
    }

    try {
      const result = await service.connectWallet();

      if (result.success && result.data) {
        set({
          accounts: result.data,
          selectedAccount: result.data[0],
          isWalletConnected: true,
        });
        await service.selectAccount(result.data[0]);
      }

      return result;
    } catch (error) {
      return {
        success: false,
        error: `Wallet connection failed: ${
          error instanceof Error ? error.message : "Unknown error"
        }`,
      };
    }
  },

  selectAccount: async (account) => {
    const service = (get() as any)._service;
    if (service) {
      await service.selectAccount(account);
    }
    set({ selectedAccount: account });
  },

  disconnectWallet: () => {
    set({
      isWalletConnected: false,
      accounts: [],
      selectedAccount: null,
    });
  },
});
