import dynamic from "next/dynamic";

// Dynamically import WalletConnect to avoid SSR issues
const WalletConnect = dynamic(() => import("../../components/WalletConnect"), {
  loading: () => (
    <div className="text-center py-8">Loading wallet connection...</div>
  ),
});

export default function ConnectPage() {
  return (
    <div className="min-h-screen bg-gradient-to-br from-gray-50 to-gray-100">
      <div className="container-max py-16">
        <div className="text-center mb-12">
          <h1 className="heading-2 text-inktix-blue-600 mb-4">
            Connect Your Wallet
          </h1>
          <p className="body-large text-gray-600 max-w-2xl mx-auto">
            Connect your Polkadot wallet to access the InkTix platform.
            Experience seamless blockchain integration for sports ticketing and
            loyalty programs.
          </p>
        </div>

        <div className="max-w-md mx-auto">
          <WalletConnect />
        </div>

        <div className="mt-12 text-center">
          <h2 className="heading-3 text-gray-800 mb-4">Supported Wallets</h2>
          <div className="flex flex-wrap justify-center gap-4 text-sm text-gray-600">
            <span className="px-3 py-1 bg-white rounded-full shadow-sm">
              Polkadot.js
            </span>
            <span className="px-3 py-1 bg-white rounded-full shadow-sm">
              Talisman
            </span>
            <span className="px-3 py-1 bg-white rounded-full shadow-sm">
              SubWallet
            </span>
            <span className="px-3 py-1 bg-white rounded-full shadow-sm">
              Fearless Wallet
            </span>
          </div>
        </div>
      </div>
    </div>
  );
}
