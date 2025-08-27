"use client";

import Link from "next/link";
import {
  Ticket,
  Globe,
  Building2,
  Users,
  Trophy,
  Shield,
  ArrowRight,
  Star,
  Sparkles,
  Zap,
  Lock,
  TrendingUp,
} from "lucide-react";
import { useEffect, useState } from "react";

export default function HomePage() {
  const [isVisible, setIsVisible] = useState(false);

  useEffect(() => {
    setIsVisible(true);
  }, []);

  return (
    <div className="min-h-screen">
      {/* Hero Section - Taller for Full Logo Display */}
      <section className="relative overflow-hidden bg-gradient-to-br from-inktix-blue-500 via-inktix-blue-600 to-inktix-blue-700 text-white py-32">
        {/* Animated Background Elements - Reduced for Logo Focus */}
        <div className="absolute inset-0">
          <div className="absolute top-20 left-20 w-48 h-48 bg-inktix-blue-500/15 rounded-full blur-3xl animate-pulse"></div>
          <div className="absolute bottom-20 right-20 w-64 h-64 bg-inktix-orange-500/15 rounded-full blur-3xl animate-pulse delay-1000"></div>
          <div className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-64 h-64 bg-inktix-blue-400/8 rounded-full blur-3xl animate-pulse delay-500"></div>

          {/* Large Background Logo - The Star of the Show */}
          <div className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-[1000px] h-[1000px] opacity-40 pointer-events-none">
            {/* Glow effect behind the logo */}
            <div className="absolute inset-0 bg-white/10 rounded-full blur-3xl"></div>
            <img
              src="/InkTix_logo.png"
              alt="InkTix Background Logo"
              className="w-full h-full object-contain drop-shadow-2xl relative z-10"
            />
          </div>
        </div>

        <div className="relative py-20 container-max">
          <div className="text-center">
            {/* Enhanced Typography */}
            <h1
              className={`heading-1 mb-8 ${isVisible ? "animate-fade-in" : ""}`}
            >
              Welcome to <span className="text-inktix-orange-500">InkTix</span>
            </h1>

            <p
              className={`text-xl md:text-2xl lg:text-3xl mb-12 text-blue-100 max-w-4xl mx-auto leading-relaxed ${
                isVisible ? "animate-slide-up" : ""
              }`}
            >
              The future of sports ticketing with cross-chain functionality,
              venue management, and DeFi integration powered by{" "}
              <span className="text-white font-semibold">Polkadot</span>.
            </p>

            {/* Enhanced CTA Buttons */}
            <div
              className={`flex flex-col sm:flex-row gap-6 justify-center ${
                isVisible ? "animate-scale-in" : ""
              }`}
            >
              <Link
                href="/events"
                className="btn-accent text-lg px-10 py-4 inline-flex items-center gap-3 group"
              >
                <span>Browse Events</span>
                <ArrowRight className="w-5 h-5 group-hover:translate-x-1 transition-transform" />
              </Link>
              <Link
                href="/connect"
                className="btn-outline text-lg px-10 py-4 inline-flex items-center gap-3 group"
              >
                <Shield className="w-5 h-5 group-hover:scale-110 transition-transform" />
                <span>Connect Wallet</span>
              </Link>
            </div>

            {/* Trust Indicators */}
            <div
              className={`mt-16 flex flex-wrap justify-center items-center gap-8 text-blue-200 ${
                isVisible ? "animate-fade-in" : ""
              }`}
            >
              <div className="flex items-center gap-2">
                <Lock className="w-5 h-5" />
                <span className="text-sm font-medium">Secure & Private</span>
              </div>
              <div className="flex items-center gap-2">
                <Globe className="w-5 h-5" />
                <span className="text-sm font-medium">Cross-Chain Ready</span>
              </div>
              <div className="flex items-center gap-2">
                <TrendingUp className="w-5 h-5" />
                <span className="text-sm font-medium">DeFi Powered</span>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="bg-gradient-to-b from-white via-inktix-blue-50 to-inktix-orange-50 section-padding">
        <div className="container-max">
          <div className="text-center mb-20">
            <h2
              className={`heading-2 text-slate-900 mb-6 ${
                isVisible ? "animate-fade-in" : ""
              }`}
            >
              Revolutionary <span className="text-gradient">Features</span>
            </h2>
            <p
              className={`text-xl text-slate-600 max-w-3xl mx-auto leading-relaxed ${
                isVisible ? "animate-slide-up" : ""
              }`}
            >
              Experience the next generation of sports ticketing with
              cutting-edge blockchain technology
            </p>
          </div>

          <div className="grid-features">
            {/* Cross-Chain Functionality */}
            <div
              className={`card-hover group ${
                isVisible ? "animate-fade-in" : ""
              }`}
            >
              <div className="bg-gradient-to-br from-blue-100 to-indigo-100 rounded-2xl p-6 w-fit mb-6 group-hover:scale-110 transition-transform duration-300">
                <Globe className="w-10 h-10 text-blue-600" />
              </div>
              <h3 className="heading-3 text-slate-900 mb-4">
                Cross-Chain Functionality
              </h3>
              <p className="text-slate-600 mb-6 leading-relaxed">
                Seamlessly interact with multiple Polkadot parachains using XCM
                technology
              </p>
              <div className="flex items-center gap-2">
                <Star className="w-5 h-5 text-amber-500 fill-current" />
                <span className="text-sm text-amber-600 font-semibold">
                  XCM Integration
                </span>
              </div>
            </div>

            {/* Venue Management */}
            <div
              className={`card-hover group ${
                isVisible ? "animate-fade-in delay-100" : ""
              }`}
            >
              <div className="bg-gradient-to-br from-amber-100 to-orange-100 rounded-2xl p-6 w-fit mb-6 group-hover:scale-110 transition-transform duration-300">
                <Building2 className="w-10 h-10 text-amber-600" />
              </div>
              <h3 className="heading-3 text-slate-900 mb-4">
                Venue Management
              </h3>
              <p className="text-slate-600 mb-6 leading-relaxed">
                Comprehensive venue features including parking, concessions, and
                loyalty programs
              </p>
              <div className="flex items-center gap-2">
                <Star className="w-5 h-5 text-amber-500 fill-current" />
                <span className="text-sm text-amber-600 font-semibold">
                  Complete Integration
                </span>
              </div>
            </div>

            {/* Fantasy Sports */}
            <div
              className={`card-hover group ${
                isVisible ? "animate-fade-in delay-200" : ""
              }`}
            >
              <div className="bg-gradient-to-br from-slate-100 to-gray-100 rounded-2xl p-6 w-fit mb-6 group-hover:scale-110 transition-transform duration-300">
                <Trophy className="w-10 h-10 text-slate-600" />
              </div>
              <h3 className="heading-3 text-slate-900 mb-4">Fantasy Sports</h3>
              <p className="text-slate-600 mb-6 leading-relaxed">
                Integrated fantasy sports with ticket purchases and loyalty
                rewards
              </p>
              <div className="flex items-center gap-2">
                <Star className="w-5 h-5 text-amber-500 fill-current" />
                <span className="text-sm text-amber-600 font-semibold">
                  Loyalty Integration
                </span>
              </div>
            </div>

            {/* Team Loyalty */}
            <div
              className={`card-hover group ${
                isVisible ? "animate-fade-in delay-300" : ""
              }`}
            >
              <div className="bg-gradient-to-br from-blue-100 to-indigo-100 rounded-2xl p-6 w-fit mb-6 group-hover:scale-110 transition-transform duration-300">
                <Users className="w-10 h-10 text-blue-600" />
              </div>
              <h3 className="heading-3 text-slate-900 mb-4">
                Team Loyalty Programs
              </h3>
              <p className="text-slate-600 mb-6 leading-relaxed">
                Advanced loyalty systems with staking, rewards, and performance
                tracking
              </p>
              <div className="flex items-center gap-2">
                <Star className="w-5 h-5 text-amber-500 fill-current" />
                <span className="text-sm text-amber-600 font-semibold">
                  Staking Rewards
                </span>
              </div>
            </div>

            {/* Season Passes */}
            <div
              className={`card-hover group ${
                isVisible ? "animate-fade-in delay-400" : ""
              }`}
            >
              <div className="bg-gradient-to-br from-amber-100 to-orange-100 rounded-2xl p-6 w-fit mb-6 group-hover:scale-110 transition-transform duration-300">
                <Ticket className="w-10 h-10 text-amber-600" />
              </div>
              <h3 className="heading-3 text-slate-900 mb-4">
                Season Pass Management
              </h3>
              <p className="text-slate-600 mb-6 leading-relaxed">
                DeFi-powered season passes with staking requirements and dynamic
                pricing
              </p>
              <div className="flex items-center gap-2">
                <Star className="w-5 h-5 text-amber-500 fill-current" />
                <span className="text-sm text-amber-600 font-semibold">
                  DeFi Integration
                </span>
              </div>
            </div>

            {/* Security */}
            <div
              className={`card-hover group ${
                isVisible ? "animate-fade-in delay-500" : ""
              }`}
            >
              <div className="bg-gradient-to-br from-slate-100 to-gray-100 rounded-2xl p-6 w-fit mb-6 group-hover:scale-110 transition-transform duration-300">
                <Shield className="w-10 h-10 text-slate-600" />
              </div>
              <h3 className="heading-3 text-slate-900 mb-4">
                Enterprise Security
              </h3>
              <p className="text-slate-600 mb-6 leading-relaxed">
                Built on Polkadot with Ink! smart contracts for maximum security
              </p>
              <div className="flex items-center gap-2">
                <Star className="w-5 h-5 text-amber-500 fill-current" />
                <span className="text-sm text-amber-600 font-semibold">
                  Polkadot Native
                </span>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Enhanced CTA Section */}
      <section className="bg-gradient-to-r from-inktix-blue-500 via-inktix-blue-600 to-inktix-blue-700 text-white section-padding relative overflow-hidden">
        {/* Background Pattern */}
        <div className="absolute inset-0 opacity-10">
          <div className="absolute inset-0 bg-gradient-to-br from-white/5 to-transparent"></div>
        </div>

        <div className="relative container-max text-center">
          <h2
            className={`heading-2 mb-8 ${isVisible ? "animate-fade-in" : ""}`}
          >
            Ready to Experience the Future?
          </h2>
          <p
            className={`text-xl mb-12 text-blue-100 max-w-3xl mx-auto leading-relaxed ${
              isVisible ? "animate-slide-up" : ""
            }`}
          >
            Join the revolution in sports ticketing with InkTix
          </p>
          <div
            className={`flex flex-col sm:flex-row gap-6 justify-center ${
              isVisible ? "animate-scale-in" : ""
            }`}
          >
            <Link
              href="/demo"
              className="btn-accent text-lg px-10 py-4 inline-flex items-center gap-3 group"
            >
              <span>Try Demo</span>
              <ArrowRight className="w-5 h-5 group-hover:translate-x-1 transition-transform" />
            </Link>
            <Link
              href="/docs"
              className="btn-outline text-lg px-10 py-4 inline-flex items-center gap-3 group"
            >
              <span>Read Documentation</span>
            </Link>
          </div>
        </div>
      </section>
    </div>
  );
}
