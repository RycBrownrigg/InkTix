import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "InkTix - Revolutionary Sports Ticketing Platform",
  description:
    "Experience the future of sports ticketing with InkTix. Built on Polkadot blockchain technology, featuring advanced loyalty programs, cross-chain functionality, and seamless fan experiences.",
  keywords:
    "sports ticketing, blockchain, Polkadot, InkTix, loyalty programs, cross-chain",
  authors: [{ name: "InkTix Team" }],
  openGraph: {
    title: "InkTix - Revolutionary Sports Ticketing Platform",
    description:
      "Experience the future of sports ticketing with InkTix. Built on Polkadot blockchain technology.",
    type: "website",
    locale: "en_US",
  },
};

export const viewport = {
  width: "device-width",
  initialScale: 1,
  maximumScale: 1,
  userScalable: false,
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className="antialiased">{children}</body>
    </html>
  );
}
