// InkTix Web Server Configuration
// =================================

const config = {
  // Network Configuration
  network: {
    nodeEnv: process.env.NODE_ENV || 'development',
    port: parseInt(process.env.PORT) || 3000,
    host: process.env.HOST || 'localhost',
  },

  // Substrate/Polkadot Configuration
  substrate: {
    rpcUrl: process.env.SUBSTRATE_RPC_URL || 'ws://127.0.0.1:9944',
    httpUrl: process.env.SUBSTRATE_HTTP_URL || 'http://127.0.0.1:9944',
  },

  // Contract Configuration
  contracts: {
    sportsBroker: {
      address: process.env.SPORTS_BROKER_CONTRACT_ADDRESS || '5CR7KXVKZ8tuNh7u3xY7tekt6s6HF2ZpemytdGrH5bt1jFbk',
      codeHash: process.env.SPORTS_BROKER_CODE_HASH || '0xa120f8e17c5ba8eb3f58ca44807f71c8376bd031527cac86c07c7cd1a95d3679',
      metadataPath: process.env.SPORTS_BROKER_METADATA_PATH || './contracts/sports_broker/target/ink/sports_broker.json',
      wasmPath: process.env.SPORTS_BROKER_WASM_PATH || './contracts/sports_broker/target/ink/sports_broker.wasm',
      bundlePath: process.env.SPORTS_BROKER_BUNDLE_PATH || './contracts/sports_broker/target/ink/sports_broker.contract',
    },
    concertBroker: {
      address: process.env.CONCERT_BROKER_CONTRACT_ADDRESS || '5EQcT6gpuQtTYfpy3ygbBC5UF9Y8rnCKMvuJ3NC7pCgtej4y',
      codeHash: process.env.CONCERT_BROKER_CODE_HASH || '0x10849f5a49376fc29f9251a5b8062d03799192d4a5bcec18942620fbf3a7a23a',
      metadataPath: process.env.CONCERT_BROKER_METADATA_PATH || './contracts/concert_broker/target/ink/concert_broker.json',
      wasmPath: process.env.CONCERT_BROKER_WASM_PATH || './contracts/concert_broker/target/ink/concert_broker.wasm',
      bundlePath: process.env.CONCERT_BROKER_BUNDLE_PATH || './contracts/concert_broker/target/ink/concert_broker.contract',
    },
    inktixCore: {
      address: process.env.INKTIX_CORE_CONTRACT_ADDRESS || '5FN2wJEWQXus8k3wZdQM8Q1bmDquawNNGH97kAFr4WETF8fE',
      codeHash: process.env.INKTIX_CORE_CODE_HASH || '0xab088fbf68551186112bdf6876d0a81536804d8c14b1c2cfc47b07f1d2653678',
      metadataPath: process.env.INKTIX_CORE_METADATA_PATH || './contracts/inktix_core/target/ink/inktix_core.json',
      wasmPath: process.env.INKTIX_CORE_WASM_PATH || './contracts/inktix_core/target/ink/inktix_core.wasm',
      bundlePath: process.env.INKTIX_CORE_BUNDLE_PATH || './contracts/inktix_core/target/ink/inktix_core.contract',
    },
  },

  // Account Configuration
  accounts: {
    alice: {
      suri: process.env.ALICE_SURI || '//Alice',
      address: process.env.ALICE_ADDRESS || '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY',
    },
    defaultSuri: process.env.DEFAULT_ACCOUNT_SURI || '//Alice',
  },

  // Gas Configuration
  gas: {
    defaultLimit: parseInt(process.env.DEFAULT_GAS_LIMIT) || 200000000000,
    defaultProofSize: parseInt(process.env.DEFAULT_PROOF_SIZE) || 1000000,
    defaultStorageDepositLimit: parseInt(process.env.DEFAULT_STORAGE_DEPOSIT_LIMIT) || 1000000000000000,
  },

  // Currency Configuration
  currencies: {
    supported: (process.env.SUPPORTED_CURRENCIES || 'DOT,ACA,AUSD,LDOT,KSM').split(','),
    rates: {
      DOT: parseInt(process.env.CURRENCY_RATES_DOT) || 1000000000000,
      ACA: parseInt(process.env.CURRENCY_RATES_ACA) || 50000000000,
      AUSD: parseInt(process.env.CURRENCY_RATES_AUSD) || 150000000000,
      LDOT: parseInt(process.env.CURRENCY_RATES_LDOT) || 950000000000,
      KSM: parseInt(process.env.CURRENCY_RATES_KSM) || 15000000000000,
    },
  },

  // Database Configuration
  database: {
    url: process.env.DATABASE_URL || 'sqlite:./inktix.db',
  },

  // Redis Configuration
  redis: {
    url: process.env.REDIS_URL || 'redis://localhost:6379',
    password: process.env.REDIS_PASSWORD || '',
    db: parseInt(process.env.REDIS_DB) || 0,
  },

  // API Configuration
  api: {
    baseUrl: process.env.API_BASE_URL || 'http://localhost:3000/api',
    version: process.env.API_VERSION || 'v1',
    corsOrigin: process.env.CORS_ORIGIN || 'http://localhost:3000',
  },

  // Authentication Configuration
  auth: {
    jwtSecret: process.env.JWT_SECRET || 'your-super-secret-jwt-key-change-in-production',
    jwtExpiresIn: process.env.JWT_EXPIRES_IN || '24h',
    sessionSecret: process.env.SESSION_SECRET || 'your-super-secret-session-key-change-in-production',
  },

  // File Upload Configuration
  upload: {
    maxFileSize: parseInt(process.env.MAX_FILE_SIZE) || 10485760, // 10MB
    uploadPath: process.env.UPLOAD_PATH || './uploads',
    allowedFileTypes: (process.env.ALLOWED_FILE_TYPES || 'image/jpeg,image/png,image/gif,image/webp').split(','),
  },

  // Logging Configuration
  logging: {
    level: process.env.LOG_LEVEL || 'info',
    format: process.env.LOG_FORMAT || 'combined',
    file: process.env.LOG_FILE || './logs/inktix.log',
  },

  // Email Configuration
  email: {
    smtp: {
      host: process.env.SMTP_HOST || 'smtp.gmail.com',
      port: parseInt(process.env.SMTP_PORT) || 587,
      user: process.env.SMTP_USER || 'your-email@gmail.com',
      pass: process.env.SMTP_PASS || 'your-app-password',
    },
    from: process.env.SMTP_FROM || 'noreply@inktix.com',
  },

  // WebSocket Configuration
  websocket: {
    port: parseInt(process.env.WS_PORT) || 3001,
    path: process.env.WS_PATH || '/ws',
  },

  // Frontend Configuration
  frontend: {
    url: process.env.FRONTEND_URL || 'http://localhost:3000',
    adminUrl: process.env.ADMIN_URL || 'http://localhost:3000/admin',
  },

  // Contract Interaction Configuration
  contractInteraction: {
    transactionTimeout: parseInt(process.env.TRANSACTION_TIMEOUT) || 60000,
    maxRetryAttempts: parseInt(process.env.MAX_RETRY_ATTEMPTS) || 3,
  },

  // Feature Flags
  features: {
    sportsBroker: process.env.ENABLE_SPORTS_BROKER === 'true',
    concertBroker: process.env.ENABLE_CONCERT_BROKER === 'true',
    inktixCore: process.env.ENABLE_INKTIX_CORE === 'true',
    multiCurrency: process.env.ENABLE_MULTI_CURRENCY === 'true',
    crossChain: process.env.ENABLE_CROSS_CHAIN === 'true',
    analytics: process.env.ENABLE_ANALYTICS === 'true',
    notifications: process.env.ENABLE_NOTIFICATIONS === 'true',
  },

  // Development Configuration
  development: {
    debug: process.env.DEBUG === 'true',
    hotReload: process.env.HOT_RELOAD === 'true',
    mockContracts: process.env.MOCK_CONTRACTS === 'true',
    enableContractLogs: process.env.ENABLE_CONTRACT_LOGS === 'true',
  },
};

module.exports = config;
