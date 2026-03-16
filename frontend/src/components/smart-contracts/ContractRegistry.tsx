"use client";

import React from "react";
import { Database, CheckCircle } from "lucide-react";
import { useBlockchain } from "../../contexts/BlockchainContext";
import { ContractType, getContractMethods } from "../../utils/contractMethods";

interface ContractRegistryProps {
  contractType: ContractType;
}

const ContractRegistry: React.FC<ContractRegistryProps> = ({
  contractType,
}) => {
  const { contractAddress } = useBlockchain();

  const methods = getContractMethods(contractType);

  return (
    <div className="border border-gray-200 rounded-lg p-4">
      <h3 className="text-lg font-semibold text-gray-800 mb-4 flex items-center">
        <Database className="w-5 h-5 mr-2 text-inktix-purple-600" />
        Contract Registry
      </h3>
      <div className="space-y-4">
        <div className="bg-gray-50 rounded-lg p-4">
          <h4 className="font-medium text-gray-800 mb-2">
            Deployed Contracts
          </h4>
          <div className="space-y-2">
            {contractAddress ? (
              <div className="flex items-center justify-between p-3 bg-white rounded border">
                <div className="flex items-center">
                  <CheckCircle className="w-4 h-4 text-green-600 mr-2" />
                  <span className="text-sm font-mono text-gray-800">
                    {contractAddress.slice(0, 20)}...
                    {contractAddress.slice(-8)}
                  </span>
                </div>
                <span className="text-xs text-green-600 bg-green-100 px-2 py-1 rounded">
                  Active
                </span>
              </div>
            ) : (
              <div className="text-center py-4 text-gray-500">
                <Database className="w-8 h-8 mx-auto mb-2 text-gray-400" />
                <p className="text-sm">No contracts deployed yet</p>
              </div>
            )}
          </div>
        </div>

        <div className="bg-blue-50 rounded-lg p-4">
          <h4 className="font-medium text-blue-800 mb-2">
            Contract Methods Available (
            {contractType === "sports"
              ? "Sports Broker"
              : contractType === "concert"
              ? "Concert Broker"
              : "Unknown"}
            )
          </h4>
          <div className="grid grid-cols-2 gap-2 text-sm">
            {methods
              .getters.slice(0, 2)
              .map((method) => (
                <div key={method.value} className="flex items-center">
                  <span className="w-2 h-2 bg-blue-500 rounded-full mr-2"></span>
                  <span>{method.value}</span>
                </div>
              ))}
            {methods
              .creators.slice(0, 2)
              .map((method) => (
                <div key={method.value} className="flex items-center">
                  <span className="w-2 h-2 bg-green-500 rounded-full mr-2"></span>
                  <span>{method.value}</span>
                </div>
              ))}
          </div>
        </div>
      </div>
    </div>
  );
};

export default ContractRegistry;
