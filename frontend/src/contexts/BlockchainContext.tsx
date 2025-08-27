"use client";

import React, {
  createContext,
  useContext,
  useState,
  useEffect,
  useCallback,
  ReactNode,
} from "react";
import { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import {
  blockchainService,
  ContractCallResult,
  EventData,
  VenueData,
} from "../services/blockchain";

interface BlockchainContextType {
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

  // Actions
  connectToNetwork: (endpoint?: string) => Promise<ContractCallResult>;
  connectWallet: () => Promise<ContractCallResult<InjectedAccountWithMeta[]>>;
  selectAccount: (account: InjectedAccountWithMeta) => Promise<void>;
  disconnect: () => Promise<void>;
  refreshData: () => Promise<void>;

  // Loading states
  isLoadingEvents: boolean;
  isLoadingVenues: boolean;
  isLoadingBalance: boolean;
}

const BlockchainContext = createContext<BlockchainContextType | undefined>(
  undefined
);

export const useBlockchain = () => {
  const context = useContext(BlockchainContext);
  if (context === undefined) {
    throw new Error("useBlockchain must be used within a BlockchainProvider");
  }
  return context;
};

interface BlockchainProviderProps {
  children: ReactNode;
}

export const BlockchainProvider: React.FC<BlockchainProviderProps> = ({
  children,
}) => {
  // Connection states
  const [isConnected, setIsConnected] = useState(false);
  const [isConnecting, setIsConnecting] = useState(false);
  const [isWalletConnected, setIsWalletConnected] = useState(false);

  // Account states
  const [accounts, setAccounts] = useState<InjectedAccountWithMeta[]>([]);
  const [selectedAccount, setSelectedAccount] =
    useState<InjectedAccountWithMeta | null>(null);

  // Data states
  const [events, setEvents] = useState<EventData[]>([]);
  const [venues, setVenues] = useState<VenueData[]>([]);
  const [balance, setBalance] = useState<string | null>(null);
  const [networkInfo, setNetworkInfo] = useState<any>(null);

  // Loading states
  const [isLoadingEvents, setIsLoadingEvents] = useState(false);
  const [isLoadingVenues, setIsLoadingVenues] = useState(false);
  const [isLoadingBalance, setIsLoadingBalance] = useState(false);

  const loadMockData = useCallback(async () => {
    try {
      // Load mock events and venues even without blockchain connection
      setIsLoadingEvents(true);
      setIsLoadingVenues(true);

      // Simulate loading delay
      await new Promise((resolve) => setTimeout(resolve, 500));

      // Set mock data
      setEvents([
        {
          id: "1",
          name: "Lakers vs Warriors",
          description: "NBA Regular Season Game",
          venue: "Crypto.com Arena",
          date: "2024-01-15T19:30:00Z",
          sportType: "Basketball",
          basePrice: "150.00",
          availableTickets: 500,
          totalCapacity: 19000,
        },
        {
          id: "2",
          name: "Dodgers vs Giants",
          description: "MLB Rivalry Game",
          venue: "Dodger Stadium",
          date: "2024-01-20T18:00:00Z",
          sportType: "Baseball",
          basePrice: "75.00",
          availableTickets: 1200,
          totalCapacity: 56000,
        },
      ]);

      setVenues([
        {
          id: "1",
          name: "Crypto.com Arena",
          location: "Los Angeles, CA",
          capacity: 19000,
          amenities: ["Parking", "Concessions", "Merchandise", "VIP Lounges"],
          parkingSpaces: 2000,
          concessionStands: 25,
        },
        {
          id: "2",
          name: "Dodger Stadium",
          location: "Los Angeles, CA",
          capacity: 56000,
          amenities: ["Parking", "Concessions", "Merchandise", "Family Zone"],
          parkingSpaces: 16000,
          concessionStands: 50,
        },
      ]);

      setIsLoadingEvents(false);
      setIsLoadingVenues(false);
    } catch (error) {
      console.error("Failed to load mock data:", error);
      setIsLoadingEvents(false);
      setIsLoadingVenues(false);
    }
  }, []);

  const loadBalance = useCallback(async () => {
    if (!selectedAccount || !blockchainService || !isConnected) return;

    try {
      setIsLoadingBalance(true);
      const result = await blockchainService.getBalance(
        selectedAccount.address
      );
      if (result.success && result.data) {
        setBalance(result.data);
      }
    } catch (error) {
      console.error("Failed to load balance:", error);
    } finally {
      setIsLoadingBalance(false);
    }
  }, [selectedAccount, isConnected]);

  const refreshData = useCallback(async () => {
    if (!isConnected || !blockchainService) return;

    try {
      // Load events
      setIsLoadingEvents(true);
      const eventsResult = await blockchainService.getEvents();
      if (eventsResult.success && eventsResult.data) {
        setEvents(eventsResult.data);
      }
      setIsLoadingEvents(false);

      // Load venues
      setIsLoadingVenues(true);
      const venuesResult = await blockchainService.getVenues();
      if (venuesResult.success && venuesResult.data) {
        setVenues(venuesResult.data);
      }
      setIsLoadingVenues(false);
    } catch (error) {
      console.error("Failed to refresh data:", error);
      setIsLoadingEvents(false);
      setIsLoadingVenues(false);
    }
  }, [isConnected]);

  // Load initial data without requiring network connection
  useEffect(() => {
    if (isConnected) {
      refreshData();
    } else {
      // Load mock data even without network connection
      loadMockData();
    }
  }, [isConnected, refreshData, loadMockData]);

  // Update balance when account changes
  useEffect(() => {
    if (selectedAccount && isConnected) {
      loadBalance();
    }
  }, [selectedAccount, isConnected, loadBalance]);

  const connectToNetwork = async (
    endpoint: string = "wss://rpc.polkadot.io"
  ) => {
    if (!blockchainService) {
      return { success: false, error: "Blockchain service not available" };
    }

    try {
      setIsConnecting(true);
      const result = await blockchainService.connectToNetwork(endpoint);

      if (result.success) {
        setIsConnected(true);

        // Get network info
        const networkResult = await blockchainService.getNetworkInfo();
        if (networkResult.success) {
          setNetworkInfo(networkResult.data);
        }

        // Load real data from blockchain
        await refreshData();
      }

      return result;
    } catch (error) {
      console.error("Failed to connect to network:", error);
      return {
        success: false,
        error: `Failed to connect: ${
          error instanceof Error ? error.message : "Unknown error"
        }`,
      };
    } finally {
      setIsConnecting(false);
    }
  };

  const connectWallet = async () => {
    if (!blockchainService) {
      return { success: false, error: "Blockchain service not available" };
    }

    try {
      const result = await blockchainService.connectWallet();

      if (result.success && result.data) {
        setAccounts(result.data);
        setSelectedAccount(result.data[0]);
        setIsWalletConnected(true);

        // Load balance for selected account if connected to network
        if (result.data[0] && isConnected) {
          await loadBalance();
        }
      }

      return result;
    } catch (error) {
      console.error("Failed to connect wallet:", error);
      return {
        success: false,
        error: `Wallet connection failed: ${
          error instanceof Error ? error.message : "Unknown error"
        }`,
      };
    }
  };

  const selectAccount = async (account: InjectedAccountWithMeta) => {
    if (!blockchainService) return;

    try {
      await blockchainService.selectAccount(account);
      setSelectedAccount(account);

      // Load balance for new account if connected to network
      if (isConnected) {
        await loadBalance();
      }
    } catch (error) {
      console.error("Failed to select account:", error);
    }
  };

  const disconnect = async () => {
    if (!blockchainService) return;

    try {
      await blockchainService.disconnect();
      setIsConnected(false);
      setIsWalletConnected(false);
      setAccounts([]);
      setSelectedAccount(null);
      setBalance(null);
      setNetworkInfo(null);
      setEvents([]);
      setVenues([]);

      // Reload mock data after disconnection
      await loadMockData();
    } catch (error) {
      console.error("Failed to disconnect:", error);
    }
  };

  const value: BlockchainContextType = {
    // Connection state
    isConnected,
    isConnecting,
    isWalletConnected,

    // Account state
    accounts,
    selectedAccount,

    // Data state
    events,
    venues,
    balance,
    networkInfo,

    // Actions
    connectToNetwork,
    connectWallet,
    selectAccount,
    disconnect,
    refreshData,

    // Loading states
    isLoadingEvents,
    isLoadingVenues,
    isLoadingBalance,
  };

  return (
    <BlockchainContext.Provider value={value}>
      {children}
    </BlockchainContext.Provider>
  );
};
