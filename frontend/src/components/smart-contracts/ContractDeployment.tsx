"use client";

import React, { useState } from "react";
import { Upload } from "lucide-react";
import { useBlockchain } from "../../contexts/BlockchainContext";
import { ContractType } from "../../utils/contractMethods";

interface ContractDeploymentProps {
  onContractTypeChange: (type: ContractType) => void;
}

const ContractDeployment: React.FC<ContractDeploymentProps> = ({
  onContractTypeChange,
}) => {
  const {
    isConnected,
    isWalletConnected,
    isDeployingContract,
    deployContract,
  } = useBlockchain();

  const [selectedFile, setSelectedFile] = useState<File | null>(null);
  const [endowment, setEndowment] = useState("1.0");
  const [deploymentResult, setDeploymentResult] = useState<string>("");

  const handleFileSelect = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (file && file.name.endsWith(".wasm")) {
      setSelectedFile(file);

      // Detect contract type based on filename
      const filename = file.name.toLowerCase();
      if (filename.includes("sports") || filename.includes("sports_broker")) {
        onContractTypeChange("sports");
      } else if (
        filename.includes("concert") ||
        filename.includes("concert_broker")
      ) {
        onContractTypeChange("concert");
      } else {
        onContractTypeChange("unknown");
      }
    } else {
      alert("Please select a valid .wasm file");
    }
  };

  const handleDeploy = async () => {
    if (!selectedFile) {
      alert("Please select a contract file first");
      return;
    }

    try {
      console.log("🚀 SmartContractManager: Starting deployment...");
      console.log("🚀 SmartContractManager: isConnected:", isConnected);
      console.log(
        "🚀 SmartContractManager: isWalletConnected:",
        isWalletConnected
      );

      const arrayBuffer = await selectedFile.arrayBuffer();
      const endowmentInPlanck = Math.floor(parseFloat(endowment) * 1e10); // Convert WND to planck

      console.log("🚀 SmartContractManager: Calling deployContract...");
      const result = await deployContract(
        arrayBuffer,
        endowmentInPlanck.toString()
      );
      console.log("🚀 SmartContractManager: deployContract result:", result);

      if (result.success) {
        setDeploymentResult(
          `✅ Contract deployed successfully!\nAddress: ${result.data}\nTransaction: ${result.txHash}`
        );
      } else {
        setDeploymentResult(`❌ Deployment failed: ${result.error}`);
      }
    } catch (error) {
      setDeploymentResult(
        `❌ Error: ${error instanceof Error ? error.message : "Unknown error"}`
      );
    }
  };

  return (
    <div className="border border-gray-200 rounded-lg p-4">
      <h3 className="text-lg font-semibold text-gray-800 mb-4 flex items-center">
        <Upload className="w-5 h-5 mr-2 text-inktix-blue-600" />
        Deploy Contract
      </h3>

      <div className="space-y-4">
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            Contract File (.wasm)
          </label>
          <input
            type="file"
            accept=".wasm"
            onChange={handleFileSelect}
            className="block w-full text-sm text-gray-500 file:mr-4 file:py-2 file:px-4 file:rounded-full file:border-0 file:text-sm file:font-semibold file:bg-inktix-blue-50 file:text-inktix-blue-700 hover:file:bg-inktix-blue-100"
          />
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            Endowment (WND)
          </label>
          <input
            type="number"
            step="0.1"
            min="0.1"
            value={endowment}
            onChange={(e) => setEndowment(e.target.value)}
            className="block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-inktix-blue-500 focus:border-inktix-blue-500"
            placeholder="1.0"
          />
        </div>

        <button
          onClick={handleDeploy}
          disabled={!selectedFile || isDeployingContract}
          className="w-full bg-gradient-to-r from-inktix-blue-600 to-inktix-blue-700 hover:from-inktix-blue-700 hover:to-inktix-blue-800 disabled:from-gray-400 disabled:to-gray-500 text-white font-semibold py-3 px-6 rounded-lg transition-all duration-200 transform hover:scale-105 disabled:transform-none disabled:cursor-not-allowed shadow-lg"
        >
          {isDeployingContract ? (
            <div className="flex items-center justify-center">
              <div className="animate-spin rounded-full h-5 w-5 border-b-2 border-white mr-2"></div>
              Deploying...
            </div>
          ) : (
            <>
              <Upload className="w-5 h-5 inline mr-2" />
              Deploy Contract
            </>
          )}
        </button>

        {deploymentResult && (
          <div className="mt-4 p-3 bg-gray-100 rounded-lg">
            <pre className="text-sm text-gray-800 whitespace-pre-wrap">
              {deploymentResult}
            </pre>
          </div>
        )}
      </div>
    </div>
  );
};

export default ContractDeployment;
