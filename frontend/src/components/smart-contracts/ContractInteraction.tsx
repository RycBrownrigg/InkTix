"use client";

import React, { useState } from "react";
import { Play } from "lucide-react";
import { useBlockchain } from "../../contexts/BlockchainContext";
import { ContractType, getContractMethods } from "../../utils/contractMethods";
import { getDefaultArgs } from "../../utils/methodArgs";

interface ContractInteractionProps {
  contractType: ContractType;
}

const ContractInteraction: React.FC<ContractInteractionProps> = ({
  contractType,
}) => {
  const { callContract } = useBlockchain();

  const [selectedMethod, setSelectedMethod] = useState("get_stats");
  const [methodArgs, setMethodArgs] = useState("[]");
  const [contractCallResult, setContractCallResult] = useState<string>("");

  const handleMethodChange = (method: string) => {
    setSelectedMethod(method);
    setMethodArgs(getDefaultArgs(method, contractType));
  };

  const handleContractCall = async () => {
    try {
      let args: any[] = [];
      try {
        args = JSON.parse(methodArgs);
      } catch {
        args = [];
      }

      const result = await callContract(selectedMethod, args);

      if (result.success) {
        setContractCallResult(
          `✅ Call successful!\nResult: ${JSON.stringify(result.data, null, 2)}`
        );
      } else {
        setContractCallResult(`❌ Call failed: ${result.error}`);
      }
    } catch (error) {
      setContractCallResult(
        `❌ Error: ${error instanceof Error ? error.message : "Unknown error"}`
      );
    }
  };

  const methods = getContractMethods(contractType);

  return (
    <div className="border border-gray-200 rounded-lg p-4">
      <h3 className="text-lg font-semibold text-gray-800 mb-4 flex items-center">
        <Play className="w-5 h-5 mr-2 text-inktix-green-600" />
        Interact with Contract
      </h3>

      <div className="space-y-4">
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            Method
          </label>
          <select
            value={selectedMethod}
            onChange={(e) => handleMethodChange(e.target.value)}
            className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-inktix-blue-500 focus:border-inktix-blue-500"
          >
            {/* Dynamic methods based on contract type */}
            <optgroup label="Get Information">
              {methods.getters.map((method) => (
                <option key={method.value} value={method.value}>
                  {method.label}
                </option>
              ))}
            </optgroup>

            <optgroup label="Registration & Creation">
              {methods.creators.map((method) => (
                <option key={method.value} value={method.value}>
                  {method.label}
                </option>
              ))}
            </optgroup>

            <optgroup label="Queries with Parameters">
              {methods.queries.map((method) => (
                <option key={method.value} value={method.value}>
                  {method.label}
                </option>
              ))}
            </optgroup>
          </select>
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            Arguments (JSON array)
          </label>
          <input
            type="text"
            value={methodArgs}
            onChange={(e) => setMethodArgs(e.target.value)}
            className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-inktix-blue-500 focus:border-inktix-blue-500"
            placeholder="[]"
          />
          <p className="text-xs text-gray-500 mt-1">
            Arguments will auto-fill when you select a method. Edit as
            needed.
          </p>
        </div>

        <button
          onClick={handleContractCall}
          className="w-full bg-gradient-to-r from-inktix-green-600 to-inktix-green-700 hover:from-inktix-green-700 hover:to-inktix-green-800 text-white font-semibold py-3 px-6 rounded-lg transition-all duration-200 transform hover:scale-105 shadow-lg"
        >
          <Play className="w-5 h-5 inline mr-2" />
          Call Method
        </button>

        {contractCallResult && (
          <div className="mt-4 p-3 bg-gray-100 rounded-lg">
            <pre className="text-sm text-gray-800 whitespace-pre-wrap">
              {contractCallResult}
            </pre>
          </div>
        )}
      </div>
    </div>
  );
};

export default ContractInteraction;
