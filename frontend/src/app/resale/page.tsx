"use client";

import { useState, useEffect, useCallback } from "react";
import Link from "next/link";
import {
  ArrowLeft,
  Tag,
  Shield,
  Clock,
  TrendingUp,
  AlertTriangle,
  CheckCircle,
  Search,
  Filter,
  Ticket,
  DollarSign,
  ArrowRightLeft,
  ShoppingCart,
  Calendar,
  MapPin,
} from "lucide-react";
import { useBlockchain } from "../../contexts/BlockchainContext";
import type { ResaleListing } from "../../sdk/types";
import { MockProvider } from "../../sdk/mockProvider";

export default function ResaleMarketplacePage() {
  const {
    isConnected,
    isWalletConnected,
    selectedAccount,
    callContract,
    balance,
  } = useBlockchain();

  const [listings, setListings] = useState<ResaleListing[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [searchQuery, setSearchQuery] = useState("");
  const [sortBy, setSortBy] = useState<"price-asc" | "price-desc" | "expiry">(
    "price-asc"
  );
  const [selectedListing, setSelectedListing] = useState<ResaleListing | null>(
    null
  );
  const [buyingTicketId, setBuyingTicketId] = useState<number | null>(null);
  const [buyResult, setBuyResult] = useState<string>("");

  // Sell modal state
  const [showSellModal, setShowSellModal] = useState(false);
  const [sellPrice, setSellPrice] = useState("");
  const [sellCurrency, setSellCurrency] = useState("DOT");
  const [sellResult, setSellResult] = useState<string>("");
  const [isSelling, setIsSelling] = useState(false);
  const [userTickets, setUserTickets] = useState<any[]>([]);
  const [selectedTicketToSell, setSelectedTicketToSell] = useState<any | null>(null);
  const [isLoadingUserTickets, setIsLoadingUserTickets] = useState(false);
  const [sellTab, setSellTab] = useState<"owned" | "manual">("owned");
  const [manualEntry, setManualEntry] = useState({
    ticketId: "",
    eventName: "",
    venue: "",
    eventDate: "",
    section: "",
    row: "",
    seatNumber: "",
    purchasePrice: "",
  });

  const loadListings = useCallback(async () => {
    setIsLoading(true);
    try {
      // Try contract first, fall back to mock
      const result = await callContract("get_resale_listings", []);
      if (result.success && result.data) {
        setListings(result.data);
      } else {
        // Fallback to mock provider directly
        const mock = new MockProvider();
        const mockResult = await mock.getResaleListings();
        if (mockResult.success && mockResult.data) {
          setListings(mockResult.data);
        }
      }
    } catch {
      const mock = new MockProvider();
      const mockResult = await mock.getResaleListings();
      if (mockResult.success && mockResult.data) {
        setListings(mockResult.data);
      }
    } finally {
      setIsLoading(false);
    }
  }, [callContract]);

  useEffect(() => {
    loadListings();
  }, [loadListings]);

  const handleBuy = async (listing: ResaleListing) => {
    if (!isWalletConnected) {
      setBuyResult("Please connect your wallet first");
      return;
    }
    setBuyingTicketId(listing.ticketId);
    setBuyResult("");

    try {
      let result;
      try {
        result = await callContract("buy_resale_ticket", [listing.ticketId]);
      } catch {
        result = { success: false, error: "contract unavailable" };
      }

      if (!result.success && result.error?.includes("not deployed")) {
        const mock = new MockProvider();
        result = await mock.buyResaleTicket(listing.ticketId);
      }

      if (result.success) {
        setBuyResult(
          `Ticket #${listing.ticketId} purchased for ${listing.askingPrice} ${listing.currency}!`
        );
        setSelectedListing(null);
        await loadListings();
      } else {
        setBuyResult(`Purchase failed: ${result.error}`);
      }
    } catch (error) {
      setBuyResult(
        `Error: ${error instanceof Error ? error.message : "Unknown error"}`
      );
    } finally {
      setBuyingTicketId(null);
    }
  };

  const loadUserTickets = useCallback(async () => {
    if (!selectedAccount) return;
    setIsLoadingUserTickets(true);
    try {
      // In production, this would call get_user_tickets then get_ticket for each
      // For demo, show mock owned tickets
      setUserTickets([
        {
          id: 1,
          eventName: "Lakers vs Warriors",
          eventDate: "2025-01-15",
          venue: "Crypto.com Arena",
          location: "Los Angeles, CA",
          section: "A",
          row: "15",
          seatNumber: 101,
          purchasePrice: "150",
          currency: "DOT",
          seatType: "Reserved",
          accessLevel: "Premium",
        },
        {
          id: 2,
          eventName: "Lakers vs Warriors",
          eventDate: "2025-01-15",
          venue: "Crypto.com Arena",
          location: "Los Angeles, CA",
          section: "A",
          row: "15",
          seatNumber: 102,
          purchasePrice: "150",
          currency: "DOT",
          seatType: "Reserved",
          accessLevel: "Premium",
        },
        {
          id: 3,
          eventName: "Taylor Swift - Eras Tour",
          eventDate: "2025-03-22",
          venue: "SoFi Stadium",
          location: "Inglewood, CA",
          section: "Floor",
          row: "8",
          seatNumber: 14,
          purchasePrice: "350",
          currency: "DOT",
          seatType: "Floor",
          accessLevel: "VIP",
        },
      ]);
    } finally {
      setIsLoadingUserTickets(false);
    }
  }, [selectedAccount]);

  const openSellModal = () => {
    setShowSellModal(true);
    setSelectedTicketToSell(null);
    setSellPrice("");
    setSellResult("");
    setSellTab("owned");
    setManualEntry({ ticketId: "", eventName: "", venue: "", eventDate: "", section: "", row: "", seatNumber: "", purchasePrice: "" });
    loadUserTickets();
  };

  const selectManualTicket = () => {
    if (!manualEntry.ticketId || !manualEntry.eventName) return;
    setSelectedTicketToSell({
      id: parseInt(manualEntry.ticketId),
      eventName: manualEntry.eventName,
      eventDate: manualEntry.eventDate || "TBD",
      venue: manualEntry.venue || "TBD",
      location: "",
      section: manualEntry.section || "GA",
      row: manualEntry.row || "1",
      seatNumber: parseInt(manualEntry.seatNumber) || 1,
      purchasePrice: manualEntry.purchasePrice || "0",
      currency: "DOT",
      seatType: "General",
      accessLevel: "Standard",
    });
    setSellPrice(manualEntry.purchasePrice || "");
  };

  const handleSell = async () => {
    if (!selectedTicketToSell || !sellPrice) return;
    setIsSelling(true);
    setSellResult("");

    try {
      // Try contract call first, fall back to mock
      let result;
      try {
        result = await callContract("resell_ticket", [
          selectedTicketToSell.id,
          sellPrice,
          sellCurrency,
        ]);
      } catch {
        result = { success: false, error: "contract unavailable" };
      }

      if (!result.success && (result.error?.includes("not deployed") || result.error?.includes("contract unavailable"))) {
        const mock = new MockProvider();
        result = await mock.resellTicket(
          selectedTicketToSell.id,
          sellPrice,
          sellCurrency,
          {
            eventName: selectedTicketToSell.eventName,
            section: selectedTicketToSell.section,
            row: selectedTicketToSell.row,
            seatNumber: selectedTicketToSell.seatNumber,
            originalPrice: selectedTicketToSell.purchasePrice,
          }
        );
      }

      if (result.success) {
        setSellResult("Ticket listed for resale successfully!");
        setSelectedTicketToSell(null);
        setSellPrice("");
        setShowSellModal(false);
        await loadListings();
      } else {
        setSellResult(`Failed: ${result.error}`);
      }
    } catch (error) {
      setSellResult(
        `Error: ${error instanceof Error ? error.message : "Unknown error"}`
      );
    } finally {
      setIsSelling(false);
    }
  };

  const filteredListings = listings
    .filter(
      (l) =>
        l.isActive &&
        l.expiryTime > Date.now() &&
        (searchQuery === "" ||
          l.eventName.toLowerCase().includes(searchQuery.toLowerCase()))
    )
    .sort((a, b) => {
      switch (sortBy) {
        case "price-asc":
          return parseFloat(a.askingPrice) - parseFloat(b.askingPrice);
        case "price-desc":
          return parseFloat(b.askingPrice) - parseFloat(a.askingPrice);
        case "expiry":
          return a.expiryTime - b.expiryTime;
        default:
          return 0;
      }
    });

  const formatTimeLeft = (expiryTime: number) => {
    const diff = expiryTime - Date.now();
    if (diff <= 0) return "Expired";
    const hours = Math.floor(diff / 3600000);
    const mins = Math.floor((diff % 3600000) / 60000);
    if (hours >= 24) return `${Math.floor(hours / 24)}d ${hours % 24}h`;
    return `${hours}h ${mins}m`;
  };

  const getPriceChange = (asking: string, original: string) => {
    const a = parseFloat(asking);
    const o = parseFloat(original);
    if (o === 0) return 0;
    return Math.round(((a - o) / o) * 100);
  };

  const truncateAddress = (addr: string) =>
    `${addr.slice(0, 6)}...${addr.slice(-4)}`;

  return (
    <div className="min-h-screen bg-gradient-to-br from-gray-50 to-blue-50">
      {/* Header */}
      <div className="bg-gradient-to-r from-inktix-purple-700 via-inktix-purple-600 to-inktix-blue-700 text-white">
        <div className="container-max py-8">
          <Link
            href="/"
            className="inline-flex items-center text-purple-200 hover:text-white mb-4 transition-colors"
          >
            <ArrowLeft className="w-4 h-4 mr-1" />
            Back to Home
          </Link>
          <div className="flex items-center justify-between">
            <div>
              <h1 className="text-3xl font-bold flex items-center gap-3">
                <ArrowRightLeft className="w-8 h-8" />
                Resale Marketplace
              </h1>
              <p className="text-purple-200 mt-2">
                Buy and sell tickets securely with anti-scalping protection
              </p>
            </div>
            {isWalletConnected && (
              <button
                onClick={openSellModal}
                className="bg-white text-inktix-purple-700 font-semibold px-6 py-3 rounded-lg hover:bg-purple-50 transition-colors flex items-center gap-2"
              >
                <Tag className="w-5 h-5" />
                List a Ticket
              </button>
            )}
          </div>

          {/* Stats bar */}
          <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mt-6">
            <div className="bg-white/10 rounded-lg p-3 text-center">
              <div className="text-2xl font-bold">{filteredListings.length}</div>
              <div className="text-sm text-purple-200">Active Listings</div>
            </div>
            <div className="bg-white/10 rounded-lg p-3 text-center">
              <div className="text-2xl font-bold">
                {filteredListings.length > 0
                  ? Math.min(
                      ...filteredListings.map((l) => parseFloat(l.askingPrice))
                    )
                  : 0}{" "}
                DOT
              </div>
              <div className="text-sm text-purple-200">Lowest Price</div>
            </div>
            <div className="bg-white/10 rounded-lg p-3 text-center">
              <div className="text-2xl font-bold">
                <Shield className="w-5 h-5 inline mr-1" />
                Verified
              </div>
              <div className="text-sm text-purple-200">Anti-Scalp Protected</div>
            </div>
            <div className="bg-white/10 rounded-lg p-3 text-center">
              <div className="text-2xl font-bold">0%</div>
              <div className="text-sm text-purple-200">Fraud Rate</div>
            </div>
          </div>
        </div>
      </div>

      <div className="container-max py-8">
        {/* Anti-scalping info banner */}
        <div className="bg-green-50 border border-green-200 rounded-lg p-4 mb-6 flex items-start gap-3">
          <Shield className="w-5 h-5 text-green-600 mt-0.5 flex-shrink-0" />
          <div>
            <h3 className="font-semibold text-green-800">
              Anti-Scalping Protection Active
            </h3>
            <p className="text-sm text-green-700 mt-1">
              All resale prices are capped at 1.5x original price. Transfer
              cooldowns prevent rapid flipping. Seller identities are verified
              on-chain.
            </p>
          </div>
        </div>

        {/* Search and filters */}
        <div className="flex flex-col md:flex-row gap-4 mb-6">
          <div className="relative flex-1">
            <Search className="w-5 h-5 absolute left-3 top-1/2 -translate-y-1/2 text-gray-400" />
            <input
              type="text"
              placeholder="Search events..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full pl-10 pr-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-inktix-purple-500 focus:border-inktix-purple-500"
            />
          </div>
          <div className="flex items-center gap-2">
            <Filter className="w-5 h-5 text-gray-500" />
            <select
              value={sortBy}
              onChange={(e) => setSortBy(e.target.value as any)}
              className="px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-inktix-purple-500"
            >
              <option value="price-asc">Price: Low to High</option>
              <option value="price-desc">Price: High to Low</option>
              <option value="expiry">Expiring Soon</option>
            </select>
          </div>
        </div>

        {/* Listings */}
        {isLoading ? (
          <div className="text-center py-16">
            <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-inktix-purple-600 mx-auto mb-4"></div>
            <p className="text-gray-600">Loading resale listings...</p>
          </div>
        ) : filteredListings.length === 0 ? (
          <div className="text-center py-16 bg-white rounded-xl shadow-sm">
            <Ticket className="w-16 h-16 mx-auto mb-4 text-gray-300" />
            <h3 className="text-xl font-semibold text-gray-600 mb-2">
              No listings found
            </h3>
            <p className="text-gray-500">
              {searchQuery
                ? "Try a different search term"
                : "Check back later for resale tickets"}
            </p>
          </div>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {filteredListings.map((listing) => {
              const priceChange = getPriceChange(
                listing.askingPrice,
                listing.originalPrice
              );
              const isExpiringSoon =
                listing.expiryTime - Date.now() < 7200000;

              return (
                <div
                  key={listing.ticketId}
                  className="bg-white rounded-xl shadow-sm border border-gray-200 hover:shadow-md transition-all overflow-hidden"
                >
                  {/* Event header */}
                  <div className="bg-gradient-to-r from-inktix-blue-600 to-inktix-purple-600 p-4 text-white">
                    <h3 className="font-semibold text-lg truncate">
                      {listing.eventName}
                    </h3>
                    <div className="flex items-center justify-between mt-1">
                      <span className="text-sm text-blue-100">
                        Ticket #{listing.ticketId}
                      </span>
                      {listing.approved ? (
                        <span className="inline-flex items-center gap-1 text-xs bg-green-500/20 text-green-100 px-2 py-0.5 rounded-full">
                          <CheckCircle className="w-3 h-3" /> Verified
                        </span>
                      ) : (
                        <span className="inline-flex items-center gap-1 text-xs bg-yellow-500/20 text-yellow-100 px-2 py-0.5 rounded-full">
                          <Clock className="w-3 h-3" /> Pending
                        </span>
                      )}
                    </div>
                  </div>

                  {/* Details */}
                  <div className="p-4 space-y-3">
                    {/* Seat info */}
                    <div className="flex items-center justify-between text-sm">
                      <span className="text-gray-500">Seat</span>
                      <span className="font-medium text-gray-800">
                        Section {listing.section}, Row {listing.row}, Seat{" "}
                        {listing.seatNumber}
                      </span>
                    </div>

                    {/* Seller */}
                    <div className="flex items-center justify-between text-sm">
                      <span className="text-gray-500">Seller</span>
                      <span className="font-mono text-xs text-gray-600">
                        {truncateAddress(listing.seller)}
                      </span>
                    </div>

                    {/* Price comparison */}
                    <div className="bg-gray-50 rounded-lg p-3">
                      <div className="flex items-center justify-between mb-1">
                        <span className="text-sm text-gray-500">
                          Original Price
                        </span>
                        <span className="text-sm text-gray-600">
                          {listing.originalPrice} {listing.currency}
                        </span>
                      </div>
                      <div className="flex items-center justify-between">
                        <span className="text-sm font-semibold text-gray-800">
                          Asking Price
                        </span>
                        <div className="flex items-center gap-2">
                          <span className="text-lg font-bold text-inktix-purple-700">
                            {listing.askingPrice} {listing.currency}
                          </span>
                          <span
                            className={`text-xs font-medium px-1.5 py-0.5 rounded ${
                              priceChange > 0
                                ? "bg-red-100 text-red-700"
                                : priceChange < 0
                                ? "bg-green-100 text-green-700"
                                : "bg-gray-100 text-gray-600"
                            }`}
                          >
                            {priceChange > 0 ? "+" : ""}
                            {priceChange}%
                          </span>
                        </div>
                      </div>
                    </div>

                    {/* Expiry */}
                    <div
                      className={`flex items-center gap-2 text-sm ${
                        isExpiringSoon ? "text-orange-600" : "text-gray-500"
                      }`}
                    >
                      <Clock className="w-4 h-4" />
                      <span>
                        {isExpiringSoon && (
                          <AlertTriangle className="w-3 h-3 inline mr-1" />
                        )}
                        Expires in {formatTimeLeft(listing.expiryTime)}
                      </span>
                    </div>

                    {/* Buy button */}
                    <button
                      onClick={() => handleBuy(listing)}
                      disabled={
                        !isWalletConnected ||
                        !listing.approved ||
                        buyingTicketId === listing.ticketId
                      }
                      className="w-full bg-gradient-to-r from-inktix-purple-600 to-inktix-blue-600 hover:from-inktix-purple-700 hover:to-inktix-blue-700 disabled:from-gray-400 disabled:to-gray-500 text-white font-semibold py-3 rounded-lg transition-all flex items-center justify-center gap-2"
                    >
                      {buyingTicketId === listing.ticketId ? (
                        <>
                          <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white"></div>
                          Processing...
                        </>
                      ) : !isWalletConnected ? (
                        "Connect Wallet to Buy"
                      ) : !listing.approved ? (
                        "Pending Approval"
                      ) : (
                        <>
                          <ShoppingCart className="w-4 h-4" />
                          Buy for {listing.askingPrice} {listing.currency}
                        </>
                      )}
                    </button>
                  </div>
                </div>
              );
            })}
          </div>
        )}

        {/* Buy result toast */}
        {buyResult && (
          <div
            className={`fixed bottom-6 right-6 max-w-md p-4 rounded-lg shadow-lg z-50 ${
              buyResult.includes("purchased") || buyResult.includes("success")
                ? "bg-green-600 text-white"
                : "bg-red-600 text-white"
            }`}
          >
            <div className="flex items-center gap-2">
              {buyResult.includes("purchased") ||
              buyResult.includes("success") ? (
                <CheckCircle className="w-5 h-5 flex-shrink-0" />
              ) : (
                <AlertTriangle className="w-5 h-5 flex-shrink-0" />
              )}
              <span>{buyResult}</span>
            </div>
            <button
              onClick={() => setBuyResult("")}
              className="absolute top-1 right-2 text-white/70 hover:text-white"
            >
              x
            </button>
          </div>
        )}

        {/* How it works */}
        <div className="mt-12 bg-white rounded-xl shadow-sm border border-gray-200 p-8">
          <h2 className="text-2xl font-bold text-gray-800 mb-6 text-center">
            How Resale Works
          </h2>
          <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
            <div className="text-center">
              <div className="w-12 h-12 bg-inktix-purple-100 rounded-full flex items-center justify-center mx-auto mb-3">
                <Tag className="w-6 h-6 text-inktix-purple-600" />
              </div>
              <h3 className="font-semibold text-gray-800 mb-1">1. List</h3>
              <p className="text-sm text-gray-600">
                Set your price within the anti-scalping cap (max 1.5x original)
              </p>
            </div>
            <div className="text-center">
              <div className="w-12 h-12 bg-inktix-blue-100 rounded-full flex items-center justify-center mx-auto mb-3">
                <Shield className="w-6 h-6 text-inktix-blue-600" />
              </div>
              <h3 className="font-semibold text-gray-800 mb-1">2. Verify</h3>
              <p className="text-sm text-gray-600">
                On-chain verification ensures ticket authenticity and ownership
              </p>
            </div>
            <div className="text-center">
              <div className="w-12 h-12 bg-green-100 rounded-full flex items-center justify-center mx-auto mb-3">
                <DollarSign className="w-6 h-6 text-green-600" />
              </div>
              <h3 className="font-semibold text-gray-800 mb-1">3. Buy</h3>
              <p className="text-sm text-gray-600">
                Pay with DOT or any supported currency, instant settlement
              </p>
            </div>
            <div className="text-center">
              <div className="w-12 h-12 bg-orange-100 rounded-full flex items-center justify-center mx-auto mb-3">
                <ArrowRightLeft className="w-6 h-6 text-orange-600" />
              </div>
              <h3 className="font-semibold text-gray-800 mb-1">4. Transfer</h3>
              <p className="text-sm text-gray-600">
                Ownership transfers on-chain with full audit trail
              </p>
            </div>
          </div>
        </div>
      </div>

      {/* Sell Modal */}
      {showSellModal && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
          <div className="bg-white rounded-xl shadow-2xl w-full max-w-lg max-h-[90vh] flex flex-col">
            {/* Modal header */}
            <div className="p-6 border-b border-gray-200">
              <h2 className="text-xl font-bold text-gray-800 flex items-center gap-2">
                <Tag className="w-5 h-5 text-inktix-purple-600" />
                {selectedTicketToSell
                  ? "Set Your Price"
                  : "Select a Ticket to Sell"}
              </h2>
              {!selectedTicketToSell && (
                <p className="text-sm text-gray-500 mt-1">
                  Choose from your owned tickets below
                </p>
              )}
            </div>

            <div className="p-6 overflow-y-auto flex-1">
              {!selectedTicketToSell ? (
                /* Step 1: Pick a ticket */
                <div className="space-y-4">
                  {/* Tabs */}
                  <div className="flex border-b border-gray-200">
                    <button
                      onClick={() => setSellTab("owned")}
                      className={`flex-1 py-2 text-sm font-medium border-b-2 transition-colors ${
                        sellTab === "owned"
                          ? "border-inktix-purple-600 text-inktix-purple-700"
                          : "border-transparent text-gray-500 hover:text-gray-700"
                      }`}
                    >
                      My Tickets
                    </button>
                    <button
                      onClick={() => setSellTab("manual")}
                      className={`flex-1 py-2 text-sm font-medium border-b-2 transition-colors ${
                        sellTab === "manual"
                          ? "border-inktix-purple-600 text-inktix-purple-700"
                          : "border-transparent text-gray-500 hover:text-gray-700"
                      }`}
                    >
                      Enter Manually
                    </button>
                  </div>

                  {sellTab === "owned" ? (
                    /* Owned tickets list */
                    <div className="space-y-3">
                      {isLoadingUserTickets ? (
                        <div className="text-center py-8">
                          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-inktix-purple-600 mx-auto mb-3"></div>
                          <p className="text-gray-500 text-sm">
                            Loading your tickets...
                          </p>
                        </div>
                      ) : userTickets.length === 0 ? (
                        <div className="text-center py-8">
                          <Ticket className="w-12 h-12 mx-auto mb-3 text-gray-300" />
                          <p className="text-gray-500 mb-2">
                            No tickets found in your wallet
                          </p>
                          <button
                            onClick={() => setSellTab("manual")}
                            className="text-sm text-inktix-purple-600 hover:underline"
                          >
                            Enter ticket details manually
                          </button>
                        </div>
                      ) : (
                        userTickets.map((ticket) => (
                          <button
                            key={ticket.id}
                            onClick={() => {
                              setSelectedTicketToSell(ticket);
                              setSellPrice(ticket.purchasePrice);
                              setSellCurrency(ticket.currency);
                            }}
                            className="w-full text-left border border-gray-200 rounded-lg p-4 hover:border-inktix-purple-400 hover:bg-purple-50 transition-all"
                          >
                            <div className="flex items-start justify-between">
                              <div className="flex-1">
                                <h4 className="font-semibold text-gray-800">
                                  {ticket.eventName}
                                </h4>
                                <div className="mt-1 space-y-1 text-sm text-gray-600">
                                  <div className="flex items-center gap-1">
                                    <Calendar className="w-3.5 h-3.5" />
                                    {ticket.eventDate}
                                  </div>
                                  <div className="flex items-center gap-1">
                                    <MapPin className="w-3.5 h-3.5" />
                                    {ticket.venue}, {ticket.location}
                                  </div>
                                  <div className="flex items-center gap-1">
                                    <Ticket className="w-3.5 h-3.5" />
                                    Section {ticket.section}, Row {ticket.row},
                                    Seat {ticket.seatNumber}
                                  </div>
                                </div>
                              </div>
                              <div className="text-right ml-4">
                                <div className="text-lg font-bold text-inktix-purple-700">
                                  {ticket.purchasePrice} {ticket.currency}
                                </div>
                                <div className="text-xs text-gray-500">Paid</div>
                                <span className="inline-block mt-1 text-xs bg-inktix-blue-100 text-inktix-blue-700 px-2 py-0.5 rounded">
                                  {ticket.accessLevel}
                                </span>
                              </div>
                            </div>
                          </button>
                        ))
                      )}
                    </div>
                  ) : (
                    /* Manual entry form */
                    <div className="space-y-3">
                      <p className="text-sm text-gray-500">
                        Enter your ticket details. Ownership will be verified on-chain before the listing goes live.
                      </p>
                      <div className="grid grid-cols-2 gap-3">
                        <div className="col-span-2">
                          <label className="block text-xs font-medium text-gray-600 mb-1">
                            Ticket ID (from your purchase receipt)
                          </label>
                          <input
                            type="number"
                            value={manualEntry.ticketId}
                            onChange={(e) =>
                              setManualEntry({ ...manualEntry, ticketId: e.target.value })
                            }
                            placeholder="e.g. 1042"
                            className="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-inktix-purple-500"
                          />
                        </div>
                        <div className="col-span-2">
                          <label className="block text-xs font-medium text-gray-600 mb-1">
                            Event Name
                          </label>
                          <input
                            type="text"
                            value={manualEntry.eventName}
                            onChange={(e) =>
                              setManualEntry({ ...manualEntry, eventName: e.target.value })
                            }
                            placeholder="e.g. Lakers vs Warriors"
                            className="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-inktix-purple-500"
                          />
                        </div>
                        <div>
                          <label className="block text-xs font-medium text-gray-600 mb-1">
                            Venue
                          </label>
                          <input
                            type="text"
                            value={manualEntry.venue}
                            onChange={(e) =>
                              setManualEntry({ ...manualEntry, venue: e.target.value })
                            }
                            placeholder="e.g. Staples Center"
                            className="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-inktix-purple-500"
                          />
                        </div>
                        <div>
                          <label className="block text-xs font-medium text-gray-600 mb-1">
                            Event Date
                          </label>
                          <input
                            type="date"
                            value={manualEntry.eventDate}
                            onChange={(e) =>
                              setManualEntry({ ...manualEntry, eventDate: e.target.value })
                            }
                            className="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-inktix-purple-500"
                          />
                        </div>
                        <div>
                          <label className="block text-xs font-medium text-gray-600 mb-1">
                            Section
                          </label>
                          <input
                            type="text"
                            value={manualEntry.section}
                            onChange={(e) =>
                              setManualEntry({ ...manualEntry, section: e.target.value })
                            }
                            placeholder="e.g. A"
                            className="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-inktix-purple-500"
                          />
                        </div>
                        <div>
                          <label className="block text-xs font-medium text-gray-600 mb-1">
                            Row
                          </label>
                          <input
                            type="text"
                            value={manualEntry.row}
                            onChange={(e) =>
                              setManualEntry({ ...manualEntry, row: e.target.value })
                            }
                            placeholder="e.g. 12"
                            className="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-inktix-purple-500"
                          />
                        </div>
                        <div>
                          <label className="block text-xs font-medium text-gray-600 mb-1">
                            Seat Number
                          </label>
                          <input
                            type="number"
                            value={manualEntry.seatNumber}
                            onChange={(e) =>
                              setManualEntry({ ...manualEntry, seatNumber: e.target.value })
                            }
                            placeholder="e.g. 45"
                            className="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-inktix-purple-500"
                          />
                        </div>
                        <div>
                          <label className="block text-xs font-medium text-gray-600 mb-1">
                            Original Price (DOT)
                          </label>
                          <input
                            type="number"
                            value={manualEntry.purchasePrice}
                            onChange={(e) =>
                              setManualEntry({ ...manualEntry, purchasePrice: e.target.value })
                            }
                            placeholder="e.g. 150"
                            className="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-inktix-purple-500"
                          />
                        </div>
                      </div>
                      <button
                        onClick={selectManualTicket}
                        disabled={!manualEntry.ticketId || !manualEntry.eventName}
                        className="w-full bg-inktix-purple-600 hover:bg-inktix-purple-700 disabled:bg-gray-400 text-white font-medium py-2.5 rounded-lg transition-colors text-sm"
                      >
                        Continue to Set Price
                      </button>
                    </div>
                  )}
                </div>
              ) : (
                /* Step 2: Set price for selected ticket */
                <div className="space-y-4">
                  {/* Selected ticket summary */}
                  <div className="bg-gray-50 rounded-lg p-4">
                    <h4 className="font-semibold text-gray-800 mb-2">
                      {selectedTicketToSell.eventName}
                    </h4>
                    <div className="grid grid-cols-2 gap-2 text-sm text-gray-600">
                      <div>
                        <span className="text-gray-400">Venue:</span>{" "}
                        {selectedTicketToSell.venue}
                      </div>
                      <div>
                        <span className="text-gray-400">Date:</span>{" "}
                        {selectedTicketToSell.eventDate}
                      </div>
                      <div>
                        <span className="text-gray-400">Seat:</span> Section{" "}
                        {selectedTicketToSell.section}, Row{" "}
                        {selectedTicketToSell.row}, Seat{" "}
                        {selectedTicketToSell.seatNumber}
                      </div>
                      <div>
                        <span className="text-gray-400">Type:</span>{" "}
                        {selectedTicketToSell.seatType} /{" "}
                        {selectedTicketToSell.accessLevel}
                      </div>
                    </div>
                    <div className="mt-2 pt-2 border-t border-gray-200 flex justify-between text-sm">
                      <span className="text-gray-500">Original price</span>
                      <span className="font-medium text-gray-800">
                        {selectedTicketToSell.purchasePrice}{" "}
                        {selectedTicketToSell.currency}
                      </span>
                    </div>
                    <button
                      onClick={() => setSelectedTicketToSell(null)}
                      className="text-xs text-inktix-purple-600 hover:underline mt-2"
                    >
                      Choose a different ticket
                    </button>
                  </div>

                  {/* Price input */}
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">
                      Your Asking Price
                    </label>
                    <div className="flex gap-2">
                      <input
                        type="number"
                        value={sellPrice}
                        onChange={(e) => setSellPrice(e.target.value)}
                        placeholder="0.00"
                        className="flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-inktix-purple-500"
                      />
                      <select
                        value={sellCurrency}
                        onChange={(e) => setSellCurrency(e.target.value)}
                        className="px-3 py-2 border border-gray-300 rounded-lg"
                      >
                        <option value="DOT">DOT</option>
                        <option value="KSM">KSM</option>
                        <option value="AUSD">aUSD</option>
                      </select>
                    </div>
                    {/* Price guidance */}
                    <div className="mt-2 flex items-center justify-between text-xs">
                      <span className="text-gray-500">
                        Max allowed: {(parseFloat(selectedTicketToSell.purchasePrice) * 1.5).toFixed(0)}{" "}
                        {selectedTicketToSell.currency} (1.5x)
                      </span>
                      {sellPrice &&
                        parseFloat(sellPrice) >
                          parseFloat(selectedTicketToSell.purchasePrice) *
                            1.5 && (
                          <span className="text-red-600 font-medium">
                            Exceeds anti-scalping cap
                          </span>
                        )}
                    </div>
                  </div>

                  <div className="bg-yellow-50 border border-yellow-200 rounded-lg p-3">
                    <div className="flex items-start gap-2">
                      <AlertTriangle className="w-4 h-4 text-yellow-600 mt-0.5" />
                      <div className="text-sm text-yellow-700">
                        <p className="font-medium">
                          Anti-scalping rules apply
                        </p>
                        <p className="mt-1">
                          Listing active for 24 hours. On-chain verification
                          required before visible to buyers.
                        </p>
                      </div>
                    </div>
                  </div>

                  {sellResult && (
                    <div
                      className={`p-3 rounded-lg text-sm ${
                        sellResult.includes("success")
                          ? "bg-green-50 text-green-700"
                          : "bg-red-50 text-red-700"
                      }`}
                    >
                      {sellResult}
                    </div>
                  )}
                </div>
              )}
            </div>

            {/* Modal footer */}
            <div className="p-6 border-t border-gray-200 flex gap-3">
              <button
                onClick={() => {
                  setShowSellModal(false);
                  setSellResult("");
                  setSelectedTicketToSell(null);
                }}
                className="flex-1 px-4 py-3 border border-gray-300 rounded-lg text-gray-700 hover:bg-gray-50 font-medium"
              >
                Cancel
              </button>
              {selectedTicketToSell && (
                <button
                  onClick={handleSell}
                  disabled={
                    !sellPrice ||
                    isSelling ||
                    parseFloat(sellPrice) >
                      parseFloat(selectedTicketToSell.purchasePrice) * 1.5
                  }
                  className="flex-1 bg-gradient-to-r from-inktix-purple-600 to-inktix-blue-600 hover:from-inktix-purple-700 hover:to-inktix-blue-700 disabled:from-gray-400 disabled:to-gray-500 text-white font-semibold px-4 py-3 rounded-lg transition-all"
                >
                  {isSelling ? "Listing..." : "List for Resale"}
                </button>
              )}
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
