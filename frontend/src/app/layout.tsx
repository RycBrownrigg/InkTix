import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";
import ClientOnly from "../components/ClientOnly";
import { BlockchainProvider } from "../contexts/BlockchainContext";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "InkTix - Sports Ticketing on Polkadot",
  description:
    "Decentralized sports ticketing platform built on Polkadot blockchain",
};

export const viewport = {
  width: "device-width",
  initialScale: 1,
  maximumScale: 1,
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <ClientOnly fallback={<div>Loading InkTix...</div>}>
          <BlockchainProvider>{children}</BlockchainProvider>
        </ClientOnly>
      </body>
    </html>
  );
}
