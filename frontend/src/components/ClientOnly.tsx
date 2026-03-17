/**
 * Renders children only after the component has mounted on the client.
 *
 * Used to wrap browser-only code (wallet extensions, localStorage) and
 * prevent React hydration mismatches in the Next.js static export.
 *
 * @module components/ClientOnly
 */
"use client";

import { useEffect, useState } from "react";

interface ClientOnlyProps {
  children: React.ReactNode;
  fallback?: React.ReactNode;
}

const ClientOnly: React.FC<ClientOnlyProps> = ({
  children,
  fallback = null,
}) => {
  const [hasMounted, setHasMounted] = useState(false);

  useEffect(() => {
    setHasMounted(true);
  }, []);

  if (!hasMounted) {
    return <>{fallback}</>;
  }

  return <>{children}</>;
};

export default ClientOnly;
