"use client";

import { useEffect } from "react";
import { useRouter } from "next/navigation";

export default function CrossChainRedirect() {
  const router = useRouter();

  useEffect(() => {
    router.replace("/cross-chain-demo");
  }, [router]);

  return (
    <div className="min-h-screen bg-gradient-to-br from-inktix-blue-50 via-white to-inktix-orange-50 flex items-center justify-center">
      <div className="text-center">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-inktix-blue-600 mx-auto mb-4"></div>
        <p className="text-gray-600">
          Redirecting to the new cross-chain demo...
        </p>
      </div>
    </div>
  );
}

