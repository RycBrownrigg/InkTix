/**
 * Chain configuration - single source of truth for all network settings.
 * Replaces hardcoded endpoint references throughout the codebase.
 */

export interface ChainConfig {
  name: string;
  rpcEndpoint: string;
  tokenSymbol: string;
  tokenDecimals: number;
  isTestnet: boolean;
  hasContractsPallet: boolean;
}

export const CHAINS: Record<string, ChainConfig> = {
  "westend-asset-hub": {
    name: "Westend AssetHub",
    rpcEndpoint: "wss://westend-asset-hub-rpc.polkadot.io",
    tokenSymbol: "WND",
    tokenDecimals: 10,
    isTestnet: true,
    hasContractsPallet: true,
  },
  local: {
    name: "Development",
    rpcEndpoint: "ws://127.0.0.1:9944",
    tokenSymbol: "UNIT",
    tokenDecimals: 10,
    isTestnet: true,
    hasContractsPallet: true,
  },
  westend: {
    name: "Westend",
    rpcEndpoint: "wss://westend-rpc.polkadot.io",
    tokenSymbol: "WND",
    tokenDecimals: 12,
    isTestnet: true,
    hasContractsPallet: false,
  },
  kusama: {
    name: "Kusama",
    rpcEndpoint: "wss://kusama-rpc.polkadot.io",
    tokenSymbol: "KSM",
    tokenDecimals: 12,
    isTestnet: false,
    hasContractsPallet: false,
  },
  polkadot: {
    name: "Polkadot",
    rpcEndpoint: "wss://rpc.polkadot.io",
    tokenSymbol: "DOT",
    tokenDecimals: 10,
    isTestnet: false,
    hasContractsPallet: false,
  },
} as const;

/**
 * Get the active chain config from environment variables,
 * falling back to the default RPC endpoint.
 */
export function getActiveChainConfig(): ChainConfig {
  const envEndpoint = process.env.NEXT_PUBLIC_RPC_ENDPOINT;
  const envChainName = process.env.NEXT_PUBLIC_CHAIN_NAME;

  // Try to match by endpoint
  if (envEndpoint) {
    const matched = Object.values(CHAINS).find(
      (c) => c.rpcEndpoint === envEndpoint
    );
    if (matched) return matched;
  }

  // Try to match by name
  if (envChainName) {
    const matched = Object.values(CHAINS).find(
      (c) => c.name === envChainName
    );
    if (matched) return matched;
  }

  // Default to Westend AssetHub
  return CHAINS["westend-asset-hub"];
}

/**
 * Get the default RPC endpoint from environment or config.
 */
export function getDefaultEndpoint(): string {
  return (
    process.env.NEXT_PUBLIC_RPC_ENDPOINT ||
    CHAINS["westend-asset-hub"].rpcEndpoint
  );
}

/**
 * Get the local development endpoint.
 */
export function getLocalEndpoint(): string {
  return process.env.NEXT_PUBLIC_LOCAL_RPC_ENDPOINT || CHAINS.local.rpcEndpoint;
}

/**
 * Whether mock mode is enabled via environment variable.
 */
export function isMockMode(): boolean {
  return process.env.NEXT_PUBLIC_MOCK_MODE === "true";
}

/**
 * Get preconfigured contract address from environment.
 */
export function getContractAddress(): string | undefined {
  return process.env.NEXT_PUBLIC_CONTRACT_ADDRESS || undefined;
}
