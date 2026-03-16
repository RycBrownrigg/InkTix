/**
 * Mock implementation of the InkTix SDK.
 * All mock data extracted from blockchain.ts callContract switch cases.
 */

import type { InkTixSDK } from "./inktixContract";
import type {
  Team,
  Artist,
  Venue,
  Event,
  Ticket,
  ResaleListing,
  PlatformStats,
  AntiScalpingConfig,
  TicketNft,
  TicketVerification,
  PriceQuote,
  ContractCallResult,
} from "./types";

// Mock data stores
const mockTeams: Team[] = [
  {
    id: 1,
    name: "Lakers",
    city: "Los Angeles",
    sportType: "Basketball",
    verified: true,
  },
  {
    id: 2,
    name: "Warriors",
    city: "San Francisco",
    sportType: "Basketball",
    verified: true,
  },
];

const mockArtists: Artist[] = [
  {
    id: 1,
    name: "Taylor Swift",
    verified: true,
    account: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
  },
];

const mockVenues: Venue[] = [
  {
    id: 1,
    name: "Crypto.com Arena",
    capacity: 19068,
    location: "Los Angeles, CA",
    venueType: "Arena",
  },
  {
    id: 2,
    name: "TD Garden",
    capacity: 19156,
    location: "Boston, MA",
    venueType: "Arena",
  },
];

const mockEvents: Event[] = [
  {
    id: 1,
    name: "Lakers vs Warriors",
    venueId: 1,
    date: 1705276800,
    capacity: 19068,
    basePrice: "150 DOT",
    soldTickets: 500,
    status: "OnSale",
    category: {
      type: "Sports",
      homeTeamId: 1,
      awayTeamId: 2,
      seasonId: 1,
      gameType: "RegularSeason",
      sportType: "Basketball",
    },
  },
  {
    id: 2,
    name: "Celtics vs Heat",
    venueId: 2,
    date: 1705708800,
    capacity: 19156,
    basePrice: "120 DOT",
    soldTickets: 300,
    status: "OnSale",
    category: {
      type: "Sports",
      homeTeamId: 1,
      awayTeamId: 2,
      seasonId: 1,
      gameType: "RegularSeason",
      sportType: "Basketball",
    },
  },
];

const mockTickets: Ticket[] = [
  {
    id: 1,
    eventId: 1,
    owner: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    seatNumber: 101,
    section: "A",
    row: "15",
    purchasePrice: "150 DOT",
    purchaseDate: 1705276800,
    currency: "DOT",
    loyaltyPointsEarned: 150,
  },
  {
    id: 2,
    eventId: 1,
    owner: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    seatNumber: 102,
    section: "A",
    row: "15",
    purchasePrice: "150 DOT",
    purchaseDate: 1705276800,
    currency: "DOT",
    loyaltyPointsEarned: 150,
  },
];

const mockResaleListings: ResaleListing[] = [
  {
    ticketId: 101,
    seller: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
    askingPrice: "180",
    originalPrice: "150",
    currency: "DOT",
    expiryTime: Date.now() + 86400000,
    isActive: true,
    approved: true,
    eventId: 1,
    eventName: "Lakers vs Warriors",
    section: "A",
    row: "12",
    seatNumber: 45,
  },
  {
    ticketId: 102,
    seller: "5GNJqTPyNqANBkUVMN1LPPrxXnFouWA2MRQg3gKrUYgw6J9",
    askingPrice: "95",
    originalPrice: "85",
    currency: "DOT",
    expiryTime: Date.now() + 172800000,
    isActive: true,
    approved: true,
    eventId: 3,
    eventName: "Taylor Swift - Eras Tour",
    section: "Floor",
    row: "3",
    seatNumber: 18,
  },
  {
    ticketId: 103,
    seller: "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw",
    askingPrice: "200",
    originalPrice: "150",
    currency: "DOT",
    expiryTime: Date.now() + 43200000,
    isActive: true,
    approved: true,
    eventId: 1,
    eventName: "Lakers vs Warriors",
    section: "VIP",
    row: "1",
    seatNumber: 8,
  },
  {
    ticketId: 104,
    seller: "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmkMEpch5HKGR5kiVh",
    askingPrice: "60",
    originalPrice: "50",
    currency: "DOT",
    expiryTime: Date.now() + 259200000,
    isActive: true,
    approved: true,
    eventId: 5,
    eventName: "Kendrick Lamar Live",
    section: "B",
    row: "8",
    seatNumber: 22,
  },
  {
    ticketId: 105,
    seller: "5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy",
    askingPrice: "130",
    originalPrice: "120",
    currency: "DOT",
    expiryTime: Date.now() + 345600000,
    isActive: true,
    approved: false,
    eventId: 2,
    eventName: "Celtics vs Heat",
    section: "C",
    row: "15",
    seatNumber: 33,
  },
];

const mockNftTickets: TicketNft[] = [];
let nextNftTokenId = 1;

let nextTeamId = 3;
let nextArtistId = 2;
let nextVenueId = 3;
let nextEventId = 3;
let nextTicketId = 3;

export class MockProvider implements InkTixSDK {
  // Team management
  async registerTeam(
    name: string,
    city: string,
    sportType: string
  ): Promise<ContractCallResult<number>> {
    const id = nextTeamId++;
    mockTeams.push({ id, name, city, sportType, verified: false });
    return {
      success: true,
      data: id,
      message: `Team "${name}" registered successfully in ${city} for ${sportType}`,
    };
  }

  async getTeam(teamId: number): Promise<ContractCallResult<Team>> {
    const team = mockTeams.find((t) => t.id === teamId);
    return team
      ? { success: true, data: team }
      : { success: false, error: "Team not found" };
  }

  async getAllTeams(): Promise<ContractCallResult<Team[]>> {
    return { success: true, data: [...mockTeams] };
  }

  // Artist management
  async registerArtist(name: string): Promise<ContractCallResult<number>> {
    const id = nextArtistId++;
    mockArtists.push({ id, name, verified: false, account: null });
    return {
      success: true,
      data: id,
      message: `Artist "${name}" registered successfully`,
    };
  }

  async getArtist(artistId: number): Promise<ContractCallResult<Artist>> {
    const artist = mockArtists.find((a) => a.id === artistId);
    return artist
      ? { success: true, data: artist }
      : { success: false, error: "Artist not found" };
  }

  async verifyArtist(artistId: number): Promise<ContractCallResult<void>> {
    const artist = mockArtists.find((a) => a.id === artistId);
    if (!artist) return { success: false, error: "Artist not found" };
    artist.verified = true;
    return { success: true, message: `Artist "${artist.name}" verified` };
  }

  // Venue management
  async registerVenue(
    name: string,
    capacity: number,
    location: string
  ): Promise<ContractCallResult<number>> {
    const id = nextVenueId++;
    mockVenues.push({ id, name, capacity, location, venueType: "Arena" });
    return {
      success: true,
      data: id,
      message: `Venue "${name}" registered successfully in ${location} with capacity ${capacity}`,
    };
  }

  async getVenue(venueId: number): Promise<ContractCallResult<Venue>> {
    const venue = mockVenues.find((v) => v.id === venueId);
    return venue
      ? { success: true, data: venue }
      : { success: false, error: "Venue not found" };
  }

  async getAllVenues(): Promise<ContractCallResult<Venue[]>> {
    return { success: true, data: [...mockVenues] };
  }

  // Event management
  async createSportsEvent(
    name: string,
    venueId: number,
    date: number,
    capacity: number,
    basePrice: string,
    homeTeamId: number,
    awayTeamId: number,
    seasonId: number,
    gameType: string,
    sportType: string
  ): Promise<ContractCallResult<number>> {
    const id = nextEventId++;
    mockEvents.push({
      id,
      name,
      venueId,
      date,
      capacity,
      basePrice,
      soldTickets: 0,
      status: "Scheduled",
      category: {
        type: "Sports",
        homeTeamId,
        awayTeamId,
        seasonId,
        gameType,
        sportType,
      },
    });
    return {
      success: true,
      data: id,
      message: `Event "${name}" created successfully`,
    };
  }

  async createConcertEvent(
    name: string,
    artistId: number,
    venueId: number,
    date: number,
    capacity: number,
    basePrice: string
  ): Promise<ContractCallResult<number>> {
    const id = nextEventId++;
    mockEvents.push({
      id,
      name,
      venueId,
      date,
      capacity,
      basePrice,
      soldTickets: 0,
      status: "Scheduled",
      category: { type: "Concert", artistId },
    });
    return {
      success: true,
      data: id,
      message: `Concert "${name}" created successfully`,
    };
  }

  async getEvent(eventId: number): Promise<ContractCallResult<Event>> {
    const event = mockEvents.find((e) => e.id === eventId);
    return event
      ? { success: true, data: event }
      : { success: false, error: "Event not found" };
  }

  async getAllEvents(): Promise<ContractCallResult<Event[]>> {
    return { success: true, data: [...mockEvents] };
  }

  // Ticket management
  async purchaseTicket(
    eventId: number,
    seatNumber: number,
    section = "GA",
    row = "1"
  ): Promise<ContractCallResult<number>> {
    const id = nextTicketId++;
    mockTickets.push({
      id,
      eventId,
      owner: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
      seatNumber,
      section,
      row,
      purchasePrice: "150 DOT",
      purchaseDate: Date.now(),
      currency: "DOT",
      loyaltyPointsEarned: 150,
    });
    return {
      success: true,
      data: id,
      message: `Ticket purchased for Event ${eventId}: Section ${section}, Row ${row}, Seat ${seatNumber}`,
    };
  }

  async getTicket(ticketId: number): Promise<ContractCallResult<Ticket>> {
    const ticket = mockTickets.find((t) => t.id === ticketId);
    return ticket
      ? { success: true, data: ticket }
      : { success: false, error: "Ticket not found" };
  }

  async getUserTickets(
    userId: string
  ): Promise<ContractCallResult<number[]>> {
    const ids = mockTickets
      .filter((t) => t.owner === userId)
      .map((t) => t.id);
    return { success: true, data: ids };
  }

  async transferTicket(
    ticketId: number,
    to: string
  ): Promise<ContractCallResult<void>> {
    const ticket = mockTickets.find((t) => t.id === ticketId);
    if (!ticket) return { success: false, error: "Ticket not found" };
    ticket.owner = to;
    return { success: true, message: "Ticket transferred successfully" };
  }

  // Resale marketplace
  async resellTicket(
    ticketId: number,
    price: string,
    currency: string,
    metadata?: { eventName?: string; section?: string; row?: string; seatNumber?: number; originalPrice?: string }
  ): Promise<ContractCallResult<void>> {
    const ticket = mockTickets.find((t) => t.id === ticketId);
    mockResaleListings.push({
      ticketId,
      seller: ticket?.owner || "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
      askingPrice: price,
      originalPrice: metadata?.originalPrice || ticket?.purchasePrice || price,
      currency,
      expiryTime: Date.now() + 86400000,
      isActive: true,
      approved: false,
      eventId: ticket?.eventId || 0,
      eventName: metadata?.eventName || (ticket ? `Event #${ticket.eventId}` : `Ticket #${ticketId}`),
      section: metadata?.section || ticket?.section || "GA",
      row: metadata?.row || ticket?.row || "1",
      seatNumber: metadata?.seatNumber || ticket?.seatNumber || ticketId,
    });
    return { success: true, message: "Ticket listed for resale successfully" };
  }

  async getResaleListings(): Promise<ContractCallResult<ResaleListing[]>> {
    const active = mockResaleListings.filter(
      (l) => l.isActive && l.expiryTime > Date.now()
    );
    return { success: true, data: active };
  }

  async buyResaleTicket(
    ticketId: number
  ): Promise<ContractCallResult<void>> {
    const listing = mockResaleListings.find(
      (l) => l.ticketId === ticketId && l.isActive
    );
    if (!listing) return { success: false, error: "Listing not found or expired" };
    listing.isActive = false;
    return { success: true, message: "Resale ticket purchased successfully" };
  }

  // NFT management
  async mintTicketNft(ticketId: number): Promise<ContractCallResult<number>> {
    const existing = mockNftTickets.find(n => n.ticketId === ticketId);
    if (existing) return { success: false, error: "NFT already minted for this ticket" };

    const ticket = mockTickets.find(t => t.id === ticketId);
    const tokenId = nextNftTokenId++;
    const nft: TicketNft = {
      tokenId,
      ticketId,
      owner: ticket?.owner || "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
      eventId: ticket?.eventId || 1,
      eventName: "Lakers vs Warriors",
      venueName: "Crypto.com Arena",
      eventDate: Date.now() + 86400000,
      section: ticket?.section || "GA",
      row: ticket?.row || "1",
      seatNumber: ticket?.seatNumber || 1,
      seatType: "Reserved",
      accessLevel: "Standard",
      mintedAt: Date.now(),
      metadataUri: "",
      verificationHash: `0x${Array.from({length: 64}, () => Math.floor(Math.random() * 16).toString(16)).join('')}`,
      isUsed: false,
    };
    mockNftTickets.push(nft);
    return { success: true, data: tokenId, message: "NFT minted successfully" };
  }

  async verifyTicketNft(tokenId: number): Promise<ContractCallResult<TicketVerification>> {
    const nft = mockNftTickets.find(n => n.tokenId === tokenId);
    if (!nft) return { success: false, error: "NFT not found" };
    return {
      success: true,
      data: {
        isValid: true,
        isUsed: nft.isUsed,
        owner: nft.owner,
        eventId: nft.eventId,
        eventName: nft.eventName,
        section: nft.section,
        row: nft.row,
        seatNumber: nft.seatNumber,
      },
    };
  }

  async useTicketNft(tokenId: number): Promise<ContractCallResult<number>> {
    const nft = mockNftTickets.find(n => n.tokenId === tokenId);
    if (!nft) return { success: false, error: "NFT not found" };
    if (nft.isUsed) return { success: false, error: "Ticket already used" };
    nft.isUsed = true;
    return { success: true, data: tokenId, message: "Ticket used, attendance token minted" };
  }

  async getUserNftTickets(userId: string): Promise<ContractCallResult<TicketNft[]>> {
    const nfts = mockNftTickets.filter(n => n.owner === userId);
    return { success: true, data: nfts };
  }

  async getNftByTicket(ticketId: number): Promise<ContractCallResult<TicketNft>> {
    const nft = mockNftTickets.find(n => n.ticketId === ticketId);
    return nft
      ? { success: true, data: nft }
      : { success: false, error: "No NFT for this ticket" };
  }

  // Dynamic pricing
  async getPriceQuote(
    eventId: number,
    seatType: string,
    isSeasonPass: boolean
  ): Promise<ContractCallResult<PriceQuote>> {
    const event = mockEvents.find((e) => e.id === eventId);
    const basePrice = event ? parseFloat(event.basePrice) : 100;
    const capacity = event?.capacity || 1000;
    const sold = event?.soldTickets || 0;
    const demandPct = Math.round((sold / capacity) * 100);

    // Simulate pricing factors
    const seatMults: Record<string, number> = {
      GeneralAdmission: 10000, Reserved: 11000, PremiumReserved: 13000,
      Club: 15000, Suite: 20000, FieldLevel: 18000, Courtside: 25000, StudentSection: 7000,
    };
    const seatMult = seatMults[seatType] || 10000;

    let demandMult = 10000;
    if (demandPct >= 95) demandMult = 18000;
    else if (demandPct >= 90) demandMult = 15000;
    else if (demandPct >= 75) demandMult = 12500;
    else if (demandPct >= 60) demandMult = 11000;
    else if (demandPct < 20) demandMult = 8000;

    const timeMult = 10000; // Can't know event date from mock
    const rivalryMult = 10000;
    const discount = isSeasonPass ? 15 : 0;
    const discountMult = 10000 - discount * 100;

    let finalPrice = basePrice;
    finalPrice = finalPrice * demandMult / 10000;
    finalPrice = finalPrice * seatMult / 10000;
    finalPrice = finalPrice * discountMult / 10000;
    finalPrice = Math.max(finalPrice, basePrice / 2);
    finalPrice = Math.min(finalPrice, basePrice * 3);

    const multiplier = basePrice > 0 ? Math.round(finalPrice * 10000 / basePrice) : 10000;

    return {
      success: true,
      data: {
        basePrice,
        finalPrice: Math.round(finalPrice * 100) / 100,
        multiplier,
        demandPercentage: demandPct,
        demandMultiplier: demandMult,
        timeMultiplier: timeMult,
        seatMultiplier: seatMult,
        rivalryMultiplier: rivalryMult,
        seasonPassDiscount: discount,
      },
    };
  }

  // Analytics
  async getPlatformStats(): Promise<ContractCallResult<PlatformStats>> {
    return {
      success: true,
      data: {
        totalEvents: mockEvents.length,
        totalTicketsSold: mockTickets.length,
        totalRevenue: "300 DOT",
        totalUsers: 1,
        averageTicketPrice: "150 DOT",
      },
    };
  }

  // Anti-scalping
  async getAntiScalpingConfig(
    eventId: number
  ): Promise<ContractCallResult<AntiScalpingConfig>> {
    return {
      success: true,
      data: {
        maxTicketsPerUser: 4,
        transferCooldown: 86400,
        resalePriceCap: 150,
        enabled: true,
      },
    };
  }

  // Utility
  async getOwner(): Promise<ContractCallResult<string>> {
    return {
      success: true,
      data: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    };
  }

  async getTotals(): Promise<
    ContractCallResult<{
      teams: number;
      venues: number;
      events: number;
      tickets: number;
    }>
  > {
    return {
      success: true,
      data: {
        teams: mockTeams.length,
        venues: mockVenues.length,
        events: mockEvents.length,
        tickets: mockTickets.length,
      },
    };
  }
}
