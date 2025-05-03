# Solana Indexer

A high-performance Rust-based indexer for the Solana blockchain that provides real-time transaction indexing and querying capabilities.

## Overview

This project consists of two main components:
- **Indexer**: Connects to Solana via WebSocket and indexes confirmed transactions in real-time
- **API Server**: Provides REST endpoints to query indexed blockchain data

## Features

- Real-time transaction indexing via WebSocket subscription
- Efficient in-memory storage with configurable capacity
- RESTful API for querying blockchain data
- Account balance and token information retrieval
- Transaction lookup by signature
- Recent transactions query with pagination

## Architecture

```
┌─────────────────┐         ┌──────────────────┐
│  Solana Node    │         │   API Server     │
│  (WebSocket)    │         │   (Port 3000)    │
└────────┬────────┘         └────────▲─────────┘
│                           │
│ New Blocks                │ Query Data
│                           │
▼                           │
┌─────────────────┐         ┌────────┴─────────┐
│    Indexer      │◄───────►│  In-Memory Store │
│ (WSS Listener)  │         │  (Transactions)  │
└─────────────────┘         └──────────────────┘
```

## API Endpoints

#### `GET /` - Indexer Statistics
Returns current blockchain metadata and indexer status.

**Response:**
```json
{
  "previous_blockhash": "...",
  "blockhash": "...",
  "slot": 123456789,
  "parent_slot": 123456788,
  "block_time": 1635724800,
  "block_height": 100000000,
  "transaction_count": 150
}
```

#### GET /account/{address} - Account Information
Retrieves SOL balance and token balances for a given account.\

Parameters:

```js
{ address: "Solana account address" }
```

**Response:**
```json
{
  "balance": 1.5,
  "tokens": [
    {
      "mint": "...",
      "balance": 100.0
    }
  ]
}
```

#### GET /signature/{signature} - Transaction Details
Fetches transaction details by signature hash.

Parameters:

```js
{ signature: "Transaction signature hash" }
```

**Response:**
```json
{
  "signature": "...",
  "messages": [...],
  "status": "confirmed"
}
```

#### GET /recent - Recent Transactions
Returns the most recent transactions.
Query Parameters:

```js
{ limit: "Number of transactions to return (required)" }
```

**Response:**
```json
[
  {
    "signature": "...",
    "block_time": 1635724800,
    "slot": 123456789
  }
]
```

## Installation

1. Ensure you have Rust installed (latest stable version)
2. Clone the repository
2. Install dependencies:

```bash
cargo build --release
```

## Configuration
Configure the following environment variables:

```env
SOLANA_RPC_URL=wss://something.com # see .env.example
RUST_LOG=INFO
```

## Running
Start the indexer and API server:
```bash
./target/release/server
```

The API server will be available at http://localhost:3000


## Performance

Indexes transactions in real-time with minimal latency
Maintains a sliding window of recent transactions for efficient memory usage
Handles high throughput with concurrent processing
O(1) lookups for transaction queries

## Contributing

1. Fork the repository
2. Create your feature branch (git checkout -b feature/amazing-feature)
3. Commit your changes (git commit -m 'Add some amazing feature')
4. Push to the branch (git push origin feature/amazing-feature)
5. Open a Pull Request

## License
This project is licensed under the MIT License - see the LICENSE file for details
