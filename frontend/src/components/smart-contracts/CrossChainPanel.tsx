"use client";

import React from "react";
import { Database } from "lucide-react";

const CrossChainPanel: React.FC = () => {
  return (
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
  );
};

export default CrossChainPanel;
