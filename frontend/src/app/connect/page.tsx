"use client";

import React from "react";
import dynamic from "next/dynamic";
import Link from "next/link";
import { Shield, Zap, Globe, Lock, ArrowLeft } from "lucide-react";

// Dynamically import the wallet component to prevent SSR issues
const BlockchainWallet = dynamic(
  () => import("../../components/BlockchainWallet"),
  {
    ssr: false,
    loading: () => (
      <div className="bg-white rounded-lg shadow-lg p-6 border border-gray-200 text-center">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-inktix-blue-600 mx-auto mb-4"></div>
        <p className="text-gray-600">Loading wallet...</p>
      </div>
    ),
  }
);

const ConnectPage: React.FC = () => {
  return (
    <div className="min-h-screen bg-gradient-to-br from-inktix-blue-50 via-white to-inktix-orange-50">
      {/* Header */}
      <div className="bg-white shadow-sm border-b border-gray-200">
        <div className="container mx-auto px-4 py-6">
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

      <div className="container mx-auto px-4 py-12">
        {/* Page Header */}
        <div className="text-center mb-12">
          <h1 className="text-4xl md:text-5xl font-bold text-gray-900 mb-6">
            Connect to{" "}
            <span className="text-transparent bg-clip-text bg-gradient-to-r from-inktix-blue-600 to-inktix-orange-600">
              InkTix
            </span>
          </h1>
          <p className="text-xl text-gray-600 max-w-3xl mx-auto">
            Connect your Polkadot.js wallet to access the future of sports
            ticketing. Experience secure, transparent, and decentralized
            ticketing on the blockchain.
          </p>
        </div>

        <div className="grid lg:grid-cols-2 gap-8 max-w-6xl mx-auto">
          {/* Wallet Connection Section */}
          <div>
            <h2 className="text-2xl font-semibold text-gray-800 mb-6 text-center lg:text-left">
              Wallet Connection
            </h2>
            <BlockchainWallet />
          </div>

          {/* Benefits Section */}
          <div className="space-y-6">
            <h2 className="text-2xl font-semibold text-gray-800 mb-6 text-center lg:text-left">
              Why Choose InkTix?
            </h2>

            <div className="space-y-6">
              <div className="bg-white rounded-lg p-6 shadow-lg border border-gray-200">
                <div className="flex items-start">
                  <div className="flex-shrink-0">
                    <Shield className="w-8 h-8 text-inktix-blue-600" />
                  </div>
                  <div className="ml-4">
                    <h3 className="text-lg font-semibold text-gray-800 mb-2">
                      Secure & Transparent
                    </h3>
                    <p className="text-gray-600">
                      All transactions are recorded on the Polkadot blockchain,
                      ensuring complete transparency and immutability. No more
                      hidden fees or fraudulent tickets.
                    </p>
                  </div>
                </div>
              </div>

              <div className="bg-white rounded-lg p-6 shadow-lg border border-gray-200">
                <div className="flex items-start">
                  <div className="flex-shrink-0">
                    <Zap className="w-8 h-8 text-inktix-orange-500" />
                  </div>
                  <div className="ml-4">
                    <h3 className="text-lg font-semibold text-gray-800 mb-2">
                      Lightning Fast
                    </h3>
                    <p className="text-gray-600">
                      Built on Polkadot&apos;s high-performance infrastructure,
                      InkTix provides instant ticket transfers and real-time
                      availability updates.
                    </p>
                  </div>
                </div>
              </div>

              <div className="bg-white rounded-lg p-6 shadow-lg border border-gray-200">
                <div className="flex items-start">
                  <div className="flex-shrink-0">
                    <Globe className="w-8 h-8 text-inktix-green-600" />
                  </div>
                  <div className="ml-4">
                    <h3 className="text-lg font-semibold text-gray-800 mb-2">
                      Cross-Chain Compatible
                    </h3>
                    <p className="text-gray-600">
                      Access tickets across multiple blockchain networks through
                      Polkadot&apos;s cross-chain messaging system. Buy tickets
                      on one chain, use them on another.
                    </p>
                  </div>
                </div>
              </div>

              <div className="bg-white rounded-lg p-6 shadow-lg border border-gray-200">
                <div className="flex items-start">
                  <div className="flex-shrink-0">
                    <Lock className="w-8 h-8 text-inktix-purple-600" />
                  </div>
                  <div className="ml-4">
                    <h3 className="text-lg font-semibold text-gray-800 mb-2">
                      Anti-Scalping Protection
                    </h3>
                    <p className="text-gray-600">
                      Advanced algorithms prevent ticket scalping and ensure
                      fair distribution. Real fans get real tickets at real
                      prices.
                    </p>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>

        {/* Getting Started Section */}
        <div className="mt-16 bg-white rounded-xl shadow-xl border border-gray-200 p-8">
          <h2 className="text-3xl font-bold text-center text-gray-800 mb-8">
            Getting Started
          </h2>

          <div className="grid md:grid-cols-3 gap-8">
            <div className="text-center">
              <div className="bg-inktix-blue-100 w-16 h-16 rounded-full flex items-center justify-center mx-auto mb-4">
                <span className="text-2xl font-bold text-inktix-blue-600">
                  1
                </span>
              </div>
              <h3 className="text-xl font-semibold text-gray-800 mb-3">
                Install Extension
              </h3>
              <p className="text-gray-600">
                Install the Polkadot.js browser extension from the official
                website and create or import your wallet.
              </p>
            </div>

            <div className="text-center">
              <div className="bg-inktix-orange-100 w-16 h-16 rounded-full flex items-center justify-center mx-auto mb-4">
                <span className="text-2xl font-bold text-inktix-orange-600">
                  2
                </span>
              </div>
              <h3 className="text-xl font-semibold text-gray-800 mb-3">
                Connect Wallet
              </h3>
              <p className="text-gray-600">
                Click the &quot;Connect Wallet&quot; button above and authorize
                InkTix to access your wallet accounts.
              </p>
            </div>

            <div className="text-center">
              <div className="bg-inktix-green-100 w-16 h-16 rounded-full flex items-center justify-center mx-auto mb-4">
                <span className="text-2xl font-bold text-inktix-green-600">
                  3
                </span>
              </div>
              <h3 className="text-xl font-semibold text-gray-800 mb-3">
                Start Trading
              </h3>
              <p className="text-gray-600">
                Browse events, purchase tickets, and enjoy the seamless
                experience of blockchain-powered sports ticketing.
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default ConnectPage;
