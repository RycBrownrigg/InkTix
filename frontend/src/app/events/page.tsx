"use client";

import {
  Calendar,
  MapPin,
  Users,
  Ticket,
  Star,
  Filter,
  Clock,
  DollarSign,
  Globe,
  ArrowLeft,
} from "lucide-react";
import { useState, useEffect } from "react";
import Link from "next/link";

// Current events data for 2025
const mockEvents = [
  {
    id: 1,
    title: "Lakers vs Warriors",
    sport: "Basketball",
    date: "2025-01-15",
    time: "19:30",
    venue: "Crypto.com Arena",
    location: "Los Angeles, CA",
    price: 150,
    availableTickets: 45,
    image: "/api/placeholder/400/250",
    category: "NBA",
    featured: true,
    popularity: 95,
  },
  {
    id: 2,
    title: "Dodgers vs Giants",
    sport: "Baseball",
    date: "2025-01-18",
    time: "20:00",
    venue: "Dodger Stadium",
    location: "Los Angeles, CA",
    price: 85,
    availableTickets: 120,
    image: "/api/placeholder/400/250",
    category: "MLB",
    featured: false,
    popularity: 78,
  },
  {
    id: 3,
    title: "Rams vs 49ers",
    sport: "Football",
    date: "2025-01-21",
    time: "16:25",
    venue: "SoFi Stadium",
    location: "Inglewood, CA",
    price: 200,
    availableTickets: 23,
    image: "/api/placeholder/400/250",
    category: "NFL",
    featured: true,
    popularity: 98,
  },
  {
    id: 4,
    title: "Kings vs Oilers",
    sport: "Hockey",
    date: "2025-01-24",
    time: "19:00",
    venue: "Crypto.com Arena",
    location: "Los Angeles, CA",
    price: 95,
    availableTickets: 67,
    image: "/api/placeholder/400/250",
    category: "NHL",
    featured: false,
    popularity: 72,
  },
  {
    id: 5,
    title: "Galaxy vs LAFC",
    sport: "Soccer",
    date: "2025-01-27",
    time: "19:30",
    venue: "Dignity Health Sports Park",
    location: "Carson, CA",
    price: 65,
    availableTickets: 89,
    image: "/api/placeholder/400/250",
    category: "Soccer",
    featured: false,
    popularity: 85,
  },
  {
    id: 6,
    title: "Clippers vs Suns",
    sport: "Basketball",
    date: "2025-01-30",
    time: "19:30",
    venue: "Crypto.com Arena",
    location: "Los Angeles, CA",
    price: 120,
    availableTickets: 156,
    image: "/api/placeholder/400/250",
    category: "NBA",
    featured: false,
    popularity: 68,
  },
];

const categories = [
  "All",
  "NBA",
  "MLB",
  "NFL",
  "NHL",
  "Soccer",
  "College Sports",
];

export default function EventsPage() {
  const [selectedCategory, setSelectedCategory] = useState("All");
  const [isVisible, setIsVisible] = useState(false);

  useEffect(() => {
    setIsVisible(true);
  }, []);

  const filteredEvents =
    selectedCategory === "All"
      ? mockEvents
      : mockEvents.filter((event) => event.category === selectedCategory);

  return (
    <div className="min-h-screen py-12 bg-gradient-to-br from-slate-50 via-blue-50 to-indigo-50">
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

      <div className="container-max px-4 sm:px-6 lg:px-8">
        {/* Enhanced Header */}
        <div
          className={`text-center mb-16 ${isVisible ? "animate-fade-in" : ""}`}
        >
          <div className="flex justify-center mb-6">
            <div className="bg-gradient-to-r from-blue-600 to-indigo-600 rounded-full p-4 shadow-lg">
              <Ticket className="w-12 h-12 text-white" />
            </div>
          </div>
          <h1 className="heading-1 text-slate-900 mb-6">
            Sports <span className="text-gradient">Events</span>
          </h1>
          <p className="text-xl text-slate-600 max-w-3xl mx-auto leading-relaxed">
            Discover and book tickets for the best sports events across all
            chains
          </p>
        </div>

        {/* Enhanced Filters */}
        <div className={`mb-12 ${isVisible ? "animate-slide-up" : ""}`}>
          <div className="bg-white/80 backdrop-blur-sm rounded-2xl p-6 shadow-xl border border-white/20">
            <div className="flex flex-wrap items-center gap-4">
              <div className="flex items-center gap-3">
                <div className="bg-gradient-to-r from-blue-500 to-indigo-500 rounded-lg p-2">
                  <Filter className="w-5 h-5 text-white" />
                </div>
                <span className="text-lg font-semibold text-slate-700">
                  Filter by:
                </span>
              </div>

              {categories.map((category, index) => (
                <button
                  key={category}
                  onClick={() => setSelectedCategory(category)}
                  className={`px-6 py-3 rounded-xl text-sm font-semibold transition-all duration-300 transform hover:scale-105 ${
                    selectedCategory === category
                      ? "bg-gradient-to-r from-blue-600 to-indigo-600 text-white shadow-lg"
                      : "bg-slate-100 text-slate-700 hover:bg-slate-200 hover:shadow-md"
                  }`}
                  style={{ animationDelay: `${index * 100}ms` }}
                >
                  {category}
                </button>
              ))}
            </div>
          </div>
        </div>

        {/* Enhanced Events Grid */}
        <div className="grid-events">
          {filteredEvents.map((event, index) => (
            <div
              key={event.id}
              className={`card-hover group ${
                isVisible ? "animate-fade-in" : ""
              }`}
              style={{ animationDelay: `${index * 150}ms` }}
            >
              {/* Enhanced Event Image */}
              <div className="relative mb-6">
                <div className="w-full h-56 bg-gradient-to-br from-blue-100 via-indigo-100 to-purple-100 rounded-2xl flex items-center justify-center overflow-hidden">
                  <div className="relative">
                    <Ticket className="w-20 h-20 text-blue-400" />
                    <div className="absolute inset-0 bg-gradient-to-t from-black/20 to-transparent"></div>
                  </div>
                </div>

                {/* Featured Badge */}
                {event.featured && (
                  <div className="absolute top-4 left-4">
                    <div className="bg-gradient-to-r from-amber-500 to-orange-500 text-white px-3 py-1.5 rounded-full text-xs font-bold flex items-center gap-1 shadow-lg">
                      <Star className="w-3 h-3 fill-current" />
                      Featured
                    </div>
                  </div>
                )}

                {/* Category Badge */}
                <div className="absolute top-4 right-4">
                  <div className="bg-gradient-to-r from-slate-800 to-slate-900 text-white px-3 py-1.5 rounded-full text-xs font-bold shadow-lg">
                    {event.category}
                  </div>
                </div>

                {/* Popularity Indicator */}
                <div className="absolute bottom-4 right-4">
                  <div className="bg-white/90 backdrop-blur-sm rounded-full px-3 py-1.5 shadow-lg">
                    <div className="flex items-center gap-1">
                      <div className="w-2 h-2 bg-green-500 rounded-full"></div>
                      <span className="text-xs font-semibold text-slate-700">
                        {event.popularity}%
                      </span>
                    </div>
                  </div>
                </div>
              </div>

              {/* Enhanced Event Details */}
              <div className="space-y-4">
                <h3 className="text-xl font-bold text-slate-900 group-hover:text-blue-600 transition-colors leading-tight">
                  {event.title}
                </h3>

                <div className="space-y-3 text-sm text-slate-600">
                  <div className="flex items-center gap-3">
                    <div className="bg-blue-100 rounded-lg p-2">
                      <Calendar className="w-4 h-4 text-blue-600" />
                    </div>
                    <span className="font-medium">
                      {event.date} at {event.time}
                    </span>
                  </div>

                  <div className="flex items-center gap-3">
                    <div className="bg-green-100 rounded-lg p-2">
                      <MapPin className="w-4 h-4 text-green-600" />
                    </div>
                    <span className="font-medium">{event.venue}</span>
                  </div>

                  <div className="flex items-center gap-3">
                    <div className="bg-purple-100 rounded-lg p-2">
                      <Users className="w-4 h-4 text-purple-600" />
                    </div>
                    <span className="font-medium">
                      {event.availableTickets} tickets available
                    </span>
                  </div>
                </div>

                {/* Enhanced Price and Action */}
                <div className="flex items-center justify-between pt-4 border-t border-slate-200">
                  <div className="flex items-center gap-2">
                    <DollarSign className="w-5 h-5 text-green-600" />
                    <div>
                      <span className="text-3xl font-bold text-green-600">
                        ${event.price}
                      </span>
                      <span className="text-sm text-slate-500 ml-1">
                        per ticket
                      </span>
                    </div>
                  </div>

                  <button className="btn-primary text-sm px-6 py-3 group-hover:scale-105 transition-transform">
                    View Details
                  </button>
                </div>
              </div>
            </div>
          ))}
        </div>

        {/* Enhanced Load More */}
        <div
          className={`text-center mt-16 ${isVisible ? "animate-fade-in" : ""}`}
        >
          <button className="btn-secondary px-10 py-4 text-lg hover:scale-105 transition-transform">
            Load More Events
          </button>
        </div>

        {/* Stats Section */}
        <div className={`mt-20 ${isVisible ? "animate-slide-up" : ""}`}>
          <div className="bg-white/80 backdrop-blur-sm rounded-2xl p-8 shadow-xl border border-white/20">
            <h3 className="text-2xl font-bold text-slate-900 mb-6 text-center">
              Platform Statistics
            </h3>
            <div className="grid md:grid-cols-4 gap-6">
              <div className="text-center">
                <div className="bg-gradient-to-r from-blue-500 to-indigo-500 rounded-full p-4 w-fit mx-auto mb-3">
                  <Ticket className="w-8 h-8 text-white" />
                </div>
                <div className="text-3xl font-bold text-slate-900">1,247</div>
                <div className="text-slate-600">Total Events</div>
              </div>
              <div className="text-center">
                <div className="bg-gradient-to-r from-green-500 to-emerald-500 rounded-full p-4 w-fit mx-auto mb-3">
                  <Users className="w-8 h-8 text-white" />
                </div>
                <div className="text-3xl font-bold text-slate-900">45.2K</div>
                <div className="text-slate-600">Active Users</div>
              </div>
              <div className="text-center">
                <div className="bg-gradient-to-r from-purple-500 to-pink-500 rounded-full p-4 w-fit mx-auto mb-3">
                  <Globe className="w-8 h-8 text-white" />
                </div>
                <div className="text-3xl font-bold text-slate-900">12</div>
                <div className="text-slate-600">Connected Chains</div>
              </div>
              <div className="text-center">
                <div className="bg-gradient-to-r from-amber-500 to-orange-500 rounded-full p-4 w-fit mx-auto mb-3">
                  <Star className="w-8 h-8 text-white" />
                </div>
                <div className="text-3xl font-bold text-slate-900">98.7%</div>
                <div className="text-slate-600">Satisfaction</div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
