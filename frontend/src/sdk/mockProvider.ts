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
  PlatformStats,
  AntiScalpingConfig,
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
