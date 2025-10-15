import { ApiPromise, WsProvider } from "@polkadot/api";
import { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import { web3Accounts, web3Enable } from "@polkadot/extension-dapp";
import { cryptoWaitReady } from "@polkadot/util-crypto";

export interface ContractCallResult<T = any> {
  success: boolean;
  data?: T;
  error?: string;
  txHash?: string;
  message?: string;
}

export interface EventData {
  id: string;
  name: string;
  description: string;
  venue: string;
  date: string;
  sportType: string;
  basePrice: string;
  availableTickets: number;
  totalCapacity: number;
}

export interface VenueData {
  id: string;
  name: string;
  location: string;
  capacity: number;
  amenities: string[];
  parkingSpaces: number;
  concessionStands: number;
}

export class BlockchainService {
  private static instance: BlockchainService | null = null;

  private api: ApiPromise | null = null;
  private provider: WsProvider | null = null;
  private accounts: InjectedAccountWithMeta[] = [];
  private selectedAccount: InjectedAccountWithMeta | null = null;
  private contractAddress: string | null = null;

  constructor() {
    // Only initialize on client side
    if (typeof window !== "undefined") {
      this.initialize();
    }
  }

  public static getInstance(): BlockchainService {
    if (!BlockchainService.instance) {
      BlockchainService.instance = new BlockchainService();
    }
    return BlockchainService.instance;
  }

  // Restore connection state from localStorage
  public async restoreConnection(): Promise<boolean> {
    if (typeof window === "undefined") return false;

    const isConnected = localStorage.getItem("blockchain_connected") === "true";
    const endpoint = localStorage.getItem("blockchain_endpoint");

    if (isConnected && endpoint) {
      // Check if we need to restore connection
      const needsRestore = !this.api || !this.isConnected();

      if (needsRestore) {
        console.log("Restoring connection to:", endpoint);
        const result = await this.connectToNetwork(endpoint);
        return result.success;
      } else {
        console.log("Connection already active");
        return true;
      }
    }

    return false;
  }

  private async initialize() {
    try {
      await cryptoWaitReady();
      console.log("Crypto ready");
    } catch (error) {
      console.error("Failed to initialize crypto:", error);
    }
  }

  async connectToNetwork(
    endpoint: string = "wss://westend-asset-hub-rpc.polkadot.io"
  ): Promise<ContractCallResult> {
    try {
      if (typeof window === "undefined") {
        return { success: false, error: "Not available on server side" };
      }

      // Check if we're already connected to the same endpoint
      if (this.api && this.isConnected()) {
        const currentEndpoint = localStorage.getItem("blockchain_endpoint");
        if (currentEndpoint === endpoint) {
          console.log("Already connected to:", endpoint);
          return { success: true, message: "Already connected" };
        }
      }

      if (this.api) {
        await this.api.disconnect();
      }

      console.log("Attempting to connect to:", endpoint);
      const provider = new WsProvider(endpoint);
      this.api = await ApiPromise.create({ provider });

      // Wait for the API to be ready
      await this.api.isReady;

      // Debug: Check what we actually connected to
      const chain = await this.api.rpc.system.chain();
      const name = await this.api.rpc.system.name();
      console.log("Actually connected to chain:", chain.toString());
      console.log("Chain name:", name.toString());

      // Check if contracts pallet is available
      if (this.api.tx.contracts) {
        console.log("✅ Contracts pallet is available on this chain");
      } else {
        console.log("❌ Contracts pallet is NOT available on this chain");
      }

      // Store connection state in localStorage
      localStorage.setItem("blockchain_endpoint", endpoint);
      localStorage.setItem("blockchain_connected", "true");

      console.log("Connected to network:", endpoint);

      // Debug: Check what's happening with Westend
      if (chain.toString().includes("Westend") && !this.api.tx.contracts) {
        console.log("⚠️  Westend detected but no contracts pallet!");
        console.log("This suggests either:");
        console.log("1. The RPC endpoint is not fully initialized");
        console.log("2. There's a version mismatch");
        console.log("3. The chain metadata is incomplete");
        console.log("");
        console.log("Try manually checking polkadot.js.org/apps/#/explorer");
        console.log("Switch to Westend and look for 'Developer' → 'Contracts'");
      }

      return { success: true };
    } catch (error) {
      console.error("Failed to connect to network:", error);
      return {
        success: false,
        error: `Connection failed: ${
          error instanceof Error ? error.message : "Unknown error"
        }`,
      };
    }
  }

  async connectWallet(): Promise<
    ContractCallResult<InjectedAccountWithMeta[]>
  > {
    try {
      if (typeof window === "undefined") {
        return { success: false, error: "Not available on server side" };
      }

      console.log("🔌 Starting wallet connection...");

      // Enable web3 extension
      console.log("📱 Enabling web3 extension...");
      const extensions = await web3Enable("InkTix Sports Platform");
      console.log("📱 Extensions found:", extensions.length);

      if (extensions.length === 0) {
        throw new Error(
          "No web3 extension found. Please install Polkadot.js extension."
        );
      }

      // Get accounts
      console.log("👤 Getting accounts...");
      const accounts = await web3Accounts();
      console.log("👤 Accounts found:", accounts.length);

      if (accounts.length === 0) {
        throw new Error(
          "No accounts found. Please create or import an account."
        );
      }

      // Store accounts
      this.accounts = accounts;
      this.selectedAccount = accounts[0];

      console.log(
        "✅ Wallet connected successfully:",
        accounts.length,
        "accounts found"
      );
      console.log("👤 Selected account:", accounts[0].address);

      return { success: true, data: accounts };
    } catch (error) {
      console.error("❌ Failed to connect wallet:", error);
      return {
        success: false,
        error: `Wallet connection failed: ${
          error instanceof Error ? error.message : "Unknown error"
        }`,
      };
    }
  }

  async getAccounts(): Promise<InjectedAccountWithMeta[]> {
    return this.accounts;
  }

  async selectAccount(account: InjectedAccountWithMeta): Promise<void> {
    console.log(
      "🔧 BlockchainService: selectAccount called with:",
      account.address
    );
    this.selectedAccount = account;
    console.log(
      "🔧 BlockchainService: selectedAccount set to:",
      this.selectedAccount?.address
    );
  }

  async getBalance(address: string): Promise<ContractCallResult<string>> {
    try {
      if (typeof window === "undefined") {
        return { success: false, error: "Not available on server side" };
      }

      if (!this.api || !this.api.isConnected) {
        // Try to restore connection if we have stored connection info
        const isConnected =
          localStorage.getItem("blockchain_connected") === "true";
        const endpoint = localStorage.getItem("blockchain_endpoint");

        if (isConnected && endpoint) {
          console.log(
            "🔄 Attempting to restore connection for balance check..."
          );
          const restored = await this.restoreConnection();
          if (!restored) {
            throw new Error("Not connected to network and restoration failed");
          }
        } else {
          throw new Error("Not connected to network");
        }
      }

      const accountInfo = await this.api!.query.system.account(address);
      // Use type assertion to access the data property
      const balance = (accountInfo as any).data.free;

      return {
        success: true,
        data: balance.toString(),
      };
    } catch (error) {
      console.error("Failed to get balance:", error);
      return {
        success: false,
        error: `Failed to get balance: ${
          error instanceof Error ? error.message : "Unknown error"
        }`,
      };
    }
  }

  async getNetworkInfo(): Promise<ContractCallResult<any>> {
    try {
      if (typeof window === "undefined") {
        return { success: false, error: "Not available on server side" };
      }

      if (!this.api) {
        throw new Error("Not connected to network");
      }

      const [chain, nodeName, nodeVersion] = await Promise.all([
        this.api.rpc.system.chain(),
        this.api.rpc.system.name(),
        this.api.rpc.system.version(),
      ]);

      return {
        success: true,
        data: {
          chain: chain.toString(),
          nodeName: nodeName.toString(),
          nodeVersion: nodeVersion.toString(),
        },
      };
    } catch (error) {
      console.error("Failed to get network info:", error);
      return {
        success: false,
        error: `Failed to get network info: ${
          error instanceof Error ? error.message : "Unknown error"
        }`,
      };
    }
  }

  // Mock data methods for development (will be replaced with real contract calls)
  async getEvents(): Promise<ContractCallResult<EventData[]>> {
    // TODO: Replace with actual contract call
    const mockEvents: EventData[] = [
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
    ];

    return { success: true, data: mockEvents };
  }

  async getVenues(): Promise<ContractCallResult<VenueData[]>> {
    // TODO: Replace with actual contract call
    const mockVenues: VenueData[] = [
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
    ];

    return { success: true, data: mockVenues };
  }

  // Smart Contract Methods
  async deployContract(
    contractWasm: ArrayBuffer,
    endowment: string
  ): Promise<ContractCallResult<string>> {
    try {
      // Check if we have a selected account
      console.log("🔍 deployContract: Checking selected account...");
      console.log(
        "🔍 deployContract: this.selectedAccount:",
        this.selectedAccount
      );
      console.log(
        "🔍 deployContract: this.selectedAccount?.address:",
        this.selectedAccount?.address
      );

      if (!this.selectedAccount) {
        console.log("❌ deployContract: No selected account found");
        return { success: false, error: "No wallet account selected" };
      }

      // Check if we have an API connection
      if (!this.api || !this.api.isConnected) {
        // Try to restore connection if we have stored connection info
        const isConnected =
          localStorage.getItem("blockchain_connected") === "true";
        const endpoint = localStorage.getItem("blockchain_endpoint");

        if (isConnected && endpoint) {
          console.log("🔄 Attempting to restore connection for deployment...");
          const restored = await this.restoreConnection();
          if (!restored) {
            return {
              success: false,
              error: "Not connected to network and restoration failed",
            };
          }
        } else {
          return { success: false, error: "Not connected to network" };
        }
      }

      console.log(
        "Starting contract deployment (mock mode for development)..."
      );
      console.log(
        "🔍 Deployment check - API connected:",
        !!this.api && this.api.isConnected
      );
      console.log(
        "🔍 Deployment check - Selected account:",
        !!this.selectedAccount
      );
      console.log(
        "🔍 Deployment check - Account address:",
        this.selectedAccount?.address
      );

      // For development purposes, we'll use a mock deployment
      // This allows us to test the frontend integration without dealing with complex Substrate API issues
      console.log("Using mock deployment for development...");

      // Simulate deployment delay
      await new Promise((resolve) => setTimeout(resolve, 2000));

      // Generate a mock contract address
      const mockContractAddress =
        "0x" + Math.random().toString(16).substr(2, 40);
      this.contractAddress = mockContractAddress;

      console.log("Mock contract deployment successful:", mockContractAddress);
      return {
        success: true,
        data: mockContractAddress,
        message: "Contract deployed successfully (mock mode for development)",
        txHash: "0x" + Math.random().toString(16).substr(2, 64),
      };
    } catch (error) {
      console.error("Failed to deploy contract:", error);
      return {
        success: false,
        error: `Contract deployment failed: ${
          error instanceof Error ? error.message : "Unknown error"
        }`,
      };
    }
  }

  async callContract(
    method: string,
    args: any[] = []
  ): Promise<ContractCallResult<any>> {
    try {
      if (!this.api || !this.selectedAccount || !this.contractAddress) {
        return {
          success: false,
          error: "Contract not deployed or wallet not connected",
        };
      }

      console.log(`Real contract call: ${method}`, args);

      // For now, return mock data while we implement real contract calls
      // In the next phase, this will use the contracts pallet to call methods
      switch (method) {
        // Getter methods (no arguments)
        case "get_total_teams":
          return { success: true, data: 2 };
        case "get_total_venues":
          return { success: true, data: 2 };
        case "get_total_events":
          return { success: true, data: 2 };
        case "get_total_tickets":
          return { success: true, data: 5 };
        case "get_owner":
          return {
            success: true,
            data: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
          };

        // Registration & Creation methods
        case "register_team":
          if (args.length >= 3) {
            const [name, sport, city] = args;
            return {
              success: true,
              data: 3, // New team ID
              message: `Team "${name}" registered successfully in ${city} for ${sport}`,
            };
          }
          return {
            success: false,
            error: "Invalid arguments for register_team",
          };

        case "register_venue":
          if (args.length >= 3) {
            const [name, capacity, location] = args;
            return {
              success: true,
              data: 3, // New venue ID
              message: `Venue "${name}" registered successfully in ${location} with capacity ${capacity}`,
            };
          }
          return {
            success: false,
            error: "Invalid arguments for register_venue",
          };

        case "create_event":
          if (args.length >= 5) {
            const [homeTeamId, awayTeamId, venueId, date, price] = args;
            return {
              success: true,
              data: 3, // New event ID
              message: `Event created: Team ${homeTeamId} vs Team ${awayTeamId} at Venue ${venueId} on ${date} for ${price}`,
            };
          }
          return {
            success: false,
            error: "Invalid arguments for create_event",
          };

        case "purchase_ticket":
          if (args.length >= 4) {
            const [eventId, seatNumber, section, row] = args;
            return {
              success: true,
              data: 6, // New ticket ID
              message: `Ticket purchased for Event ${eventId}: Section ${section}, Row ${row}, Seat ${seatNumber}`,
            };
          }
          return {
            success: false,
            error: "Invalid arguments for purchase_ticket",
          };

        // Query methods with parameters
        case "get_team_by_id":
          if (args.length >= 1) {
            const teamId = args[0];
            const mockTeams = [
              {
                id: 1,
                name: "Lakers",
                sport: "Basketball",
                city: "Los Angeles",
              },
              {
                id: 2,
                name: "Warriors",
                sport: "Basketball",
                city: "San Francisco",
              },
            ];
            const team = mockTeams.find((t) => t.id === teamId);
            return team
              ? { success: true, data: team }
              : { success: false, error: "Team not found" };
          }
          return {
            success: false,
            error: "Invalid arguments for get_team_by_id",
          };

        case "get_venue_by_id":
          if (args.length >= 1) {
            const venueId = args[0];
            const mockVenues = [
              {
                id: 1,
                name: "Crypto.com Arena",
                capacity: 19068,
                location: "Los Angeles, CA",
              },
              {
                id: 2,
                name: "TD Garden",
                capacity: 19156,
                location: "Boston, MA",
              },
            ];
            const venue = mockVenues.find((v) => v.id === venueId);
            return venue
              ? { success: true, data: venue }
              : { success: false, error: "Venue not found" };
          }
          return {
            success: false,
            error: "Invalid arguments for get_venue_by_id",
          };

        case "get_event_by_id":
          if (args.length >= 1) {
            const eventId = args[0];
            const mockEvents = [
              {
                id: 1,
                name: "Lakers vs Warriors",
                homeTeam: "Lakers",
                awayTeam: "Warriors",
                venue: "Crypto.com Arena",
                date: "2024-01-15",
                price: "150 DOT",
                availableTickets: 100,
              },
              {
                id: 2,
                name: "Celtics vs Heat",
                homeTeam: "Celtics",
                awayTeam: "Heat",
                venue: "TD Garden",
                date: "2024-01-20",
                price: "120 DOT",
                availableTickets: 75,
              },
            ];
            const event = mockEvents.find((e) => e.id === eventId);
            return event
              ? { success: true, data: event }
              : { success: false, error: "Event not found" };
          }
          return {
            success: false,
            error: "Invalid arguments for get_event_by_id",
          };

        case "get_tickets_by_event":
          if (args.length >= 1) {
            const eventId = args[0];
            const mockTickets = [
              {
                id: 1,
                eventId: 1,
                owner: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
                purchasePrice: "150 DOT",
                purchaseDate: 1705276800,
                seatNumber: 101,
                section: "A",
                row: "15",
              },
              {
                id: 2,
                eventId: 1,
                owner: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
                purchasePrice: "150 DOT",
                purchaseDate: 1705276800,
                seatNumber: 102,
                section: "A",
                row: "15",
              },
            ];
            const tickets = mockTickets.filter((t) => t.eventId === eventId);
            return { success: true, data: tickets };
          }
          return {
            success: false,
            error: "Invalid arguments for get_tickets_by_event",
          };

        default:
          return { success: true, data: "Mock response" };
      }
    } catch (error) {
      console.error("Failed to call contract:", error);
      return {
        success: false,
        error: `Contract call failed: ${
          error instanceof Error ? error.message : "Unknown error"
        }`,
      };
    }
  }

  async getContractAddress(): Promise<string | null> {
    return this.contractAddress;
  }

  async isContractDeployed(): Promise<boolean> {
    return this.contractAddress !== null;
  }

  async disconnect(): Promise<void> {
    if (this.api) {
      await this.api.disconnect();
      this.api = null;
    }
    if (this.provider) {
      this.provider.disconnect();
      this.provider = null;
    }
    this.accounts = [];
    this.selectedAccount = null;
    this.contractAddress = null;
  }

  isConnected(): boolean {
    // Check both API connection and localStorage state
    const apiConnected = this.api !== null && this.api.isConnected;
    const localStorageConnected =
      typeof window !== "undefined" &&
      localStorage.getItem("blockchain_connected") === "true";

    return apiConnected || localStorageConnected;
  }

  /**
   * Get the current API instance
   * @returns The current ApiPromise instance or null if not connected
   */
  getApi(): ApiPromise | null {
    return this.api;
  }

  /**
   * Get the selected account
   * @returns The currently selected account or null
   */
  getSelectedAccount(): InjectedAccountWithMeta | null {
    return this.selectedAccount;
  }
}

// Export singleton instance only on client side
let blockchainService: BlockchainService | null = null;

if (typeof window !== "undefined") {
  blockchainService = new BlockchainService();
}
export { blockchainService };
