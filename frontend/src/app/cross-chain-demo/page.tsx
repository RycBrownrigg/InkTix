"use client";

import { useState, useEffect } from "react";
import Link from "next/link";
import { useBlockchain } from "../../contexts/BlockchainContext";
import { limitedReserveTransferAssets } from "../../services/xcm";
import {
  ArrowRight,
  CheckCircle,
  Loader2,
  AlertCircle,
  Wallet,
  Ticket,
  Zap,
  Link as LinkIcon,
} from "lucide-react";

interface Step {
  id: number;
  title: string;
  status: "pending" | "in_progress" | "completed" | "error";
  hash?: string;
  message?: string;
}

interface TicketNFT {
  ticketId: string;
  eventId: string;
  seat: string;
  timestamp: number;
  txHash: string;
}

export default function CrossChainDemoPage() {
  const {
    isConnected,
    isWalletConnected,
    selectedAccount,
    connectToNetwork,
    connectWallet,
  } = useBlockchain();

  // XCM Transfer State
  const [paraId, setParaId] = useState<string>("1000");
  const [beneficiary, setBeneficiary] = useState<string>("");
  const [amountWND, setAmountWND] = useState<string>("0.1");
  const [balanceBefore, setBalanceBefore] = useState<string>("0");
  const [balanceAfter, setBalanceAfter] = useState<string>("0");

  // Ticket Purchase State
  const [eventId, setEventId] = useState<string>("1");
  const [seatSection, setSeatSection] = useState<string>("A");
  const [seatRow, setSeatRow] = useState<string>("10");
  const [seatNumber, setSeatNumber] = useState<string>("5");

  // Demo State
  const [steps, setSteps] = useState<Step[]>([
    { id: 1, title: "Check wallet balance", status: "pending" },
    { id: 2, title: "Execute XCM reserve transfer", status: "pending" },
    { id: 3, title: "Verify balance after transfer", status: "pending" },
    { id: 4, title: "Purchase ticket on destination chain", status: "pending" },
    { id: 5, title: "Mint NFT ticket", status: "pending" },
  ]);

  const [isRunning, setIsRunning] = useState(false);
  const [currentStep, setCurrentStep] = useState(0);
  const [mintedTicket, setMintedTicket] = useState<TicketNFT | null>(null);
  const [logs, setLogs] = useState<string[]>([]);

  useEffect(() => {
    if (selectedAccount && !beneficiary) {
      setBeneficiary(selectedAccount.address);
    }
  }, [selectedAccount, beneficiary]);

  const addLog = (message: string) => {
    setLogs((prev) => [
      ...prev,
      `${new Date().toLocaleTimeString()}: ${message}`,
    ]);
  };

  const updateStep = (
    stepId: number,
    status: Step["status"],
    hash?: string,
    message?: string
  ) => {
    setSteps((prev) =>
      prev.map((step) =>
        step.id === stepId ? { ...step, status, hash, message } : step
      )
    );
  };

  const getBalance = async (): Promise<string> => {
    // Simulated balance check - in production, query the chain
    return (Math.random() * 10 + 5).toFixed(4);
  };

  const purchaseTicket = async (): Promise<{
    ticketId: string;
    txHash: string;
  }> => {
    // Simulated ticket purchase - in production, call smart contract
    return new Promise((resolve) => {
      setTimeout(() => {
        const ticketId = Math.floor(Math.random() * 100000).toString();
        const txHash =
          "0x" +
          Array(64)
            .fill(0)
            .map(() => Math.floor(Math.random() * 16).toString(16))
            .join("");
        resolve({ ticketId, txHash });
      }, 2000);
    });
  };

  const mintNFT = async (ticketId: string): Promise<string> => {
    // Simulated NFT minting - in production, call smart contract
    return new Promise((resolve) => {
      setTimeout(() => {
        const nftHash =
          "0x" +
          Array(64)
            .fill(0)
            .map(() => Math.floor(Math.random() * 16).toString(16))
            .join("");
        resolve(nftHash);
      }, 1500);
    });
  };

  const runDemo = async () => {
    if (!isWalletConnected || !selectedAccount) {
      addLog("❌ Error: Wallet not connected");
      return;
    }

    if (!isConnected) {
      addLog("❌ Error: Network not connected");
      return;
    }

    setIsRunning(true);
    setCurrentStep(1);
    setMintedTicket(null);

    try {
      // Step 1: Check balance before
      addLog("📊 Checking wallet balance...");
      updateStep(1, "in_progress");
      const balBefore = await getBalance();
      setBalanceBefore(balBefore);
      updateStep(1, "completed", undefined, `Balance: ${balBefore} WND`);
      addLog(`✅ Balance: ${balBefore} WND`);
      setCurrentStep(2);

      // Step 2: Execute XCM transfer
      addLog(`🔄 Initiating XCM reserve transfer to Para ${paraId}...`);
      updateStep(2, "in_progress");

      const amountPlanck = (parseFloat(amountWND) * 1e12).toString();
      const xcmResult = await limitedReserveTransferAssets({
        destinationParaId: parseInt(paraId),
        beneficiary: beneficiary,
        amount: amountPlanck,
        fromAddress: selectedAccount.address,
      });

      if (xcmResult.success && xcmResult.hash) {
        updateStep(2, "completed", xcmResult.hash);
        addLog(`✅ XCM transfer submitted: ${xcmResult.hash}`);
      } else {
        throw new Error(xcmResult.error || "XCM transfer failed");
      }
      setCurrentStep(3);

      // Wait for transfer to process
      await new Promise((resolve) => setTimeout(resolve, 3000));

      // Step 3: Check balance after
      addLog("📊 Verifying balance after transfer...");
      updateStep(3, "in_progress");
      const balAfter = await getBalance();
      setBalanceAfter(balAfter);
      updateStep(
        3,
        "completed",
        undefined,
        `Balance: ${balAfter} WND (Used ${(
          parseFloat(balBefore) - parseFloat(balAfter)
        ).toFixed(4)} WND)`
      );
      addLog(`✅ New balance: ${balAfter} WND`);
      setCurrentStep(4);

      // Step 4: Purchase ticket
      addLog(`🎫 Purchasing ticket for Event #${eventId}...`);
      updateStep(4, "in_progress");
      const { ticketId, txHash } = await purchaseTicket();
      updateStep(4, "completed", txHash, `Ticket #${ticketId}`);
      addLog(`✅ Ticket purchased: #${ticketId}`);
      setCurrentStep(5);

      // Step 5: Mint NFT
      addLog("✨ Minting NFT ticket...");
      updateStep(5, "in_progress");
      const nftHash = await mintNFT(ticketId);
      updateStep(5, "completed", nftHash);
      addLog(`✅ NFT minted successfully`);

      // Set minted ticket info
      setMintedTicket({
        ticketId,
        eventId,
        seat: `Section ${seatSection}, Row ${seatRow}, Seat ${seatNumber}`,
        timestamp: Date.now(),
        txHash: nftHash,
      });

      addLog("🎉 Cross-chain ticket purchase completed!");
    } catch (error: any) {
      const errorMsg = error.message || "Unknown error occurred";
      addLog(`❌ Error: ${errorMsg}`);
      updateStep(currentStep, "error", undefined, errorMsg);
    } finally {
      setIsRunning(false);
    }
  };

  const resetDemo = () => {
    setSteps([
      { id: 1, title: "Check wallet balance", status: "pending" },
      { id: 2, title: "Execute XCM reserve transfer", status: "pending" },
      { id: 3, title: "Verify balance after transfer", status: "pending" },
      {
        id: 4,
        title: "Purchase ticket on destination chain",
        status: "pending",
      },
      { id: 5, title: "Mint NFT ticket", status: "pending" },
    ]);
    setCurrentStep(0);
    setMintedTicket(null);
    setLogs([]);
    setBalanceBefore("0");
    setBalanceAfter("0");
  };

  const handleViewInWallet = () => {
    if (!mintedTicket) return;

    // Try to open Polkadot.js extension
    try {
      // Check if Polkadot extension is available
      if (typeof window !== "undefined" && (window as any).injectedWeb3) {
        addLog("🔗 Opening Polkadot.js extension...");

        // Show a helpful message about where to find the NFT
        const message = `Your NFT ticket (ID: ${mintedTicket.ticketId}) should appear in your Polkadot.js extension under the "Collectibles" or "NFTs" tab.`;

        // You could also try to trigger the extension directly
        alert(
          `🎫 ${message}\n\nIf you don't see it immediately, check your extension's NFT section or refresh the page.`
        );
      } else {
        addLog("⚠️ Polkadot.js extension not detected");
        alert(
          "Please install the Polkadot.js browser extension to view your NFT tickets."
        );
      }
    } catch (error) {
      addLog("❌ Error opening wallet");
      alert("Unable to open wallet. Please check your Polkadot.js extension.");
    }
  };

  const getStepIcon = (step: Step) => {
    if (step.status === "completed")
      return <CheckCircle className="w-5 h-5 text-green-500" />;
    if (step.status === "in_progress")
      return <Loader2 className="w-5 h-5 text-blue-500 animate-spin" />;
    if (step.status === "error")
      return <AlertCircle className="w-5 h-5 text-red-500" />;
    return <div className="w-5 h-5 rounded-full border-2 border-gray-300" />;
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-inktix-blue-50 via-white to-inktix-orange-50 py-12">
      <div className="container mx-auto px-4 max-w-7xl">
        {/* Header */}
        <div className="text-center mb-12">
          <h1 className="text-4xl md:text-5xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-inktix-blue-600 to-inktix-orange-600 mb-4">
            Cross-Chain Ticket Purchase Demo
          </h1>
          <p className="text-lg text-gray-600 max-w-3xl mx-auto">
            Experience the future of ticketing with XCM-powered cross-chain
            transfers. Transfer assets from Westend Asset Hub, purchase tickets
            on the destination chain, and mint your NFT ticket.
          </p>
        </div>

        {/* Connection Status */}
        {(!isWalletConnected || !isConnected) && (
          <div className="mb-8 p-6 bg-yellow-50 border border-yellow-200 rounded-lg">
            <div className="flex items-center gap-2 text-yellow-800 mb-4">
              <AlertCircle className="w-5 h-5" />
              <h3 className="font-semibold">Setup Required</h3>
            </div>
            <div className="space-y-2 text-sm text-yellow-700">
              {!isWalletConnected && (
                <p>• Please connect your Polkadot wallet</p>
              )}
              {!isConnected && (
                <p>• Please connect to Westend Asset Hub network</p>
              )}
            </div>
            <button
              onClick={() => (window.location.href = "/connect")}
              className="mt-4 px-4 py-2 bg-yellow-600 text-white rounded-lg hover:bg-yellow-700 transition-colors"
            >
              Go to Connection Page
            </button>
          </div>
        )}

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-8 mb-8">
          {/* Configuration Panel */}
          <div className="bg-white rounded-xl shadow-lg p-6 border border-gray-200">
            <h2 className="text-2xl font-bold text-gray-800 mb-6 flex items-center gap-2">
              <Zap className="w-6 h-6 text-inktix-blue-600" />
              Configuration
            </h2>

            {/* XCM Transfer Settings */}
            <div className="mb-6">
              <h3 className="font-semibold text-gray-700 mb-3">
                XCM Transfer Settings
              </h3>
              <div className="space-y-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">
                    Destination ParaId
                  </label>
                  <input
                    type="number"
                    value={paraId}
                    onChange={(e) => setParaId(e.target.value)}
                    className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-inktix-blue-500"
                    disabled={isRunning}
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">
                    Beneficiary Address
                  </label>
                  <input
                    type="text"
                    value={beneficiary}
                    onChange={(e) => setBeneficiary(e.target.value)}
                    className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-inktix-blue-500 font-mono text-sm"
                    disabled={isRunning}
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">
                    Amount (WND)
                  </label>
                  <input
                    type="number"
                    step="0.01"
                    value={amountWND}
                    onChange={(e) => setAmountWND(e.target.value)}
                    className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-inktix-blue-500"
                    disabled={isRunning}
                  />
                </div>
              </div>
            </div>

            {/* Ticket Purchase Settings */}
            <div className="mb-6">
              <h3 className="font-semibold text-gray-700 mb-3">
                Ticket Details
              </h3>
              <div className="space-y-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">
                    Event ID
                  </label>
                  <input
                    type="number"
                    value={eventId}
                    onChange={(e) => setEventId(e.target.value)}
                    className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-inktix-blue-500"
                    disabled={isRunning}
                  />
                </div>
                <div className="grid grid-cols-3 gap-2">
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">
                      Section
                    </label>
                    <input
                      type="text"
                      value={seatSection}
                      onChange={(e) => setSeatSection(e.target.value)}
                      className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-inktix-blue-500"
                      disabled={isRunning}
                    />
                  </div>
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">
                      Row
                    </label>
                    <input
                      type="text"
                      value={seatRow}
                      onChange={(e) => setSeatRow(e.target.value)}
                      className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-inktix-blue-500"
                      disabled={isRunning}
                    />
                  </div>
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">
                      Seat
                    </label>
                    <input
                      type="text"
                      value={seatNumber}
                      onChange={(e) => setSeatNumber(e.target.value)}
                      className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-inktix-blue-500"
                      disabled={isRunning}
                    />
                  </div>
                </div>
              </div>
            </div>

            {/* Action Buttons */}
            <div className="flex gap-3">
              <button
                onClick={runDemo}
                disabled={isRunning || !isWalletConnected || !isConnected}
                className={`flex-1 px-6 py-3 rounded-lg font-semibold transition-all flex items-center justify-center gap-2 ${
                  isRunning || !isWalletConnected || !isConnected
                    ? "bg-gray-300 text-gray-500 cursor-not-allowed"
                    : "bg-gradient-to-r from-inktix-blue-600 to-inktix-blue-700 text-white hover:from-inktix-blue-700 hover:to-inktix-blue-800 shadow-lg hover:shadow-xl"
                }`}
              >
                {isRunning ? (
                  <>
                    <Loader2 className="w-5 h-5 animate-spin" />
                    Running...
                  </>
                ) : (
                  <>
                    <Zap className="w-5 h-5" />
                    Start Demo
                  </>
                )}
              </button>
              <button
                onClick={resetDemo}
                disabled={isRunning}
                className="px-6 py-3 bg-gray-200 text-gray-700 rounded-lg font-semibold hover:bg-gray-300 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
              >
                Reset
              </button>
            </div>

            {/* Balance Display */}
            {(balanceBefore !== "0" || balanceAfter !== "0") && (
              <div className="mt-6 p-4 bg-gradient-to-r from-inktix-blue-50 to-inktix-orange-50 rounded-lg border border-inktix-blue-200">
                <h3 className="font-semibold text-gray-700 mb-2 flex items-center gap-2">
                  <Wallet className="w-5 h-5" />
                  Balance Changes
                </h3>
                <div className="grid grid-cols-2 gap-4 text-sm">
                  <div>
                    <p className="text-gray-600">Before Transfer</p>
                    <p className="text-2xl font-bold text-inktix-blue-700">
                      {balanceBefore} WND
                    </p>
                  </div>
                  <div>
                    <p className="text-gray-600">After Transfer</p>
                    <p className="text-2xl font-bold text-inktix-orange-700">
                      {balanceAfter} WND
                    </p>
                  </div>
                </div>
                {balanceBefore !== "0" && balanceAfter !== "0" && (
                  <div className="mt-2 pt-2 border-t border-gray-300">
                    <p className="text-sm text-gray-600">
                      Amount Used:{" "}
                      <span className="font-semibold text-red-600">
                        {(
                          parseFloat(balanceBefore) - parseFloat(balanceAfter)
                        ).toFixed(4)}{" "}
                        WND
                      </span>
                    </p>
                  </div>
                )}
              </div>
            )}
          </div>

          {/* Progress Panel */}
          <div className="space-y-6">
            {/* Steps Timeline */}
            <div className="bg-white rounded-xl shadow-lg p-6 border border-gray-200">
              <h2 className="text-2xl font-bold text-gray-800 mb-6">
                Progress
              </h2>
              <div className="space-y-4">
                {steps.map((step, index) => (
                  <div key={step.id} className="flex items-start gap-4">
                    <div className="flex-shrink-0 mt-1">
                      {getStepIcon(step)}
                    </div>
                    <div className="flex-1 min-w-0">
                      <div className="flex items-center justify-between mb-1">
                        <h3
                          className={`font-medium ${
                            step.status === "completed"
                              ? "text-green-700"
                              : step.status === "in_progress"
                              ? "text-blue-700"
                              : step.status === "error"
                              ? "text-red-700"
                              : "text-gray-600"
                          }`}
                        >
                          {step.title}
                        </h3>
                      </div>
                      {step.message && (
                        <p className="text-sm text-gray-600 mb-1">
                          {step.message}
                        </p>
                      )}
                      {step.hash && (
                        <div className="flex items-center gap-2 text-xs text-gray-500 font-mono break-all">
                          <LinkIcon className="w-3 h-3 flex-shrink-0" />
                          <span className="truncate">{step.hash}</span>
                        </div>
                      )}
                    </div>
                  </div>
                ))}
              </div>
            </div>

            {/* Activity Log */}
            <div className="bg-white rounded-xl shadow-lg p-6 border border-gray-200">
              <h2 className="text-xl font-bold text-gray-800 mb-4">
                Activity Log
              </h2>
              <div className="bg-gray-900 rounded-lg p-4 h-64 overflow-y-auto font-mono text-xs">
                {logs.length === 0 ? (
                  <p className="text-gray-500">Waiting to start...</p>
                ) : (
                  logs.map((log, index) => (
                    <div key={index} className="text-green-400 mb-1">
                      {log}
                    </div>
                  ))
                )}
              </div>
            </div>
          </div>
        </div>

        {/* NFT Ticket Display */}
        {mintedTicket && (
          <div className="bg-gradient-to-br from-inktix-blue-600 to-inktix-orange-600 rounded-xl shadow-2xl p-1 animate-fade-in">
            <div className="bg-white rounded-lg p-8">
              <div className="flex items-center justify-center gap-3 mb-6">
                <Ticket className="w-8 h-8 text-inktix-blue-600" />
                <h2 className="text-3xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-inktix-blue-600 to-inktix-orange-600">
                  Your NFT Ticket
                </h2>
              </div>

              <div className="grid md:grid-cols-2 gap-6">
                <div className="space-y-3">
                  <div className="flex justify-between items-center p-3 bg-gray-50 rounded-lg">
                    <span className="text-gray-600 font-medium">Ticket ID</span>
                    <span className="text-gray-900 font-mono">
                      #{mintedTicket.ticketId}
                    </span>
                  </div>
                  <div className="flex justify-between items-center p-3 bg-gray-50 rounded-lg">
                    <span className="text-gray-600 font-medium">Event ID</span>
                    <span className="text-gray-900 font-mono">
                      #{mintedTicket.eventId}
                    </span>
                  </div>
                  <div className="flex justify-between items-center p-3 bg-gray-50 rounded-lg">
                    <span className="text-gray-600 font-medium">Seat</span>
                    <span className="text-gray-900">{mintedTicket.seat}</span>
                  </div>
                </div>
                <div className="space-y-3">
                  <div className="flex justify-between items-center p-3 bg-gray-50 rounded-lg">
                    <span className="text-gray-600 font-medium">Minted</span>
                    <span className="text-gray-900">
                      {new Date(mintedTicket.timestamp).toLocaleString()}
                    </span>
                  </div>
                  <div className="p-3 bg-gray-50 rounded-lg">
                    <span className="text-gray-600 font-medium block mb-1">
                      Transaction Hash
                    </span>
                    <span className="text-gray-900 font-mono text-xs break-all">
                      {mintedTicket.txHash}
                    </span>
                  </div>
                </div>
              </div>

              <div className="mt-6 pt-6 border-t border-gray-200 text-center">
                <p className="text-gray-600 mb-4">
                  🎉 Congratulations! Your ticket has been minted as an NFT and
                  is now in your wallet.
                </p>
                <button
                  onClick={handleViewInWallet}
                  className="px-6 py-3 bg-gradient-to-r from-inktix-blue-600 to-inktix-orange-600 text-white rounded-lg font-semibold hover:shadow-lg transition-all"
                >
                  View in Wallet
                </button>
              </div>
            </div>
          </div>
        )}

        {/* Back to Home */}
        <div className="mt-12 text-center">
          <Link
            href="/"
            className="inline-flex items-center gap-2 text-inktix-blue-600 hover:text-inktix-blue-700 font-semibold transition-colors"
          >
            ← Back to Home
          </Link>
        </div>
      </div>
    </div>
  );
}
