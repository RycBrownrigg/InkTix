/**
 * Renders the current deployment status and address of the active smart contract.
 *
 * @module components/smart-contracts/ContractStatus
 */
"use client";

import React from "react";
import { CheckCircle } from "lucide-react";

interface ContractStatusProps {
  isContractDeployed: boolean;
  contractAddress: string | null;
}

const ContractStatus: React.FC<ContractStatusProps> = ({
  isContractDeployed,
  contractAddress,
}) => {
  return (
    <div className="bg-gray-50 rounded-lg p-4">
      <h3 className="text-lg font-semibold text-gray-800 mb-3 flex items-center">
        <CheckCircle
          className={`w-5 h-5 mr-2 ${
            isContractDeployed ? "text-green-600" : "text-gray-400"
          }`}
        />
        Contract Status
      </h3>
      <div className="space-y-2">
        <div className="flex justify-between text-sm">
          <span className="text-gray-600">Deployed:</span>
          <span
            className={`font-medium ${
              isContractDeployed ? "text-green-600" : "text-red-600"
            }`}
          >
            {isContractDeployed ? "Yes" : "No"}
          </span>
        </div>
        {contractAddress && (
          <div className="flex justify-between text-sm">
            <span className="text-gray-600">Address:</span>
            <span className="font-mono text-xs text-gray-800 break-all">
              {contractAddress}
            </span>
          </div>
        )}
      </div>
    </div>
  );
};

export default ContractStatus;
