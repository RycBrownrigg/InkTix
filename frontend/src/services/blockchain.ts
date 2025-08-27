import { ApiPromise, WsProvider } from "@polkadot/api";
import { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import { web3Accounts, web3Enable } from "@polkadot/extension-dapp";
import { cryptoWaitReady } from "@polkadot/util-crypto";

export interface ContractCallResult<T = any> {
  success: boolean;
  data?: T;
  error?: string;
  txHash?: string;
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
  private api: ApiPromise | null = null;
  private provider: WsProvider | null = null;
  private accounts: InjectedAccountWithMeta[] = [];
  private selectedAccount: InjectedAccountWithMeta | null = null;

  constructor() {
    // Only initialize on client side
    if (typeof window !== "undefined") {
      this.initialize();
    }
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
    endpoint: string = "wss://rpc.polkadot.io"
  ): Promise<ContractCallResult> {
    try {
      if (typeof window === "undefined") {
        return { success: false, error: "Not available on server side" };
      }

      if (this.api) {
        await this.api.disconnect();
      }

      const provider = new WsProvider(endpoint);
      this.api = await ApiPromise.create({ provider });

      // Wait for the API to be ready
      await this.api.isReady;

      console.log("Connected to network:", endpoint);
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

      // Enable web3 extension
      const extensions = await web3Enable("InkTix Sports Platform");
      if (extensions.length === 0) {
        throw new Error(
          "No web3 extension found. Please install Polkadot.js extension."
        );
      }

      // Get accounts
      this.accounts = await web3Accounts();
      if (this.accounts.length === 0) {
        throw new Error(
          "No accounts found. Please create or import an account."
        );
      }

      // Select first account by default
      this.selectedAccount = this.accounts[0];

      console.log("Wallet connected:", this.accounts.length, "accounts found");
      return { success: true, data: this.accounts };
    } catch (error) {
      console.error("Failed to connect wallet:", error);
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
    this.selectedAccount = account;
  }

  getSelectedAccount(): InjectedAccountWithMeta | null {
    return this.selectedAccount;
  }

  async getBalance(address: string): Promise<ContractCallResult<string>> {
    try {
      if (typeof window === "undefined") {
        return { success: false, error: "Not available on server side" };
      }

      if (!this.api) {
        throw new Error("Not connected to network");
      }

      const accountInfo = await this.api.query.system.account(address);
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
  }

  isConnected(): boolean {
    return this.api !== null && this.api.isConnected;
  }
}

// Export singleton instance only on client side
let blockchainService: BlockchainService | null = null;

if (typeof window !== "undefined") {
  blockchainService = new BlockchainService();
}

export { blockchainService };
