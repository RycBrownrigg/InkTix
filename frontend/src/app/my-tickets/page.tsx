/**
 * NFT ticket portfolio page for viewing, minting, using, and verifying on-chain ticket NFTs.
 *
 * @module app/my-tickets/page
 */
"use client";

import Link from "next/link";
import { useState } from "react";
import { QRCodeSVG } from "qrcode.react";
import {
  ArrowLeft,
  Ticket,
  Shield,
  CheckCircle,
  XCircle,
  Sparkles,
  Search,
  QrCode,
  Copy,
  Check,
} from "lucide-react";
import type { TicketNft, TicketVerification } from "@/sdk/types";

// Mock NFT ticket data for demo display
const demoNftTickets: TicketNft[] = [
  {
    tokenId: 1,
    ticketId: 1,
    owner: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    eventId: 1,
    eventName: "Lakers vs Warriors",
    venueName: "Crypto.com Arena",
    eventDate: Date.now() + 86400000 * 7,
    section: "A",
    row: "15",
    seatNumber: 101,
    seatType: "Reserved",
    accessLevel: "Standard",
    mintedAt: Date.now() - 86400000 * 2,
    metadataUri: "",
    verificationHash:
      "0x7a3b9c1d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b",
    isUsed: false,
  },
  {
    tokenId: 2,
    ticketId: 2,
    owner: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    eventId: 1,
    eventName: "Lakers vs Warriors",
    venueName: "Crypto.com Arena",
    eventDate: Date.now() + 86400000 * 7,
    section: "A",
    row: "15",
    seatNumber: 102,
    seatType: "Reserved",
    accessLevel: "Standard",
    mintedAt: Date.now() - 86400000,
    metadataUri: "",
    verificationHash:
      "0x1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c",
    isUsed: true,
  },
];

// A ticket without NFT minted
const demoUnmintedTicket = {
  id: 3,
  eventId: 2,
  eventName: "Celtics vs Heat",
  venueName: "TD Garden",
  eventDate: Date.now() + 86400000 * 14,
  section: "B",
  row: "8",
  seatNumber: 45,
};

function QRCodeDisplay({ data }: { data: string }) {
  return (
    <div className="w-32 h-32 bg-white rounded-lg border-2 border-gray-200 flex items-center justify-center p-2">
      <QRCodeSVG
        value={data}
        size={112}
        level="M"
        includeMargin={false}
        bgColor="#ffffff"
        fgColor="#1e293b"
      />
    </div>
  );
}

function NftTicketCard({
  nft,
  onUse,
}: {
  nft: TicketNft;
  onUse: (tokenId: number) => void;
}) {
  const [copied, setCopied] = useState(false);

  const qrData = JSON.stringify({
    tokenId: nft.tokenId,
    eventId: nft.eventId,
    seatNumber: nft.seatNumber,
    hash: nft.verificationHash.slice(0, 18) + "...",
  });

  const truncatedHash =
    nft.verificationHash.slice(0, 10) + "..." + nft.verificationHash.slice(-8);

  const eventDate = new Date(nft.eventDate);

  const handleCopyHash = () => {
    navigator.clipboard.writeText(nft.verificationHash);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="bg-white rounded-2xl shadow-lg border border-gray-100 overflow-hidden hover:shadow-xl transition-shadow duration-300">
      {/* Header gradient */}
      <div className="bg-gradient-to-r from-inktix-blue-600 to-inktix-purple-600 px-6 py-4">
        <div className="flex items-center justify-between">
          <div>
            <h3 className="text-white font-bold text-lg">{nft.eventName}</h3>
            <p className="text-blue-100 text-sm">{nft.venueName}</p>
          </div>
          <div className="flex items-center gap-2">
            {nft.isUsed ? (
              <span className="bg-red-500/20 text-red-100 px-3 py-1 rounded-full text-xs font-semibold flex items-center gap-1">
                <CheckCircle className="w-3 h-3" />
                Used
              </span>
            ) : (
              <span className="bg-green-500/20 text-green-100 px-3 py-1 rounded-full text-xs font-semibold flex items-center gap-1">
                <Shield className="w-3 h-3" />
                Valid
              </span>
            )}
            <span className="bg-white/20 text-white px-3 py-1 rounded-full text-xs font-semibold">
              NFT #{nft.tokenId}
            </span>
          </div>
        </div>
      </div>

      <div className="p-6">
        <div className="flex gap-6">
          {/* QR Code */}
          <div className="flex-shrink-0">
            <QRCodeDisplay data={qrData} />
            <p className="text-xs text-gray-400 text-center mt-1">
              Scan to verify
            </p>
          </div>

          {/* Ticket Details */}
          <div className="flex-grow space-y-3">
            <div className="grid grid-cols-3 gap-3">
              <div>
                <p className="text-xs text-gray-400 uppercase tracking-wide">
                  Section
                </p>
                <p className="font-bold text-gray-900">{nft.section}</p>
              </div>
              <div>
                <p className="text-xs text-gray-400 uppercase tracking-wide">
                  Row
                </p>
                <p className="font-bold text-gray-900">{nft.row}</p>
              </div>
              <div>
                <p className="text-xs text-gray-400 uppercase tracking-wide">
                  Seat
                </p>
                <p className="font-bold text-gray-900">{nft.seatNumber}</p>
              </div>
            </div>

            <div className="grid grid-cols-2 gap-3">
              <div>
                <p className="text-xs text-gray-400 uppercase tracking-wide">
                  Date
                </p>
                <p className="text-sm font-medium text-gray-700">
                  {eventDate.toLocaleDateString("en-US", {
                    month: "short",
                    day: "numeric",
                    year: "numeric",
                  })}
                </p>
              </div>
              <div>
                <p className="text-xs text-gray-400 uppercase tracking-wide">
                  Access
                </p>
                <p className="text-sm font-medium text-gray-700">
                  {nft.accessLevel}
                </p>
              </div>
            </div>

            {/* Verification hash */}
            <div className="flex items-center gap-2 bg-gray-50 rounded-lg px-3 py-2">
              <Shield className="w-4 h-4 text-inktix-blue-500 flex-shrink-0" />
              <code className="text-xs text-gray-600 truncate flex-grow">
                {truncatedHash}
              </code>
              <button
                onClick={handleCopyHash}
                className="text-gray-400 hover:text-gray-600 flex-shrink-0"
              >
                {copied ? (
                  <Check className="w-4 h-4 text-green-500" />
                ) : (
                  <Copy className="w-4 h-4" />
                )}
              </button>
            </div>
          </div>
        </div>

        {/* Action buttons */}
        {!nft.isUsed && (
          <div className="mt-4 pt-4 border-t border-gray-100">
            <button
              onClick={() => onUse(nft.tokenId)}
              className="w-full bg-gradient-to-r from-inktix-blue-600 to-inktix-purple-600 text-white py-2.5 rounded-xl font-semibold hover:opacity-90 transition-opacity flex items-center justify-center gap-2"
            >
              <Ticket className="w-4 h-4" />
              Use Ticket for Entry
            </button>
          </div>
        )}
      </div>
    </div>
  );
}

export default function MyTicketsPage() {
  const [nftTickets, setNftTickets] =
    useState<TicketNft[]>(demoNftTickets);
  const [showUnminted] = useState(true);
  const [mintingTicketId, setMintingTicketId] = useState<number | null>(null);
  const [verifyTokenId, setVerifyTokenId] = useState("");
  const [verificationResult, setVerificationResult] =
    useState<TicketVerification | null>(null);
  const [verificationError, setVerificationError] = useState("");

  const handleMintNft = async (ticketId: number) => {
    setMintingTicketId(ticketId);
    // Simulate minting delay
    await new Promise((resolve) => setTimeout(resolve, 1500));

    const newNft: TicketNft = {
      tokenId: nftTickets.length + 1,
      ticketId,
      owner: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
      eventId: demoUnmintedTicket.eventId,
      eventName: demoUnmintedTicket.eventName,
      venueName: demoUnmintedTicket.venueName,
      eventDate: demoUnmintedTicket.eventDate,
      section: demoUnmintedTicket.section,
      row: demoUnmintedTicket.row,
      seatNumber: demoUnmintedTicket.seatNumber,
      seatType: "Reserved",
      accessLevel: "Standard",
      mintedAt: Date.now(),
      metadataUri: "",
      verificationHash: `0x${Array.from({ length: 64 }, () =>
        Math.floor(Math.random() * 16).toString(16)
      ).join("")}`,
      isUsed: false,
    };

    setNftTickets((prev) => [...prev, newNft]);
    setMintingTicketId(null);
  };

  const handleUseTicket = (tokenId: number) => {
    setNftTickets((prev) =>
      prev.map((nft) =>
        nft.tokenId === tokenId ? { ...nft, isUsed: true } : nft
      )
    );
  };

  const handleVerify = () => {
    setVerificationError("");
    setVerificationResult(null);

    const id = parseInt(verifyTokenId, 10);
    if (isNaN(id)) {
      setVerificationError("Please enter a valid token ID");
      return;
    }

    const nft = nftTickets.find((n) => n.tokenId === id);
    if (!nft) {
      setVerificationError(`No NFT found with token ID ${id}`);
      return;
    }

    setVerificationResult({
      isValid: true,
      isUsed: nft.isUsed,
      owner: nft.owner,
      eventId: nft.eventId,
      eventName: nft.eventName,
      section: nft.section,
      row: nft.row,
      seatNumber: nft.seatNumber,
    });
  };

  const isMinted = (ticketId: number) =>
    nftTickets.some((n) => n.ticketId === ticketId);

  return (
    <div className="min-h-screen bg-gradient-to-b from-gray-50 to-white">
      {/* Header */}
      <section className="bg-gradient-to-r from-inktix-blue-600 via-inktix-blue-700 to-inktix-purple-700 text-white py-16">
        <div className="container-max">
          <Link
            href="/"
            className="inline-flex items-center gap-2 text-blue-200 hover:text-white mb-6 transition-colors"
          >
            <ArrowLeft className="w-4 h-4" />
            Back to Home
          </Link>
          <div className="flex items-center gap-4">
            <div className="bg-white/10 rounded-2xl p-4">
              <Sparkles className="w-10 h-10 text-inktix-orange-400" />
            </div>
            <div>
              <h1 className="text-4xl font-bold">My NFT Tickets</h1>
              <p className="text-blue-200 mt-1">
                View, mint, and verify your on-chain ticket NFTs
              </p>
            </div>
          </div>
        </div>
      </section>

      <div className="container-max py-12">
        {/* Stats bar */}
        <div className="grid grid-cols-3 gap-4 mb-10">
          <div className="bg-white rounded-xl shadow-sm border border-gray-100 p-4 text-center">
            <p className="text-3xl font-bold text-inktix-blue-600">
              {nftTickets.length}
            </p>
            <p className="text-sm text-gray-500">NFTs Minted</p>
          </div>
          <div className="bg-white rounded-xl shadow-sm border border-gray-100 p-4 text-center">
            <p className="text-3xl font-bold text-green-600">
              {nftTickets.filter((n) => !n.isUsed).length}
            </p>
            <p className="text-sm text-gray-500">Active Tickets</p>
          </div>
          <div className="bg-white rounded-xl shadow-sm border border-gray-100 p-4 text-center">
            <p className="text-3xl font-bold text-inktix-purple-600">
              {nftTickets.filter((n) => n.isUsed).length}
            </p>
            <p className="text-sm text-gray-500">Events Attended</p>
          </div>
        </div>

        {/* NFT Tickets */}
        <div className="space-y-6 mb-12">
          <h2 className="text-2xl font-bold text-gray-900">Minted Tickets</h2>

          {nftTickets.length === 0 ? (
            <div className="bg-white rounded-2xl shadow-sm border border-gray-100 p-12 text-center">
              <Ticket className="w-16 h-16 text-gray-300 mx-auto mb-4" />
              <p className="text-gray-500 text-lg">No NFT tickets yet</p>
              <p className="text-gray-400 text-sm mt-1">
                Purchase a ticket and mint it as an NFT
              </p>
            </div>
          ) : (
            <div className="grid gap-6 md:grid-cols-1 lg:grid-cols-2">
              {nftTickets.map((nft) => (
                <NftTicketCard
                  key={nft.tokenId}
                  nft={nft}
                  onUse={handleUseTicket}
                />
              ))}
            </div>
          )}
        </div>

        {/* Unminted ticket */}
        {showUnminted && !isMinted(demoUnmintedTicket.id) && (
          <div className="mb-12">
            <h2 className="text-2xl font-bold text-gray-900 mb-6">
              Tickets Without NFT
            </h2>
            <div className="bg-white rounded-2xl shadow-sm border border-dashed border-gray-300 p-6">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-4">
                  <div className="bg-gray-100 rounded-xl p-3">
                    <Ticket className="w-8 h-8 text-gray-400" />
                  </div>
                  <div>
                    <h3 className="font-bold text-gray-900">
                      {demoUnmintedTicket.eventName}
                    </h3>
                    <p className="text-sm text-gray-500">
                      {demoUnmintedTicket.venueName} &middot; Section{" "}
                      {demoUnmintedTicket.section}, Row {demoUnmintedTicket.row}
                      , Seat {demoUnmintedTicket.seatNumber}
                    </p>
                    <p className="text-xs text-gray-400 mt-1">
                      {new Date(demoUnmintedTicket.eventDate).toLocaleDateString(
                        "en-US",
                        {
                          month: "long",
                          day: "numeric",
                          year: "numeric",
                        }
                      )}
                    </p>
                  </div>
                </div>
                <button
                  onClick={() => handleMintNft(demoUnmintedTicket.id)}
                  disabled={mintingTicketId === demoUnmintedTicket.id}
                  className="bg-gradient-to-r from-inktix-orange-500 to-inktix-orange-600 text-white px-6 py-3 rounded-xl font-semibold hover:opacity-90 transition-opacity disabled:opacity-50 flex items-center gap-2"
                >
                  {mintingTicketId === demoUnmintedTicket.id ? (
                    <>
                      <div className="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin" />
                      Minting...
                    </>
                  ) : (
                    <>
                      <Sparkles className="w-4 h-4" />
                      Mint NFT
                    </>
                  )}
                </button>
              </div>
            </div>
          </div>
        )}

        {/* Verify Ticket Section */}
        <div className="bg-white rounded-2xl shadow-lg border border-gray-100 p-8">
          <h2 className="text-2xl font-bold text-gray-900 mb-2 flex items-center gap-3">
            <Search className="w-6 h-6 text-inktix-blue-600" />
            Verify Ticket
          </h2>
          <p className="text-gray-500 mb-6">
            Enter a token ID to verify the authenticity and status of an NFT
            ticket
          </p>

          <div className="flex gap-3">
            <input
              type="text"
              value={verifyTokenId}
              onChange={(e) => setVerifyTokenId(e.target.value)}
              placeholder="Enter Token ID (e.g., 1)"
              className="flex-grow px-4 py-3 border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-inktix-blue-500 focus:border-transparent"
              onKeyDown={(e) => e.key === "Enter" && handleVerify()}
            />
            <button
              onClick={handleVerify}
              className="bg-inktix-blue-600 text-white px-8 py-3 rounded-xl font-semibold hover:bg-inktix-blue-700 transition-colors flex items-center gap-2"
            >
              <Shield className="w-4 h-4" />
              Verify
            </button>
          </div>

          {verificationError && (
            <div className="mt-4 bg-red-50 border border-red-200 rounded-xl p-4 flex items-center gap-3">
              <XCircle className="w-5 h-5 text-red-500 flex-shrink-0" />
              <p className="text-red-700">{verificationError}</p>
            </div>
          )}

          {verificationResult && (
            <div className="mt-4 bg-green-50 border border-green-200 rounded-xl p-6">
              <div className="flex items-center gap-3 mb-4">
                <CheckCircle className="w-6 h-6 text-green-500" />
                <span className="font-bold text-green-800">
                  Ticket Verified
                </span>
                {verificationResult.isUsed && (
                  <span className="bg-amber-100 text-amber-700 px-2 py-0.5 rounded-full text-xs font-semibold">
                    Already Used
                  </span>
                )}
              </div>
              <div className="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
                <div>
                  <p className="text-gray-500">Event</p>
                  <p className="font-semibold text-gray-900">
                    {verificationResult.eventName}
                  </p>
                </div>
                <div>
                  <p className="text-gray-500">Section</p>
                  <p className="font-semibold text-gray-900">
                    {verificationResult.section}
                  </p>
                </div>
                <div>
                  <p className="text-gray-500">Row</p>
                  <p className="font-semibold text-gray-900">
                    {verificationResult.row}
                  </p>
                </div>
                <div>
                  <p className="text-gray-500">Seat</p>
                  <p className="font-semibold text-gray-900">
                    {verificationResult.seatNumber}
                  </p>
                </div>
              </div>
              <div className="mt-3 pt-3 border-t border-green-200">
                <p className="text-xs text-gray-500">Owner</p>
                <code className="text-xs text-gray-600 break-all">
                  {verificationResult.owner}
                </code>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
