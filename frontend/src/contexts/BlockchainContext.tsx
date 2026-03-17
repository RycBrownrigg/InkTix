/**
 * Backward-compatible shim for the legacy BlockchainContext API.
 *
 * Re-exports {@link useBlockchain} from the hooks layer and provides a
 * no-op {@link BlockchainProvider} wrapper so that existing consumer code
 * and BlockchainWrapper continue to work after state management moved
 * to Zustand.
 *
 * @module contexts/BlockchainContext
 *
 * Re-exported hooks:
 * - {@link useBlockchain} - Zustand-backed blockchain state hook
 *
 * Exported components:
 * - {@link BlockchainProvider} - Pass-through wrapper (no-op)
 */
"use client";
export { useBlockchain } from "../hooks/useBlockchain";

// BlockchainProvider is no longer needed - Zustand manages state globally.
// Keep a pass-through wrapper for backward compatibility with BlockchainWrapper.
import React, { ReactNode } from "react";

interface BlockchainProviderProps {
  children: ReactNode;
}

export const BlockchainProvider: React.FC<BlockchainProviderProps> = ({
  children,
}) => {
  return <>{children}</>;
};
