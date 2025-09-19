// InkTix Production Configuration
// ================================
// Domain: inktix.com (or your domain)
// Server: 135.148.61.99

const config = {
  // Network Configuration
  network: {
    nodeEnv: "production",
    port: 3000,
    host: "0.0.0.0",
  },

  // Substrate/Polkadot Configuration
  substrate: {
    rpcUrl: "wss://westend-rpc.polkadot.io",
    httpUrl: "https://westend-rpc.polkadot.io",
  },

  // Contract Configuration
  contracts: {
    sportsBroker: {
      address: "5CR7KXVKZ8tuNh7u3xY7tekt6s6HF2ZpemytdGrH5bt1jFbk",
      codeHash:
        "0xa120f8e17c5ba8eb3f58ca44807f71c8376bd031527cac86c07c7cd1a95d3679",
      metadataPath: "./contracts/sports_broker/target/ink/sports_broker.json",
      wasmPath: "./contracts/sports_broker/target/ink/sports_broker.wasm",
      bundlePath: "./contracts/sports_broker/target/ink/sports_broker.contract",
    },
    concertBroker: {
      address: "5EQcT6gpuQtTYfpy3ygbBC5UF9Y8rnCKMvuJ3NC7pCgtej4y",
      codeHash:
        "0x10849f5a49376fc29f9251a5b8062d03799192d4a5bcec18942620fbf3a7a23a",
      metadataPath: "./contracts/concert_broker/target/ink/concert_broker.json",
      wasmPath: "./contracts/concert_broker/target/ink/concert_broker.wasm",
      bundlePath:
        "./contracts/concert_broker/target/ink/concert_broker.contract",
    },
    inktixCore: {
      address: "5FN2wJEWQXus8k3wZdQM8Q1bmDquawNNGH97kAFr4WETF8fE",
      codeHash:
        "0xab088fbf68551186112bdf6876d0a81536804d8c14b1c2cfc47b07f1d2653678",
      metadataPath: "./contracts/inktix_core/target/ink/inktix_core.json",
      wasmPath: "./contracts/inktix_core/target/ink/inktix_core.wasm",
      bundlePath: "./contracts/inktix_core/target/ink/inktix_core.contract",
    },
  },

  // Account Configuration
  accounts: {
    production: {
      suri: "//YourProductionAccount",
      address: "YourProductionAddress",
    },
    defaultSuri: "//YourProductionAccount",
  },

  // Gas Configuration (Production Optimized)
  gas: {
    defaultLimit: 500000000000,
    defaultProofSize: 2000000,
    defaultStorageDepositLimit: 2000000000000000,
  },

  // Currency Configuration
  currencies: {
    supported: ["DOT", "ACA", "AUSD", "LDOT", "KSM"],
    rates: {
      DOT: 1000000000000,
      ACA: 50000000000,
      AUSD: 150000000000,
      LDOT: 950000000000,
      KSM: 15000000000000,
    },
  },

  // Database Configuration (Production)
  database: {
    url: "postgresql://inktix_user:your_secure_password@localhost:5432/inktix_production",
  },

  // Redis Configuration (Production)
  redis: {
    url: "redis://localhost:6379",
    password: "your_redis_password",
    db: 0,
  },

  // API Configuration (Production)
  api: {
    baseUrl: "https://135.148.61.99/api",
    version: "v1",
    corsOrigin: "https://135.148.61.99",
  },

  // Authentication Configuration (Production)
  auth: {
    jwtSecret: "your-super-secure-jwt-secret-key-for-production-change-this",
    jwtExpiresIn: "24h",
    sessionSecret:
      "your-super-secure-session-secret-key-for-production-change-this",
  },

  // File Upload Configuration (Production)
  upload: {
    maxFileSize: 52428800, // 50MB
    uploadPath: "/var/www/inktix.com/uploads",
    allowedFileTypes: [
      "image/jpeg",
      "image/png",
      "image/gif",
      "image/webp",
      "application/pdf",
    ],
  },

  // Logging Configuration (Production)
  logging: {
    level: "warn",
    format: "combined",
    file: "/var/log/inktix/inktix.log",
  },

  // Email Configuration (Production)
  email: {
    smtp: {
      host: "smtp.gmail.com",
      port: 587,
      user: "noreply@inktix.com",
      pass: "your-app-specific-password",
    },
    from: "noreply@inktix.com",
  },

  // WebSocket Configuration (Production)
  websocket: {
    port: 3001,
    path: "/ws",
  },

  // Frontend Configuration (Production)
  frontend: {
    url: "https://135.148.61.99",
    adminUrl: "https://135.148.61.99/admin",
  },

  // Contract Interaction Configuration (Production)
  contractInteraction: {
    transactionTimeout: 120000,
    maxRetryAttempts: 5,
  },

  // Feature Flags (Production)
  features: {
    sportsBroker: true,
    concertBroker: true,
    inktixCore: true,
    multiCurrency: true,
    crossChain: false,
    analytics: true,
    notifications: true,
  },

  // Production Configuration
  production: {
    debug: false,
    hotReload: false,
    mockContracts: false,
    enableContractLogs: false,
  },

  // SSL/TLS Configuration
  ssl: {
    certPath: "/etc/ssl/certs/inktix.com.crt",
    keyPath: "/etc/ssl/private/inktix.com.key",
    forceHttps: true,
  },

  // Security Configuration
  security: {
    rateLimitWindowMs: 900000,
    rateLimitMaxRequests: 100,
    helmetEnabled: true,
    cspEnabled: true,
  },

  // Monitoring Configuration
  monitoring: {
    enableMetrics: true,
    metricsPort: 9090,
    healthCheckEndpoint: "/health",
  },

  // Backup Configuration
  backup: {
    enabled: true,
    schedule: "0 2 * * *",
    retentionDays: 30,
    path: "/var/backups/inktix",
  },

  // CDN Configuration
  cdn: {
    enabled: true,
    url: "https://cdn.inktix.com",
    staticAssetsUrl: "https://cdn.inktix.com/assets",
  },

  // Cache Configuration
  cache: {
    ttl: 3600,
    maxSize: 1000,
    enabled: true,
  },

  // Performance Configuration
  performance: {
    compressionEnabled: true,
    gzipLevel: 6,
    clusterMode: true,
    workerProcesses: 4,
  },

  // Environment Specific
  environment: {
    env: "production",
    domain: "inktix.com", // Update with your domain
    serverIp: "135.148.61.99",
  },
};

module.exports = config;
