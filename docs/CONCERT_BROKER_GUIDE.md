# Concert Broker Smart Contract Interaction Guide

This guide explains how to use the Smart Contract Manager to interact with the `concert_broker` Ink! contract: what each method does, required parameters, and a quick end‑to‑end overview flow.

> Prereqs
>
> - Connect Polkadot{.js} wallet in the app
> - Connect to Westend AssetHub endpoint
> - Load the `concert_broker.wasm` in Smart Contract Manager
> - Ensure your account has test funds (WND)

## Method Reference

### Get Information (read, free)

- `get_stats()` → high‑level contract stats (counts, totals)
- `get_owner()` → contract owner account

### Registration & Creation (write, on‑chain)

- `register_artist(name: String)` → returns `artistId`
- `register_venue(name: String, capacity: u32, address: String)` → returns `venueId`
- `create_concert_event(name: String, artistId: u32, venueId: u32, date: String, capacity: u32, price: u128)` → returns `eventId`
- `purchase_ticket(eventId: u32, seatNumber: u32)` → returns `ticketId`

### Queries with Parameters (read, free)

- `get_artist(artistId: u32)`
- `get_venue(venueId: u32)`
- `get_event(eventId: u32)`
- `get_ticket(ticketId: u32)`
- `get_user_tickets(userId: AccountId | u32)`

## Parameter Formats

- `name`, `address`: plain strings, e.g. "Taylor Swift", "1111 S Figueroa St, Los Angeles, CA"
- `capacity`, `seatNumber`, IDs: positive integers
- `date`: ISO 8601 recommended, e.g. `2025-02-05T20:00:00Z`
- `price`: integer amount (base units). If UI provides an example, prefer that value/format.

## Using Smart Contract Manager

Each method card shows fields for arguments plus actions:

- `Query`: executes a read (or dry‑run preview on some writes)
- `Submit`: sends the transaction for write methods

## Quick Start Flow

1. Register an artist

- Method: `register_artist`
- Args: `name = "Taylor Swift"`
- Action: Submit
- Output: `artistId` (e.g., 1)

2. Register a venue

- Method: `register_venue`
- Args:
  - `name = "Crypto.com Arena"`
  - `capacity = 20000`
  - `address = "1111 S Figueroa St, Los Angeles, CA"`
- Action: Submit
- Output: `venueId` (e.g., 1)

3. Create a concert event

- Method: `create_concert_event`
- Args:
  - `name = "The Eras Tour"`
  - `artistId = 1`
  - `venueId = 1`
  - `date = "2025-02-05T20:00:00Z"`
  - `capacity = 20000`
  - `price = 1500000000000` (example base units)
- Action: Submit
- Output: `eventId` (e.g., 1)

4. Inspect (free)

- `get_artist(1)` → Query
- `get_venue(1)` → Query
- `get_event(1)` → Query
- `get_stats()` → Query

5. Purchase a ticket

- Method: `purchase_ticket`
- Args:
  - `eventId = 1`
  - `seatNumber = 101`
- Value: leave default unless contract expects payment; follow UI hint if shown
- Action: Submit
- Output: `ticketId` (e.g., 1)

6. Verify tickets

- `get_ticket(1)` → Query
- `get_user_tickets(<your account>)` → Query

## Troubleshooting

- If a write fails, try a `Query` (dry‑run) first to see the reason.
- Ensure IDs exist (artistId, venueId, eventId).
- Keep `date` and `price` formats exactly as the UI suggests.
- If weight/gas errors appear, use the UI estimate/defaults.

---

Appendix: Common Examples

- Artist names: "Taylor Swift", "Coldplay"
- Venues: "Crypto.com Arena", capacity `20000`, address free‑form
- Dates: `YYYY-MM-DDThh:mm:ssZ`
- Price: integer in base units; consult UI example
