"use client";

import { useState, useEffect, useCallback } from "react";
import Link from "next/link";
import {
  ArrowLeft,
  BarChart3,
  TrendingUp,
  TrendingDown,
  Ticket,
  Users,
  DollarSign,
  Calendar,
  Activity,
  Database,
  Wifi,
  PieChart,
  Target,
  Zap,
  Shield,
  Globe,
} from "lucide-react";
import { useBlockchain } from "../../contexts/BlockchainContext";
import { MockProvider } from "../../sdk/mockProvider";

// Mock analytics data for demo
const mockPlatformStats = {
  totalRevenue: 4_750_000,
  totalTicketsSold: 31_420,
  totalEvents: 1_247,
  totalUsers: 45_200,
  averageTicketPrice: 151,
  totalSeasonPasses: 2_840,
};

const mockRevenueByMonth = [
  { month: "Sep", revenue: 320000, tickets: 2100 },
  { month: "Oct", revenue: 485000, tickets: 3200 },
  { month: "Nov", revenue: 612000, tickets: 4050 },
  { month: "Dec", revenue: 780000, tickets: 5100 },
  { month: "Jan", revenue: 925000, tickets: 6200 },
  { month: "Feb", revenue: 870000, tickets: 5700 },
  { month: "Mar", revenue: 758000, tickets: 5070 },
];

const mockCategoryBreakdown = [
  { category: "NBA", events: 245, revenue: 980000, color: "bg-blue-500" },
  { category: "NFL", events: 128, revenue: 1150000, color: "bg-green-500" },
  { category: "MLB", events: 192, revenue: 620000, color: "bg-red-500" },
  { category: "Concerts", events: 310, revenue: 1200000, color: "bg-purple-500" },
  { category: "Theater", events: 156, revenue: 420000, color: "bg-amber-500" },
  { category: "Other", events: 216, revenue: 380000, color: "bg-gray-400" },
];

const mockTopEvents = [
  { name: "Lakers vs Warriors", tickets: 18500, revenue: 2775000, sellout: 97 },
  { name: "Taylor Swift - Eras Tour", tickets: 72000, revenue: 25200000, sellout: 100 },
  { name: "Super Bowl LX", tickets: 68000, revenue: 47600000, sellout: 100 },
  { name: "Kendrick Lamar Live", tickets: 15200, revenue: 3040000, sellout: 89 },
  { name: "Hamilton - Broadway", tickets: 8400, revenue: 2520000, sellout: 95 },
];

const mockDemandTrends = [
  { hour: "6am", demand: 12 }, { hour: "8am", demand: 28 },
  { hour: "10am", demand: 45 }, { hour: "12pm", demand: 78 },
  { hour: "2pm", demand: 65 }, { hour: "4pm", demand: 52 },
  { hour: "6pm", demand: 88 }, { hour: "8pm", demand: 95 },
  { hour: "10pm", demand: 72 }, { hour: "12am", demand: 35 },
];

function BarChartSimple({
  data,
  valueKey,
  labelKey,
  maxValue,
  color = "from-inktix-blue-500 to-inktix-purple-500",
  suffix = "",
}: {
  data: any[];
  valueKey: string;
  labelKey: string;
  maxValue?: number;
  color?: string;
  suffix?: string;
}) {
  const max = maxValue || Math.max(...data.map((d) => d[valueKey]));
  return (
    <div className="space-y-2">
      {data.map((item, i) => {
        const pct = max > 0 ? (item[valueKey] / max) * 100 : 0;
        return (
          <div key={i} className="flex items-center gap-3">
            <span className="text-xs text-gray-500 w-10 text-right">{item[labelKey]}</span>
            <div className="flex-1 bg-gray-100 rounded-full h-6 relative overflow-hidden">
              <div
                className={`h-full bg-gradient-to-r ${color} rounded-full transition-all duration-700`}
                style={{ width: `${pct}%` }}
              />
              <span className="absolute right-2 top-0.5 text-xs font-medium text-gray-700">
                {typeof item[valueKey] === "number" && item[valueKey] > 1000
                  ? `${(item[valueKey] / 1000).toFixed(0)}K`
                  : item[valueKey]}{suffix}
              </span>
            </div>
          </div>
        );
      })}
    </div>
  );
}

function StatCard({
  icon: Icon,
  label,
  value,
  change,
  changeLabel,
  gradient,
}: {
  icon: any;
  label: string;
  value: string;
  change?: number;
  changeLabel?: string;
  gradient: string;
}) {
  return (
    <div className="bg-white rounded-xl shadow-sm border border-gray-200 p-6 hover:shadow-md transition-shadow">
      <div className="flex items-start justify-between">
        <div>
          <p className="text-sm text-gray-500 mb-1">{label}</p>
          <p className="text-3xl font-bold text-gray-900">{value}</p>
          {change !== undefined && (
            <div className={`flex items-center gap-1 mt-2 text-sm ${change >= 0 ? "text-green-600" : "text-red-600"}`}>
              {change >= 0 ? <TrendingUp className="w-4 h-4" /> : <TrendingDown className="w-4 h-4" />}
              <span className="font-medium">{change >= 0 ? "+" : ""}{change}%</span>
              <span className="text-gray-400">{changeLabel}</span>
            </div>
          )}
        </div>
        <div className={`p-3 rounded-xl bg-gradient-to-br ${gradient}`}>
          <Icon className="w-6 h-6 text-white" />
        </div>
      </div>
    </div>
  );
}

export default function AnalyticsPage() {
  const { isConnected, isContractDeployed, callContract } = useBlockchain();
  const [isVisible, setIsVisible] = useState(false);
  const [dataSource, setDataSource] = useState<"mock" | "contract">("mock");
  const [stats, setStats] = useState(mockPlatformStats);
  const [isLoading, setIsLoading] = useState(true);

  const loadAnalytics = useCallback(async () => {
    setIsLoading(true);
    try {
      if (isConnected && isContractDeployed) {
        const result = await callContract("get_platform_stats", []);
        if (result.success && result.data) {
          setStats({
            totalRevenue: result.data.total_revenue || result.data.totalRevenue || mockPlatformStats.totalRevenue,
            totalTicketsSold: result.data.total_tickets_sold || result.data.totalTicketsSold || mockPlatformStats.totalTicketsSold,
            totalEvents: result.data.total_events || result.data.totalEvents || mockPlatformStats.totalEvents,
            totalUsers: result.data.total_users || result.data.totalUsers || mockPlatformStats.totalUsers,
            averageTicketPrice: result.data.average_ticket_price || result.data.averageTicketPrice || mockPlatformStats.averageTicketPrice,
            totalSeasonPasses: result.data.total_season_passes || result.data.totalSeasonPasses || mockPlatformStats.totalSeasonPasses,
          });
          setDataSource("contract");
        }
      }
    } catch {
      // Keep mock data
    } finally {
      setIsLoading(false);
    }
  }, [isConnected, isContractDeployed, callContract]);

  useEffect(() => {
    setIsVisible(true);
    loadAnalytics();
  }, [loadAnalytics]);

  const totalCategoryRevenue = mockCategoryBreakdown.reduce((sum, c) => sum + c.revenue, 0);

  return (
    <div className="min-h-screen bg-gradient-to-br from-gray-50 to-slate-100">
      {/* Header */}
      <div className="bg-gradient-to-r from-slate-800 via-slate-900 to-gray-900 text-white">
        <div className="container-max py-8">
          <Link
            href="/"
            className="inline-flex items-center text-slate-400 hover:text-white mb-4 transition-colors"
          >
            <ArrowLeft className="w-4 h-4 mr-1" />
            Back to Home
          </Link>
          <div className="flex items-center justify-between">
            <div>
              <h1 className="text-3xl font-bold flex items-center gap-3">
                <BarChart3 className="w-8 h-8" />
                Analytics Dashboard
              </h1>
              <p className="text-slate-400 mt-2">
                Platform performance, revenue metrics, and event insights
              </p>
            </div>
            <div className={`inline-flex items-center gap-2 px-4 py-2 rounded-full text-sm font-medium ${
              dataSource === "contract"
                ? "bg-green-500/20 text-green-300"
                : "bg-blue-500/20 text-blue-300"
            }`}>
              {dataSource === "contract" ? (
                <><Wifi className="w-4 h-4" /> Live data</>
              ) : (
                <><Database className="w-4 h-4" /> Demo data</>
              )}
            </div>
          </div>
        </div>
      </div>

      <div className="container-max py-8 space-y-8">
        {/* KPI Cards */}
        <div className={`grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 ${isVisible ? "animate-fade-in" : ""}`}>
          <StatCard
            icon={DollarSign}
            label="Total Revenue"
            value={`${(stats.totalRevenue / 1_000_000).toFixed(1)}M DOT`}
            change={12.5}
            changeLabel="vs last month"
            gradient="from-green-500 to-emerald-600"
          />
          <StatCard
            icon={Ticket}
            label="Tickets Sold"
            value={stats.totalTicketsSold.toLocaleString()}
            change={8.3}
            changeLabel="vs last month"
            gradient="from-blue-500 to-indigo-600"
          />
          <StatCard
            icon={Calendar}
            label="Total Events"
            value={stats.totalEvents.toLocaleString()}
            change={15.2}
            changeLabel="vs last month"
            gradient="from-purple-500 to-pink-600"
          />
          <StatCard
            icon={Users}
            label="Active Users"
            value={`${(stats.totalUsers / 1000).toFixed(1)}K`}
            change={22.1}
            changeLabel="vs last month"
            gradient="from-amber-500 to-orange-600"
          />
        </div>

        {/* Secondary Stats */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <StatCard
            icon={Target}
            label="Avg Ticket Price"
            value={`${stats.averageTicketPrice} DOT`}
            change={3.2}
            changeLabel="vs last month"
            gradient="from-cyan-500 to-blue-600"
          />
          <StatCard
            icon={Shield}
            label="Season Passes"
            value={stats.totalSeasonPasses.toLocaleString()}
            change={18.7}
            changeLabel="vs last month"
            gradient="from-rose-500 to-red-600"
          />
          <StatCard
            icon={Zap}
            label="Fraud Rate"
            value="0.02%"
            change={-45}
            changeLabel="vs last month"
            gradient="from-teal-500 to-green-600"
          />
        </div>

        {/* Charts Row */}
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          {/* Revenue Trend */}
          <div className="bg-white rounded-xl shadow-sm border border-gray-200 p-6">
            <h3 className="text-lg font-bold text-gray-900 mb-1 flex items-center gap-2">
              <TrendingUp className="w-5 h-5 text-green-600" />
              Monthly Revenue
            </h3>
            <p className="text-sm text-gray-500 mb-4">Revenue trend over the last 7 months</p>
            <BarChartSimple
              data={mockRevenueByMonth}
              valueKey="revenue"
              labelKey="month"
              color="from-green-400 to-emerald-500"
            />
          </div>

          {/* Ticket Sales Trend */}
          <div className="bg-white rounded-xl shadow-sm border border-gray-200 p-6">
            <h3 className="text-lg font-bold text-gray-900 mb-1 flex items-center gap-2">
              <Ticket className="w-5 h-5 text-blue-600" />
              Monthly Ticket Sales
            </h3>
            <p className="text-sm text-gray-500 mb-4">Tickets sold per month</p>
            <BarChartSimple
              data={mockRevenueByMonth}
              valueKey="tickets"
              labelKey="month"
              color="from-blue-400 to-indigo-500"
            />
          </div>
        </div>

        {/* Category Breakdown + Demand */}
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          {/* Category Breakdown */}
          <div className="bg-white rounded-xl shadow-sm border border-gray-200 p-6">
            <h3 className="text-lg font-bold text-gray-900 mb-1 flex items-center gap-2">
              <PieChart className="w-5 h-5 text-purple-600" />
              Revenue by Category
            </h3>
            <p className="text-sm text-gray-500 mb-4">Distribution across event types</p>
            <div className="space-y-3">
              {mockCategoryBreakdown.map((cat) => {
                const pct = totalCategoryRevenue > 0 ? (cat.revenue / totalCategoryRevenue) * 100 : 0;
                return (
                  <div key={cat.category}>
                    <div className="flex items-center justify-between mb-1">
                      <div className="flex items-center gap-2">
                        <div className={`w-3 h-3 rounded-full ${cat.color}`} />
                        <span className="text-sm font-medium text-gray-700">{cat.category}</span>
                      </div>
                      <div className="text-sm text-gray-500">
                        {cat.events} events &middot; {(cat.revenue / 1000).toFixed(0)}K DOT
                      </div>
                    </div>
                    <div className="w-full bg-gray-100 rounded-full h-2.5">
                      <div
                        className={`h-full rounded-full ${cat.color} transition-all duration-700`}
                        style={{ width: `${pct}%` }}
                      />
                    </div>
                  </div>
                );
              })}
            </div>
          </div>

          {/* Demand Heatmap */}
          <div className="bg-white rounded-xl shadow-sm border border-gray-200 p-6">
            <h3 className="text-lg font-bold text-gray-900 mb-1 flex items-center gap-2">
              <Activity className="w-5 h-5 text-orange-600" />
              Purchase Activity
            </h3>
            <p className="text-sm text-gray-500 mb-4">Ticket purchase volume by time of day</p>
            <BarChartSimple
              data={mockDemandTrends}
              valueKey="demand"
              labelKey="hour"
              color="from-orange-400 to-red-500"
              suffix="%"
            />
          </div>
        </div>

        {/* Top Events */}
        <div className="bg-white rounded-xl shadow-sm border border-gray-200 p-6">
          <h3 className="text-lg font-bold text-gray-900 mb-1 flex items-center gap-2">
            <Globe className="w-5 h-5 text-inktix-blue-600" />
            Top Performing Events
          </h3>
          <p className="text-sm text-gray-500 mb-4">Highest revenue events on the platform</p>
          <div className="overflow-x-auto">
            <table className="w-full text-sm">
              <thead>
                <tr className="border-b border-gray-200">
                  <th className="text-left py-3 px-4 font-semibold text-gray-600">#</th>
                  <th className="text-left py-3 px-4 font-semibold text-gray-600">Event</th>
                  <th className="text-right py-3 px-4 font-semibold text-gray-600">Tickets</th>
                  <th className="text-right py-3 px-4 font-semibold text-gray-600">Revenue</th>
                  <th className="text-right py-3 px-4 font-semibold text-gray-600">Sellout</th>
                </tr>
              </thead>
              <tbody>
                {mockTopEvents.map((event, i) => (
                  <tr key={i} className="border-b border-gray-100 hover:bg-gray-50">
                    <td className="py-3 px-4 text-gray-400 font-medium">{i + 1}</td>
                    <td className="py-3 px-4 font-medium text-gray-900">{event.name}</td>
                    <td className="py-3 px-4 text-right text-gray-700">{event.tickets.toLocaleString()}</td>
                    <td className="py-3 px-4 text-right text-green-600 font-medium">
                      {(event.revenue / 1_000_000).toFixed(1)}M DOT
                    </td>
                    <td className="py-3 px-4 text-right">
                      <span className={`inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium ${
                        event.sellout >= 95
                          ? "bg-green-100 text-green-700"
                          : event.sellout >= 80
                          ? "bg-yellow-100 text-yellow-700"
                          : "bg-gray-100 text-gray-700"
                      }`}>
                        {event.sellout}%
                      </span>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>

        {/* Anti-Scalping & Platform Health */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div className="bg-white rounded-xl shadow-sm border border-gray-200 p-6 text-center">
            <Shield className="w-10 h-10 text-green-600 mx-auto mb-3" />
            <div className="text-2xl font-bold text-gray-900">99.98%</div>
            <div className="text-sm text-gray-500">Fraud-free Transactions</div>
            <div className="mt-2 text-xs text-green-600 font-medium">Anti-scalping active on all events</div>
          </div>
          <div className="bg-white rounded-xl shadow-sm border border-gray-200 p-6 text-center">
            <Zap className="w-10 h-10 text-blue-600 mx-auto mb-3" />
            <div className="text-2xl font-bold text-gray-900">&lt;2s</div>
            <div className="text-sm text-gray-500">Avg Transaction Time</div>
            <div className="mt-2 text-xs text-blue-600 font-medium">Powered by Polkadot</div>
          </div>
          <div className="bg-white rounded-xl shadow-sm border border-gray-200 p-6 text-center">
            <Globe className="w-10 h-10 text-purple-600 mx-auto mb-3" />
            <div className="text-2xl font-bold text-gray-900">99.9%</div>
            <div className="text-sm text-gray-500">Platform Uptime</div>
            <div className="mt-2 text-xs text-purple-600 font-medium">Cross-chain redundancy</div>
          </div>
        </div>
      </div>
    </div>
  );
}
