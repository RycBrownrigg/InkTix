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
  const [selectedMethod, setSelectedMethod] = useState("get_stats");
  const [methodArgs, setMethodArgs] = useState("[]");
  const [contractType, setContractType] = useState<
    "sports" | "concert" | "unknown"
  >("unknown");

  const handleFileSelect = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (file && file.name.endsWith(".wasm")) {
      setSelectedFile(file);

      // Detect contract type based on filename
      const filename = file.name.toLowerCase();
      if (filename.includes("sports") || filename.includes("sports_broker")) {
        setContractType("sports");
        setSelectedMethod("get_total_teams");
      } else if (
        filename.includes("concert") ||
        filename.includes("concert_broker")
      ) {
        setContractType("concert");
        setSelectedMethod("get_stats");
      } else {
        setContractType("unknown");
        setSelectedMethod("get_stats");
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

  // Get contract methods based on type
  const getContractMethods = () => {
    if (contractType === "sports") {
      return {
        getters: [
          { value: "get_total_teams", label: "get_total_teams" },
          { value: "get_total_venues", label: "get_total_venues" },
          { value: "get_total_events", label: "get_total_events" },
          { value: "get_total_tickets", label: "get_total_tickets" },
          { value: "get_owner", label: "get_owner" },
        ],
        creators: [
          { value: "register_team", label: "register_team(name, sport, city)" },
          {
            value: "register_venue",
            label: "register_venue(name, capacity, location)",
          },
          {
            value: "create_event",
            label: "create_event(homeTeamId, awayTeamId, venueId, date, price)",
          },
          {
            value: "purchase_ticket",
            label: "purchase_ticket(eventId, seatNumber, section, row)",
          },
        ],
        queries: [
          { value: "get_team_by_id", label: "get_team_by_id(teamId)" },
          { value: "get_venue_by_id", label: "get_venue_by_id(venueId)" },
          { value: "get_event_by_id", label: "get_event_by_id(eventId)" },
          {
            value: "get_tickets_by_event",
            label: "get_tickets_by_event(eventId)",
          },
        ],
      };
    } else if (contractType === "concert") {
      return {
        getters: [
          { value: "get_stats", label: "get_stats" },
          { value: "get_owner", label: "get_owner" },
        ],
        creators: [
          { value: "register_artist", label: "register_artist(name)" },
          {
            value: "register_venue",
            label: "register_venue(name, capacity, address)",
          },
          {
            value: "create_concert_event",
            label:
              "create_concert_event(name, artistId, venueId, date, capacity, price)",
          },
          {
            value: "purchase_ticket",
            label: "purchase_ticket(eventId, seatNumber)",
          },
        ],
        queries: [
          { value: "get_artist", label: "get_artist(artistId)" },
          { value: "get_venue", label: "get_venue(venueId)" },
          { value: "get_event", label: "get_event(eventId)" },
          { value: "get_ticket", label: "get_ticket(ticketId)" },
          { value: "get_user_tickets", label: "get_user_tickets(userId)" },
        ],
      };
    } else {
      // Default/unknown contract - show both
      return {
        getters: [
          { value: "get_stats", label: "get_stats" },
          { value: "get_owner", label: "get_owner" },
        ],
        creators: [
          { value: "register_artist", label: "register_artist(name)" },
          {
            value: "register_venue",
            label: "register_venue(name, capacity, address)",
          },
          { value: "create_event", label: "create_event(...)" },
          { value: "purchase_ticket", label: "purchase_ticket(...)" },
        ],
        queries: [
          { value: "get_artist", label: "get_artist(id)" },
          { value: "get_venue", label: "get_venue(id)" },
          { value: "get_event", label: "get_event(id)" },
          { value: "get_ticket", label: "get_ticket(id)" },
        ],
      };
    }
  };

  // Auto-fill arguments based on selected method
  const handleMethodChange = (method: string) => {
    setSelectedMethod(method);

    // Auto-fill example arguments based on contract type
    if (contractType === "sports") {
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
        case "get_venue_by_id":
        case "get_event_by_id":
        case "get_tickets_by_event":
          setMethodArgs("[1]");
          break;
        default:
          setMethodArgs("[]");
      }
    } else if (contractType === "concert") {
      switch (method) {
        case "register_artist":
          setMethodArgs('["Taylor Swift"]');
          break;
        case "register_venue":
          setMethodArgs('["Madison Square Garden", 20000, "New York, NY"]');
          break;
        case "create_concert_event":
          setMethodArgs(
            '["Taylor Swift Concert", 1, 1, 1700000000, 20000, "1500000000000000000"]'
          );
          break;
        case "purchase_ticket":
          setMethodArgs("[1, 101]");
          break;
        case "get_artist":
        case "get_venue":
        case "get_event":
        case "get_ticket":
          setMethodArgs("[1]");
          break;
        case "get_user_tickets":
          setMethodArgs('["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"]');
          break;
        default:
          setMethodArgs("[]");
      }
    } else {
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
                  {/* Dynamic methods based on contract type */}
                  <optgroup label="Get Information">
                    {getContractMethods().getters.map((method) => (
                      <option key={method.value} value={method.value}>
                        {method.label}
                      </option>
                    ))}
                  </optgroup>

                  <optgroup label="Registration & Creation">
                    {getContractMethods().creators.map((method) => (
                      <option key={method.value} value={method.value}>
                        {method.label}
                      </option>
                    ))}
                  </optgroup>

                  <optgroup label="Queries with Parameters">
                    {getContractMethods().queries.map((method) => (
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
        )}

        {/* Contract Registry */}
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
                {getContractMethods()
                  .getters.slice(0, 2)
                  .map((method) => (
                    <div key={method.value} className="flex items-center">
                      <span className="w-2 h-2 bg-blue-500 rounded-full mr-2"></span>
                      <span>{method.value}</span>
                    </div>
                  ))}
                {getContractMethods()
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

        {/* Cross-Chain Manager */}
        <div className="border border-gray-200 rounded-lg p-4">
          <h3 className="text-lg font-semibold text-gray-800 mb-4 flex items-center">
            <Database className="w-5 h-5 mr-2 text-inktix-green-600" />
            Cross-Chain Manager
          </h3>
          <div className="space-y-4">
            <div className="bg-gray-50 rounded-lg p-4">
              <h4 className="font-medium text-gray-800 mb-2">Network Status</h4>
              <div className="space-y-2">
                <div className="flex items-center justify-between">
                  <span className="text-sm text-gray-600">
                    Current Network:
                  </span>
                  <span className="text-sm font-medium text-green-600">
                    Westend AssetHub
                  </span>
                </div>
                <div className="flex items-center justify-between">
                  <span className="text-sm text-gray-600">Connection:</span>
                  <span className="text-sm font-medium text-green-600">
                    Connected
                  </span>
                </div>
                <div className="flex items-center justify-between">
                  <span className="text-sm text-gray-600">
                    Contracts Pallet:
                  </span>
                  <span className="text-sm font-medium text-green-600">
                    Available
                  </span>
                </div>
              </div>
            </div>

            <div className="bg-yellow-50 rounded-lg p-4">
              <h4 className="font-medium text-yellow-800 mb-2">
                Cross-Chain Features
              </h4>
              <div className="text-sm text-yellow-700 space-y-1">
                <p>• XCM message passing between parachains</p>
                <p>• Cross-chain asset transfers</p>
                <p>• Multi-chain contract deployment</p>
                <p>• Inter-parachain communication</p>
              </div>
            </div>

            <div className="bg-blue-50 rounded-lg p-4">
              <h4 className="font-medium text-blue-800 mb-2">
                Supported Networks
              </h4>
              <div className="grid grid-cols-2 gap-2 text-sm">
                <div className="flex items-center">
                  <span className="w-2 h-2 bg-blue-500 rounded-full mr-2"></span>
                  <span>Westend AssetHub</span>
                </div>
                <div className="flex items-center">
                  <span className="w-2 h-2 bg-gray-400 rounded-full mr-2"></span>
                  <span>Westend (Coming Soon)</span>
                </div>
                <div className="flex items-center">
                  <span className="w-2 h-2 bg-gray-400 rounded-full mr-2"></span>
                  <span>Kusama (Coming Soon)</span>
                </div>
                <div className="flex items-center">
                  <span className="w-2 h-2 bg-gray-400 rounded-full mr-2"></span>
                  <span>Polkadot (Coming Soon)</span>
                </div>
              </div>
            </div>
          </div>
        </div>

        {/* Contract Info */}
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
      </div>
    </div>
  );
};

export default SmartContractManager;
