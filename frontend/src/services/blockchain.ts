import { ApiPromise, WsProvider } from "@polkadot/api";
import { CodePromise } from "@polkadot/api-contract";
import { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import { web3Accounts, web3Enable, web3FromAddress } from "@polkadot/extension-dapp";
import { cryptoWaitReady } from "@polkadot/util-crypto";
import { createInkTixSDK, type InkTixSDK } from "../sdk";
import inktixMetadata from "../sdk/abi/inktix.json";

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
  private sdk: InkTixSDK | null = null;

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
    this.selectedAccount = account;
    this.refreshSDK();
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
      if (!this.selectedAccount) {
        return { success: false, error: "No wallet account selected" };
      }

      if (!this.api || !this.api.isConnected) {
        const restored = await this.restoreConnection();
        if (!restored) {
          return { success: false, error: "Not connected to network" };
        }
      }

      // Check if contracts pallet is available
      if (!this.api!.tx.contracts) {
        console.log("Contracts pallet not available, using mock deployment");
        await new Promise((resolve) => setTimeout(resolve, 2000));
        const mockAddr = "0x" + Math.random().toString(16).substr(2, 40);
        this.contractAddress = mockAddr;
        this.refreshSDK();
        return {
          success: true,
          data: mockAddr,
          message: "Contract deployed (mock — chain has no contracts pallet)",
          txHash: "0x" + Math.random().toString(16).substr(2, 64),
        };
      }

      console.log("Deploying contract on-chain...");

      const code = new CodePromise(
        this.api!,
        inktixMetadata,
        new Uint8Array(contractWasm)
      );

      const injector = await web3FromAddress(this.selectedAccount.address);
      const endowmentValue = BigInt(endowment);

      const tx = code.tx.new(
        { gasLimit: { refTime: 300_000_000_000, proofSize: 1_000_000 } as any, storageDepositLimit: null, value: endowmentValue }
      );

      return new Promise((resolve) => {
        tx.signAndSend(
            this.selectedAccount!.address,
            { signer: injector.signer },
            ({ status, contract, dispatchError }: any) => {
              if (status.isInBlock || status.isFinalized) {
                if (dispatchError) {
                  let errorMsg = "Deployment failed";
                  if (dispatchError.isModule) {
                    const decoded = this.api!.registry.findMetaError(
                      dispatchError.asModule
                    );
                    errorMsg = `${decoded.section}.${decoded.name}`;
                  }
                  resolve({ success: false, error: errorMsg });
                } else if (contract) {
                  this.contractAddress = contract.address.toString();
                  this.refreshSDK();
                  console.log("Contract deployed at:", this.contractAddress);
                  resolve({
                    success: true,
                    data: this.contractAddress!,
                    txHash: status.isInBlock
                      ? status.asInBlock.toHex()
                      : status.asFinalized.toHex(),
                  });
                } else {
                  resolve({ success: false, error: "No contract address returned" });
                }
              }
            }
          )
          .catch((err: any) => {
            resolve({
              success: false,
              error: `Signing cancelled or failed: ${err instanceof Error ? err.message : String(err)}`,
            });
          });
      });
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

  /**
   * Refresh the SDK instance when contract address or account changes
   */
  private refreshSDK() {
    if (this.contractAddress && this.api && this.selectedAccount) {
      this.sdk = createInkTixSDK(this.contractAddress, this.api, this.selectedAccount);
    }
  }

  /**
   * Get the SDK instance for typed contract calls
   */
  getSDK(): InkTixSDK | null {
    if (!this.sdk && this.contractAddress && this.api && this.selectedAccount) {
      this.refreshSDK();
    }
    return this.sdk;
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

      console.log(`Contract call: ${method}`, args);

      const sdk = this.getSDK();
      if (!sdk) {
        return { success: false, error: "SDK not initialized" };
      }

      // Route method calls through the typed SDK
      switch (method) {
        // ─── Getters ───
        case "get_owner":
          return sdk.getOwner();
        case "get_totals":
        case "get_stats":
          return sdk.getTotals();
        case "get_platform_stats":
          return sdk.getPlatformStats();

        // ─── Team ───
        case "register_team":
          return sdk.registerTeam(args[0], args[1], args[2]);
        case "get_team":
        case "get_team_by_id":
          return sdk.getTeam(args[0]);
        case "get_all_teams":
          return sdk.getAllTeams();

        // ─── Artist ───
        case "register_artist":
          return sdk.registerArtist(args[0]);
        case "get_artist":
          return sdk.getArtist(args[0]);
        case "verify_artist":
          return sdk.verifyArtist(args[0]);

        // ─── Venue ───
        case "register_venue":
          return sdk.registerVenue(args[0], args[1], args[2]);
        case "get_venue":
        case "get_venue_by_id":
          return sdk.getVenue(args[0]);
        case "get_all_venues":
          return sdk.getAllVenues();

        // ─── Event ───
        case "create_event":
          return sdk.createSportsEvent(
            args[0], args[1], args[2], args[3], args[4],
            args[5], args[6], args[7], args[8], args[9]
          );
        case "create_concert_event":
          return sdk.createConcertEvent(
            args[0], args[1], args[2], args[3], args[4], args[5]
          );
        case "get_event":
        case "get_event_by_id":
          return sdk.getEvent(args[0]);
        case "get_all_events":
          return sdk.getAllEvents();

        // ─── Ticket ───
        case "purchase_ticket":
          return sdk.purchaseTicket(args[0], args[1], args[2], args[3]);
        case "get_ticket":
          return sdk.getTicket(args[0]);
        case "get_user_tickets":
          return sdk.getUserTickets(args[0]);
        case "transfer_ticket":
          return sdk.transferTicket(args[0], args[1]);
        case "get_tickets_by_event":
          return sdk.getUserTickets(args[0]); // Fallback

        // ─── Resale ───
        case "resell_ticket":
          return sdk.resellTicket(args[0], args[1], args[2]);
        case "get_resale_listings":
          return sdk.getResaleListings();
        case "buy_resale_ticket":
          return sdk.buyResaleTicket(args[0]);

        // ─── NFT ───
        case "mint_ticket_nft":
          return sdk.mintTicketNft(args[0]);
        case "verify_ticket_nft":
          return sdk.verifyTicketNft(args[0]);
        case "use_ticket_nft":
          return sdk.useTicketNft(args[0]);
        case "get_user_nft_tickets":
          return sdk.getUserNftTickets(args[0]);
        case "get_nft_by_ticket":
          return sdk.getNftByTicket(args[0]);

        // ─── Anti-scalping ───
        case "get_anti_scalping_config":
          return sdk.getAntiScalpingConfig(args[0]);

        default:
          console.warn(`Unknown contract method: ${method}, using SDK fallback`);
          return { success: true, data: null, message: `Method ${method} not mapped` };
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
