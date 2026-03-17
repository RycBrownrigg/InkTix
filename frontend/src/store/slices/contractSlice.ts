/**
 * Zustand slice managing smart contract deployment and interaction state.
 *
 * Tracks the deployed contract address and provides actions to deploy a
 * new contract from a .wasm binary or call methods on the active contract.
 *
 * @module store/slices/contractSlice
 *
 * Exported interfaces:
 * - {@link ContractSlice} - State and actions for contract lifecycle
 *
 * Exported functions:
 * - {@link createContractSlice} - Zustand StateCreator factory
 */
import { StateCreator } from "zustand";
import {
  BlockchainService,
  ContractCallResult,
} from "../../services/blockchain";

export interface ContractSlice {
  // State
  contractAddress: string | null;
  isContractDeployed: boolean;
  isDeployingContract: boolean;

  // Actions
  deployContract: (
    contractWasm: ArrayBuffer,
    endowment: string
  ) => Promise<ContractCallResult<string>>;
  callContract: (
    method: string,
    args?: any[]
  ) => Promise<ContractCallResult<any>>;
  resetContract: () => void;
}

export const createContractSlice: StateCreator<
  ContractSlice & { _service: BlockchainService | null },
  [],
  [],
  ContractSlice
> = (set, get) => ({
  contractAddress: null,
  isContractDeployed: false,
  isDeployingContract: false,

  deployContract: async (contractWasm, endowment) => {
    const service = (get() as any)._service;
    if (!service) {
      return { success: false, error: "Blockchain service not available" };
    }

    try {
      set({ isDeployingContract: true });
      const result = await service.deployContract(contractWasm, endowment);

      if (result.success && result.data) {
        set({
          contractAddress: result.data,
          isContractDeployed: true,
        });
      }

      return result;
    } catch (error) {
      return {
        success: false,
        error: `Contract deployment failed: ${
          error instanceof Error ? error.message : "Unknown error"
        }`,
      };
    } finally {
      set({ isDeployingContract: false });
    }
  },

  callContract: async (method, args = []) => {
    const service = (get() as any)._service;
    if (!service) {
      return { success: false, error: "Blockchain service not available" };
    }

    try {
      return await service.callContract(method, args);
    } catch (error) {
      return {
        success: false,
        error: `Contract call failed: ${
          error instanceof Error ? error.message : "Unknown error"
        }`,
      };
    }
  },

  resetContract: () => {
    set({
      contractAddress: null,
      isContractDeployed: false,
      isDeployingContract: false,
    });
  },
});
