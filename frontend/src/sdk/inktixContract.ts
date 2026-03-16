/**
 * InkTix SDK interface - defines the contract for interacting with
 * the InkTix smart contract (both mock and real implementations).
 */

import type {
  Team,
  Artist,
  Venue,
  Event,
  Ticket,
  PlatformStats,
  AntiScalpingConfig,
  ResaleListing,
  TicketNft,
  TicketVerification,
  ContractCallResult,
} from "./types";

export interface InkTixSDK {
  // Team management
  registerTeam(
    name: string,
    city: string,
    sportType: string
  ): Promise<ContractCallResult<number>>;
  getTeam(teamId: number): Promise<ContractCallResult<Team>>;
  getAllTeams(): Promise<ContractCallResult<Team[]>>;

  // Artist management
  registerArtist(name: string): Promise<ContractCallResult<number>>;
  getArtist(artistId: number): Promise<ContractCallResult<Artist>>;
  verifyArtist(artistId: number): Promise<ContractCallResult<void>>;

  // Venue management
  registerVenue(
    name: string,
    capacity: number,
    location: string
  ): Promise<ContractCallResult<number>>;
  getVenue(venueId: number): Promise<ContractCallResult<Venue>>;
  getAllVenues(): Promise<ContractCallResult<Venue[]>>;

  // Event management
  createSportsEvent(
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
  ): Promise<ContractCallResult<number>>;
  createConcertEvent(
    name: string,
    artistId: number,
    venueId: number,
    date: number,
    capacity: number,
    basePrice: string
  ): Promise<ContractCallResult<number>>;
  getEvent(eventId: number): Promise<ContractCallResult<Event>>;
  getAllEvents(): Promise<ContractCallResult<Event[]>>;

  // Ticket management
  purchaseTicket(
    eventId: number,
    seatNumber: number,
    section?: string,
    row?: string
  ): Promise<ContractCallResult<number>>;
  getTicket(ticketId: number): Promise<ContractCallResult<Ticket>>;
  getUserTickets(userId: string): Promise<ContractCallResult<number[]>>;
  transferTicket(
    ticketId: number,
    to: string
  ): Promise<ContractCallResult<void>>;

  // Resale marketplace
  resellTicket(
    ticketId: number,
    price: string,
    currency: string
  ): Promise<ContractCallResult<void>>;
  getResaleListings(): Promise<ContractCallResult<ResaleListing[]>>;
  buyResaleTicket(ticketId: number): Promise<ContractCallResult<void>>;

  // NFT management
  mintTicketNft(ticketId: number): Promise<ContractCallResult<number>>;
  verifyTicketNft(tokenId: number): Promise<ContractCallResult<TicketVerification>>;
  useTicketNft(tokenId: number): Promise<ContractCallResult<number>>;
  getUserNftTickets(userId: string): Promise<ContractCallResult<TicketNft[]>>;
  getNftByTicket(ticketId: number): Promise<ContractCallResult<TicketNft>>;

  // Analytics
  getPlatformStats(): Promise<ContractCallResult<PlatformStats>>;

  // Anti-scalping
  getAntiScalpingConfig(
    eventId: number
  ): Promise<ContractCallResult<AntiScalpingConfig>>;

  // Utility
  getOwner(): Promise<ContractCallResult<string>>;
  getTotals(): Promise<
    ContractCallResult<{
      teams: number;
      venues: number;
      events: number;
      tickets: number;
    }>
  >;
}
