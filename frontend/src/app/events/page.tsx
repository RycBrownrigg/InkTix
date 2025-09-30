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

// Comprehensive events data for 2025
const mockEvents = [
  // Sports Events
  {
    id: 1,
    title: "Lakers vs Warriors",
    type: "sports",
    sport: "Basketball",
    date: "2025-01-15",
    time: "19:30",
    venue: "Crypto.com Arena",
    location: "Los Angeles, CA",
    price: 150,
    availableTickets: 45,
    image:
      "https://images.unsplash.com/photo-1546519638-68e109498ffc?w=400&h=250&fit=crop&crop=center",
    category: "NBA",
    featured: true,
    popularity: 95,
  },
  {
    id: 2,
    title: "Dodgers vs Giants",
    type: "sports",
    sport: "Baseball",
    date: "2025-01-18",
    time: "20:00",
    venue: "Dodger Stadium",
    location: "Los Angeles, CA",
    price: 85,
    availableTickets: 120,
    image:
      "https://images.unsplash.com/photo-1566577739112-5180d4bf9390?w=400&h=250&fit=crop&crop=center",
    category: "MLB",
    featured: false,
    popularity: 78,
  },
  {
    id: 3,
    title: "Rams vs 49ers",
    type: "sports",
    sport: "Football",
    date: "2025-01-21",
    time: "16:25",
    venue: "SoFi Stadium",
    location: "Inglewood, CA",
    price: 200,
    availableTickets: 23,
    image:
      "https://images.unsplash.com/photo-1571019613454-1cb2f99b2d8b?w=400&h=250&fit=crop&crop=center",
    category: "NFL",
    featured: true,
    popularity: 98,
  },
  {
    id: 4,
    title: "Kings vs Oilers",
    type: "sports",
    sport: "Hockey",
    date: "2025-01-24",
    time: "19:00",
    venue: "Crypto.com Arena",
    location: "Los Angeles, CA",
    price: 95,
    availableTickets: 67,
    image:
      "https://images.unsplash.com/photo-1551698618-1dfe5d97d256?w=400&h=250&fit=crop&crop=center",
    category: "NHL",
    featured: false,
    popularity: 72,
  },
  {
    id: 5,
    title: "Galaxy vs LAFC",
    type: "sports",
    sport: "Soccer",
    date: "2025-01-27",
    time: "19:30",
    venue: "Dignity Health Sports Park",
    location: "Carson, CA",
    price: 65,
    availableTickets: 89,
    image:
      "https://images.unsplash.com/photo-1431324155629-1a6deb1dec8d?w=400&h=250&fit=crop&crop=center",
    category: "Soccer",
    featured: false,
    popularity: 85,
  },
  {
    id: 6,
    title: "Clippers vs Suns",
    type: "sports",
    sport: "Basketball",
    date: "2025-01-30",
    time: "19:30",
    venue: "Crypto.com Arena",
    location: "Los Angeles, CA",
    price: 120,
    availableTickets: 156,
    image:
      "https://images.unsplash.com/photo-1546519638-68e109498ffc?w=400&h=250&fit=crop&crop=center",
    category: "NBA",
    featured: false,
    popularity: 68,
  },
  // Concert Events
  {
    id: 7,
    title: "Taylor Swift - The Eras Tour",
    type: "concert",
    artist: "Taylor Swift",
    date: "2025-02-05",
    time: "20:00",
    venue: "SoFi Stadium",
    location: "Inglewood, CA",
    price: 350,
    availableTickets: 12,
    image:
      "https://images.unsplash.com/photo-1493225457124-a3eb161ffa5f?w=400&h=250&fit=crop&crop=center",
    category: "Pop",
    featured: true,
    popularity: 99,
  },
  {
    id: 8,
    title: "Drake & 21 Savage - It's All A Blur Tour",
    type: "concert",
    artist: "Drake & 21 Savage",
    date: "2025-02-12",
    time: "21:00",
    venue: "Crypto.com Arena",
    location: "Los Angeles, CA",
    price: 280,
    availableTickets: 34,
    image:
      "https://images.unsplash.com/photo-1470229722913-7c0e2dbbafd3?w=400&h=250&fit=crop&crop=center",
    category: "Hip-Hop",
    featured: true,
    popularity: 92,
  },
  {
    id: 9,
    title: "Coldplay - Music of the Spheres",
    type: "concert",
    artist: "Coldplay",
    date: "2025-02-18",
    time: "19:30",
    venue: "Rose Bowl Stadium",
    location: "Pasadena, CA",
    price: 195,
    availableTickets: 78,
    image:
      "https://images.unsplash.com/photo-1493225457124-a3eb161ffa5f?w=400&h=250&fit=crop&crop=center",
    category: "Rock",
    featured: false,
    popularity: 88,
  },
  {
    id: 10,
    title: "Beyoncé - Renaissance World Tour",
    type: "concert",
    artist: "Beyoncé",
    date: "2025-02-25",
    time: "20:30",
    venue: "SoFi Stadium",
    location: "Inglewood, CA",
    price: 425,
    availableTickets: 8,
    image:
      "https://images.unsplash.com/photo-1493225457124-a3eb161ffa5f?w=400&h=250&fit=crop&crop=center",
    category: "R&B",
    featured: true,
    popularity: 97,
  },
  // Festival Events
  {
    id: 11,
    title: "Coachella 2025 - Weekend 1",
    type: "festival",
    festival: "Coachella",
    date: "2025-04-12",
    time: "12:00",
    venue: "Empire Polo Club",
    location: "Indio, CA",
    price: 550,
    availableTickets: 156,
    image:
      "https://images.unsplash.com/photo-1470229722913-7c0e2dbbafd3?w=400&h=250&fit=crop&crop=center",
    category: "Festival",
    featured: true,
    popularity: 96,
  },
  {
    id: 12,
    title: "Stagecoach 2025",
    type: "festival",
    festival: "Stagecoach",
    date: "2025-04-26",
    time: "11:00",
    venue: "Empire Polo Club",
    location: "Indio, CA",
    price: 325,
    availableTickets: 89,
    image:
      "https://images.unsplash.com/photo-1470229722913-7c0e2dbbafd3?w=400&h=250&fit=crop&crop=center",
    category: "Country",
    featured: false,
    popularity: 82,
  },
  // Theater Events
  {
    id: 13,
    title: "Hamilton - Broadway Musical",
    type: "theater",
    show: "Hamilton",
    date: "2025-03-08",
    time: "19:30",
    venue: "Pantages Theatre",
    location: "Hollywood, CA",
    price: 180,
    availableTickets: 23,
    image:
      "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=400&h=250&fit=crop&crop=center",
    category: "Theater",
    featured: false,
    popularity: 89,
  },
  {
    id: 14,
    title: "The Lion King - Musical",
    type: "theater",
    show: "The Lion King",
    date: "2025-03-15",
    time: "14:00",
    venue: "Pantages Theatre",
    location: "Hollywood, CA",
    price: 145,
    availableTickets: 45,
    image:
      "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=400&h=250&fit=crop&crop=center",
    category: "Theater",
    featured: false,
    popularity: 76,
  },
  // Additional Sports Events
  {
    id: 15,
    title: "US Open Tennis - Finals",
    type: "sports",
    sport: "Tennis",
    date: "2025-02-08",
    time: "16:00",
    venue: "Arthur Ashe Stadium",
    location: "New York, NY",
    price: 450,
    availableTickets: 18,
    image:
      "https://images.unsplash.com/photo-1551698618-1dfe5d97d256?w=400&h=250&fit=crop&crop=center",
    category: "Tennis",
    featured: true,
    popularity: 94,
  },
  {
    id: 16,
    title: "Masters Golf Tournament",
    type: "sports",
    sport: "Golf",
    date: "2025-04-10",
    time: "08:00",
    venue: "Augusta National Golf Club",
    location: "Augusta, GA",
    price: 850,
    availableTickets: 5,
    image:
      "https://images.unsplash.com/photo-1535131749006-b7f58c99034b?w=400&h=250&fit=crop&crop=center",
    category: "Golf",
    featured: true,
    popularity: 98,
  },
  {
    id: 17,
    title: "UFC 300 - Championship Fight",
    type: "sports",
    sport: "MMA",
    date: "2025-03-20",
    time: "19:00",
    venue: "T-Mobile Arena",
    location: "Las Vegas, NV",
    price: 320,
    availableTickets: 12,
    image:
      "https://images.unsplash.com/photo-1549719386-74dfcbf7dbed?w=400&h=250&fit=crop&crop=center",
    category: "MMA",
    featured: true,
    popularity: 96,
  },
  {
    id: 18,
    title: "Formula 1 - Miami Grand Prix",
    type: "sports",
    sport: "Racing",
    date: "2025-05-05",
    time: "15:30",
    venue: "Miami International Autodrome",
    location: "Miami, FL",
    price: 650,
    availableTickets: 8,
    image:
      "https://images.unsplash.com/photo-1558618666-fcd25c85cd64?w=400&h=250&fit=crop&crop=center",
    category: "Racing",
    featured: true,
    popularity: 97,
  },
  {
    id: 19,
    title: "Celtics vs Heat - Playoffs",
    type: "sports",
    sport: "Basketball",
    date: "2025-04-15",
    time: "20:00",
    venue: "TD Garden",
    location: "Boston, MA",
    price: 180,
    availableTickets: 67,
    image:
      "https://images.unsplash.com/photo-1546519638-68e109498ffc?w=400&h=250&fit=crop&crop=center",
    category: "NBA",
    featured: false,
    popularity: 89,
  },
  // Additional Concert Events
  {
    id: 20,
    title: "Ed Sheeran - Mathematics Tour",
    type: "concert",
    artist: "Ed Sheeran",
    date: "2025-03-22",
    time: "20:30",
    venue: "Madison Square Garden",
    location: "New York, NY",
    price: 275,
    availableTickets: 23,
    image:
      "https://images.unsplash.com/photo-1493225457124-a3eb161ffa5f?w=400&h=250&fit=crop&crop=center",
    category: "Pop",
    featured: true,
    popularity: 93,
  },
  {
    id: 21,
    title: "Metallica - WorldWired Tour",
    type: "concert",
    artist: "Metallica",
    date: "2025-04-18",
    time: "19:30",
    venue: "Soldier Field",
    location: "Chicago, IL",
    price: 195,
    availableTickets: 45,
    image:
      "https://images.unsplash.com/photo-1470229722913-7c0e2dbbafd3?w=400&h=250&fit=crop&crop=center",
    category: "Metal",
    featured: false,
    popularity: 87,
  },
  {
    id: 22,
    title: "Adele - 30 Tour",
    type: "concert",
    artist: "Adele",
    date: "2025-05-12",
    time: "20:00",
    venue: "Caesars Palace",
    location: "Las Vegas, NV",
    price: 425,
    availableTickets: 6,
    image:
      "https://images.unsplash.com/photo-1493225457124-a3eb161ffa5f?w=400&h=250&fit=crop&crop=center",
    category: "Pop",
    featured: true,
    popularity: 99,
  },
  {
    id: 23,
    title: "Kendrick Lamar - Big Steppers Tour",
    type: "concert",
    artist: "Kendrick Lamar",
    date: "2025-03-28",
    time: "21:00",
    venue: "Barclays Center",
    location: "Brooklyn, NY",
    price: 220,
    availableTickets: 34,
    image:
      "https://images.unsplash.com/photo-1470229722913-7c0e2dbbafd3?w=400&h=250&fit=crop&crop=center",
    category: "Hip-Hop",
    featured: false,
    popularity: 91,
  },
  {
    id: 24,
    title: "Billie Eilish - Happier Than Ever Tour",
    type: "concert",
    artist: "Billie Eilish",
    date: "2025-04-25",
    time: "20:30",
    venue: "Hollywood Bowl",
    location: "Los Angeles, CA",
    price: 185,
    availableTickets: 28,
    image:
      "https://images.unsplash.com/photo-1493225457124-a3eb161ffa5f?w=400&h=250&fit=crop&crop=center",
    category: "Pop",
    featured: false,
    popularity: 88,
  },
  // Additional Festival Events
  {
    id: 25,
    title: "Lollapalooza 2025",
    type: "festival",
    festival: "Lollapalooza",
    date: "2025-08-01",
    time: "12:00",
    venue: "Grant Park",
    location: "Chicago, IL",
    price: 375,
    availableTickets: 234,
    image:
      "https://images.unsplash.com/photo-1470229722913-7c0e2dbbafd3?w=400&h=250&fit=crop&crop=center",
    category: "Festival",
    featured: true,
    popularity: 94,
  },
  {
    id: 26,
    title: "Burning Man 2025",
    type: "festival",
    festival: "Burning Man",
    date: "2025-08-25",
    time: "00:00",
    venue: "Black Rock Desert",
    location: "Nevada, NV",
    price: 575,
    availableTickets: 89,
    image:
      "https://images.unsplash.com/photo-1470229722913-7c0e2dbbafd3?w=400&h=250&fit=crop&crop=center",
    category: "Festival",
    featured: true,
    popularity: 92,
  },
  {
    id: 27,
    title: "Electric Daisy Carnival (EDC)",
    type: "festival",
    festival: "EDC",
    date: "2025-05-17",
    time: "19:00",
    venue: "Las Vegas Motor Speedway",
    location: "Las Vegas, NV",
    price: 425,
    availableTickets: 156,
    image:
      "https://images.unsplash.com/photo-1470229722913-7c0e2dbbafd3?w=400&h=250&fit=crop&crop=center",
    category: "Electronic",
    featured: false,
    popularity: 89,
  },
  {
    id: 28,
    title: "Austin City Limits Music Festival",
    type: "festival",
    festival: "ACL",
    date: "2025-10-04",
    time: "11:00",
    venue: "Zilker Park",
    location: "Austin, TX",
    price: 285,
    availableTickets: 178,
    image:
      "https://images.unsplash.com/photo-1470229722913-7c0e2dbbafd3?w=400&h=250&fit=crop&crop=center",
    category: "Festival",
    featured: false,
    popularity: 85,
  },
  // Additional Theater Events
  {
    id: 29,
    title: "Wicked - Broadway Musical",
    type: "theater",
    show: "Wicked",
    date: "2025-03-12",
    time: "19:30",
    venue: "Gershwin Theatre",
    location: "New York, NY",
    price: 165,
    availableTickets: 34,
    image:
      "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=400&h=250&fit=crop&crop=center",
    category: "Theater",
    featured: false,
    popularity: 82,
  },
  {
    id: 30,
    title: "The Phantom of the Opera",
    type: "theater",
    show: "Phantom of the Opera",
    date: "2025-04-08",
    time: "20:00",
    venue: "Majestic Theatre",
    location: "New York, NY",
    price: 195,
    availableTickets: 19,
    image:
      "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=400&h=250&fit=crop&crop=center",
    category: "Theater",
    featured: true,
    popularity: 91,
  },
  {
    id: 31,
    title: "Les Misérables",
    type: "theater",
    show: "Les Misérables",
    date: "2025-05-20",
    time: "19:30",
    venue: "Imperial Theatre",
    location: "New York, NY",
    price: 175,
    availableTickets: 42,
    image:
      "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=400&h=250&fit=crop&crop=center",
    category: "Theater",
    featured: false,
    popularity: 78,
  },
  // Comedy Events
  {
    id: 32,
    title: "Dave Chappelle - Stand-Up Special",
    type: "comedy",
    comedian: "Dave Chappelle",
    date: "2025-03-30",
    time: "20:00",
    venue: "Radio City Music Hall",
    location: "New York, NY",
    price: 125,
    availableTickets: 67,
    image:
      "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=400&h=250&fit=crop&crop=center",
    category: "Comedy",
    featured: true,
    popularity: 93,
  },
  {
    id: 33,
    title: "Kevin Hart - Reality Check Tour",
    type: "comedy",
    comedian: "Kevin Hart",
    date: "2025-04-14",
    time: "19:30",
    venue: "Staples Center",
    location: "Los Angeles, CA",
    price: 95,
    availableTickets: 89,
    image:
      "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=400&h=250&fit=crop&crop=center",
    category: "Comedy",
    featured: false,
    popularity: 87,
  },
  // Classical Music Events
  {
    id: 34,
    title: "New York Philharmonic - Beethoven's 9th",
    type: "classical",
    orchestra: "New York Philharmonic",
    date: "2025-06-15",
    time: "19:30",
    venue: "Lincoln Center",
    location: "New York, NY",
    price: 145,
    availableTickets: 78,
    image:
      "https://images.unsplash.com/photo-1493225457124-a3eb161ffa5f?w=400&h=250&fit=crop&crop=center",
    category: "Classical",
    featured: false,
    popularity: 74,
  },
  {
    id: 35,
    title: "Yo-Yo Ma - Cello Recital",
    type: "classical",
    artist: "Yo-Yo Ma",
    date: "2025-07-22",
    time: "20:00",
    venue: "Carnegie Hall",
    location: "New York, NY",
    price: 185,
    availableTickets: 23,
    image:
      "https://images.unsplash.com/photo-1493225457124-a3eb161ffa5f?w=400&h=250&fit=crop&crop=center",
    category: "Classical",
    featured: true,
    popularity: 89,
  },
  // International Events
  {
    id: 36,
    title: "Real Madrid vs Barcelona - El Clásico",
    type: "sports",
    sport: "Soccer",
    date: "2025-04-21",
    time: "16:00",
    venue: "Santiago Bernabéu",
    location: "Madrid, Spain",
    price: 450,
    availableTickets: 12,
    image:
      "https://images.unsplash.com/photo-1431324155629-1a6deb1dec8d?w=400&h=250&fit=crop&crop=center",
    category: "Soccer",
    featured: true,
    popularity: 99,
  },
  {
    id: 37,
    title: "Wimbledon Championships - Finals",
    type: "sports",
    sport: "Tennis",
    date: "2025-07-13",
    time: "14:00",
    venue: "All England Club",
    location: "London, UK",
    price: 650,
    availableTickets: 8,
    image:
      "https://images.unsplash.com/photo-1551698618-1dfe5d97d256?w=400&h=250&fit=crop&crop=center",
    category: "Tennis",
    featured: true,
    popularity: 97,
  },
  {
    id: 38,
    title: "Glastonbury Festival 2025",
    type: "festival",
    festival: "Glastonbury",
    date: "2025-06-25",
    time: "12:00",
    venue: "Worthy Farm",
    location: "Somerset, UK",
    price: 350,
    availableTickets: 45,
    image:
      "https://images.unsplash.com/photo-1470229722913-7c0e2dbbafd3?w=400&h=250&fit=crop&crop=center",
    category: "Festival",
    featured: true,
    popularity: 96,
  },
  // Special Events
  {
    id: 39,
    title: "New Year's Eve - Times Square Ball Drop",
    type: "special",
    event: "New Year's Eve",
    date: "2025-12-31",
    time: "23:30",
    venue: "Times Square",
    location: "New York, NY",
    price: 0,
    availableTickets: 10000,
    image:
      "https://images.unsplash.com/photo-1513475382585-d06e58bcb0e0?w=400&h=250&fit=crop&crop=center",
    category: "Special",
    featured: true,
    popularity: 95,
  },
];

const categories = [
  "All",
  "NBA",
  "MLB",
  "NFL",
  "NHL",
  "Soccer",
  "Tennis",
  "Golf",
  "MMA",
  "Racing",
  "Pop",
  "Hip-Hop",
  "Rock",
  "R&B",
  "Metal",
  "Festival",
  "Electronic",
  "Country",
  "Theater",
  "Comedy",
  "Classical",
  "Special",
];

export default function EventsPage() {
  const [selectedCategory, setSelectedCategory] = useState("All");
  const [isVisible, setIsVisible] = useState(false);
  const [displayedEvents, setDisplayedEvents] = useState(6);
  const [selectedEvent, setSelectedEvent] = useState<any>(null);

  useEffect(() => {
    setIsVisible(true);
  }, []);

  const filteredEvents =
    selectedCategory === "All"
      ? mockEvents
      : mockEvents.filter((event) => event.category === selectedCategory);

  const eventsToShow = filteredEvents.slice(0, displayedEvents);
  const hasMoreEvents = displayedEvents < filteredEvents.length;

  const handleLoadMore = () => {
    setDisplayedEvents((prev) => Math.min(prev + 6, filteredEvents.length));
  };

  const handleViewDetails = (event: any) => {
    setSelectedEvent(event);
  };

  const closeEventDetails = () => {
    setSelectedEvent(null);
  };

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
            All <span className="text-gradient">Events</span>
          </h1>
          <p className="text-xl text-slate-600 max-w-3xl mx-auto leading-relaxed">
            Discover and book tickets for sports, concerts, festivals, and
            theater events across all chains
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
          {eventsToShow.map((event, index) => (
            <div
              key={event.id}
              className={`card-hover group ${
                isVisible ? "animate-fade-in" : ""
              }`}
              style={{ animationDelay: `${index * 150}ms` }}
            >
              {/* Enhanced Event Image */}
              <div className="relative mb-6">
                <div className="w-full h-56 rounded-2xl overflow-hidden">
                  <img
                    src={event.image}
                    alt={event.title}
                    className="w-full h-full object-cover"
                    onError={(e) => {
                      const target = e.target as HTMLImageElement;
                      target.style.display = "none";
                      const nextSibling = target.nextSibling as HTMLElement;
                      if (nextSibling) nextSibling.style.display = "flex";
                    }}
                  />
                  <div
                    className="w-full h-full bg-gradient-to-br from-blue-100 via-indigo-100 to-purple-100 flex items-center justify-center"
                    style={{ display: "none" }}
                  >
                    <div className="relative">
                      <Ticket className="w-20 h-20 text-blue-400" />
                      <div className="absolute inset-0 bg-gradient-to-t from-black/20 to-transparent"></div>
                    </div>
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

                {/* Event Type Specific Info */}
                {event.type === "concert" && event.artist && (
                  <div className="flex items-center gap-2 text-sm text-blue-600 font-medium">
                    <span>🎵</span>
                    <span>{event.artist}</span>
                  </div>
                )}
                {event.type === "festival" && event.festival && (
                  <div className="flex items-center gap-2 text-sm text-purple-600 font-medium">
                    <span>🎪</span>
                    <span>{event.festival}</span>
                  </div>
                )}
                {event.type === "theater" && event.show && (
                  <div className="flex items-center gap-2 text-sm text-green-600 font-medium">
                    <span>🎭</span>
                    <span>{event.show}</span>
                  </div>
                )}
                {event.type === "comedy" && event.comedian && (
                  <div className="flex items-center gap-2 text-sm text-orange-600 font-medium">
                    <span>😂</span>
                    <span>{event.comedian}</span>
                  </div>
                )}
                {event.type === "classical" &&
                  (event.orchestra || event.artist) && (
                    <div className="flex items-center gap-2 text-sm text-indigo-600 font-medium">
                      <span>🎼</span>
                      <span>{event.orchestra || event.artist}</span>
                    </div>
                  )}
                {event.type === "special" && event.event && (
                  <div className="flex items-center gap-2 text-sm text-red-600 font-medium">
                    <span>🎉</span>
                    <span>{event.event}</span>
                  </div>
                )}

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

                  <button
                    onClick={() => handleViewDetails(event)}
                    className="btn-primary text-sm px-6 py-3 group-hover:scale-105 transition-transform"
                  >
                    View Details
                  </button>
                </div>
              </div>
            </div>
          ))}
        </div>

        {/* Enhanced Load More */}
        {hasMoreEvents && (
          <div
            className={`text-center mt-16 ${
              isVisible ? "animate-fade-in" : ""
            }`}
          >
            <button
              onClick={handleLoadMore}
              className="btn-secondary px-10 py-4 text-lg hover:scale-105 transition-transform"
            >
              Load More Events ({filteredEvents.length - displayedEvents}{" "}
              remaining)
            </button>
          </div>
        )}

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

      {/* Event Details Modal */}
      {selectedEvent && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center p-4 z-50">
          <div className="bg-white rounded-2xl max-w-2xl w-full max-h-[90vh] overflow-y-auto">
            <div className="relative">
              {/* Modal Header */}
              <div className="relative h-64 rounded-t-2xl overflow-hidden">
                <img
                  src={selectedEvent.image}
                  alt={selectedEvent.title}
                  className="w-full h-full object-cover"
                  onError={(e) => {
                    const target = e.target as HTMLImageElement;
                    target.style.display = "none";
                    const nextSibling = target.nextSibling as HTMLElement;
                    if (nextSibling) nextSibling.style.display = "flex";
                  }}
                />
                <div
                  className="w-full h-full bg-gradient-to-br from-blue-100 via-indigo-100 to-purple-100 flex items-center justify-center"
                  style={{ display: "none" }}
                >
                  <Ticket className="w-20 h-20 text-blue-400" />
                </div>
                <button
                  onClick={closeEventDetails}
                  className="absolute top-4 right-4 bg-white/90 hover:bg-white rounded-full p-2 transition-colors"
                >
                  <span className="text-xl">×</span>
                </button>
                {selectedEvent.featured && (
                  <div className="absolute top-4 left-4">
                    <div className="bg-gradient-to-r from-amber-500 to-orange-500 text-white px-3 py-1.5 rounded-full text-xs font-bold flex items-center gap-1 shadow-lg">
                      <Star className="w-3 h-3 fill-current" />
                      Featured
                    </div>
                  </div>
                )}
              </div>

              {/* Modal Content */}
              <div className="p-6">
                <h2 className="text-3xl font-bold text-slate-900 mb-4">
                  {selectedEvent.title}
                </h2>

                {/* Event Type Specific Info */}
                {selectedEvent.type === "concert" && selectedEvent.artist && (
                  <div className="flex items-center gap-2 text-lg text-blue-600 font-medium mb-4">
                    <span>🎵</span>
                    <span>{selectedEvent.artist}</span>
                  </div>
                )}
                {selectedEvent.type === "festival" &&
                  selectedEvent.festival && (
                    <div className="flex items-center gap-2 text-lg text-purple-600 font-medium mb-4">
                      <span>🎪</span>
                      <span>{selectedEvent.festival}</span>
                    </div>
                  )}
                {selectedEvent.type === "theater" && selectedEvent.show && (
                  <div className="flex items-center gap-2 text-lg text-green-600 font-medium mb-4">
                    <span>🎭</span>
                    <span>{selectedEvent.show}</span>
                  </div>
                )}
                {selectedEvent.type === "comedy" && selectedEvent.comedian && (
                  <div className="flex items-center gap-2 text-lg text-orange-600 font-medium mb-4">
                    <span>😂</span>
                    <span>{selectedEvent.comedian}</span>
                  </div>
                )}
                {selectedEvent.type === "classical" &&
                  (selectedEvent.orchestra || selectedEvent.artist) && (
                    <div className="flex items-center gap-2 text-lg text-indigo-600 font-medium mb-4">
                      <span>🎼</span>
                      <span>
                        {selectedEvent.orchestra || selectedEvent.artist}
                      </span>
                    </div>
                  )}
                {selectedEvent.type === "special" && selectedEvent.event && (
                  <div className="flex items-center gap-2 text-lg text-red-600 font-medium mb-4">
                    <span>🎉</span>
                    <span>{selectedEvent.event}</span>
                  </div>
                )}

                <div className="grid md:grid-cols-2 gap-6 mb-6">
                  <div className="space-y-4">
                    <div className="flex items-center gap-3">
                      <div className="bg-blue-100 rounded-lg p-3">
                        <Calendar className="w-5 h-5 text-blue-600" />
                      </div>
                      <div>
                        <div className="font-semibold text-slate-900">
                          Date & Time
                        </div>
                        <div className="text-slate-600">
                          {selectedEvent.date} at {selectedEvent.time}
                        </div>
                      </div>
                    </div>

                    <div className="flex items-center gap-3">
                      <div className="bg-green-100 rounded-lg p-3">
                        <MapPin className="w-5 h-5 text-green-600" />
                      </div>
                      <div>
                        <div className="font-semibold text-slate-900">
                          Venue
                        </div>
                        <div className="text-slate-600">
                          {selectedEvent.venue}
                        </div>
                        <div className="text-sm text-slate-500">
                          {selectedEvent.location}
                        </div>
                      </div>
                    </div>
                  </div>

                  <div className="space-y-4">
                    <div className="flex items-center gap-3">
                      <div className="bg-purple-100 rounded-lg p-3">
                        <Users className="w-5 h-5 text-purple-600" />
                      </div>
                      <div>
                        <div className="font-semibold text-slate-900">
                          Availability
                        </div>
                        <div className="text-slate-600">
                          {selectedEvent.availableTickets} tickets available
                        </div>
                      </div>
                    </div>

                    <div className="flex items-center gap-3">
                      <div className="bg-amber-100 rounded-lg p-3">
                        <Star className="w-5 h-5 text-amber-600" />
                      </div>
                      <div>
                        <div className="font-semibold text-slate-900">
                          Popularity
                        </div>
                        <div className="text-slate-600">
                          {selectedEvent.popularity}%
                        </div>
                      </div>
                    </div>
                  </div>
                </div>

                {/* Price and Action */}
                <div className="border-t border-slate-200 pt-6">
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-3">
                      <DollarSign className="w-6 h-6 text-green-600" />
                      <div>
                        <span className="text-4xl font-bold text-green-600">
                          ${selectedEvent.price}
                        </span>
                        <span className="text-lg text-slate-500 ml-2">
                          per ticket
                        </span>
                      </div>
                    </div>
                    <button className="btn-primary px-8 py-4 text-lg">
                      Buy Tickets
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
