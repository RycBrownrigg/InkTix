# InkTix: Cross-Chain Ticket Marketplace Network

<p align="center">
  <img src="./InkTix_logo.png" alt="InkTix Logo" width="350">
</p>

## 2. System Architecture

### 2.1 High-Level Network Architecture

```
                    ┌─────────────────────────────────────┐
                    │        Polkadot Relay Chain         │
                    │     (Cross-Chain Governance)        │
                    └─────────────────┬───────────────────┘
                                      │ XCM Message Router
                    ┌─────────────────┼─────────────────────┐
                    │                 │                     │
        ┌───────────▼──┐    ┌─────────▼─────────┐    ┌──────▼─────┐
        │   Hub Chain  │    │     Acala         │    │ ConcertCh. │
        │ (Moonbeam)   │    │  Multi-Service    │    │  (Astar)   │
        │              │    │   Parachain       │    │            │
        │┌────────────┐│    │ ┌───────────────┐ │    │ ┌────────┐ │
        ││ Discovery  ││    │ │ SportChain +  │ │    │ │Concert │ │
        ││   Engine   ││    │ │ CultureChain  │ │    │ │Market- │ │
        ││ XCM Router ││    │ │   (ink!)      │ │    │ │place   │ │
        │└────────────┘│    │ └───────────────┘ │    │ └────────┘ │
        │              │    │                   │    │            │
        │ • Cross-Chain│    │ • Sports Events   │    │• Concerts  │
        │   Discovery  │    │ • Cultural Arts   │    │• Festivals │
        │ • Escrow Hub │    │ • DeFi Integration│    │• Fan Tokens│
        └──────────────┘    └───────────────────┘    └────────────┘
                    │                 │                   │
                    └─────────────────┼───────────────────┘
                                      │
                            ┌─────────▼─────────┐
                            │   LocalChain      │
                            │  (Custom Sub.)    │
                            │                   │
                            │┌─────────────────┐│
                            ││Community Events ││
                            ││  Marketplace    ││
                            │└─────────────────┘│
                            │                   │
                            │ • Local Venues    │
                            │ • Community Fairs │
                            │ • Small Concerts  │
                            └───────────────────┘
```

### 2.2 Detailed System Architecture

#### 2.2.1 Infrastructure Layers

**Layer 1: Polkadot Relay Chain**
```
┌─────────────────────────────────────────────────────────────┐
│                 Polkadot Relay Chain                        │
│                                                             │
│  • Cross-chain message validation and routing               │
│  • Shared security for all parachains                       │
│  • Governance coordination                                  │
│  • Validator set management                                 │
│  • DOT token economics                                      │
└─────────────────────────────────────────────────────────────┘
```

**Layer 2: XCM Communication Layer**
```
┌──────────────────────────────────────────────────────┐
│                   XCM Message Bus                    │
│                                                      │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐   │
│  │   XCMP      │  │    VMP      │  │   HRMP      │   │
│  │ (Parachain  │  │ (Vertical   │  │(Horizontal  │   │
│  │    to       │  │  Message    │  │  Relay      │   │
│  │ Parachain)  │  │  Passing)   │  │  Message    │   │
│  └─────────────┘  └─────────────┘  │  Passing)   │   │
│                                    └─────────────┘   │
│  • Message queuing and prioritization                │
│  • Fee calculation and payment                       │
│  • Message lifecycle management                      │
│  • Error handling and retry logic                    │
└──────────────────────────────────────────────────────┘
```

**Layer 3: Parachain Application Layer**
```
┌──────────────┐ ┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│ SportChain   │ │ ConcertChain │ │ CultureChain │ │ LocalChain   │
│ (Moonbeam)   │ │   (Astar)    │ │   (Acala)    │ │  (Custom)    │
│              │ │              │ │              │ │              │
│ EVM Runtime  │ │ WASM Runtime │ │Native Runtime│ │Custom Runtime│
│ Solidity     │ │AssemblyScript│ │    ink!      │ │    Rust      │
│ Contracts    │ │  Contracts   │ │  Contracts   │ │   Pallets    │
└──────────────┘ └──────────────┘ └──────────────┘ └──────────────┘
```

#### 2.2.2 Central Hub Components (Moonbeam)

**Discovery Engine Architecture**
```rust
// Core discovery service written in Rust
pub struct DiscoveryEngine {
    // Cross-chain event indexer
    event_indexer: CrossChainIndexer,
    // AI recommendation engine
    recommendation_ai: RecommendationEngine,
    // Real-time price monitoring
    price_monitor: PriceMonitor,
    // User behavior analytics
    user_analytics: UserAnalytics,
    // Search optimization
    search_engine: SearchEngine,
}

pub struct CrossChainIndexer {
    // Connections to all broker chains
    chain_connections: HashMap<ChainId, ChainConnection>,
    // Event cache with TTL
    event_cache: LRUCache<EventId, CachedEvent>,
    // Real-time update subscriptions
    update_subscriptions: Vec<EventSubscription>,
}
```

**XCM Message Router**
```rust
pub struct XCMRouter {
    // Message queue management
    message_queues: HashMap<ChainId, MessageQueue>,
    // Route optimization engine
    route_optimizer: RouteOptimizer,
    // Fee estimation service
    fee_estimator: FeeEstimator,
    // Retry and error handling
    retry_manager: RetryManager,
    // Message lifecycle tracking
    message_tracker: MessageTracker,
}

pub struct RouteOptimizer {
    // Network topology mapping
    topology: NetworkTopology,
    // Historical performance data
    performance_metrics: PerformanceTracker,
    // Cost analysis engine
    cost_analyzer: CostAnalyzer,
}
```

**Cross-Chain Escrow System**
```rust
pub struct EscrowManager {
    // Active escrow contracts
    active_escrows: HashMap<EscrowId, EscrowContract>,
    // Multi-signature coordination
    multisig_coordinator: MultisigCoordinator,
    // Automated settlement triggers
    settlement_engine: SettlementEngine,
    // Dispute resolution system
    dispute_resolver: DisputeResolver,
}
```

#### 2.2.3 Specialized Broker Architecture

**Hub Chain (Moonbeam EVM + Rust Services)**
```
┌───────────────────────────────────────────────────┐
│               InkTix Hub Architecture             │
├───────────────────────────────────────────────────┤
│       Frontend Layer (React + Polkadot.js)        │
├───────────────────────────────────────────────────┤
│              API Layer (Rust Actix-Web)           │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐  │
│  │ Discovery   │ │    XCM      │ │   Escrow    │  │
│  │   Engine    │ │   Router    │ │ Management  │  │
│  │             │ │             │ │             │  │
│  └─────────────┘ └─────────────┘ └─────────────┘  │
├───────────────────────────────────────────────────┤
│  Smart Contract Layer (Solidity on Moonbeam EVM)  │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐  │
│  │  Discovery  │ │    XCM      │ │ Cross-Chain │  │
│  │     Hub     │ │ Message Hub │ │   Escrow    │  │
│  │  Contract   │ │  Contract   │ │  Contract   │  │
│  └─────────────┘ └─────────────┘ └─────────────┘  │
├───────────────────────────────────────────────────┤
│              Substrate Runtime (Moonbeam)         │
└───────────────────────────────────────────────────┘
```

**Acala Multi-Service Parachain (ink! + Native DeFi)**
```
┌───────────────────────────────────────────────────┐
│               Acala InkTix Services               │
├───────────────────────────────────────────────────┤
│         Frontend Layer (React + Polkadot.js)      │
├───────────────────────────────────────────────────┤
│           API Layer (Rust Warp Framework)         │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐  │
│  │   Sports    │ │  Cultural   │ │    DeFi     │  │
│  │   Events    │ │   Events    │ │ Integration │  │
│  │  Service    │ │   Service   │ │   Service   │  │
│  └─────────────┘ └─────────────┘ └─────────────┘  │
├───────────────────────────────────────────────────┤
│         Smart Contract Layer (ink! - Rust)        │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐  │
│  │   Sports    │ │   Culture   │ │    DeFi     │  │
│  │ Marketplace │ │ Marketplace │ │  Payment    │  │
│  │  (ink!)     │ │   (ink!)    │ │ Processor   │  │
│  └─────────────┘ └─────────────┘ └─────────────┘  │
├───────────────────────────────────────────────────┤
│            Native Acala Integration (Rust)        │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐  │
│  │   Acala     │ │   Liquid    │ │   Acala     │  │
│  │    DEX      │ │   Staking   │ │   Lending   │  │
│  └─────────────┘ └─────────────┘ └─────────────┘  │
├───────────────────────────────────────────────────┤
│   Substrate Runtime (Acala with Custom Pallets)   │
└───────────────────────────────────────────────────┘
```

**ConcertChain (Astar WASM + Rust Services)**
```
┌───────────────────────────────────────────────────┐
│              ConcertChain Architecture            │
├───────────────────────────────────────────────────┤
│        Frontend Layer (React + Polkadot.js)       │
├───────────────────────────────────────────────────┤
│           API Layer (Rust Axum Framework)         │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐  │
│  │   Music     │ │   Artist    │ │   Fan Token │  │
│  │   Metadata  │ │ Management  │ │   Service   │  │
│  │  Service    │ │   Service   │ │             │  │
│  └─────────────┘ └─────────────┘ └─────────────┘  │
├───────────────────────────────────────────────────┤
│    Smart Contract Layer (AssemblyScript WASM)     │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐  │
│  │   Concert   │ │   VIP       │ │  Merchandise│  │
│  │ Marketplace │ │ Experience  │ │   Bundle    │  │
│  │  Contract   │ │  Contract   │ │  Contract   │  │
│  └─────────────┘ └─────────────┘ └─────────────┘  │
├───────────────────────────────────────────────────┤
│            XCM Integration Layer (Rust)           │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐  │
│  │    XCM      │ │   WASM      │ │  Cross-Chain│  │
│  │  Handler    │ │  Interface  │ │    Events   │  │
│  └─────────────┘ └─────────────┘ └─────────────┘  │
├───────────────────────────────────────────────────┤
│              Substrate Runtime (Astar)            │
└───────────────────────────────────────────────────┘
```

**CultureChain (Acala Native + Full Rust Stack)**
```
┌───────────────────────────────────────────────────┐
│             CultureChain Architecture             │
├───────────────────────────────────────────────────┤
│        Frontend Layer (React + Polkadot.js)       │
├───────────────────────────────────────────────────┤
│           API Layer (Rust Warp Framework)         │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐  │
│  │  Cultural   │ │ Institution │ │   Grant     │  │
│  │   Events    │ │ Partnership │ │ Management  │  │
│  │  Service    │ │   Service   │ │   Service   │  │
│  └─────────────┘ └─────────────┘ └─────────────┘  │
├───────────────────────────────────────────────────┤
│       Smart Contract Layer (ink! - Pure Rust)     │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐  │
│  │   Culture   │ │ Subscription│ │   Patron    │  │
│  │ Marketplace │ │   Season    │ │   Program   │  │
│  │  Contract   │ │  Contract   │ │  Contract   │  │
│  └─────────────┘ └─────────────┘ └─────────────┘  │
├───────────────────────────────────────────────────┤
│          Runtime Integration Layer (Rust)         │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐  │
│  │   Custom    │ │    XCM      │ │  Cross-Chain│  │
│  │   Pallets   │ │ Integration │ │  Governance │  │
│  └─────────────┘ └─────────────┘ └─────────────┘  │
├───────────────────────────────────────────────────┤
│    Substrate Runtime (Acala with Custom Pallets)  │
└───────────────────────────────────────────────────┘
```

#### 2.2.4 Data Flow Architecture

**Cross-Chain Search Flow**
```
                 User Search Request
                         ↓
             Discovery Engine (Moonbeam)
                         ↓
                XCM Message Builder
                         ↓
┌─────────────┐   ┌─────────────┐   ┌─────────────┐
│ SportChain  │   │ConcertChain │   │CultureChain │
│   Search    │   │   Search    │   │   Search    │
└─────────────┘   └─────────────┘   └─────────────┘
         ↓              ↓                  ↓
         Local Event Database Search & Filter
         ↓              ↓                  ↓
          XCM Response Messages with Results
         ↓              ↓                  ↓
        ┌───────────────┼──────────────────┐
        ↓               ↓                  ↓
        Discovery Engine Result Aggregation
                        ↓
          Result Ranking & Personalization
                        ↓
             Formatted Response to User
```

**Cross-Chain Purchase Flow**
```
User Purchase Intent
        ↓
Hub Escrow Creation (Moonbeam)
        ↓
XCM Purchase Message to Broker Chain
        ↓
Broker Chain Ticket Availability Check
        ↓
Dynamic Price Calculation
        ↓
NFT Ticket Minting (if successful)
        ↓
XCM Success/Failure Response
        ↓
Hub Escrow Release/Refund
        ↓
Cross-Chain Reputation Update
```

### 2.3 Data Flow Architecture

```
User Query → Discovery Engine → Cross-Chain Search → Broker Response
     ↓              ↓                 ↓                    ↓
Price Alert ← Recommendation ← Availability Check ← Inventory Update
     ↓              ↓                 ↓                    ↓
Purchase Intent → Escrow Creation → XCM Transfer → Ticket Mint
     ↓              ↓                 ↓                    ↓
Payment Confirm ← Escrow Release ← Ownership Transfer ← NFT Creation
```

### 2.4 Technology Stack

#### 2.4.1 Smart Contract Languages & Runtimes

**Rust/ink! Stack**
- **ink!** (Moonbeam Hub): Discovery engine, XCM routing, escrow management
- **ink!** (Acala): SportChain and CultureChain with DeFi integration
- **Rust WASM** (Astar): ConcertChain optimized for music marketplace
- **Custom Substrate Pallets**: LocalChain runtime with community governance

**Why ink! for InkTix:**
```rust
// Memory safety without garbage collection
// Direct Substrate/XCM integration  
// Significantly lower gas costs
// Rust's strong type system prevents common smart contract bugs

#[ink::contract]
mod inktix_core {
    #[ink(storage)]
    pub struct InkTixCore {
        // Native Substrate types
        events: Mapping<u32, Event>,
        // Direct XCM integration
        xcm_router: XCMRouter,
    }
    
    #[ink(message)]
    pub fn cross_chain_purchase(&mut self) -> Result<u64, Error> {
        // Native XCM message construction
        // No EVM overhead or gas inefficiency
    }
}
```

#### 2.4.2 Acala Integration Advantages

**Why Acala for Sports & Cultural Events:**
```rust
// Native DeFi integration for ticket payments
use acala_primitives::{CurrencyId, TradingPair};

impl SportsMarketplace {
    fn process_multi_currency_payment(
        &self,
        buyer: AccountId,
        amount: Balance,
        currency: CurrencyId
    ) -> Result<Balance, Error> {
        // Accept DOT, ACA, aUSD, LDOT, etc.
        match currency {
            CurrencyId::ACA => self.process_aca_payment(buyer, amount),
            CurrencyId::AUSD => self.process_ausd_payment(buyer, amount),
            CurrencyId::DOT => self.process_dot_payment(buyer, amount),
            CurrencyId::LDOT => self.process_liquid_dot_payment(buyer, amount),
            _ => Err(Error::UnsupportedCurrency)
        }
    }
    
    fn enable_staking_rewards(&mut self, user: AccountId, team_id: u32) {
        // Integrate with Acala liquid staking for fan loyalty rewards
        // Users earn staking rewards on their ticket purchases
    }
}
```

**DeFi-Enhanced Ticketing Features:**
- **Multi-Currency Payments**: Accept DOT, ACA, aUSD, LDOT
- **Liquid Staking Integration**: Earn staking rewards on season passes
- **DEX Integration**: Automatic currency conversion for payments
- **Lending Integration**: Buy-now-pay-later for expensive tickets
- **Yield Generation**: Ticket escrow funds earn yield while processing

#### 2.4.3 Cross-Chain Infrastructure

**XCM Integration (All Rust)**
- XCM v3 message construction and parsing optimized for ink! contracts
- XCMP for parachain-to-parachain communication  
- Custom XCM instruction sequences for complex workflows
- Rust-based XCM simulators for testing

**Substrate Runtime Development**
- Custom pallets for specialized functionality
- Runtime APIs for cross-chain queries
- Event emission and subscription systems
- Off-chain workers for automated tasks

#### 2.4.4 Off-Chain Services (Rust Ecosystem)

**Backend Services**
```rust
// All backend services written in Rust
use actix_web::web;     // SportChain API
use axum::Router;       // ConcertChain API  
use warp::Filter;       // CultureChain API
use tokio;              // Async runtime
use serde;              // Serialization
use sqlx;               // Database access
```

**Blockchain Integration**
```rust
use subxt;              // Substrate client
use sp_core;            // Substrate primitives
use parity_scale_codec; // SCALE codec
use polkadot_primitives; // Polkadot types
```

**Infrastructure Components**
- **IPFS Integration**: Rust IPFS client for metadata storage
- **Oracle Services**: Rust-based price and event data oracles  
- **Indexing**: The Graph Protocol with Rust subgraph development
- **AI/ML**: Candle.rs for recommendation engine development

#### 2.4.5 Complete Rust Technology Stack

**Frontend Integration**
```rust
// WASM frontend components
use yew;                // Rust frontend framework
use web_sys;            // Web APIs
use js_sys;             // JavaScript integration
use wasm_bindgen;       // JS/WASM bindings
```

**Development Tools**
```rust
use cargo_contract;     // ink! contract development
use substrate_contracts_node; // Local testing node
use xcm_simulator;      // XCM testing framework
```
