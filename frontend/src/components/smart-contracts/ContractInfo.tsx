/**
 * Renders metadata about the active contract type (name, features, target network).
 *
 * @module components/smart-contracts/ContractInfo
 */
"use client";

import React from "react";
import { FileText } from "lucide-react";
import { ContractType } from "../../utils/contractMethods";

interface ContractInfoProps {
  contractType: ContractType;
}

const ContractInfo: React.FC<ContractInfoProps> = ({ contractType }) => {
  return (
    <div className="bg-blue-50 rounded-lg p-4">
      <h3 className="text-lg font-semibold text-blue-800 mb-3 flex items-center">
        <FileText className="w-5 h-5 mr-2" />
        Contract Information
      </h3>
      <div className="text-sm text-blue-700 space-y-2">
        <p>
          • <strong>Name:</strong>{" "}
          {contractType === "sports"
            ? "Sports Broker Contract"
            : contractType === "concert"
            ? "Concert Broker Contract"
            : "Smart Contract"}
        </p>
        <p>
          • <strong>Type:</strong> Ink! Smart Contract
        </p>
        <p>
          • <strong>Features:</strong>{" "}
          {contractType === "sports"
            ? "Team management, venue registration, event creation, ticket purchasing"
            : contractType === "concert"
            ? "Artist management, venue registration, concert creation, ticket purchasing"
            : "Smart contract functionality"}
        </p>
        <p>
          • <strong>Network:</strong> Westend AssetHub
          (wss://westend-asset-hub-rpc.polkadot.io)
        </p>
      </div>
    </div>
  );
};

export default ContractInfo;
