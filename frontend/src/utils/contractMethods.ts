/**
 * Contract method registry, organized by contract type and category.
 *
 * Provides curated lists of getter, creator, and query methods for
 * each supported contract variant (sports_broker, concert_broker, or
 * unknown), used by the ContractInteraction UI to populate method
 * selectors.
 *
 * @module utils/contractMethods
 *
 * Exported types:
 * - {@link ContractType} - Discriminator for contract variants
 *
 * Exported interfaces:
 * - {@link ContractMethod} - Value/label pair for a single method
 * - {@link ContractMethodGroups} - Grouped method lists (getters, creators, queries)
 *
 * Exported functions:
 * - {@link getContractMethods} - Returns grouped methods for a contract type
 */
export type ContractType = "sports" | "concert" | "unknown";

export interface ContractMethod {
  value: string;
  label: string;
}

export interface ContractMethodGroups {
  getters: ContractMethod[];
  creators: ContractMethod[];
  queries: ContractMethod[];
}

export function getContractMethods(contractType: ContractType): ContractMethodGroups {
  if (contractType === "sports") {
    return {
      getters: [
        { value: "get_total_teams", label: "get_total_teams" },
        { value: "get_total_venues", label: "get_total_venues" },
        { value: "get_total_events", label: "get_total_events" },
        { value: "get_total_tickets", label: "get_total_tickets" },
        { value: "get_owner", label: "get_owner" },
      ],
      creators: [
        { value: "register_team", label: "register_team(name, sport, city)" },
        {
          value: "register_venue",
          label: "register_venue(name, capacity, location)",
        },
        {
          value: "create_event",
          label: "create_event(homeTeamId, awayTeamId, venueId, date, price)",
        },
        {
          value: "purchase_ticket",
          label: "purchase_ticket(eventId, seatNumber, section, row)",
        },
      ],
      queries: [
        { value: "get_team_by_id", label: "get_team_by_id(teamId)" },
        { value: "get_venue_by_id", label: "get_venue_by_id(venueId)" },
        { value: "get_event_by_id", label: "get_event_by_id(eventId)" },
        {
          value: "get_tickets_by_event",
          label: "get_tickets_by_event(eventId)",
        },
      ],
    };
  } else if (contractType === "concert") {
    return {
      getters: [
        { value: "get_stats", label: "get_stats" },
        { value: "get_owner", label: "get_owner" },
      ],
      creators: [
        { value: "register_artist", label: "register_artist(name)" },
        {
          value: "register_venue",
          label: "register_venue(name, capacity, address)",
        },
        {
          value: "create_concert_event",
          label:
            "create_concert_event(name, artistId, venueId, date, capacity, price)",
        },
        {
          value: "purchase_ticket",
          label: "purchase_ticket(eventId, seatNumber)",
        },
      ],
      queries: [
        { value: "get_artist", label: "get_artist(artistId)" },
        { value: "get_venue", label: "get_venue(venueId)" },
        { value: "get_event", label: "get_event(eventId)" },
        { value: "get_ticket", label: "get_ticket(ticketId)" },
        { value: "get_user_tickets", label: "get_user_tickets(userId)" },
      ],
    };
  } else {
    // Default/unknown contract - show both
    return {
      getters: [
        { value: "get_stats", label: "get_stats" },
        { value: "get_owner", label: "get_owner" },
      ],
      creators: [
        { value: "register_artist", label: "register_artist(name)" },
        {
          value: "register_venue",
          label: "register_venue(name, capacity, address)",
        },
        { value: "create_event", label: "create_event(...)" },
        { value: "purchase_ticket", label: "purchase_ticket(...)" },
      ],
      queries: [
        { value: "get_artist", label: "get_artist(id)" },
        { value: "get_venue", label: "get_venue(id)" },
        { value: "get_event", label: "get_event(id)" },
        { value: "get_ticket", label: "get_ticket(id)" },
      ],
    };
  }
}
