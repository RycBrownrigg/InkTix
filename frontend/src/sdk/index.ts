/**
 * Public entry point and factory for the InkTix SDK.
 *
 * Re-exports all SDK types and provides {@link createInkTixSDK} which
 * returns either a {@link MockProvider} or {@link ContractProvider}
 * depending on environment variables and available connection state.
 *
 * @module sdk/index
 *
 * Exported functions:
 * - {@link createInkTixSDK} - Factory that selects the appropriate provider
 *
 * Re-exported types:
 * - All interfaces from `sdk/types`
 * - {@link InkTixSDK} from `sdk/inktixContract`
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
