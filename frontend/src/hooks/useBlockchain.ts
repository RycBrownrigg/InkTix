"use client";

import { useEffect } from "react";
import { useInkTixStore } from "../store";
import { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import { ContractCallResult, EventData, VenueData } from "../services/blockchain";

/**
 * Backward-compatible hook that wraps the Zustand store.
 * Drop-in replacement for the old useBlockchain() from BlockchainContext.
 */
export interface UseBlockchainReturn {
  // Connection state
  isConnected: boolean;
  isConnecting: boolean;
  isWalletConnected: boolean;

  // Account state
  accounts: InjectedAccountWithMeta[];
  selectedAccount: InjectedAccountWithMeta | null;

  // Data state
  events: EventData[];
  venues: VenueData[];
  balance: string | null;
  networkInfo: any;

  // Smart Contract state
  contractAddress: string | null;
  isContractDeployed: boolean;

  // Actions
  connectToNetwork: (endpoint?: string) => Promise<ContractCallResult>;
  connectWallet: () => Promise<ContractCallResult<InjectedAccountWithMeta[]>>;
  selectAccount: (account: InjectedAccountWithMeta) => Promise<void>;
  disconnect: () => Promise<void>;
  refreshData: () => Promise<void>;

  // Smart Contract actions
  deployContract: (
    contractWasm: ArrayBuffer,
    endowment: string
  ) => Promise<ContractCallResult<string>>;
  callContract: (
    method: string,
    args?: any[]
  ) => Promise<ContractCallResult<any>>;

  // Loading states
  isLoadingEvents: boolean;
  isLoadingVenues: boolean;
  isLoadingBalance: boolean;
  isDeployingContract: boolean;
}

export function useBlockchain(): UseBlockchainReturn {
  const store = useInkTixStore();

  // Initialize on first use
  useEffect(() => {
    store.initialize();
  }, []); // eslint-disable-line react-hooks/exhaustive-deps

  // Load balance when account or connection changes
  useEffect(() => {
    if (store.selectedAccount && store.isConnected && store._initialized) {
      store.loadBalance();
    }
  }, [store.selectedAccount, store.isConnected, store._initialized]); // eslint-disable-line react-hooks/exhaustive-deps

  // Wrap connectWallet to auto-connect to network after
  const connectWallet = async () => {
    const result = await store.connectWallet();
    if (result.success) {
      try {
        await store.connectToNetwork();
        await store.refreshData();
      } catch {
        // Network connection is optional after wallet connect
      }
    }
    return result;
  };

  // Wrap disconnect to reset all state
  const disconnect = async () => {
    await store.disconnect();
    store.disconnectWallet();
    store.resetContract();
    store.setBalance(null);
    await store.loadMockData();
  };

  return {
    isConnected: store.isConnected,
    isConnecting: store.isConnecting,
    isWalletConnected: store.isWalletConnected,
    accounts: store.accounts,
    selectedAccount: store.selectedAccount,
    events: store.events,
    venues: store.venues,
    balance: store.balance,
    networkInfo: store.networkInfo,
    contractAddress: store.contractAddress,
    isContractDeployed: store.isContractDeployed,
    connectToNetwork: store.connectToNetwork,
    connectWallet,
    selectAccount: store.selectAccount,
    disconnect,
    refreshData: store.refreshData,
    deployContract: store.deployContract,
    callContract: store.callContract,
    isLoadingEvents: store.isLoadingEvents,
    isLoadingVenues: store.isLoadingVenues,
    isLoadingBalance: store.isLoadingBalance,
    isDeployingContract: store.isDeployingContract,
  };
}
