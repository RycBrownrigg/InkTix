/**
 * Tests for the combined InkTix Zustand store.
 *
 * Covers connection state transitions, wallet connect/disconnect, contract
 * deployment state, and data slice balance updates using a mocked
 * BlockchainService.
 */
import { describe, it, expect, beforeEach, vi } from "vitest";
import { useInkTixStore } from "../index";

// Mock BlockchainService
vi.mock("../../services/blockchain", () => ({
  BlockchainService: {
    getInstance: () => ({
      connectToNetwork: vi.fn().mockResolvedValue({ success: true }),
      connectWallet: vi.fn().mockResolvedValue({
        success: true,
        data: [
          {
            address: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
            meta: { name: "Alice" },
          },
        ],
      }),
      selectAccount: vi.fn(),
      getBalance: vi.fn().mockResolvedValue({ success: true, data: "1000" }),
      getNetworkInfo: vi.fn().mockResolvedValue({
        success: true,
        data: { chain: "Westend", nodeName: "Parity", nodeVersion: "1.0" },
      }),
      getEvents: vi.fn().mockResolvedValue({ success: true, data: [] }),
      getVenues: vi.fn().mockResolvedValue({ success: true, data: [] }),
      getContractAddress: vi.fn().mockResolvedValue(null),
      isContractDeployed: vi.fn().mockResolvedValue(false),
      restoreConnection: vi.fn().mockResolvedValue(false),
      disconnect: vi.fn(),
      isConnected: vi.fn().mockReturnValue(false),
      deployContract: vi.fn().mockResolvedValue({
        success: true,
        data: "0x1234",
      }),
      callContract: vi.fn().mockResolvedValue({
        success: true,
        data: "mock",
      }),
    }),
  },
}));

describe("InkTix Store", () => {
  beforeEach(() => {
    // Reset store state between tests
    useInkTixStore.setState({
      isConnected: false,
      isConnecting: false,
      isWalletConnected: false,
      accounts: [],
      selectedAccount: null,
      contractAddress: null,
      isContractDeployed: false,
      isDeployingContract: false,
      events: [],
      venues: [],
      balance: null,
      networkInfo: null,
      isLoadingEvents: false,
      isLoadingVenues: false,
      isLoadingBalance: false,
      _initialized: false,
      _service: null,
    });
  });

  describe("Connection State", () => {
    it("should start disconnected", () => {
      const state = useInkTixStore.getState();
      expect(state.isConnected).toBe(false);
      expect(state.isConnecting).toBe(false);
    });

    it("should update connection state", () => {
      useInkTixStore.getState().setIsConnected(true);
      expect(useInkTixStore.getState().isConnected).toBe(true);
    });

    it("should update network info", () => {
      const info = { chain: "Westend", nodeName: "Parity" };
      useInkTixStore.getState().setNetworkInfo(info);
      expect(useInkTixStore.getState().networkInfo).toEqual(info);
    });
  });

  describe("Wallet State", () => {
    it("should start with no wallet connected", () => {
      const state = useInkTixStore.getState();
      expect(state.isWalletConnected).toBe(false);
      expect(state.accounts).toEqual([]);
      expect(state.selectedAccount).toBeNull();
    });

    it("should update wallet connection state", () => {
      useInkTixStore.getState().setIsWalletConnected(true);
      expect(useInkTixStore.getState().isWalletConnected).toBe(true);
    });

    it("should disconnect wallet", () => {
      useInkTixStore.setState({
        isWalletConnected: true,
        accounts: [{ address: "test", meta: { name: "Test" } }] as any,
      });
      useInkTixStore.getState().disconnectWallet();
      const state = useInkTixStore.getState();
      expect(state.isWalletConnected).toBe(false);
      expect(state.accounts).toEqual([]);
    });
  });

  describe("Contract State", () => {
    it("should start with no contract deployed", () => {
      const state = useInkTixStore.getState();
      expect(state.isContractDeployed).toBe(false);
      expect(state.contractAddress).toBeNull();
    });

    it("should reset contract state", () => {
      useInkTixStore.setState({
        contractAddress: "0x1234",
        isContractDeployed: true,
      });
      useInkTixStore.getState().resetContract();
      const state = useInkTixStore.getState();
      expect(state.contractAddress).toBeNull();
      expect(state.isContractDeployed).toBe(false);
    });
  });

  describe("Data State", () => {
    it("should update balance", () => {
      useInkTixStore.getState().setBalance("5000");
      expect(useInkTixStore.getState().balance).toBe("5000");
    });
  });
});
