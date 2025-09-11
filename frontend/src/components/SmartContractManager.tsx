"use client";

import React, { useState, useEffect } from "react";
import { useBlockchain } from "../contexts/BlockchainContext";
import {
  Upload,
  Play,
  Database,
  FileText,
  CheckCircle,
  AlertCircle,
} from "lucide-react";

const SmartContractManager: React.FC = () => {
  const {
    isConnected,
    isWalletConnected,
    contractAddress,
    isContractDeployed,
    isDeployingContract,
    deployContract,
    callContract,
  } = useBlockchain();

  const [selectedFile, setSelectedFile] = useState<File | null>(null);
  const [endowment, setEndowment] = useState("1.0");
  const [deploymentResult, setDeploymentResult] = useState<string>("");
  const [contractCallResult, setContractCallResult] = useState<string>("");
  const [selectedMethod, setSelectedMethod] = useState("get_total_teams");
  const [methodArgs, setMethodArgs] = useState("[]");

  const handleFileSelect = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (file && file.name.endsWith(".wasm")) {
      setSelectedFile(file);
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

  // Auto-fill arguments based on selected method
  const handleMethodChange = (method: string) => {
    setSelectedMethod(method);

    // Auto-fill example arguments
    switch (method) {
      case "register_team":
        setMethodArgs('["Lakers", "Basketball", "Los Angeles"]');
        break;
      case "register_venue":
        setMethodArgs('["Staples Center", 20000, "Los Angeles, CA"]');
        break;
      case "create_event":
        setMethodArgs('[1, 2, 1, "2024-02-15", "150 DOT"]');
        break;
      case "purchase_ticket":
        setMethodArgs('[1, 101, "A", "15"]');
        break;
      case "get_team_by_id":
        setMethodArgs("[1]");
        break;
      case "get_venue_by_id":
        setMethodArgs("[1]");
        break;
      case "get_event_by_id":
        setMethodArgs("[1]");
        break;
      case "get_tickets_by_event":
        setMethodArgs("[1]");
        break;
      default:
        setMethodArgs("[]");
    }
  };

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

        {/* Contract Deployment */}
        {!isContractDeployed && (
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
        )}

        {/* Contract Interaction */}
        {isContractDeployed && (
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
                  {/* Getter methods (no arguments) */}
                  <optgroup label="Get Information">
                    <option value="get_total_teams">get_total_teams</option>
                    <option value="get_total_venues">get_total_venues</option>
                    <option value="get_total_events">get_total_events</option>
                    <option value="get_total_tickets">get_total_tickets</option>
                    <option value="get_owner">get_owner</option>
                  </optgroup>

                  {/* Methods that take arguments */}
                  <optgroup label="Registration & Creation">
                    <option value="register_team">
                      register_team(name, sport, city)
                    </option>
                    <option value="register_venue">
                      register_venue(name, capacity, location)
                    </option>
                    <option value="create_event">
                      create_event(homeTeamId, awayTeamId, venueId, date, price)
                    </option>
                    <option value="purchase_ticket">
                      purchase_ticket(eventId, seatNumber, section, row)
                    </option>
                  </optgroup>

                  {/* Query methods with parameters */}
                  <optgroup label="Queries with Parameters">
                    <option value="get_team_by_id">
                      get_team_by_id(teamId)
                    </option>
                    <option value="get_venue_by_id">
                      get_venue_by_id(venueId)
                    </option>
                    <option value="get_event_by_id">
                      get_event_by_id(eventId)
                    </option>
                    <option value="get_tickets_by_event">
                      get_tickets_by_event(eventId)
                    </option>
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
        )}

        {/* Contract Info */}
        <div className="bg-blue-50 rounded-lg p-4">
          <h3 className="text-lg font-semibold text-blue-800 mb-3 flex items-center">
            <FileText className="w-5 h-5 mr-2" />
            Contract Information
          </h3>
          <div className="text-sm text-blue-700 space-y-2">
            <p>
              • <strong>Name:</strong> Sports Broker Contract
            </p>
            <p>
              • <strong>Type:</strong> Ink! Smart Contract
            </p>
            <p>
              • <strong>Features:</strong> Team management, venue registration,
              event creation, ticket purchasing
            </p>
            <p>
              • <strong>Network:</strong> Local Substrate Node
              (ws://127.0.0.1:9944)
            </p>
          </div>
        </div>
      </div>
    </div>
  );
};

export default SmartContractManager;
