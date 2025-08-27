"use client";

import React, { useState } from "react";
import { useBlockchain } from "../contexts/BlockchainContext";
import { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import {
  Wallet,
  CheckCircle,
  X,
  RefreshCw,
  Network,
  User,
  Coins,
} from "lucide-react";

const BlockchainWallet: React.FC = () => {
  const {
    isConnected,
    isConnecting,
    isWalletConnected,
    accounts,
    selectedAccount,
    balance,
    networkInfo,
    connectWallet,
    selectAccount,
    disconnect,
    refreshData,
    connectToNetwork,
    isLoadingBalance,
  } = useBlockchain();

  const [showAccountSelector, setShowAccountSelector] = useState(false);

  const handleConnectWallet = async () => {
    try {
      await connectWallet();
    } catch (error) {
      console.error("Failed to connect wallet:", error);
    }
  };

  const handleSelectAccount = async (account: InjectedAccountWithMeta) => {
    await selectAccount(account);
    setShowAccountSelector(false);
  };

  const handleDisconnect = async () => {
    await disconnect();
  };

  const formatBalance = (balance: string | null) => {
    if (!balance) return "0.0000";
    // Convert from smallest unit (planck) to DOT
    const balanceNum = parseFloat(balance) / Math.pow(10, 10);
    return balanceNum.toFixed(4);
  };

  const truncateAddress = (address: string) => {
    return `${address.slice(0, 6)}...${address.slice(-4)}`;
  };

  // Show wallet connection interface first
  if (!isWalletConnected) {
    return (
      <div className="bg-white rounded-lg shadow-lg p-6 border border-gray-200">
        <div className="text-center">
          <Wallet className="w-16 h-16 mx-auto mb-4 text-inktix-blue-600" />
          <h3 className="text-xl font-semibold text-gray-800 mb-2">
            Connect Your Wallet
          </h3>
          <p className="text-gray-600 mb-6">
            Connect your Polkadot.js wallet to start using InkTix
          </p>
          <button
            onClick={handleConnectWallet}
            className="bg-gradient-to-r from-inktix-blue-600 to-inktix-blue-700 hover:from-inktix-blue-700 hover:to-inktix-blue-800 text-white font-semibold py-3 px-6 rounded-lg transition-all duration-200 transform hover:scale-105 shadow-lg"
          >
            <CheckCircle className="w-5 h-5 inline mr-2" />
            Connect Wallet
          </button>
        </div>
      </div>
    );
  }

  // Show network connection prompt if wallet is connected but not to network
  if (!isConnected) {
    return (
      <div className="bg-white rounded-lg shadow-lg border border-gray-200 overflow-hidden">
        {/* Header */}
        <div className="bg-gradient-to-r from-inktix-blue-600 to-inktix-blue-700 px-6 py-4 text-white">
          <div className="flex items-center justify-between">
            <div className="flex items-center">
              <Wallet className="w-5 h-5 mr-2" />
              <span className="font-semibold">Wallet Connected</span>
            </div>
            <button
              onClick={handleDisconnect}
              className="text-inktix-blue-100 hover:text-white transition-colors"
            >
              <X className="w-5 h-5" />
            </button>
          </div>
        </div>

        {/* Content */}
        <div className="p-6">
          <div className="text-center">
            <Network className="w-16 h-16 mx-auto mb-4 text-inktix-blue-600" />
            <h3 className="text-xl font-semibold text-gray-800 mb-2">
              Connect to Network
            </h3>
            <p className="text-gray-600 mb-6">
              Your wallet is connected! Now connect to a blockchain network to
              view real-time data.
            </p>
            <button
              onClick={() => connectToNetwork()}
              className="bg-gradient-to-r from-inktix-orange-500 to-inktix-orange-600 hover:from-inktix-orange-600 hover:to-inktix-orange-700 text-white font-semibold py-3 px-6 rounded-lg transition-all duration-200 transform hover:scale-105 shadow-lg"
            >
              <Network className="w-5 h-5 inline mr-2" />
              Connect to Network
            </button>
          </div>
        </div>
      </div>
    );
  }

  // Show full wallet interface when both wallet and network are connected
  return (
    <div className="bg-white rounded-lg shadow-lg border border-gray-200 overflow-hidden">
      {/* Header */}
      <div className="bg-gradient-to-r from-inktix-blue-600 to-inktix-blue-700 px-6 py-4 text-white">
        <div className="flex items-center justify-between">
          <div className="flex items-center">
            <Wallet className="w-5 h-5 mr-2" />
            <span className="font-semibold">Wallet Connected</span>
          </div>
          <button
            onClick={handleDisconnect}
            className="text-inktix-blue-100 hover:text-white transition-colors"
          >
            <X className="w-5 h-5" />
          </button>
        </div>
      </div>

      {/* Account Info */}
      <div className="p-6">
        {selectedAccount && (
          <div className="mb-6">
            <div className="flex items-center justify-between mb-4">
              <h3 className="text-lg font-semibold text-gray-800 flex items-center">
                <User className="w-5 h-5 mr-2 text-inktix-blue-600" />
                Selected Account
              </h3>
              <button
                onClick={() => setShowAccountSelector(!showAccountSelector)}
                className="text-inktix-blue-600 hover:text-inktix-blue-700 text-sm font-medium"
              >
                Switch Account
              </button>
            </div>

            <div className="bg-gray-50 rounded-lg p-4">
              <div className="flex items-center justify-between mb-2">
                <span className="text-sm text-gray-600">Address:</span>
                <span className="font-mono text-sm text-gray-800">
                  {truncateAddress(selectedAccount.address)}
                </span>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-sm text-gray-600">Name:</span>
                <span className="text-sm font-medium text-gray-800">
                  {selectedAccount.meta.name || "Unnamed Account"}
                </span>
              </div>
            </div>
          </div>
        )}

        {/* Balance */}
        <div className="mb-6">
          <h3 className="text-lg font-semibold text-gray-800 mb-3 flex items-center">
            <Coins className="w-5 h-5 mr-2 text-inktix-orange-500" />
            Balance
          </h3>
          <div className="bg-gradient-to-r from-inktix-orange-50 to-inktix-blue-50 rounded-lg p-4">
            <div className="flex items-center justify-between">
              <span className="text-2xl font-bold text-gray-800">
                {isLoadingBalance ? (
                  <RefreshCw className="w-6 h-6 animate-spin text-inktix-blue-600" />
                ) : (
                  `${formatBalance(balance)} DOT`
                )}
              </span>
              <button
                onClick={refreshData}
                className="text-inktix-blue-600 hover:text-inktix-blue-700 p-2 rounded-full hover:bg-inktix-blue-50 transition-colors"
              >
                <RefreshCw className="w-4 h-4" />
              </button>
            </div>
          </div>
        </div>

        {/* Network Info */}
        {networkInfo && (
          <div className="mb-6">
            <h3 className="text-lg font-semibold text-gray-800 mb-3 flex items-center">
              <Network className="w-5 h-5 mr-2 text-inktix-green-600" />
              Network
            </h3>
            <div className="bg-gray-50 rounded-lg p-4 space-y-2">
              <div className="flex justify-between text-sm">
                <span className="text-gray-600">Chain:</span>
                <span className="font-medium text-gray-800">
                  {networkInfo.chain}
                </span>
              </div>
              <div className="flex justify-between text-sm">
                <span className="text-gray-600">Node:</span>
                <span className="font-medium text-gray-800">
                  {networkInfo.nodeName}
                </span>
              </div>
              <div className="flex justify-between text-sm">
                <span className="text-gray-600">Version:</span>
                <span className="font-medium text-gray-800">
                  {networkInfo.nodeVersion}
                </span>
              </div>
            </div>
          </div>
        )}

        {/* Account Selector */}
        {showAccountSelector && (
          <div className="border-t pt-4">
            <h4 className="text-sm font-medium text-gray-700 mb-3">
              Available Accounts:
            </h4>
            <div className="space-y-2 max-h-40 overflow-y-auto">
              {accounts.map(
                (account: InjectedAccountWithMeta, index: number) => (
                  <button
                    key={index}
                    onClick={() => handleSelectAccount(account)}
                    className={`w-full text-left p-3 rounded-lg border transition-colors ${
                      selectedAccount?.address === account.address
                        ? "border-inktix-blue-500 bg-inktix-blue-50 text-inktix-blue-700"
                        : "border-gray-200 hover:border-inktix-blue-300 hover:bg-inktix-blue-50"
                    }`}
                  >
                    <div className="flex items-center justify-between">
                      <div>
                        <div className="font-medium text-gray-800">
                          {account.meta.name || `Account ${index + 1}`}
                        </div>
                        <div className="text-sm text-gray-500 font-mono">
                          {truncateAddress(account.address)}
                        </div>
                      </div>
                      {selectedAccount?.address === account.address && (
                        <div className="w-2 h-2 bg-inktix-blue-500 rounded-full"></div>
                      )}
                    </div>
                  </button>
                )
              )}
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default BlockchainWallet;
