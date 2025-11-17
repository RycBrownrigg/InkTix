/** @type {import('next').NextConfig} */
const nextConfig = {
  basePath: "/inktix",
  output: "export",
  trailingSlash: true,
  outputFileTracingRoot: __dirname,
  webpack: (config) => {
    config.resolve.fallback = {
      ...config.resolve.fallback,
      fs: false,
      net: false,
      tls: false,
    };
    return config;
  },
  images: {
    domains: ["localhost"],
    unoptimized: true,
  },
  env: {
    CUSTOM_KEY: "my-value",
  },
};

module.exports = nextConfig;
