/**
 * Real contract implementation of the InkTix SDK.
 * Uses @polkadot/api-contract to interact with the deployed unified contract.
 */

import { ApiPromise } from "@polkadot/api";
import { ContractPromise } from "@polkadot/api-contract";
import { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import { web3FromAddress } from "@polkadot/extension-dapp";
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
import inktixMetadata from "./abi/inktix.json";

// Gas limit for dry-run queries
const QUERY_GAS_LIMIT = -1; // Use maximum
const QUERY_STORAGE_DEPOSIT = null;

// Gas limit for transactions (will be estimated)
const TX_GAS_LIMIT = { refTime: 300_000_000_000, proofSize: 1_000_000 };

/**
 * Helper to extract the Ok value from ink! Result types.
 * ink! wraps returns in Result<Result<T, ContractError>, LangError>
 */
function unwrapResult(result: any): any {
  if (result?.ok !== undefined) {
    return unwrapResult(result.ok);
  }
  if (result?.Ok !== undefined) {
    return unwrapResult(result.Ok);
  }
  return result;
}

/**
 * Helper to convert ink! struct output to plain JS object
 */
function toPlain(obj: any): any {
  if (obj === null || obj === undefined) return obj;
  if (obj.toHuman) return obj.toHuman();
  if (obj.toJSON) return obj.toJSON();
  return obj;
}

export class ContractProvider implements InkTixSDK {
  private contract: ContractPromise;
  private callerAddress: string;

  constructor(
    private contractAddress: string,
    private api: ApiPromise,
    private account: InjectedAccountWithMeta
  ) {
    this.contract = new ContractPromise(api, inktixMetadata, contractAddress);
    this.callerAddress = account.address;
  }

  /**
   * Update the caller account (e.g., after account switch)
   */
  setAccount(account: InjectedAccountWithMeta) {
    this.account = account;
    this.callerAddress = account.address;
  }

  // ─── Query helper (read-only, no gas cost) ───

  private async query<T>(
    method: string,
    ...args: any[]
  ): Promise<ContractCallResult<T>> {
    try {
      const { result, output } = await this.contract.query[method](
        this.callerAddress,
        { gasLimit: QUERY_GAS_LIMIT, storageDepositLimit: QUERY_STORAGE_DEPOSIT },
        ...args
      );

      if (result.isErr) {
        return {
          success: false,
          error: `Query failed: ${result.asErr.toHuman()}`,
        };
      }

      if (!output) {
        return { success: false, error: "No output from query" };
      }

      const value = unwrapResult(output.toJSON());
      return { success: true, data: value as T };
    } catch (error) {
      return {
        success: false,
        error: `Query error: ${error instanceof Error ? error.message : String(error)}`,
      };
    }
  }

  // ─── Transaction helper (write, costs gas) ───

  private async tx<T>(
    method: string,
    ...args: any[]
  ): Promise<ContractCallResult<T>> {
    try {
      // Dry-run to estimate gas and check for errors
      const { result: dryResult, output, gasRequired } =
        await this.contract.query[method](
          this.callerAddress,
          { gasLimit: QUERY_GAS_LIMIT, storageDepositLimit: QUERY_STORAGE_DEPOSIT },
          ...args
        );

      if (dryResult.isErr) {
        return {
          success: false,
          error: `Dry-run failed: ${dryResult.asErr.toHuman()}`,
        };
      }

      // Get signer from extension
      const injector = await web3FromAddress(this.callerAddress);

      // Send actual transaction with estimated gas
      return new Promise((resolve) => {
        this.contract.tx[method](
          { gasLimit: gasRequired, storageDepositLimit: QUERY_STORAGE_DEPOSIT },
          ...args
        )
          .signAndSend(
            this.callerAddress,
            { signer: injector.signer },
            ({ status, events, dispatchError }) => {
              if (status.isInBlock || status.isFinalized) {
                if (dispatchError) {
                  let errorMsg = "Transaction failed";
                  if (dispatchError.isModule) {
                    const decoded = this.api.registry.findMetaError(
                      dispatchError.asModule
                    );
                    errorMsg = `${decoded.section}.${decoded.name}: ${decoded.docs.join(" ")}`;
                  }
                  resolve({ success: false, error: errorMsg });
                } else {
                  const value = output ? unwrapResult(output.toJSON()) : null;
                  resolve({
                    success: true,
                    data: value as T,
                    txHash: status.isInBlock
                      ? status.asInBlock.toHex()
                      : status.asFinalized.toHex(),
                  });
                }
              }
            }
          )
          .catch((err: any) => {
            resolve({
              success: false,
              error: `Sign failed: ${err instanceof Error ? err.message : String(err)}`,
            });
          });
      });
    } catch (error) {
      return {
        success: false,
        error: `Transaction error: ${error instanceof Error ? error.message : String(error)}`,
      };
    }
  }

  // ─── Team Management ───

  async registerTeam(
    name: string,
    city: string,
    sportType: string
  ): Promise<ContractCallResult<number>> {
    return this.tx("register_team", name, city, { [sportType]: null });
  }

  async getTeam(teamId: number): Promise<ContractCallResult<Team>> {
    return this.query("get_team", teamId);
  }

  async getAllTeams(): Promise<ContractCallResult<Team[]>> {
    return this.query("get_all_teams");
  }

  // ─── Artist Management ───

  async registerArtist(name: string): Promise<ContractCallResult<number>> {
    return this.tx("register_artist", name);
  }

  async getArtist(artistId: number): Promise<ContractCallResult<Artist>> {
    return this.query("get_artist", artistId);
  }

  async verifyArtist(artistId: number): Promise<ContractCallResult<void>> {
    return this.tx("verify_artist", artistId);
  }

  // ─── Venue Management ───

  async registerVenue(
    name: string,
    capacity: number,
    location: string
  ): Promise<ContractCallResult<number>> {
    return this.tx("register_venue", name, capacity, location, { Arena: null });
  }

  async getVenue(venueId: number): Promise<ContractCallResult<Venue>> {
    return this.query("get_venue", venueId);
  }

  async getAllVenues(): Promise<ContractCallResult<Venue[]>> {
    return this.query("get_all_venues");
  }

  // ─── Event Management ───

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
    const category = {
      Sports: {
        home_team_id: homeTeamId,
        away_team_id: awayTeamId,
        season_id: seasonId,
        game_type: { [gameType]: null },
        sport_type: { [sportType]: null },
      },
    };
    return this.tx(
      "create_event",
      name,
      venueId,
      date,
      capacity,
      basePrice,
      category
    );
  }

  async createConcertEvent(
    name: string,
    artistId: number,
    venueId: number,
    date: number,
    capacity: number,
    basePrice: string
  ): Promise<ContractCallResult<number>> {
    return this.tx(
      "create_concert_event",
      name,
      artistId,
      venueId,
      date,
      capacity,
      basePrice
    );
  }

  async getEvent(eventId: number): Promise<ContractCallResult<Event>> {
    return this.query("get_event", eventId);
  }

  async getAllEvents(): Promise<ContractCallResult<Event[]>> {
    return this.query("get_all_events");
  }

  // ─── Ticket Management ───

  async purchaseTicket(
    eventId: number,
    seatNumber: number,
    section = "GA",
    row = "1"
  ): Promise<ContractCallResult<number>> {
    const seat = {
      seat_number: seatNumber,
      section,
      row,
      seat_type: { GeneralAdmission: null },
      access_level: { Standard: null },
    };
    return this.tx("purchase_ticket", eventId, seat, { DOT: null });
  }

  async getTicket(ticketId: number): Promise<ContractCallResult<Ticket>> {
    return this.query("get_ticket", ticketId);
  }

  async getUserTickets(
    userId: string
  ): Promise<ContractCallResult<number[]>> {
    return this.query("get_user_tickets", userId);
  }

  async transferTicket(
    ticketId: number,
    to: string
  ): Promise<ContractCallResult<void>> {
    return this.tx("transfer_ticket", ticketId, to);
  }

  // ─── Analytics ───

  async getPlatformStats(): Promise<ContractCallResult<PlatformStats>> {
    return this.query("get_platform_stats");
  }

  // ─── Anti-Scalping ───

  async getAntiScalpingConfig(
    eventId: number
  ): Promise<ContractCallResult<AntiScalpingConfig>> {
    return this.query("get_anti_scalping_config", eventId);
  }

  // ─── Utility ───

  async getOwner(): Promise<ContractCallResult<string>> {
    return this.query("get_owner");
  }

  async getTotals(): Promise<
    ContractCallResult<{
      teams: number;
      venues: number;
      events: number;
      tickets: number;
    }>
  > {
    const result = await this.query<any>("get_totals");
    if (result.success && Array.isArray(result.data)) {
      // get_totals returns a tuple (teams, venues, events, tickets, seasons, season_passes)
      const [teams, venues, events, tickets] = result.data;
      return {
        success: true,
        data: { teams, venues, events, tickets },
      };
    }
    return result;
  }
}
