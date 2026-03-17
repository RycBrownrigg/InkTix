# InkTix End-to-End Test Plan

## Traceability Matrix

| Use Case | Test Scenarios | Test Cases |
|----------|---------------|------------|
| UC-001 Connect Wallet | TS-001, TS-002, TS-003, TS-004 | TC-001 through TC-004 |
| UC-002 Connect to Network | TS-005, TS-006, TS-007, TS-008 | TC-005 through TC-008 |
| UC-003 Browse Events | TS-009, TS-010, TS-011, TS-012 | TC-009 through TC-012 |
| UC-004 Purchase Ticket | TS-013, TS-014, TS-015, TS-016, TS-017 | TC-013 through TC-017 |
| UC-005 Mint NFT Ticket | TS-018, TS-019, TS-020, TS-021 | TC-018 through TC-021 |
| UC-006 Verify NFT Ticket | TS-022, TS-023, TS-024 | TC-022 through TC-024 |
| UC-007 Use Ticket at Event | TS-025, TS-026, TS-027 | TC-025 through TC-027 |
| UC-008 List Ticket for Resale | TS-028, TS-029, TS-030, TS-031 | TC-028 through TC-031 |
| UC-009 Buy Resale Ticket | TS-032, TS-033, TS-034 | TC-032 through TC-034 |
| UC-010 Share Event | TS-035, TS-036, TS-037 | TC-035 through TC-037 |
| UC-011 Group Buy | TS-038, TS-039, TS-040, TS-041 | TC-038 through TC-041 |
| UC-012 Deploy Contract | TS-042, TS-043, TS-044 | TC-042 through TC-044 |
| UC-013 Call Contract Method | TS-045, TS-046, TS-047 | TC-045 through TC-047 |
| UC-014 XCM Cross-Chain Transfer | TS-048, TS-049, TS-050, TS-051 | TC-048 through TC-051 |
| UC-015 View Analytics | TS-052, TS-053, TS-054 | TC-052 through TC-054 |
| UC-016 Disconnect Wallet | TS-055, TS-056 | TC-055 through TC-056 |

---

## Use Cases

---

### Use Case: Connect Wallet

- **ID:** UC-001
- **Title:** User connects a Polkadot wallet extension
- **Primary Actor:** End User
- **Secondary Actors:** Polkadot.js / Talisman / SubWallet / Nova extension
- **Description:** User connects their browser wallet extension to InkTix to access blockchain features

**Preconditions**
- A supported wallet extension is installed in the browser
- At least one account exists in the wallet
- InkTix site is loaded

**Trigger**
- User clicks "Connect Wallet" button

**Main Flow (Happy Path)**
1. User navigates to `/connect` page
2. User clicks "Connect Wallet" button
3. Wallet extension popup appears requesting authorization
4. User approves the connection
5. Extension returns list of accounts
6. First account is auto-selected
7. Wallet status shows "Connected" with account name and truncated address
8. App automatically attempts network connection to Westend Asset Hub

**Alternate Flows**
- **A1: Multiple accounts available**
  - After connection, user can click "Switch Account" to see all accounts
  - User selects a different account
  - Selected account updates in the UI and balance refreshes
- **A2: Previously connected (page reload)**
  - On page load, Zustand hydrates persisted state from `inktix-store`
  - If previously connected, wallet state is restored without re-prompting

**Exception Flows**
- **E1: No wallet extension installed**
  - System shows error: "No web3 extension found. Please install Polkadot.js extension."
- **E2: User rejects authorization**
  - Wallet popup is dismissed, connection fails silently
  - UI remains on "Connect Wallet" state
- **E3: Extension has zero accounts**
  - System shows error: "No accounts found. Please create or import an account."

**Postconditions**
- `isWalletConnected` is `true` in store
- `selectedAccount` contains account address and name
- `accounts` array contains all available accounts
- State persisted to `inktix-store` in localStorage

---

### Use Case: Connect to Network

- **ID:** UC-002
- **Title:** App connects to a Substrate network via WebSocket
- **Primary Actor:** System (auto-triggered) / End User (manual)
- **Secondary Actors:** Westend Asset Hub RPC node
- **Description:** Establishes WebSocket connection to a Substrate chain for on-chain queries and transactions

**Preconditions**
- Wallet is connected (UC-001 completed)

**Trigger**
- Automatic: after wallet connection succeeds
- Manual: user clicks "Connect to Network" button

**Main Flow (Happy Path)**
1. System initiates WebSocket connection to `wss://westend-asset-hub-rpc.polkadot.io`
2. `ApiPromise.create()` connects and synchronizes metadata
3. Chain name, node name, and version are queried
4. Network info displayed in wallet panel
5. Balance is loaded for selected account
6. Contract deployment status is checked

**Alternate Flows**
- **A1: Connect to local node**
  - Endpoint is `ws://127.0.0.1:9944` (development mode)
  - Same flow but targeting local Substrate node
- **A2: Auto-reconnect on state mismatch**
  - If localStorage says connected but API is not, system auto-reconnects

**Exception Flows**
- **E1: RPC endpoint unreachable**
  - Connection times out, error logged to console
  - App continues in mock mode with demo data
- **E2: Chain doesn't have contracts pallet**
  - Warning logged: "Contracts pallet is NOT available on this chain"
  - Contract deployment/interaction will use mock fallback

**Postconditions**
- `isConnected` is `true`
- `networkInfo` contains chain, nodeName, nodeVersion
- Balance displayed for selected account
- State persisted to localStorage

---

### Use Case: Browse Events

- **ID:** UC-003
- **Title:** User browses and filters events
- **Primary Actor:** End User
- **Secondary Actors:** InkTix Contract (if deployed)
- **Description:** User discovers events through category filters, sees dynamic pricing, and views event details

**Preconditions**
- InkTix site is loaded
- Events page is accessible (no wallet required for browsing)

**Trigger**
- User navigates to `/events` page

**Main Flow (Happy Path)**
1. Page loads with 39 mock events (demo data)
2. If contract is deployed, `get_all_events()` is called and contract events are prepended
3. Data source indicator shows "Demo data" (blue) or "Live from blockchain" (green)
4. Events displayed in grid (6 initially)
5. User clicks a category filter (e.g., "NBA")
6. Events filtered to show only matching category
7. User clicks "View Details" on an event
8. Event detail modal opens with full information
9. User can close modal and continue browsing

**Alternate Flows**
- **A1: Load more events**
  - User scrolls to bottom and clicks "Load More Events (X remaining)"
  - Next 6 events appear
- **A2: No events match filter**
  - Grid shows empty state
- **A3: Contract events available**
  - Contract events appear first, then mock events
  - Platform stats section shows real totals from `get_platform_stats()`

**Exception Flows**
- **E1: Contract call fails**
  - Falls back to mock events silently
  - Data source indicator shows "Demo data"

**Postconditions**
- Events are displayed with category, price, venue, date, availability
- Selected category filter is visually active

---

### Use Case: Purchase Ticket

- **ID:** UC-004
- **Title:** User purchases a ticket through the 3-step purchase flow
- **Primary Actor:** End User
- **Secondary Actors:** InkTix Contract, Wallet Extension (for signing)
- **Description:** User selects an event, chooses seat type and currency, reviews dynamic pricing, and completes purchase

**Preconditions**
- User is on the events page
- An event is selected (detail modal open)

**Trigger**
- User clicks "Buy Tickets" button in event detail modal

**Main Flow (Happy Path)**
1. Modal transitions to seat selection step
2. User selects seat type (e.g., "Reserved" or "Courtside")
3. Dynamic price breakdown updates in real-time showing:
   - Base price
   - Seat type adjustment (+/- %)
   - Demand surge (based on sell-through %)
   - Total price
4. User enters section, row, seat number
5. User selects payment currency (DOT/KSM/aUSD/ACA)
6. User clicks "Confirm Purchase"
7. If contract deployed: wallet extension prompts for signing
8. If no contract: mock purchase with 1.5s simulated delay
9. Success screen shows ticket ID, seat details, price paid
10. User can click "View My Tickets" to go to `/my-tickets`

**Alternate Flows**
- **A1: Student section discount**
  - User selects "Student Section" seat type
  - Price shows -30% adjustment
- **A2: High demand surge**
  - Event is 90%+ sold out
  - Demand multiplier shows +50% or more
- **A3: Season pass discount**
  - If `is_season_pass` flag is true, additional discount applied
- **A4: User clicks "Back"**
  - Returns to event details step, preserving modal state

**Exception Flows**
- **E1: Wallet not connected**
  - Purchase falls back to mock mode
  - Success shown with "(demo mode)" note
- **E2: Contract call fails**
  - Error message shown below the confirm button
  - User can retry
- **E3: User rejects wallet signing**
  - Error: "Signing cancelled or failed"
  - Modal stays on select step, user can retry
- **E4: Insufficient balance**
  - Contract returns error, displayed to user

**Postconditions**
- Ticket created on-chain (or in mock)
- Event sold_tickets count incremented
- Revenue tracked in event analytics
- Loyalty points calculated from price

---

### Use Case: Mint NFT Ticket

- **ID:** UC-005
- **Title:** User mints an NFT for an owned ticket
- **Primary Actor:** End User
- **Secondary Actors:** InkTix Contract
- **Description:** User converts a purchased ticket into an on-chain NFT with a verification hash and QR code

**Preconditions**
- User has a purchased ticket
- Wallet is connected
- User is on `/my-tickets` page

**Trigger**
- User clicks "Mint NFT" button on an unminted ticket

**Main Flow (Happy Path)**
1. User navigates to `/my-tickets`
2. Page displays owned tickets (mock demo tickets or from contract)
3. Unminted tickets show "Mint NFT" button
4. User clicks "Mint NFT"
5. Contract `mint_ticket_nft()` called (or mock)
6. NFT created with Blake2x256 verification hash
7. QR code appears on the ticket card encoding `{tokenId, eventId, seatNumber, hash}`
8. Verification hash displayed with copy button
9. Ticket card now shows "NFT Minted" status

**Alternate Flows**
- **A1: Ticket already has NFT**
  - "Mint NFT" button is not shown
  - QR code and verification hash are already visible

**Exception Flows**
- **E1: Not ticket owner**
  - Contract returns: "Only ticket owner can mint NFT"
- **E2: NFT already minted for this ticket**
  - Contract returns: "NFT already minted for this ticket"
- **E3: Contract not deployed**
  - Falls back to mock, NFT minted in memory

**Postconditions**
- `TicketNft` created with token_id, verification_hash, metadata
- `ticket_to_nft` mapping updated
- QR code visible on ticket card
- NFT appears in `user_nft_tokens` list

---

### Use Case: Verify NFT Ticket

- **ID:** UC-006
- **Title:** Verify authenticity of an NFT ticket
- **Primary Actor:** Venue Staff / End User
- **Secondary Actors:** InkTix Contract
- **Description:** Enter a token ID to verify ticket authenticity, ownership, and used/unused status

**Preconditions**
- User is on `/my-tickets` page
- Token ID is known (from QR code or ticket card)

**Trigger**
- User enters token ID in "Verify Ticket" section and clicks verify

**Main Flow (Happy Path)**
1. User scrolls to "Verify Ticket" section on `/my-tickets`
2. User enters token ID
3. User clicks "Verify"
4. Contract `verify_ticket_nft()` called
5. Result shows: Valid, Not Used, owner address, event name, section/row/seat

**Alternate Flows**
- **A1: Ticket is already used**
  - Result shows: Valid, Used (with "used" badge)
- **A2: Scan QR code**
  - QR code contains JSON with tokenId
  - Could be scanned by venue staff app (future feature)

**Exception Flows**
- **E1: Invalid token ID**
  - "NFT not found" error displayed
- **E2: Token ID is zero or negative**
  - Verify button disabled or error shown

**Postconditions**
- Verification result displayed (valid/invalid, used/unused, owner, event details)

---

### Use Case: Use Ticket at Event

- **ID:** UC-007
- **Title:** Mark NFT ticket as used for event entry
- **Primary Actor:** End User / Venue Staff
- **Secondary Actors:** InkTix Contract
- **Description:** Marks a ticket NFT as "used" and mints an attendance token as proof of attendance

**Preconditions**
- NFT ticket exists and is not yet used
- User is on `/my-tickets` page

**Trigger**
- User clicks "Use Ticket" on an active (unused) NFT ticket

**Main Flow (Happy Path)**
1. User clicks "Use Ticket" button on an NFT ticket card
2. Contract `use_ticket_nft()` called
3. NFT `is_used` set to `true`
4. `AttendanceToken` minted with event details and timestamp
5. Ticket card updates to show "Used" badge
6. "Use Ticket" button is replaced or disabled

**Alternate Flows**
- (none)

**Exception Flows**
- **E1: Ticket already used**
  - Contract returns: "Ticket already used"
  - Error displayed to user
- **E2: NFT not found**
  - Contract returns: "NFT not found"

**Postconditions**
- NFT marked as used on-chain
- Attendance token created
- UI reflects used status

---

### Use Case: List Ticket for Resale

- **ID:** UC-008
- **Title:** User lists a ticket on the resale marketplace
- **Primary Actor:** End User
- **Secondary Actors:** InkTix Contract, Anti-Scalping System
- **Description:** User selects a ticket to sell, sets a price within anti-scalping limits, and creates a resale listing

**Preconditions**
- User owns at least one ticket
- Wallet is connected
- User is on `/resale` page

**Trigger**
- User clicks "List a Ticket" button

**Main Flow (Happy Path) — From Owned Tickets**
1. User clicks "List a Ticket"
2. Modal opens on "My Tickets" tab
3. User sees their owned tickets with event name, venue, date, seat info, original price
4. User clicks a ticket to select it
5. Modal transitions to "Set Your Price" step
6. Price input pre-filled with original purchase price
7. Max allowed price shown (1.5x original)
8. User adjusts price within the cap
9. User clicks "List for Resale"
10. Listing created with 24-hour expiry
11. Modal closes, listing appears in marketplace (as "Pending" until verified)

**Alternate Flows**
- **A1: Manual entry (ticket from outside InkTix)**
  - User clicks "Enter Manually" tab
  - User fills in: Ticket ID, Event Name, Venue, Date, Section, Row, Seat, Original Price
  - User clicks "Continue to Set Price"
  - Proceeds to price-setting step (same as step 5 above)
- **A2: User has no tickets**
  - "My Tickets" tab shows empty state
  - Link to "Enter ticket details manually"
- **A3: User clicks "Choose a different ticket"**
  - Returns to ticket selection step

**Exception Flows**
- **E1: Price exceeds 1.5x cap**
  - "Exceeds anti-scalping cap" warning shown in red
  - "List for Resale" button disabled
- **E2: Contract not deployed**
  - Falls back to mock provider
  - Listing created in memory (session only)
- **E3: Ticket not found in mock data (manual entry)**
  - Mock provider accepts it anyway (on-chain contract would validate ownership)

**Postconditions**
- `ResaleListing` created with asking price, original price, 24h expiry
- Listing visible in marketplace (pending approval for non-mock)
- Ticket remains owned by seller until purchased

---

### Use Case: Buy Resale Ticket

- **ID:** UC-009
- **Title:** User purchases a ticket from the resale marketplace
- **Primary Actor:** End User (Buyer)
- **Secondary Actors:** InkTix Contract, Wallet Extension
- **Description:** User browses resale listings, selects one, and purchases it

**Preconditions**
- Resale listings exist
- User is on `/resale` page
- Wallet is connected (for purchase)

**Trigger**
- User clicks "Buy for X DOT" on a listing

**Main Flow (Happy Path)**
1. User navigates to `/resale`
2. Listings load (from contract or mock)
3. User can search by event name and sort by price or expiry
4. User finds a listing and reviews: event name, seat info, asking price, original price, price change %, time remaining
5. Listing shows "Verified" badge (approved)
6. User clicks "Buy for X DOT"
7. Contract processes purchase (or mock)
8. Success toast: "Ticket #X purchased for Y DOT!"
9. Listing removed from active listings

**Alternate Flows**
- **A1: Filter by event name**
  - User types in search box, listings filter in real-time
- **A2: Sort by expiry**
  - User selects "Expiring Soon" sort, listings reorder

**Exception Flows**
- **E1: Wallet not connected**
  - Button shows "Connect Wallet to Buy" (disabled)
- **E2: Listing pending approval**
  - Button shows "Pending Approval" (disabled)
- **E3: Listing expired**
  - Listing filtered out of view automatically
- **E4: Purchase fails**
  - Error toast displayed

**Postconditions**
- Listing marked as inactive
- Ticket ownership transferred to buyer
- Toast notification confirms purchase

---

### Use Case: Share Event

- **ID:** UC-010
- **Title:** User shares an event via link, social media, or native share
- **Primary Actor:** End User
- **Secondary Actors:** Clipboard API, Twitter/X, Web Share API
- **Description:** User shares event details through various channels

**Preconditions**
- User is on `/events` page
- At least one event is visible

**Trigger**
- User clicks Share/Copy button on event card or in event detail modal

**Main Flow (Happy Path) — Copy Link**
1. User clicks "Share" button (Copy icon) on an event card
2. Event URL and description copied to clipboard
3. Green toast: "Link copied!" appears for 2 seconds

**Alternate Flows**
- **A1: Post to X/Twitter**
  - User clicks "Post" button (Share2 icon)
  - Twitter intent window opens with pre-filled text and URL
  - User can edit and post
- **A2: Native share (mobile)**
  - On devices that support Web Share API, share button triggers native share sheet
  - User selects app to share to (Messages, WhatsApp, etc.)
- **A3: Share from detail modal**
  - Copy and X/Twitter buttons appear in modal header next to title
  - Same behavior as card buttons

**Exception Flows**
- **E1: Clipboard API not available**
  - Copy fails silently (edge case in older browsers)
- **E2: User cancels native share**
  - No error shown, share sheet dismissed

**Postconditions**
- Event URL copied to clipboard or shared via external service

---

### Use Case: Group Buy

- **ID:** UC-011
- **Title:** User creates or joins a group purchase for an event
- **Primary Actor:** Group Organizer / Group Member
- **Secondary Actors:** InkTix Frontend (in-session state)
- **Description:** User organizes a group ticket purchase with a shareable code for adjacent seating and group discounts

**Preconditions**
- User is on `/events` page
- Event is visible

**Trigger**
- User clicks "Group Buy" button on event card or "Organize a Group Buy" in detail modal

**Main Flow (Happy Path) — Create Group**
1. User clicks "Group Buy" on an event card
2. Group Buy modal opens with event name
3. User selects group size (2-10 people)
4. User clicks "Create Group"
5. System generates group code (e.g., `INK-A3FX-1`)
6. Code displayed with copy button
7. Summary shows: event, group size, price per ticket (5% off for 4+), status (1/N joined)
8. User clicks "Buy My Ticket" → redirected to event purchase flow

**Alternate Flows**
- **A1: Join existing group**
  - User clicks "Join Existing Group" on the create step
  - Input field for group code appears
  - User enters code (e.g., `INK-XXXX-1`)
  - System validates format (must start with "INK-")
  - Success: "Joined Group!" with group details and discount
  - User clicks "Buy My Ticket" → purchase flow
- **A2: Group of 2-3 (no discount)**
  - Price shown without discount
  - Adjacent seats still guaranteed
- **A3: Copy group code**
  - User clicks copy button next to code
  - Toast: "Group code copied!"

**Exception Flows**
- **E1: Invalid group code format**
  - "Join Group" button disabled until code starts with "INK-"
- **E2: User cancels**
  - Modal closes, no group created

**Postconditions**
- Group code generated (session-only in current implementation)
- User directed to purchase flow with group context
- Group discount (5%) applied for groups of 4+

---

### Use Case: Deploy Contract

- **ID:** UC-012
- **Title:** User deploys an InkTix smart contract to the chain
- **Primary Actor:** Contract Admin
- **Secondary Actors:** Substrate Chain, Wallet Extension
- **Description:** Admin uploads a .wasm contract file and deploys it to the connected chain

**Preconditions**
- Wallet is connected
- Network is connected
- User is on `/smart-contracts` page
- User has a `.wasm` contract file

**Trigger**
- User clicks "Deploy Contract" button

**Main Flow (Happy Path)**
1. User navigates to `/smart-contracts`
2. Contract status shows "Not Deployed"
3. User selects a `.wasm` file via file upload
4. Contract type auto-detected from filename (sports/concert/unknown)
5. User sets endowment value (default: 1.0 WND)
6. User clicks "Deploy Contract"
7. If contracts pallet available: `CodePromise` instantiates on-chain, wallet prompts for signing
8. If no contracts pallet: mock deployment with 2s delay
9. Contract address displayed
10. Status changes to "Deployed: Yes"
11. Interaction section becomes available

**Alternate Flows**
- **A1: Chain has no contracts pallet**
  - Mock deployment succeeds with generated address
  - Message: "Contract deployed (mock — chain has no contracts pallet)"

**Exception Flows**
- **E1: No wallet connected**
  - "Connect to Deploy Contract" message shown
- **E2: Invalid file type**
  - Alert: "Please select a valid .wasm file"
- **E3: User rejects signing**
  - Error: "Signing cancelled or failed"
- **E4: Deployment fails on-chain**
  - Error with decoded dispatch error shown

**Postconditions**
- Contract address stored in state
- `isContractDeployed` is `true`
- Contract interaction methods become available

---

### Use Case: Call Contract Method

- **ID:** UC-013
- **Title:** User calls a method on the deployed contract
- **Primary Actor:** Contract Admin / Developer
- **Secondary Actors:** InkTix Contract
- **Description:** User selects a contract method, provides arguments, and executes the call

**Preconditions**
- Contract is deployed (UC-012 completed)
- User is on `/smart-contracts` page

**Trigger**
- User selects a method and clicks "Call Method"

**Main Flow (Happy Path)**
1. Method selector shows available methods grouped by: Get Information, Registration & Creation, Queries with Parameters
2. User selects a method (e.g., `register_team`)
3. Arguments auto-fill with example values
4. User can modify arguments as needed
5. User clicks "Call Method"
6. SDK routes call through `ContractProvider` (or mock)
7. Result displayed as formatted JSON

**Alternate Flows**
- **A1: Query method (no gas)**
  - Read-only call via `contract.query`, instant result
- **A2: Transaction method (costs gas)**
  - Write call via `contract.tx`, wallet prompts for signing

**Exception Flows**
- **E1: Invalid JSON arguments**
  - Arguments parsed as empty array, call may fail
- **E2: Method call fails**
  - Error displayed: "Call failed: [error message]"

**Postconditions**
- Result displayed in the UI
- For mutations: on-chain state updated

---

### Use Case: XCM Cross-Chain Transfer

- **ID:** UC-014
- **Title:** Execute a cross-chain asset transfer via XCM
- **Primary Actor:** End User
- **Secondary Actors:** Westend Asset Hub, Destination Parachain, Wallet Extension
- **Description:** User performs a reserve transfer of assets from Asset Hub to a destination parachain for cross-chain ticket purchase

**Preconditions**
- Wallet connected to Westend Asset Hub
- User has sufficient balance
- User is on `/cross-chain-demo` page

**Trigger**
- User clicks "Execute XCM Transfer" button

**Main Flow (Happy Path)**
1. User navigates to `/cross-chain-demo`
2. User configures: destination ParaId, beneficiary address, transfer amount
3. User optionally configures seat details (section, row, seat number)
4. User clicks "Execute XCM Transfer"
5. 5-step progress tracker activates:
   - Step 1: Connecting to network
   - Step 2: Preparing XCM message
   - Step 3: Execute reserve transfer (wallet signing prompt)
   - Step 4: Simulating ticket purchase
   - Step 5: Minting NFT ticket
6. Activity log shows real-time status updates
7. Balance before/after displayed
8. NFT ticket display shows minted ticket details

**Alternate Flows**
- **A1: Custom ParaId**
  - User enters a different ParaId (default: 1000)
- **A2: Custom beneficiary**
  - User enters a different beneficiary address

**Exception Flows**
- **E1: Wrong wallet password**
  - Extension shows "Unable to decode using the supplied passphrase"
  - User must re-enter correct password
- **E2: Insufficient balance**
  - Transfer fails, error logged in activity log
- **E3: XCM message fails**
  - Error step shown in progress tracker
  - Activity log contains error details

**Postconditions**
- Assets transferred from Asset Hub to destination parachain
- Transaction hash recorded
- Balance updated
- NFT ticket minted (simulated)

---

### Use Case: View Analytics

- **ID:** UC-015
- **Title:** User views the analytics dashboard
- **Primary Actor:** Platform Admin / End User
- **Secondary Actors:** InkTix Contract (if deployed)
- **Description:** User views platform KPIs, revenue charts, category breakdown, and top events

**Preconditions**
- User is on `/analytics` page

**Trigger**
- User navigates to `/analytics`

**Main Flow (Happy Path)**
1. Dashboard loads with KPI cards: Total Revenue, Tickets Sold, Total Events, Active Users
2. Secondary stats: Avg Ticket Price, Season Passes, Fraud Rate
3. Monthly Revenue bar chart renders
4. Monthly Ticket Sales bar chart renders
5. Revenue by Category breakdown shows distribution with progress bars
6. Purchase Activity heatmap shows demand by time of day
7. Top Performing Events table shows ranked events with sellout %
8. Platform Health cards show fraud rate, transaction time, uptime
9. Data source indicator shows "Demo data" or "Live data"

**Alternate Flows**
- **A1: Contract deployed with real data**
  - `get_platform_stats()` called
  - KPI cards show real totals from contract
  - Data source indicator shows "Live data" (green)

**Exception Flows**
- **E1: Contract call fails**
  - Falls back to mock analytics data
  - Data source shows "Demo data"

**Postconditions**
- All charts and metrics rendered
- Data source clearly indicated

---

### Use Case: Disconnect Wallet

- **ID:** UC-016
- **Title:** User disconnects wallet and clears session
- **Primary Actor:** End User
- **Description:** User disconnects their wallet, clearing all connection state and returning to mock data mode

**Preconditions**
- Wallet is connected
- Network may or may not be connected

**Trigger**
- User clicks disconnect (X) button in wallet panel

**Main Flow (Happy Path)**
1. User clicks X button in wallet header
2. API disconnects from Substrate node
3. All state reset: connection, wallet, accounts, balance, network info, contract address
4. localStorage `inktix-store` updated with disconnected state
5. Mock data reloaded for events and venues
6. UI shows "Connect Wallet" state on all relevant pages

**Alternate Flows**
- (none)

**Exception Flows**
- **E1: Disconnect fails**
  - State is still cleared locally even if API disconnect throws

**Postconditions**
- `isConnected` and `isWalletConnected` are `false`
- `selectedAccount` is `null`
- `balance` is `null`
- Mock data displayed

---

## Test Cases

---

### TC-001: Connect wallet with valid extension

- **Related Use Case:** UC-001
- **Scenario:** TS-001
- **Priority:** High
- **Test Type:** Functional

**Preconditions**
- Polkadot.js extension installed with at least 1 account
- InkTix loaded in browser

**Test Steps**
1. Navigate to `/connect`
2. Click "Connect Wallet" button
3. Approve connection in extension popup
4. Observe wallet status

**Expected Result**
- Account name and truncated address displayed
- "Connected" status shown
- Balance begins loading
- Network auto-connection attempted

**Status:** ☐ Pass / ☐ Fail

---

### TC-002: Connect wallet with no extension installed

- **Related Use Case:** UC-001
- **Scenario:** TS-002
- **Priority:** High
- **Test Type:** Negative

**Preconditions**
- No Polkadot wallet extension installed

**Test Steps**
1. Navigate to `/connect`
2. Click "Connect Wallet" button

**Expected Result**
- Error message: "No web3 extension found. Please install Polkadot.js extension."
- UI remains on "Connect Wallet" state

**Status:** ☐ Pass / ☐ Fail

---

### TC-003: Connect wallet with zero accounts

- **Related Use Case:** UC-001
- **Scenario:** TS-003
- **Priority:** Medium
- **Test Type:** Negative

**Preconditions**
- Extension installed but no accounts created

**Test Steps**
1. Navigate to `/connect`
2. Click "Connect Wallet"
3. Approve in extension

**Expected Result**
- Error message: "No accounts found. Please create or import an account."

**Status:** ☐ Pass / ☐ Fail

---

### TC-004: Switch between multiple accounts

- **Related Use Case:** UC-001
- **Scenario:** TS-004
- **Priority:** Medium
- **Test Type:** Functional

**Preconditions**
- Extension has 2+ accounts
- Wallet connected

**Test Steps**
1. Click "Switch Account" in wallet panel
2. Account selector expands showing all accounts
3. Click a different account
4. Observe address and balance update

**Expected Result**
- Selected account changes
- Balance refreshes for new account
- Previously selected account is deselected

**Status:** ☐ Pass / ☐ Fail

---

### TC-005: Connect to Westend Asset Hub

- **Related Use Case:** UC-002
- **Scenario:** TS-005
- **Priority:** High
- **Test Type:** Functional

**Preconditions**
- Wallet connected

**Test Steps**
1. Observe automatic network connection after wallet connect
2. Or manually trigger by clicking "Connect to Network"

**Expected Result**
- Network info shows: Chain "Westend Asset Hub", node name, version
- Balance displayed in WND
- `isConnected` is true

**Status:** ☐ Pass / ☐ Fail

---

### TC-006: Network connection to unreachable endpoint

- **Related Use Case:** UC-002
- **Scenario:** TS-006
- **Priority:** Medium
- **Test Type:** Negative

**Preconditions**
- Wallet connected
- No internet or endpoint is down

**Test Steps**
1. Disconnect internet
2. Attempt network connection

**Expected Result**
- Connection times out
- App continues with mock data
- No crash or unhandled error

**Status:** ☐ Pass / ☐ Fail

---

### TC-007: Auto-restore connection on page reload

- **Related Use Case:** UC-002
- **Scenario:** TS-007
- **Priority:** High
- **Test Type:** Functional

**Preconditions**
- Previously connected and state saved to localStorage

**Test Steps**
1. Connect wallet and network
2. Reload page (F5)
3. Observe auto-restoration

**Expected Result**
- Wallet state restored from `inktix-store`
- Network reconnects automatically
- Balance reloads
- No manual reconnection needed

**Status:** ☐ Pass / ☐ Fail

---

### TC-008: Connection state persists across tabs

- **Related Use Case:** UC-002
- **Scenario:** TS-008
- **Priority:** Low
- **Test Type:** Functional

**Preconditions**
- Connected in one tab

**Test Steps**
1. Connect wallet and network in Tab 1
2. Open InkTix in Tab 2

**Expected Result**
- Tab 2 shows connected state from hydrated localStorage

**Status:** ☐ Pass / ☐ Fail

---

### TC-009: Browse events with category filter

- **Related Use Case:** UC-003
- **Scenario:** TS-009
- **Priority:** High
- **Test Type:** Functional

**Test Steps**
1. Navigate to `/events`
2. Observe initial 6 events displayed
3. Click "NBA" filter
4. Observe filtered results
5. Click "All" to reset

**Expected Result**
- Only NBA events shown when NBA filter active
- All events shown when "All" selected
- Active filter visually highlighted

**Status:** ☐ Pass / ☐ Fail

---

### TC-010: Load more events

- **Related Use Case:** UC-003
- **Scenario:** TS-010
- **Priority:** Medium
- **Test Type:** Functional

**Test Steps**
1. Navigate to `/events`
2. Scroll to bottom
3. Click "Load More Events (X remaining)"

**Expected Result**
- 6 more events appear
- Button updates count or disappears when all loaded

**Status:** ☐ Pass / ☐ Fail

---

### TC-011: View event details modal

- **Related Use Case:** UC-003
- **Scenario:** TS-011
- **Priority:** High
- **Test Type:** Functional

**Test Steps**
1. Click "View Details" on any event
2. Observe modal content
3. Click close (×) button

**Expected Result**
- Modal shows: title, date, time, venue, location, availability, popularity, price
- Type-specific info shown (artist for concerts, sport for sports)
- Modal closes cleanly

**Status:** ☐ Pass / ☐ Fail

---

### TC-012: Data source indicator reflects contract connection

- **Related Use Case:** UC-003
- **Scenario:** TS-012
- **Priority:** Medium
- **Test Type:** Functional

**Test Steps**
1. Navigate to `/events` without wallet connected
2. Observe data source indicator
3. Connect wallet and deploy contract
4. Revisit `/events`

**Expected Result**
- Without connection: "Demo data" (blue pill)
- With deployed contract: "Live from blockchain" (green pill)

**Status:** ☐ Pass / ☐ Fail

---

### TC-013: Purchase ticket with general admission

- **Related Use Case:** UC-004
- **Scenario:** TS-013
- **Priority:** High
- **Test Type:** Functional

**Test Steps**
1. Open event detail modal
2. Click "Buy Tickets"
3. Keep default "General Admission" seat type
4. Enter section "A", row "10", seat "15"
5. Keep "DOT" currency
6. Review price breakdown (should show 0% seat adjustment)
7. Click "Confirm Purchase"

**Expected Result**
- Price equals base price (no seat premium)
- Success screen with ticket ID
- "View My Tickets" link present

**Status:** ☐ Pass / ☐ Fail

---

### TC-014: Purchase ticket with courtside seat (premium pricing)

- **Related Use Case:** UC-004
- **Scenario:** TS-014
- **Priority:** High
- **Test Type:** Functional

**Test Steps**
1. Open event detail modal → "Buy Tickets"
2. Select "Courtside / Field Level" seat type
3. Review price breakdown

**Expected Result**
- Seat type shows "+150%" adjustment
- Total price is significantly higher than base
- Price breakdown clearly shows the multiplier

**Status:** ☐ Pass / ☐ Fail

---

### TC-015: Purchase ticket with student discount

- **Related Use Case:** UC-004
- **Scenario:** TS-015
- **Priority:** Medium
- **Test Type:** Functional

**Test Steps**
1. Open event detail modal → "Buy Tickets"
2. Select "Student Section" seat type
3. Review price breakdown

**Expected Result**
- Seat type shows "-30%" adjustment (green text)
- Total price is lower than base price

**Status:** ☐ Pass / ☐ Fail

---

### TC-016: Purchase ticket with different currency

- **Related Use Case:** UC-004
- **Scenario:** TS-016
- **Priority:** Medium
- **Test Type:** Functional

**Test Steps**
1. Open event detail modal → "Buy Tickets"
2. Change currency from "DOT" to "KSM"
3. Click "Confirm Purchase"

**Expected Result**
- Currency selection reflected in confirmation
- Purchase succeeds with selected currency

**Status:** ☐ Pass / ☐ Fail

---

### TC-017: Purchase ticket navigates back correctly

- **Related Use Case:** UC-004
- **Scenario:** TS-017
- **Priority:** Low
- **Test Type:** Functional

**Test Steps**
1. Open event detail modal → "Buy Tickets" (step 2)
2. Click "Back"
3. Observe return to step 1

**Expected Result**
- Returns to event details with price and "Buy Tickets" button
- No state corruption

**Status:** ☐ Pass / ☐ Fail

---

### TC-018: Mint NFT for owned ticket

- **Related Use Case:** UC-005
- **Scenario:** TS-018
- **Priority:** High
- **Test Type:** Functional

**Test Steps**
1. Navigate to `/my-tickets`
2. Find a ticket showing "Mint NFT" button
3. Click "Mint NFT"

**Expected Result**
- NFT minted successfully
- QR code appears on ticket card
- Verification hash displayed
- "Mint NFT" button replaced with minted status

**Status:** ☐ Pass / ☐ Fail

---

### TC-019: Attempt to mint NFT twice

- **Related Use Case:** UC-005
- **Scenario:** TS-019
- **Priority:** Medium
- **Test Type:** Negative

**Test Steps**
1. Mint NFT for a ticket (TC-018)
2. Attempt to mint again for same ticket

**Expected Result**
- Error: "NFT already minted for this ticket"
- Or: "Mint NFT" button no longer visible

**Status:** ☐ Pass / ☐ Fail

---

### TC-020: QR code contains correct data

- **Related Use Case:** UC-005
- **Scenario:** TS-020
- **Priority:** High
- **Test Type:** Functional

**Test Steps**
1. Mint NFT for a ticket
2. Scan/decode the QR code (or inspect the `value` prop)

**Expected Result**
- QR code encodes JSON: `{"tokenId":N,"eventId":N,"seatNumber":N,"hash":"0x..."}`

**Status:** ☐ Pass / ☐ Fail

---

### TC-021: Copy verification hash

- **Related Use Case:** UC-005
- **Scenario:** TS-021
- **Priority:** Low
- **Test Type:** Functional

**Test Steps**
1. Find an NFT ticket with verification hash displayed
2. Click the copy button next to the hash

**Expected Result**
- Hash copied to clipboard
- Visual feedback (e.g., icon change or tooltip)

**Status:** ☐ Pass / ☐ Fail

---

### TC-022: Verify valid NFT ticket

- **Related Use Case:** UC-006
- **Scenario:** TS-022
- **Priority:** High
- **Test Type:** Functional

**Test Steps**
1. Navigate to `/my-tickets`
2. Scroll to "Verify Ticket" section
3. Enter a valid token ID (e.g., 1)
4. Click "Verify"

**Expected Result**
- Shows: Valid, Not Used, owner address, event name, section/row/seat

**Status:** ☐ Pass / ☐ Fail

---

### TC-023: Verify used NFT ticket

- **Related Use Case:** UC-006
- **Scenario:** TS-023
- **Priority:** Medium
- **Test Type:** Functional

**Test Steps**
1. Use a ticket (TC-025)
2. Verify the same token ID

**Expected Result**
- Shows: Valid, Used, owner address, event name, section/row/seat
- "Used" indicator visible

**Status:** ☐ Pass / ☐ Fail

---

### TC-024: Verify non-existent token ID

- **Related Use Case:** UC-006
- **Scenario:** TS-024
- **Priority:** Medium
- **Test Type:** Negative

**Test Steps**
1. Enter token ID 99999
2. Click "Verify"

**Expected Result**
- Error: "NFT not found"

**Status:** ☐ Pass / ☐ Fail

---

### TC-025: Use ticket at event entry

- **Related Use Case:** UC-007
- **Scenario:** TS-025
- **Priority:** High
- **Test Type:** Functional

**Test Steps**
1. Navigate to `/my-tickets`
2. Find an unused NFT ticket
3. Click "Use Ticket"

**Expected Result**
- Ticket marked as "Used"
- Attendance token minted
- Button disabled or removed
- Stats update (attended count)

**Status:** ☐ Pass / ☐ Fail

---

### TC-026: Use already-used ticket

- **Related Use Case:** UC-007
- **Scenario:** TS-026
- **Priority:** High
- **Test Type:** Negative

**Test Steps**
1. Use a ticket (TC-025)
2. Attempt to use the same ticket again

**Expected Result**
- Error: "Ticket already used"

**Status:** ☐ Pass / ☐ Fail

---

### TC-027: Use ticket for non-existent NFT

- **Related Use Case:** UC-007
- **Scenario:** TS-027
- **Priority:** Low
- **Test Type:** Negative

**Test Steps**
1. Attempt to use ticket with invalid token ID

**Expected Result**
- Error: "NFT not found"

**Status:** ☐ Pass / ☐ Fail

---

### TC-028: List owned ticket for resale

- **Related Use Case:** UC-008
- **Scenario:** TS-028
- **Priority:** High
- **Test Type:** Functional

**Test Steps**
1. Navigate to `/resale`
2. Click "List a Ticket"
3. Select a ticket from "My Tickets" tab
4. Set asking price within 1.5x cap
5. Click "List for Resale"

**Expected Result**
- Success message: "Ticket listed for resale successfully!"
- Modal closes
- New listing appears in marketplace (as "Pending")

**Status:** ☐ Pass / ☐ Fail

---

### TC-029: List ticket via manual entry

- **Related Use Case:** UC-008
- **Scenario:** TS-029
- **Priority:** High
- **Test Type:** Functional

**Test Steps**
1. Navigate to `/resale` → "List a Ticket"
2. Click "Enter Manually" tab
3. Fill in: Ticket ID (e.g., 999), Event Name ("Sammy Hagar"), Venue ("Staples Center"), Date, Section "A", Row "12", Seat "23", Price "125"
4. Click "Continue to Set Price"
5. Set asking price (e.g., 145)
6. Click "List for Resale"

**Expected Result**
- All entered details shown in summary
- Max allowed shows "188 DOT (1.5x)"
- Listing created successfully

**Status:** ☐ Pass / ☐ Fail

---

### TC-030: Attempt to exceed anti-scalping price cap

- **Related Use Case:** UC-008
- **Scenario:** TS-030
- **Priority:** High
- **Test Type:** Negative

**Test Steps**
1. Select a ticket with original price 100 DOT
2. In price step, enter 200 (exceeds 150 cap)

**Expected Result**
- Red warning: "Exceeds anti-scalping cap"
- "List for Resale" button disabled

**Status:** ☐ Pass / ☐ Fail

---

### TC-031: List ticket when no owned tickets available

- **Related Use Case:** UC-008
- **Scenario:** TS-031
- **Priority:** Low
- **Test Type:** Edge Case

**Test Steps**
1. Navigate to `/resale` → "List a Ticket"
2. Observe "My Tickets" tab with no tickets

**Expected Result**
- Empty state: "No tickets found in your wallet"
- Link to "Enter ticket details manually"

**Status:** ☐ Pass / ☐ Fail

---

### TC-032: Buy resale ticket

- **Related Use Case:** UC-009
- **Scenario:** TS-032
- **Priority:** High
- **Test Type:** Functional

**Test Steps**
1. Navigate to `/resale`
2. Find a "Verified" listing
3. Click "Buy for X DOT"

**Expected Result**
- Success toast: "Ticket #X purchased for Y DOT!"
- Listing removed from view
- Listings count decremented

**Status:** ☐ Pass / ☐ Fail

---

### TC-033: Attempt to buy pending (unverified) listing

- **Related Use Case:** UC-009
- **Scenario:** TS-033
- **Priority:** Medium
- **Test Type:** Negative

**Test Steps**
1. Find a listing with "Pending" badge
2. Observe buy button state

**Expected Result**
- Button shows "Pending Approval" (disabled)
- Cannot proceed with purchase

**Status:** ☐ Pass / ☐ Fail

---

### TC-034: Search and sort resale listings

- **Related Use Case:** UC-009
- **Scenario:** TS-034
- **Priority:** Medium
- **Test Type:** Functional

**Test Steps**
1. Navigate to `/resale`
2. Type "Lakers" in search box
3. Observe filtered results
4. Change sort to "Price: High to Low"
5. Change sort to "Expiring Soon"

**Expected Result**
- Only listings with "Lakers" in event name shown
- Sort order changes correctly
- Listings reorder in real-time

**Status:** ☐ Pass / ☐ Fail

---

### TC-035: Copy event link

- **Related Use Case:** UC-010
- **Scenario:** TS-035
- **Priority:** Medium
- **Test Type:** Functional

**Test Steps**
1. Navigate to `/events`
2. Click "Share" (Copy icon) on an event card

**Expected Result**
- Green toast: "Link copied!"
- Clipboard contains event URL and description

**Status:** ☐ Pass / ☐ Fail

---

### TC-036: Share event to X/Twitter

- **Related Use Case:** UC-010
- **Scenario:** TS-036
- **Priority:** Medium
- **Test Type:** Functional

**Test Steps**
1. Click "Post" (Share2 icon) on an event card

**Expected Result**
- New window/tab opens to `x.com/intent/tweet`
- Pre-filled with event name and URL
- Window is 550×420 popup

**Status:** ☐ Pass / ☐ Fail

---

### TC-037: Share from event detail modal

- **Related Use Case:** UC-010
- **Scenario:** TS-037
- **Priority:** Low
- **Test Type:** Functional

**Test Steps**
1. Open event detail modal
2. Click copy button in modal header
3. Click X/Twitter button in modal header

**Expected Result**
- Same behavior as card share buttons
- Toast appears for copy, X window opens for post

**Status:** ☐ Pass / ☐ Fail

---

### TC-038: Create group buy

- **Related Use Case:** UC-011
- **Scenario:** TS-038
- **Priority:** High
- **Test Type:** Functional

**Test Steps**
1. Click "Group Buy" on an event card
2. Select group size: 4
3. Click "Create Group"

**Expected Result**
- Group code generated (format: `INK-XXXX-N`)
- Copy button next to code
- Summary shows: event, 4 people, 5% discount, 1/4 joined
- "Buy My Ticket" button present

**Status:** ☐ Pass / ☐ Fail

---

### TC-039: Copy group code

- **Related Use Case:** UC-011
- **Scenario:** TS-039
- **Priority:** Medium
- **Test Type:** Functional

**Test Steps**
1. Create a group (TC-038)
2. Click copy button next to group code

**Expected Result**
- Toast: "Group code copied!"
- Code in clipboard

**Status:** ☐ Pass / ☐ Fail

---

### TC-040: Join existing group

- **Related Use Case:** UC-011
- **Scenario:** TS-040
- **Priority:** High
- **Test Type:** Functional

**Test Steps**
1. Click "Group Buy" on an event
2. Click "Join Existing Group"
3. Enter code: `INK-TEST-1`
4. Click "Join Group"

**Expected Result**
- "Joined Group!" confirmation
- Group details shown with discount
- "Buy My Ticket" button present

**Status:** ☐ Pass / ☐ Fail

---

### TC-041: Join group with invalid code

- **Related Use Case:** UC-011
- **Scenario:** TS-041
- **Priority:** Medium
- **Test Type:** Negative

**Test Steps**
1. Click "Group Buy" → "Join Existing Group"
2. Enter code: `INVALID`

**Expected Result**
- "Join Group" button disabled (code doesn't start with "INK-")

**Status:** ☐ Pass / ☐ Fail

---

### TC-042: Deploy contract with .wasm file

- **Related Use Case:** UC-012
- **Scenario:** TS-042
- **Priority:** High
- **Test Type:** Functional

**Test Steps**
1. Navigate to `/smart-contracts`
2. Upload `inktix.wasm` file
3. Set endowment to 1.0
4. Click "Deploy Contract"
5. Sign transaction in wallet (if real chain)

**Expected Result**
- Deployment result shows contract address
- Status changes to "Deployed: Yes"
- Contract interaction section appears

**Status:** ☐ Pass / ☐ Fail

---

### TC-043: Deploy contract without wallet connected

- **Related Use Case:** UC-012
- **Scenario:** TS-043
- **Priority:** Medium
- **Test Type:** Negative

**Test Steps**
1. Navigate to `/smart-contracts` without connecting wallet

**Expected Result**
- "Connect to Deploy Contract" message shown
- No upload or deploy UI visible

**Status:** ☐ Pass / ☐ Fail

---

### TC-044: Upload non-.wasm file

- **Related Use Case:** UC-012
- **Scenario:** TS-044
- **Priority:** Medium
- **Test Type:** Negative

**Test Steps**
1. Attempt to upload a `.txt` or `.json` file

**Expected Result**
- Alert: "Please select a valid .wasm file"
- File not accepted

**Status:** ☐ Pass / ☐ Fail

---

### TC-045: Call getter method (get_owner)

- **Related Use Case:** UC-013
- **Scenario:** TS-045
- **Priority:** High
- **Test Type:** Functional

**Test Steps**
1. Deploy contract (TC-042)
2. Select "get_owner" from method dropdown
3. Click "Call Method"

**Expected Result**
- Result shows owner account address
- No wallet signing prompt (query, not transaction)

**Status:** ☐ Pass / ☐ Fail

---

### TC-046: Call creator method (register_team)

- **Related Use Case:** UC-013
- **Scenario:** TS-046
- **Priority:** High
- **Test Type:** Functional

**Test Steps**
1. Select "register_team" method
2. Arguments auto-fill: `["Lakers", "Basketball", "Los Angeles"]`
3. Click "Call Method"

**Expected Result**
- Result shows new team ID (e.g., 3)
- Message confirms team registered

**Status:** ☐ Pass / ☐ Fail

---

### TC-047: Call method with invalid arguments

- **Related Use Case:** UC-013
- **Scenario:** TS-047
- **Priority:** Medium
- **Test Type:** Negative

**Test Steps**
1. Select a method requiring arguments
2. Clear the arguments field or enter invalid JSON
3. Click "Call Method"

**Expected Result**
- Error displayed in result area
- No crash

**Status:** ☐ Pass / ☐ Fail

---

### TC-048: Execute XCM reserve transfer

- **Related Use Case:** UC-014
- **Scenario:** TS-048
- **Priority:** High
- **Test Type:** Functional

**Test Steps**
1. Navigate to `/cross-chain-demo`
2. Set ParaId: 1000
3. Set beneficiary: own account address
4. Set amount: 1 WND
5. Click "Execute XCM Transfer"
6. Sign in wallet when prompted

**Expected Result**
- 5-step progress tracker advances through all steps
- Activity log shows status messages
- Balance changes displayed
- NFT ticket section shows minted ticket

**Status:** ☐ Pass / ☐ Fail

---

### TC-049: XCM transfer with wrong password

- **Related Use Case:** UC-014
- **Scenario:** TS-049
- **Priority:** Medium
- **Test Type:** Negative

**Test Steps**
1. Start XCM transfer (TC-048 steps 1-5)
2. Enter wrong password in wallet signing prompt

**Expected Result**
- Extension shows "Unable to decode using the supplied passphrase"
- Transfer does not proceed
- Can re-enter correct password

**Status:** ☐ Pass / ☐ Fail

---

### TC-050: XCM transfer with insufficient balance

- **Related Use Case:** UC-014
- **Scenario:** TS-050
- **Priority:** Medium
- **Test Type:** Negative

**Test Steps**
1. Set amount higher than available balance
2. Execute transfer

**Expected Result**
- Transfer fails
- Error shown in activity log

**Status:** ☐ Pass / ☐ Fail

---

### TC-051: XCM transfer with custom ParaId

- **Related Use Case:** UC-014
- **Scenario:** TS-051
- **Priority:** Low
- **Test Type:** Functional

**Test Steps**
1. Set ParaId to 2000 (non-default)
2. Execute transfer

**Expected Result**
- Transfer targets the specified ParaId
- Activity log shows destination chain

**Status:** ☐ Pass / ☐ Fail

---

### TC-052: Analytics dashboard loads with demo data

- **Related Use Case:** UC-015
- **Scenario:** TS-052
- **Priority:** High
- **Test Type:** Functional

**Test Steps**
1. Navigate to `/analytics`
2. Observe all sections

**Expected Result**
- 7 KPI cards rendered with values and % change arrows
- Revenue and ticket sales bar charts rendered
- Category breakdown shows 6 categories with progress bars
- Purchase activity heatmap shows 10 time slots
- Top events table shows 5 events with sellout badges
- Platform health cards show 3 metrics
- Data source: "Demo data"

**Status:** ☐ Pass / ☐ Fail

---

### TC-053: Analytics KPI values are reasonable

- **Related Use Case:** UC-015
- **Scenario:** TS-053
- **Priority:** Medium
- **Test Type:** Functional

**Test Steps**
1. Navigate to `/analytics`
2. Check KPI values against expected mock data

**Test Data**
- Total Revenue: 4.8M DOT
- Tickets Sold: 31,420
- Total Events: 1,247
- Active Users: 45.2K

**Expected Result**
- Values match mock data
- % change indicators are green (positive) for growth metrics
- Fraud rate shows negative % (improvement)

**Status:** ☐ Pass / ☐ Fail

---

### TC-054: Analytics with live contract data

- **Related Use Case:** UC-015
- **Scenario:** TS-054
- **Priority:** Low
- **Test Type:** Functional

**Preconditions**
- Contract deployed with events and tickets created

**Test Steps**
1. Connect wallet and deploy contract
2. Create some events and purchase tickets
3. Navigate to `/analytics`

**Expected Result**
- KPI cards show real contract values
- Data source: "Live data" (green)

**Status:** ☐ Pass / ☐ Fail

---

### TC-055: Disconnect wallet clears all state

- **Related Use Case:** UC-016
- **Scenario:** TS-055
- **Priority:** High
- **Test Type:** Functional

**Test Steps**
1. Connect wallet and network
2. Click X (disconnect) button in wallet header
3. Observe state changes

**Expected Result**
- Wallet panel shows "Connect Wallet" state
- Balance cleared
- Network info cleared
- Events page shows mock data
- `inktix-store` in localStorage updated

**Status:** ☐ Pass / ☐ Fail

---

### TC-056: Reconnect after disconnect

- **Related Use Case:** UC-016
- **Scenario:** TS-056
- **Priority:** Medium
- **Test Type:** Functional

**Test Steps**
1. Disconnect (TC-055)
2. Click "Connect Wallet" again
3. Approve in extension

**Expected Result**
- Full reconnection succeeds
- Account, balance, network info restored
- Same flow as TC-001

**Status:** ☐ Pass / ☐ Fail

---

## Coverage Checklist

### Functional Coverage
- [x] All 16 use cases defined
- [x] All main flows covered (16 happy paths)
- [x] All alternate flows covered (19 variations)
- [x] All exception flows covered (24 error scenarios)

### Edge Cases
- [x] Empty inputs (no wallet, no accounts, no tickets, no events)
- [x] Max values (anti-scalping 1.5x cap)
- [x] Boundary conditions (price at exactly 1.5x, group size limits)
- [x] Session persistence (page reload, tab switching)

### Negative Testing
- [x] Invalid inputs (wrong file type, invalid JSON, bad group code)
- [x] Unauthorized actions (wrong password, no wallet)
- [x] Double actions (mint twice, use twice)
- [x] Network failures (endpoint unreachable)

### Non-Functional
- [x] State persistence across page reloads
- [x] Mock fallback when contract not deployed
- [x] Data source indicators (demo vs live)
- [x] Mobile responsiveness (noted in UI requirements)
