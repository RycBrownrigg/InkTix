/**
 * Zustand slice managing event, venue, and balance data.
 *
 * Loads initial mock data on startup and can refresh from the blockchain
 * service when a live connection is established. Also handles balance
 * queries for the selected account.
 *
 * @module store/slices/dataSlice
 *
 * Exported interfaces:
 * - {@link DataSlice} - State and actions for domain data and balances
 *
 * Exported functions:
 * - {@link createDataSlice} - Zustand StateCreator factory
 */
import { StateCreator } from "zustand";
import {
  BlockchainService,
  EventData,
  VenueData,
} from "../../services/blockchain";

export interface DataSlice {
  // State
  events: EventData[];
  venues: VenueData[];
  balance: string | null;
  isLoadingEvents: boolean;
  isLoadingVenues: boolean;
  isLoadingBalance: boolean;

  // Actions
  refreshData: () => Promise<void>;
  loadBalance: () => Promise<void>;
  loadMockData: () => Promise<void>;
  setBalance: (balance: string | null) => void;
}

const MOCK_EVENTS: EventData[] = [
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
];

const MOCK_VENUES: VenueData[] = [
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
];

export const createDataSlice: StateCreator<
  DataSlice & { _service: BlockchainService | null; isConnected: boolean; selectedAccount: any },
  [],
  [],
  DataSlice
> = (set, get) => ({
  events: [],
  venues: [],
  balance: null,
  isLoadingEvents: false,
  isLoadingVenues: false,
  isLoadingBalance: false,

  setBalance: (balance) => set({ balance }),

  loadMockData: async () => {
    set({ isLoadingEvents: true, isLoadingVenues: true });
    await new Promise((resolve) => setTimeout(resolve, 500));
    set({
      events: MOCK_EVENTS,
      venues: MOCK_VENUES,
      isLoadingEvents: false,
      isLoadingVenues: false,
    });
  },

  loadBalance: async () => {
    const state = get() as any;
    const service = state._service;
    const selectedAccount = state.selectedAccount;
    const isConnected = state.isConnected;

    if (!selectedAccount || !service || !isConnected) return;

    try {
      set({ isLoadingBalance: true });
      const result = await service.getBalance(selectedAccount.address);
      if (result.success && result.data) {
        set({ balance: result.data });
      }
    } catch (error) {
      console.error("Failed to load balance:", error);
    } finally {
      set({ isLoadingBalance: false });
    }
  },

  refreshData: async () => {
    const state = get() as any;
    const service = state._service;
    const isConnected = state.isConnected;

    if (!isConnected || !service) return;

    try {
      set({ isLoadingEvents: true });
      const eventsResult = await service.getEvents();
      if (eventsResult.success && eventsResult.data) {
        set({ events: eventsResult.data });
      }
      set({ isLoadingEvents: false });

      set({ isLoadingVenues: true });
      const venuesResult = await service.getVenues();
      if (venuesResult.success && venuesResult.data) {
        set({ venues: venuesResult.data });
      }
      set({ isLoadingVenues: false });
    } catch (error) {
      console.error("Failed to refresh data:", error);
      set({ isLoadingEvents: false, isLoadingVenues: false });
    }
  },
});
