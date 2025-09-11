"use client";

import React, {
  createContext,
  useContext,
  useState,
  useEffect,
  useCallback,
  useMemo,
  ReactNode,
} from "react";
import { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import {
  BlockchainService,
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
  // Initialize blockchain service singleton
  const blockchainService = useMemo(() => {
    if (typeof window === "undefined") return null;
    return BlockchainService.getInstance();
  }, []);

  // Only log once during development
  useEffect(() => {
    console.log(`🔧 BlockchainProvider: Initializing...`);
    console.log(
      `🔧 BlockchainProvider: blockchainService available:`,
      !!blockchainService
    );
  }, [blockchainService]);

  // Connection states with localStorage persistence
  const [isConnected, setIsConnected] = useState(() => {
    if (typeof window === "undefined") return false;
    return localStorage.getItem("inktix_isConnected") === "true";
  });

  const [isConnecting, setIsConnecting] = useState(false);
  const [isConnectionReady, setIsConnectionReady] = useState(false);

  // Auto-restore connection on mount
  useEffect(() => {
    if (blockchainService && !isConnecting) {
      // Check if we need to restore connection
      const needsRestore = !isConnected || !blockchainService.isConnected();

      if (needsRestore) {
        console.log("🔄 Auto-restoring blockchain connection...");
        blockchainService.restoreConnection().then((restored) => {
          if (restored) {
            console.log("✅ Connection restored successfully");
            setIsConnected(true);
            setIsConnectionReady(true);
          } else {
            console.log("❌ Connection restoration failed");
            setIsConnectionReady(false);
          }
        });
      } else {
        console.log("✅ Connection already active");
        // Check if we need to restore wallet connection
        const isWalletConnected =
          localStorage.getItem("inktix_isWalletConnected") === "true";
        const accounts = localStorage.getItem("inktix_accounts");
        const selectedAccount = localStorage.getItem("inktix_selectedAccount");

        if (isWalletConnected && accounts && selectedAccount) {
          console.log("🔄 Auto-restoring wallet connection...");
          try {
            const accountsData = JSON.parse(accounts);
            const selectedAccountData = JSON.parse(selectedAccount);

            setAccounts(accountsData);
            setSelectedAccount(selectedAccountData);
            setIsWalletConnected(true);

            // Also set the account in the blockchain service
            blockchainService.selectAccount(selectedAccountData);
            console.log("✅ Wallet connection restored successfully");
          } catch (error) {
            console.log("❌ Failed to restore wallet connection:", error);
          }
        }
      }
    }
  }, [blockchainService, isConnected, isConnecting]);

  const [isWalletConnected, setIsWalletConnected] = useState(() => {
    if (typeof window === "undefined") return false;
    return localStorage.getItem("inktix_isWalletConnected") === "true";
  });

  // Debug state changes
  useEffect(() => {
    console.log(
      `🔧 BlockchainProvider: State changed - isConnected: ${isConnected}, isWalletConnected: ${isWalletConnected}`
    );
  }, [isConnected, isWalletConnected]);

  // Persist state changes to localStorage
  useEffect(() => {
    if (typeof window === "undefined") return;
    localStorage.setItem("inktix_isConnected", isConnected.toString());
  }, [isConnected]);

  useEffect(() => {
    if (typeof window === "undefined") return;
    localStorage.setItem(
      "inktix_isWalletConnected",
      isWalletConnected.toString()
    );
  }, [isWalletConnected]);

  // Account states with localStorage persistence
  const [accounts, setAccounts] = useState<InjectedAccountWithMeta[]>(() => {
    if (typeof window === "undefined") return [];
    const saved = localStorage.getItem("inktix_accounts");
    return saved ? JSON.parse(saved) : [];
  });

  const [selectedAccount, setSelectedAccount] =
    useState<InjectedAccountWithMeta | null>(() => {
      if (typeof window === "undefined") return null;
      const saved = localStorage.getItem("inktix_selectedAccount");
      return saved ? JSON.parse(saved) : null;
    });

  // Persist account changes to localStorage
  useEffect(() => {
    if (typeof window === "undefined") return;
    localStorage.setItem("inktix_accounts", JSON.stringify(accounts));
  }, [accounts]);

  useEffect(() => {
    if (typeof window === "undefined") return;
    localStorage.setItem(
      "inktix_selectedAccount",
      JSON.stringify(selectedAccount)
    );
  }, [selectedAccount]);

  // Data states
  const [events, setEvents] = useState<EventData[]>([]);
  const [venues, setVenues] = useState<VenueData[]>([]);
  const [balance, setBalance] = useState<string | null>(null);
  const [networkInfo, setNetworkInfo] = useState<any>(null);

  // Smart Contract states
  const [contractAddress, setContractAddress] = useState<string | null>(null);
  const [isContractDeployed, setIsContractDeployed] = useState(false);

  // Loading states
  const [isLoadingEvents, setIsLoadingEvents] = useState(false);
  const [isLoadingVenues, setIsLoadingVenues] = useState(false);
  const [isLoadingBalance, setIsLoadingBalance] = useState(false);
  const [isDeployingContract, setIsDeployingContract] = useState(false);

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
          description: "NBA regular season game between Lakers and Warriors",
          venue: "Crypto.com Arena",
          date: "2024-01-15",
          sportType: "Basketball",
          basePrice: "150 DOT",
          availableTickets: 100,
          totalCapacity: 19068,
        },
        {
          id: "2",
          name: "Celtics vs Heat",
          description: "NBA regular season game between Celtics and Heat",
          venue: "TD Garden",
          date: "2024-01-20",
          sportType: "Basketball",
          basePrice: "120 DOT",
          availableTickets: 75,
          totalCapacity: 19156,
        },
      ]);

      setVenues([
        {
          id: "1",
          name: "Crypto.com Arena",
          location: "Los Angeles, CA",
          capacity: 19068,
          amenities: ["Premium Seating", "VIP Lounges", "Food & Beverage"],
          parkingSpaces: 5000,
          concessionStands: 25,
        },
        {
          id: "2",
          name: "TD Garden",
          location: "Boston, MA",
          capacity: 19156,
          amenities: ["Club Seating", "Executive Suites", "Dining Options"],
          parkingSpaces: 3000,
          concessionStands: 20,
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

  // Smart Contract methods
  const deployContract = useCallback(
    async (contractWasm: ArrayBuffer, endowment: string) => {
      console.log("🚀 BlockchainContext: deployContract called");
      console.log(
        "🚀 BlockchainContext: blockchainService available:",
        !!blockchainService
      );
      console.log("🚀 BlockchainContext: isConnected:", isConnected);
      console.log(
        "🚀 BlockchainContext: isWalletConnected:",
        isWalletConnected
      );

      if (!blockchainService) {
        console.log("❌ BlockchainContext: No blockchain service available");
        return { success: false, error: "Blockchain service not available" };
      }

      try {
        console.log(
          "🚀 BlockchainContext: Setting isDeployingContract to true"
        );
        setIsDeployingContract(true);
        console.log(
          "🚀 BlockchainContext: Calling blockchainService.deployContract"
        );
        const result = await blockchainService.deployContract(
          contractWasm,
          endowment
        );
        console.log(
          "🚀 BlockchainContext: blockchainService.deployContract result:",
          result
        );
        console.log("🚀 BlockchainContext: result.success:", result.success);
        console.log("🚀 BlockchainContext: result.error:", result.error);
        console.log("🚀 BlockchainContext: result.data:", result.data);

        if (result.success && result.data) {
          setContractAddress(result.data);
          setIsContractDeployed(true);
        }

        return result;
      } catch (error) {
        console.error("Failed to deploy contract:", error);
        return {
          success: false,
          error: `Contract deployment failed: ${
            error instanceof Error ? error.message : "Unknown error"
          }`,
        };
      } finally {
        setIsDeployingContract(false);
      }
    },
    []
  );

  const callContract = useCallback(async (method: string, args: any[] = []) => {
    if (!blockchainService) {
      return { success: false, error: "Blockchain service not available" };
    }

    try {
      const result = await blockchainService.callContract(method, args);
      return result;
    } catch (error) {
      console.error("Failed to call contract:", error);
      return {
        success: false,
        error: `Contract call failed: ${
          error instanceof Error ? error.message : "Unknown error"
        }`,
      };
    }
  }, []);

  // Load initial data without requiring network connection
  useEffect(() => {
    if (typeof window === "undefined") return; // Skip on server side

    // Load mock data on initial load
    loadMockData();
  }, [loadMockData]); // Only depend on loadMockData

  // Update balance when account changes
  useEffect(() => {
    if (typeof window === "undefined") return; // Skip on server side

    if (
      selectedAccount &&
      isConnected &&
      !isConnecting &&
      isConnectionReady &&
      blockchainService
    ) {
      // Check if the API is actually connected before loading balance
      const isApiConnected = blockchainService.isConnected();
      if (isApiConnected) {
        // Add a small delay to ensure network connection is fully established
        const timer = setTimeout(() => {
          // Double-check connection before loading balance
          if (blockchainService && blockchainService.isConnected()) {
            loadBalance();
          } else {
            console.log(
              "🔄 Connection lost during delay, skipping balance load"
            );
          }
        }, 1000); // Reduced delay since we have connection ready flag

        return () => clearTimeout(timer);
      } else {
        console.log("🔄 API not connected yet, skipping balance load");
      }
    }
  }, [
    selectedAccount,
    isConnected,
    loadBalance,
    isConnecting,
    isConnectionReady,
    blockchainService,
  ]);

  // Check contract deployment status
  useEffect(() => {
    if (typeof window === "undefined" || !blockchainService) return; // Skip on server side or if service not available

    const checkContractStatus = async () => {
      if (!blockchainService) return; // Extra safety check

      try {
        const address = await blockchainService.getContractAddress();
        const deployed = await blockchainService.isContractDeployed();
        setContractAddress(address);
        setIsContractDeployed(deployed);
      } catch (error) {
        console.warn("Could not check contract status:", error);
      }
    };

    checkContractStatus();
  }, []);

  const connectToNetwork = async (endpoint: string = "ws://127.0.0.1:9944") => {
    if (typeof window === "undefined" || !blockchainService) {
      return { success: false, error: "Blockchain service not available" };
    }

    console.log(
      "🌐 BlockchainContext: connectToNetwork called with endpoint:",
      endpoint
    );

    try {
      setIsConnecting(true);
      console.log("🌐 BlockchainContext: isConnecting set to true");

      const result = await blockchainService.connectToNetwork(endpoint);
      console.log(
        "🌐 BlockchainContext: blockchainService.connectToNetwork result:",
        result
      );

      if (result.success) {
        console.log("🌐 BlockchainContext: Setting isConnected to true...");
        setIsConnected(true);
        setIsConnectionReady(true);
        console.log("🌐 BlockchainContext: isConnected set to true");

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
      setIsConnectionReady(false);
      return {
        success: false,
        error: `Failed to connect: ${
          error instanceof Error ? error.message : "Unknown error"
        }`,
      };
    } finally {
      console.log("🌐 BlockchainContext: Setting isConnecting to false");
      setIsConnecting(false);
    }
  };

  const connectWallet = async () => {
    console.log("🔌 BlockchainContext: connectWallet function called");
    if (typeof window === "undefined" || !blockchainService) {
      console.log("❌ BlockchainContext: No blockchain service available");
      return { success: false, error: "Blockchain service not available" };
    }

    console.log("🔌 BlockchainContext: Starting wallet connection...");

    try {
      const result = await blockchainService.connectWallet();
      console.log("🔌 BlockchainContext: Wallet connection result:", result);

      if (result.success && result.data) {
        console.log("✅ BlockchainContext: Setting wallet state...");
        setAccounts(result.data);
        setSelectedAccount(result.data[0]);
        setIsWalletConnected(true);

        // Also set the account in the blockchain service
        console.log(
          "🔧 BlockchainContext: Calling blockchainService.selectAccount..."
        );
        await blockchainService.selectAccount(result.data[0]);
        console.log(
          "🔧 BlockchainContext: blockchainService.selectAccount completed"
        );

        console.log(
          "✅ BlockchainContext: Wallet state updated - isWalletConnected set to true"
        );

        // Automatically connect to local network after wallet connection
        console.log(
          "🌐 BlockchainContext: Automatically connecting to local network..."
        );
        try {
          const networkResult = await connectToNetwork("ws://127.0.0.1:9944");
          if (networkResult.success) {
            console.log(
              "✅ BlockchainContext: Automatically connected to local network - isConnected set to true"
            );
          } else {
            console.log(
              "⚠️ BlockchainContext: Auto-connection to network failed:",
              networkResult.error
            );
          }
        } catch (networkError) {
          console.log(
            "⚠️ BlockchainContext: Auto-connection to network failed:",
            networkError
          );
        }
      } else {
        console.log(
          "❌ BlockchainContext: Wallet connection failed:",
          result.error
        );
      }

      return result;
    } catch (error) {
      console.error("❌ BlockchainContext: Wallet connection error:", error);
      return {
        success: false,
        error: `Wallet connection failed: ${
          error instanceof Error ? error.message : "Unknown error"
        }`,
      };
    }
  };

  const selectAccount = async (account: InjectedAccountWithMeta) => {
    if (typeof window === "undefined" || !blockchainService) return;

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

  // Auto-reconnect to network if we have saved connection states but no active connection
  useEffect(() => {
    if (
      isWalletConnected &&
      isConnected &&
      !isConnecting &&
      blockchainService
    ) {
      // Check if we actually have an active network connection
      const hasActiveConnection = blockchainService.isConnected();
      if (!hasActiveConnection) {
        console.log(
          `🔄 BlockchainProvider: Auto-reconnecting to network (state mismatch)...`
        );
        connectToNetwork("ws://127.0.0.1:9944");
      }
    }
  }, [
    isWalletConnected,
    isConnected,
    isConnecting,
    connectToNetwork,
    blockchainService,
  ]);

  // Prevent balance loading during network connection
  useEffect(() => {
    if (isConnecting) {
      console.log(
        `🔄 BlockchainProvider: Network connection in progress, skipping balance load`
      );
    }
  }, [isConnecting]);

  // Ensure network connection is maintained during deployment
  useEffect(() => {
    if (isWalletConnected && isConnected && blockchainService) {
      const checkConnection = async () => {
        if (blockchainService && !blockchainService.isConnected()) {
          console.log(
            `🔄 BlockchainProvider: Reconnecting during deployment...`
          );
          await connectToNetwork("ws://127.0.0.1:9944");
        }
      };

      // Check connection every 5 seconds during deployment
      const interval = setInterval(checkConnection, 5000);
      return () => clearInterval(interval);
    }
  }, [isWalletConnected, isConnected, blockchainService, connectToNetwork]);

  const disconnect = async () => {
    if (typeof window === "undefined" || !blockchainService) return;

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
      setContractAddress(null);
      setIsContractDeployed(false);

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

    // Smart Contract state
    contractAddress,
    isContractDeployed,

    // Actions
    connectToNetwork,
    connectWallet,
    selectAccount,
    disconnect,
    refreshData,

    // Smart Contract actions
    deployContract,
    callContract,

    // Loading states
    isLoadingEvents,
    isLoadingVenues,
    isLoadingBalance,
    isDeployingContract,
  };

  return (
    <BlockchainContext.Provider value={value}>
      {children}
    </BlockchainContext.Provider>
  );
};
