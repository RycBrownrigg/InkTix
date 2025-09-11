"use client";

import { useEffect, useState } from "react";
import { BlockchainProvider } from "../contexts/BlockchainContext";

interface BlockchainWrapperProps {
  children: React.ReactNode;
}

const BlockchainWrapper: React.FC<BlockchainWrapperProps> = ({ children }) => {
  const [isClient, setIsClient] = useState(false);

  useEffect(() => {
    setIsClient(true);
  }, []);

  if (!isClient) {
    return <div>Loading blockchain services...</div>;
  }

  return <BlockchainProvider>{children}</BlockchainProvider>;
};

export default BlockchainWrapper;
