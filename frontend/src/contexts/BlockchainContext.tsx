"use client";

/**
 * Backward-compatible re-export of useBlockchain.
 * The actual state management has moved to Zustand (src/store/).
 * This file exists so existing imports continue to work.
 */
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
