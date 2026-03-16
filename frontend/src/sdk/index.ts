/**
 * SDK factory - returns mock or real provider based on configuration.
 */

import { ApiPromise } from "@polkadot/api";
import { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import { isMockMode } from "../config/chains";
import type { InkTixSDK } from "./inktixContract";
import { MockProvider } from "./mockProvider";
import { ContractProvider } from "./contractProvider";

export type { InkTixSDK } from "./inktixContract";
export type {
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
  AttendanceToken,
  PriceQuote,
  ContractCallResult,
} from "./types";

/**
 * Create an InkTix SDK instance.
 * Returns ContractProvider when a contract address and API are available,
 * MockProvider otherwise (or when NEXT_PUBLIC_MOCK_MODE=true).
 */
export function createInkTixSDK(
  contractAddress?: string,
  api?: ApiPromise,
  account?: InjectedAccountWithMeta
): InkTixSDK {
  if (isMockMode() || !contractAddress || !api || !account) {
    return new MockProvider();
  }

  return new ContractProvider(contractAddress, api, account);
}
