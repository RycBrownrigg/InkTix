/**
 * TypeScript interfaces matching the unified InkTix contract types.
 */

export interface Team {
  id: number;
  name: string;
  city: string;
  sportType: string;
  verified: boolean;
}

export interface Artist {
  id: number;
  name: string;
  verified: boolean;
  account: string | null;
}

export interface Venue {
  id: number;
  name: string;
  capacity: number;
  location: string;
  venueType: string;
}

export interface Event {
  id: number;
  name: string;
  venueId: number;
  date: number;
  capacity: number;
  basePrice: string;
  soldTickets: number;
  status: EventStatus;
  category: EventCategory;
}

export type EventStatus =
  | "Scheduled"
  | "OnSale"
  | "SoldOut"
  | "InProgress"
  | "Completed"
  | "Cancelled";

export type EventCategory =
  | {
      type: "Sports";
      homeTeamId: number;
      awayTeamId: number;
      seasonId: number;
      gameType: string;
      sportType: string;
    }
  | { type: "Concert"; artistId: number }
  | { type: "Generic" };

export interface Ticket {
  id: number;
  eventId: number;
  owner: string;
  seatNumber: number;
  section: string;
  row: string;
  purchasePrice: string;
  purchaseDate: number;
  currency: string;
  loyaltyPointsEarned: number;
}

export interface AntiScalpingConfig {
  maxTicketsPerUser: number;
  transferCooldown: number;
  resalePriceCap: number;
  enabled: boolean;
}

export interface PlatformStats {
  totalEvents: number;
  totalTicketsSold: number;
  totalRevenue: string;
  totalUsers: number;
  averageTicketPrice: string;
}

export interface ResaleListing {
  ticketId: number;
  seller: string;
  askingPrice: string;
  originalPrice: string;
  currency: string;
  expiryTime: number;
  isActive: boolean;
  approved: boolean;
  eventId: number;
  eventName: string;
  section: string;
  row: string;
  seatNumber: number;
}

export interface TicketNft {
  tokenId: number;
  ticketId: number;
  owner: string;
  eventId: number;
  eventName: string;
  venueName: string;
  eventDate: number;
  section: string;
  row: string;
  seatNumber: number;
  seatType: string;
  accessLevel: string;
  mintedAt: number;
  metadataUri: string;
  verificationHash: string;
  isUsed: boolean;
}

export interface TicketVerification {
  isValid: boolean;
  isUsed: boolean;
  owner: string;
  eventId: number;
  eventName: string;
  section: string;
  row: string;
  seatNumber: number;
}

export interface AttendanceToken {
  tokenId: number;
  ticketId: number;
  eventId: number;
  owner: string;
  attendedAt: number;
}

export interface ContractCallResult<T = any> {
  success: boolean;
  data?: T;
  error?: string;
  txHash?: string;
  message?: string;
}
