/**
 * Default argument templates for contract method calls.
 *
 * Returns pre-filled JSON argument strings for each known contract
 * method, giving users a starting point when invoking methods from
 * the ContractInteraction panel.
 *
 * @module utils/methodArgs
 *
 * Exported functions:
 * - {@link getDefaultArgs} - Returns a default JSON args string for a method
 */
import { ContractType } from "./contractMethods";

export function getDefaultArgs(method: string, contractType: ContractType): string {
  if (contractType === "sports") {
    switch (method) {
      case "register_team":
        return '["Lakers", "Basketball", "Los Angeles"]';
      case "register_venue":
        return '["Staples Center", 20000, "Los Angeles, CA"]';
      case "create_event":
        return '[1, 2, 1, "2024-02-15", "150 DOT"]';
      case "purchase_ticket":
        return '[1, 101, "A", "15"]';
      case "get_team_by_id":
      case "get_venue_by_id":
      case "get_event_by_id":
      case "get_tickets_by_event":
        return "[1]";
      default:
        return "[]";
    }
  } else if (contractType === "concert") {
    switch (method) {
      case "register_artist":
        return '["Taylor Swift"]';
      case "register_venue":
        return '["Madison Square Garden", 20000, "New York, NY"]';
      case "create_concert_event":
        return '["Taylor Swift Concert", 1, 1, 1700000000, 20000, "1500000000000000000"]';
      case "purchase_ticket":
        return "[1, 101]";
      case "get_artist":
      case "get_venue":
      case "get_event":
      case "get_ticket":
        return "[1]";
      case "get_user_tickets":
        return '["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]';
      default:
        return "[]";
    }
  } else {
    return "[]";
  }
}
