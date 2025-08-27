"use client";

import { useState, useEffect } from "react";
import { Wallet, Shield, AlertCircle, CheckCircle } from "lucide-react";

interface WalletAccount {
  address: string;
  name?: string;
  type: "sr25519" | "ed25519" | "ecdsa";
}

interface WalletConnectProps {
  onConnect?: (account: WalletAccount) => void;
  onDisconnect?: () => void;
}

export default function WalletConnect({
  onConnect,
  onDisconnect,
}: WalletConnectProps) {
  const [isConnected, setIsConnected] = useState(false);
  const [isConnecting, setIsConnecting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [selectedAccount, setSelectedAccount] = useState<WalletAccount | null>(
    null
  );
  const [availableAccounts, setAvailableAccounts] = useState<WalletAccount[]>(
    []
  );
  const [isMounted, setIsMounted] = useState(false);

  useEffect(() => {
    setIsMounted(true);
    // Check if wallet is already connected
    checkWalletConnection();
  }, []);

  const checkWalletConnection = async () => {
    try {
      // Check if Polkadot extension is available
      if (typeof window !== "undefined" && window.injectedWeb3) {
        const extensions = Object.keys(window.injectedWeb3);
        if (extensions.length > 0) {
          // Wallet extension is available
          console.log("Available wallet extensions:", extensions);
        }
      }
    } catch (err) {
      console.error("Error checking wallet connection:", err);
    }
  };

  const connectWallet = async () => {
    setIsConnecting(true);
    setError(null);

    try {
      // Check if Polkadot extension is available
      if (typeof window === "undefined" || !window.injectedWeb3) {
        throw new Error(
          "Polkadot extension not found. Please install a wallet extension like Polkadot.js or Talisman."
        );
      }

      // Get available extensions
      const extensions = Object.keys(window.injectedWeb3);
      if (extensions.length === 0) {
        throw new Error(
          "No wallet extensions found. Please install a Polkadot wallet extension."
        );
      }

      // For demo purposes, we'll simulate a connection
      // In a real implementation, you would use the actual Polkadot.js API
      await new Promise((resolve) => setTimeout(resolve, 1000));

      // Simulate getting accounts
      const mockAccounts: WalletAccount[] = [
        {
          address: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
          name: "Alice",
          type: "sr25519",
        },
        {
          address: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
          name: "Bob",
          type: "sr25519",
        },
      ];

      setAvailableAccounts(mockAccounts);
      setSelectedAccount(mockAccounts[0]);
      setIsConnected(true);
      onConnect?.(mockAccounts[0]);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to connect wallet");
    } finally {
      setIsConnecting(false);
    }
  };

  const disconnectWallet = () => {
    setIsConnected(false);
    setSelectedAccount(null);
    setAvailableAccounts([]);
    setError(null);
    onDisconnect?.();
  };

  const selectAccount = (account: WalletAccount) => {
    setSelectedAccount(account);
    onConnect?.(account);
  };

  // Prevent hydration mismatch by not rendering until mounted
  if (!isMounted) {
    return (
      <div className="card">
        <div className="text-center">
          <div className="bg-primary-100 rounded-full p-3 w-fit mx-auto mb-4">
            <Wallet className="w-8 h-8 text-primary-600" />
          </div>
          <h3 className="text-xl font-semibold text-secondary-900 mb-2">
            Loading...
          </h3>
          <div className="w-4 h-4 border-2 border-primary-600 border-t-transparent rounded-full animate-spin mx-auto"></div>
        </div>
      </div>
    );
  }

  if (isConnected && selectedAccount) {
    return (
      <div className="card">
        <div className="flex items-center justify-between mb-4">
          <div className="flex items-center gap-3">
            <div className="bg-green-100 rounded-full p-2">
              <CheckCircle className="w-5 h-5 text-green-600" />
            </div>
            <div>
              <h3 className="font-semibold text-secondary-900">
                Wallet Connected
              </h3>
              <p className="text-sm text-secondary-600">
                {selectedAccount.name || "Unknown Account"}
              </p>
            </div>
          </div>
          <button
            onClick={disconnectWallet}
            className="text-secondary-500 hover:text-secondary-700 transition-colors"
          >
            Disconnect
          </button>
        </div>

        <div className="space-y-3">
          <div className="flex items-center justify-between p-3 bg-secondary-50 rounded-lg">
            <span className="text-sm font-medium text-secondary-700">
              Address:
            </span>
            <span className="text-sm font-mono text-secondary-900">
              {selectedAccount.address.slice(0, 8)}...
              {selectedAccount.address.slice(-8)}
            </span>
          </div>

          <div className="flex items-center justify-between p-3 bg-secondary-50 rounded-lg">
            <span className="text-sm font-medium text-secondary-700">
              Type:
            </span>
            <span className="text-sm text-secondary-900 capitalize">
              {selectedAccount.type}
            </span>
          </div>

          {availableAccounts.length > 1 && (
            <div className="mt-4">
              <label className="block text-sm font-medium text-secondary-700 mb-2">
                Switch Account
              </label>
              <select
                value={selectedAccount.address}
                onChange={(e) => {
                  const account = availableAccounts.find(
                    (acc) => acc.address === e.target.value
                  );
                  if (account) selectAccount(account);
                }}
                className="input-field"
              >
                {availableAccounts.map((account) => (
                  <option key={account.address} value={account.address}>
                    {account.name || "Unknown"} ({account.address.slice(0, 8)}
                    ...)
                  </option>
                ))}
              </select>
            </div>
          )}
        </div>
      </div>
    );
  }

  return (
    <div className="card">
      <div className="text-center">
        <div className="bg-primary-100 rounded-full p-3 w-fit mx-auto mb-4">
          <Wallet className="w-8 h-8 text-primary-600" />
        </div>
        <h3 className="text-xl font-semibold text-secondary-900 mb-2">
          Connect Your Wallet
        </h3>
        <p className="text-secondary-600 mb-6">
          Connect your Polkadot wallet to start using InkTix
        </p>

        {error && (
          <div className="mb-4 p-3 bg-red-50 border border-red-200 rounded-lg">
            <div className="flex items-center gap-2 text-red-700">
              <AlertCircle className="w-4 h-4" />
              <span className="text-sm">{error}</span>
            </div>
          </div>
        )}

        <button
          onClick={connectWallet}
          disabled={isConnecting}
          className="btn-primary w-full disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {isConnecting ? (
            <div className="flex items-center gap-2">
              <div className="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin"></div>
              Connecting...
            </div>
          ) : (
            <div className="flex items-center gap-2">
              <Shield className="w-4 h-4" />
              Connect Wallet
            </div>
          )}
        </button>

        <div className="mt-4 text-xs text-secondary-500">
          <p>Supported: Polkadot.js, Talisman, SubWallet</p>
        </div>
      </div>
    </div>
  );
}

// Extend Window interface for Polkadot extension
declare global {
  interface Window {
    injectedWeb3?: Record<string, any>;
  }
}
