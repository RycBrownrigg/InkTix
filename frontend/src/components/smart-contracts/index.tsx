"use client";

import React, { useState, useEffect } from "react";
import { useBlockchain } from "../../contexts/BlockchainContext";
import { Database } from "lucide-react";
import { ContractType } from "../../utils/contractMethods";
import ContractStatus from "./ContractStatus";
import ContractDeployment from "./ContractDeployment";
import ContractInteraction from "./ContractInteraction";
import ContractRegistry from "./ContractRegistry";
import CrossChainPanel from "./CrossChainPanel";
import ContractInfo from "./ContractInfo";

const SmartContractManager: React.FC = () => {
  const {
    isConnected,
    isWalletConnected,
    contractAddress,
    isContractDeployed,
  } = useBlockchain();

  const [contractType, setContractType] = useState<ContractType>("unknown");

  // Debug: Show current connection status
  console.log("SmartContractManager Debug:", {
    isConnected,
    isWalletConnected,
    contractAddress,
    isContractDeployed,
  });

  // Log when state changes
  useEffect(() => {
    console.log("🔄 SmartContractManager: State changed:", {
      isConnected,
      isWalletConnected,
      contractAddress,
      isContractDeployed,
    });
  }, [isConnected, isWalletConnected, contractAddress, isContractDeployed]);

  if (!isConnected || !isWalletConnected) {
    return (
      <div className="bg-white rounded-lg shadow-lg p-6 border border-gray-200">
        <div className="text-center">
          <Database className="w-16 h-16 mx-auto mb-4 text-gray-400" />
          <h3 className="text-xl font-semibold text-gray-600 mb-2">
            Connect to Deploy Contract
          </h3>
          <p className="text-gray-500">
            Please connect your wallet and network to deploy and manage smart
            contracts.
          </p>
          {/* Debug info */}
          <div className="mt-4 p-3 bg-gray-100 rounded text-xs text-left">
            <p>
              <strong>Debug Info:</strong>
            </p>
            <p>Wallet Connected: {isWalletConnected ? "✅ Yes" : "❌ No"}</p>
            <p>Network Connected: {isConnected ? "✅ Yes" : "❌ No"}</p>
            <p>Contract Address: {contractAddress || "None"}</p>
            <p>Contract Deployed: {isContractDeployed ? "✅ Yes" : "❌ No"}</p>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="bg-white rounded-lg shadow-lg border border-gray-200 overflow-hidden">
      {/* Header */}
      <div className="bg-gradient-to-r from-inktix-purple-600 to-inktix-purple-700 px-6 py-4 text-white">
        <div className="flex items-center">
          <Database className="w-5 h-5 mr-2" />
          <span className="font-semibold">Smart Contract Manager</span>
        </div>
      </div>

      <div className="p-6 space-y-6">
        {/* Contract Status */}
        <ContractStatus
          isContractDeployed={isContractDeployed}
          contractAddress={contractAddress}
        />

        {/* Contract Deployment */}
        {!isContractDeployed && (
          <ContractDeployment onContractTypeChange={setContractType} />
        )}

        {/* Contract Interaction */}
        {isContractDeployed && (
          <ContractInteraction contractType={contractType} />
        )}

        {/* Contract Registry */}
        <ContractRegistry contractType={contractType} />

        {/* Cross-Chain Manager */}
        <CrossChainPanel />

        {/* Contract Info */}
        <ContractInfo contractType={contractType} />
      </div>
    </div>
  );
};

export default SmartContractManager;
