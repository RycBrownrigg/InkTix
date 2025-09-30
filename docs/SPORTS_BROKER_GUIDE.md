# Sports Broker Smart Contract Interaction Guide

This guide explains how to use the Smart Contract Manager to interact with the `sports_broker` Ink! contract: methods, parameters, and an end‑to‑end process flow.

> Prereqs
>
> - Connect Polkadot{.js} wallet in the app
> - Connect to Westend AssetHub endpoint
> - Load the `sports_broker.wasm` in Smart Contract Manager
> - Ensure your account has test funds (WND)

## Method Reference

### Get Information (read, free)

- `get_stats()` → high‑level contract stats
- `get_owner()` → contract owner account

### Registration & Creation (write, on‑chain)

- `register_team(name: String)` → returns `teamId`
- `register_venue(name: String, capacity: u32, address: String)` → returns `venueId`
- `create_sports_event(name: String, teamAId: u32, teamBId: u32, venueId: u32, date: String, capacity: u32, price: u128)` → returns `eventId`
- `purchase_ticket(eventId: u32, seatNumber: u32)` → returns `ticketId`

### Queries with Parameters (read, free)

- `get_team(teamId: u32)`
- `get_venue(venueId: u32)`
- `get_event(eventId: u32)`
- `get_ticket(ticketId: u32)`
- `get_user_tickets(userId: AccountId | u32)`

## Parameter Formats

- `name`, `address`: plain strings
- `capacity`, `seatNumber`, IDs: positive integers
- `date`: ISO 8601 recommended, e.g. `2025-01-21T19:30:00Z`
- `price`: integer amount (base units). Use the UI example if shown.

## Using Smart Contract Manager

- `Query`: read/dry‑run
- `Submit`: send transaction for write methods

## Quick Start Flow

1. Register two teams

- `register_team("Lakers")` → Submit → `teamAId`
- `register_team("Warriors")` → Submit → `teamBId`

2. Register a venue

- `register_venue("Crypto.com Arena", 20000, "1111 S Figueroa St, Los Angeles, CA")` → Submit → `venueId`

3. Create a sports event

- `create_sports_event("Lakers vs Warriors", teamAId, teamBId, venueId, "2025-01-21T19:30:00Z", 20000, 1200000000000)` → Submit → `eventId`

4. Inspect (free)

- `get_team(teamAId)` → Query
- `get_venue(venueId)` → Query
- `get_event(eventId)` → Query
- `get_stats()` → Query

5. Purchase a ticket

- `purchase_ticket(eventId, 101)` → Submit → `ticketId`

6. Verify

- `get_ticket(ticketId)` → Query
- `get_user_tickets(<your account>)` → Query

## Troubleshooting

- Use `Query` first if a write fails
- Verify referenced IDs exist
- Match date/price formats to UI hints
- Leave gas/weight to defaults/estimate unless required

---

Appendix: Quick Examples

- Teams: "Lakers", "Warriors"
- Venue: name `Crypto.com Arena`, capacity `20000`, address free‑form
- Date: `YYYY-MM-DDThh:mm:ssZ`
- Price: integer base units
