"use client";

import dynamic from "next/dynamic";
import { ArrowLeft, Database, Code, Zap } from "lucide-react";
import Link from "next/link";

// Dynamically import the SmartContractManager to avoid SSR issues
const SmartContractManager = dynamic(
  () => import("../../components/SmartContractManager"),
  { ssr: false }
);

export default function SmartContractsPage() {
  return (
    <div className="min-h-screen bg-gradient-to-br from-gray-50 to-blue-50">
      {/* Header */}
      <div className="bg-white shadow-sm border-b border-gray-200">
        <div className="container-max py-6">
          <div className="flex items-center gap-4">
            <Link
              href="/"
              className="flex items-center gap-2 text-gray-600 hover:text-gray-900 transition-colors"
            >
              <ArrowLeft className="w-5 h-5" />
              <span>Back to Home</span>
            </Link>
          </div>
        </div>
      </div>

      {/* Page Content */}
      <div className="container-max py-12">
        {/* Page Header */}
        <div className="text-center mb-12">
          <div className="inline-flex items-center justify-center w-20 h-20 bg-gradient-to-br from-inktix-purple-500 to-inktix-purple-600 rounded-full mb-6">
            <Database className="w-10 h-10 text-white" />
          </div>
          <h1 className="text-4xl md:text-5xl font-bold text-gray-900 mb-6">
            Smart Contract Integration
          </h1>
          <p className="text-xl text-gray-600 max-w-3xl mx-auto leading-relaxed">
            Deploy and interact with Ink! smart contracts on the Polkadot
            blockchain. Manage your sports ticketing platform with decentralized
            automation.
          </p>
        </div>

        {/* Features Overview */}
        <div className="grid md:grid-cols-3 gap-8 mb-12">
          <div className="bg-white rounded-xl p-6 shadow-lg border border-gray-200">
            <div className="w-12 h-12 bg-blue-100 rounded-lg flex items-center justify-center mb-4">
              <Code className="w-6 h-6 text-blue-600" />
            </div>
            <h3 className="text-lg font-semibold text-gray-900 mb-2">
              Ink! Smart Contracts
            </h3>
            <p className="text-gray-600">
              Built with Rust and Ink! framework for maximum security and
              performance on Polkadot.
            </p>
          </div>

          <div className="bg-white rounded-xl p-6 shadow-lg border border-gray-200">
            <div className="w-12 h-12 bg-green-100 rounded-lg flex items-center justify-center mb-4">
              <Zap className="w-6 h-6 text-green-600" />
            </div>
            <h3 className="text-lg font-semibold text-gray-900 mb-2">
              Instant Deployment
            </h3>
            <p className="text-gray-600">
              Deploy contracts with a single click and manage them through an
              intuitive interface.
            </p>
          </div>

          <div className="bg-white rounded-xl p-6 shadow-lg border border-gray-200">
            <div className="w-12 h-12 bg-purple-100 rounded-lg flex items-center justify-center mb-4">
              <Database className="w-6 h-6 text-purple-600" />
            </div>
            <h3 className="text-lg font-semibold text-gray-900 mb-2">
              Real-time Interaction
            </h3>
            <p className="text-gray-600">
              Call contract methods, view state, and monitor transactions in
              real-time.
            </p>
          </div>
        </div>

        {/* Smart Contract Manager */}
        <div className="max-w-4xl mx-auto">
          <SmartContractManager />
        </div>

        {/* Additional Information */}
        <div className="mt-16 bg-white rounded-xl p-8 shadow-lg border border-gray-200">
          <h2 className="text-2xl font-bold text-gray-900 mb-6 text-center">
            Getting Started with Smart Contracts
          </h2>
          <div className="grid md:grid-cols-2 gap-8">
            <div>
              <h3 className="text-lg font-semibold text-gray-900 mb-4">
                Prerequisites
              </h3>
              <ul className="space-y-2 text-gray-600">
                <li className="flex items-start gap-2">
                  <span className="w-2 h-2 bg-blue-500 rounded-full mt-2 flex-shrink-0"></span>
                  Polkadot.js extension installed and configured
                </li>
                <li className="flex items-start gap-2">
                  <span className="w-2 h-2 bg-blue-500 rounded-full mt-2 flex-shrink-0"></span>
                  Connected to AssetHub on Westend testnet (recommended)
                </li>
                <li className="flex items-start gap-2">
                  <span className="w-2 h-2 bg-blue-500 rounded-full mt-2 flex-shrink-0"></span>
                  Account with sufficient balance for deployment
                </li>
                <li className="flex items-start gap-2">
                  <span className="w-2 h-2 bg-blue-500 rounded-full mt-2 flex-shrink-0"></span>
                  Compiled .wasm contract file
                </li>
              </ul>
            </div>
            <div>
              <h3 className="text-lg font-semibold text-gray-900 mb-4">
                Deployment Steps
              </h3>
              <ol className="space-y-2 text-gray-600">
                <li className="flex items-start gap-2">
                  <span className="w-6 h-6 bg-blue-500 text-white rounded-full flex items-center justify-center text-sm font-semibold flex-shrink-0">
                    1
                  </span>
                  Connect your wallet and network
                </li>
                <li className="flex items-start gap-2">
                  <span className="w-6 h-6 bg-blue-500 text-white rounded-full flex items-center justify-center text-sm font-semibold flex-shrink-0">
                    2
                  </span>
                  Select your compiled .wasm contract file
                </li>
                <li className="flex items-start gap-2">
                  <span className="w-6 h-6 bg-blue-500 text-white rounded-full flex items-center justify-center text-sm font-semibold flex-shrink-0">
                    3
                  </span>
                  Set the endowment amount in WND
                </li>
                <li className="flex items-start gap-2">
                  <span className="w-6 h-6 bg-blue-500 text-white rounded-full flex items-center justify-center text-sm font-semibold flex-shrink-0">
                    4
                  </span>
                  Deploy and wait for confirmation
                </li>
              </ol>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
