/**
 * Zustand slice managing Substrate network connection state.
 *
 * Handles connecting to and disconnecting from an RPC endpoint, tracking
 * connection progress, and fetching basic network metadata (chain name,
 * node version).
 *
 * @module store/slices/connectionSlice
 *
 * Exported interfaces:
 * - {@link ConnectionSlice} - State and actions for network connection
 *
 * Exported functions:
 * - {@link createConnectionSlice} - Zustand StateCreator factory
 */
import { StateCreator } from "zustand";
import {
  BlockchainService,
  ContractCallResult,
} from "../../services/blockchain";
import { getDefaultEndpoint } from "../../config/chains";

export interface ConnectionSlice {
  // State
  isConnected: boolean;
  isConnecting: boolean;
  endpoint: string;
  networkInfo: any;

  // Actions
  connectToNetwork: (endpoint?: string) => Promise<ContractCallResult>;
  disconnect: () => Promise<void>;
  setIsConnected: (value: boolean) => void;
  setNetworkInfo: (info: any) => void;
}

export const createConnectionSlice: StateCreator<
  ConnectionSlice & { _service: BlockchainService | null },
  [],
  [],
  ConnectionSlice
> = (set, get) => ({
  isConnected: false,
  isConnecting: false,
  endpoint: getDefaultEndpoint(),
  networkInfo: null,

  setIsConnected: (value: boolean) => set({ isConnected: value }),
  setNetworkInfo: (info: any) => set({ networkInfo: info }),

  connectToNetwork: async (
    endpoint = getDefaultEndpoint()
  ) => {
    const service = (get() as any)._service;
    if (!service) {
      return { success: false, error: "Blockchain service not available" };
    }

    try {
      set({ isConnecting: true });
      const result = await service.connectToNetwork(endpoint);

      if (result.success) {
        set({ isConnected: true, endpoint });

        const networkResult = await service.getNetworkInfo();
        if (networkResult.success) {
          set({ networkInfo: networkResult.data });
        }
      }

      return result;
    } catch (error) {
      return {
        success: false,
        error: `Failed to connect: ${
          error instanceof Error ? error.message : "Unknown error"
        }`,
      };
    } finally {
      set({ isConnecting: false });
    }
  },

  disconnect: async () => {
    const service = (get() as any)._service;
    if (service) {
      await service.disconnect();
    }
    set({
      isConnected: false,
      networkInfo: null,
      endpoint: getDefaultEndpoint(),
    });
  },
});
