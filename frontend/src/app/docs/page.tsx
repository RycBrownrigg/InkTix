"use client";

import React from "react";
import Link from "next/link";
import {
  ArrowLeft,
  FileText,
  Book,
  Settings,
  Code,
  Database,
  Shield,
  Globe,
  Zap,
} from "lucide-react";

const DocsPage: React.FC = () => {
  const documentationSections = [
    {
      title: "Project Documentation",
      icon: <FileText className="w-6 h-6" />,
      items: [
        {
          name: "Product Specification",
          description: "Complete product requirements and features",
          href: "/markdown-viewer.html?file=/docs/product_specification.md",
          type: "markdown",
        },
        {
          name: "System Architecture",
          description: "Technical architecture and system design",
          href: "/markdown-viewer.html?file=/docs/system_architecture.md",
          type: "markdown",
        },
        {
          name: "Deployment Guide",
          description: "Step-by-step deployment instructions",
          href: "/markdown-viewer.html?file=/docs/DEPLOYMENT.md",
          type: "markdown",
        },
        {
          name: "Requirements",
          description: "System requirements and dependencies",
          href: "/markdown-viewer.html?file=/docs/REQUIREMENTS.md",
          type: "markdown",
        },
        {
          name: "Debian Notes",
          description: "Debian-specific deployment notes",
          href: "/markdown-viewer.html?file=/docs/DEBIAN_NOTES.md",
          type: "markdown",
        },
        {
          name: "Changelog",
          description: "Version history and changes",
          href: "/markdown-viewer.html?file=/docs/CHANGELOG.md",
          type: "markdown",
        },
      ],
    },
    {
      title: "Smart Contracts",
      icon: <Code className="w-6 h-6" />,
      items: [
        {
          name: "Sports Broker Contract",
          description: "Sports event management smart contract",
          href: "/contracts/sports_broker/",
          type: "contract",
        },
        {
          name: "Concert Broker Contract",
          description: "Concert event management smart contract",
          href: "/contracts/concert_broker/",
          type: "contract",
        },
        {
          name: "InkTix Core Library",
          description: "Shared types and utilities",
          href: "/contracts/inktix_core/",
          type: "contract",
        },
      ],
    },
    {
      title: "Frontend Documentation",
      icon: <Globe className="w-6 h-6" />,
      items: [
        {
          name: "Smart Contracts README",
          description: "Frontend smart contract integration guide",
          href: "/markdown-viewer.html?file=/docs/README_SMART_CONTRACTS.md",
          type: "markdown",
        },
        {
          name: "Frontend README",
          description: "Frontend development and setup guide",
          href: "/markdown-viewer.html?file=/docs/README.md",
          type: "markdown",
        },
      ],
    },
    {
      title: "Development Resources",
      icon: <Settings className="w-6 h-6" />,
      items: [
        {
          name: "Package Configuration",
          description: "Project dependencies and scripts",
          href: "/package.json",
          type: "config",
        },
        {
          name: "Next.js Configuration",
          description: "Frontend build configuration",
          href: "/frontend/next.config.js",
          type: "config",
        },
        {
          name: "TypeScript Configuration",
          description: "TypeScript compiler settings",
          href: "/frontend/tsconfig.json",
          type: "config",
        },
      ],
    },
  ];

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
          <div className="flex justify-center mb-6">
            <div className="bg-gradient-to-r from-inktix-blue-600 to-inktix-orange-600 rounded-full p-4 shadow-lg">
              <Book className="w-12 h-12 text-white" />
            </div>
          </div>
          <h1 className="text-4xl md:text-5xl font-bold text-gray-900 mb-6">
            InkTix{" "}
            <span className="text-transparent bg-clip-text bg-gradient-to-r from-inktix-blue-600 to-inktix-orange-600">
              Documentation
            </span>
          </h1>
          <p className="text-xl text-gray-600 max-w-3xl mx-auto leading-relaxed">
            Complete documentation for the InkTix blockchain ticketing platform
          </p>
        </div>

        {/* Documentation Sections */}
        <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
          {documentationSections.map((section, sectionIndex) => (
            <div
              key={sectionIndex}
              className="bg-white rounded-lg shadow-lg border border-gray-200 overflow-hidden"
            >
              <div className="bg-gradient-to-r from-inktix-blue-600 to-inktix-orange-600 px-6 py-4">
                <div className="flex items-center gap-3">
                  <div className="text-white">{section.icon}</div>
                  <h2 className="text-xl font-bold text-white">
                    {section.title}
                  </h2>
                </div>
              </div>

              <div className="p-6">
                <div className="space-y-4">
                  {section.items.map((item, itemIndex) => (
                    <div
                      key={itemIndex}
                      className="border border-gray-200 rounded-lg p-4 hover:shadow-md transition-shadow"
                    >
                      <div className="flex items-start justify-between">
                        <div className="flex-1">
                          <h3 className="font-semibold text-gray-900 mb-2">
                            {item.name}
                          </h3>
                          <p className="text-sm text-gray-600 mb-3">
                            {item.description}
                          </p>
                          <div className="flex items-center gap-2">
                            <span
                              className={`px-2 py-1 rounded-full text-xs font-medium ${
                                item.type === "markdown"
                                  ? "bg-blue-100 text-blue-800"
                                  : item.type === "contract"
                                  ? "bg-green-100 text-green-800"
                                  : "bg-gray-100 text-gray-800"
                              }`}
                            >
                              {item.type}
                            </span>
                            <a
                              href={item.href}
                              target="_blank"
                              rel="noopener noreferrer"
                              className="text-inktix-blue-600 hover:text-inktix-blue-800 text-sm font-medium flex items-center gap-1"
                            >
                              View Documentation
                              <ArrowLeft className="w-3 h-3 rotate-180" />
                            </a>
                          </div>
                        </div>
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            </div>
          ))}
        </div>

        {/* Quick Links */}
        <div className="mt-12 bg-white rounded-lg shadow-lg border border-gray-200 p-8">
          <h2 className="text-2xl font-bold text-gray-900 mb-6 text-center">
            Quick Links
          </h2>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            <Link
              href="/smart-contracts"
              className="flex items-center gap-3 p-4 bg-gradient-to-r from-inktix-blue-50 to-inktix-blue-100 rounded-lg hover:from-inktix-blue-100 hover:to-inktix-blue-200 transition-all"
            >
              <Code className="w-6 h-6 text-inktix-blue-600" />
              <div>
                <h3 className="font-semibold text-gray-900">Smart Contracts</h3>
                <p className="text-sm text-gray-600">
                  Deploy and interact with contracts
                </p>
              </div>
            </Link>

            <Link
              href="/connect"
              className="flex items-center gap-3 p-4 bg-gradient-to-r from-inktix-orange-50 to-inktix-orange-100 rounded-lg hover:from-inktix-orange-100 hover:to-inktix-orange-200 transition-all"
            >
              <Shield className="w-6 h-6 text-inktix-orange-600" />
              <div>
                <h3 className="font-semibold text-gray-900">Connect Wallet</h3>
                <p className="text-sm text-gray-600">
                  Connect your Polkadot wallet
                </p>
              </div>
            </Link>

            <Link
              href="/events"
              className="flex items-center gap-3 p-4 bg-gradient-to-r from-green-50 to-green-100 rounded-lg hover:from-green-100 hover:to-green-200 transition-all"
            >
              <Zap className="w-6 h-6 text-green-600" />
              <div>
                <h3 className="font-semibold text-gray-900">Browse Events</h3>
                <p className="text-sm text-gray-600">
                  Discover sports and concert events
                </p>
              </div>
            </Link>
          </div>
        </div>
      </div>
    </div>
  );
};

export default DocsPage;
