# Market Aggregator Application

A real-time market data aggregator built with Rust and Yew framework. This application displays live cryptocurrency prices, currency exchange rates, stock indices, and US stock quotes.

## Features

- 📊 Real-time market data via WebSocket
- 💱 Cryptocurrency tracking (BTC, ETH, LTC, EOS)
- 💵 Forex rates (EUR/USD, GBP/CHF, etc.)
- 📈 Major indices (DJIA, S&P 500, NASDAQ, FTSE100, CAC40, DAX)
- 📉 US stocks (MSFT, AAPL, NVDA, GOOGL, AMZN, etc.)
- ⚡ Built with Rust and WebAssembly for optimal performance

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [Trunk](https://trunkrs.dev/) - WASM web application bundler
- Twelve Data API key (get one at [twelvedata.com](https://twelvedata.com))

## Setup

1. **Clone the repository**
   ```bash
   git clone https://github.com/MShubkin/market_aggregator.git
   cd market_aggregator
   ```

2. **Configure API Key**
   
   Create a `.env` file in the `dashboard/` directory:
   ```bash
   cd dashboard
   cp .env.example .env
   ```
   
   Edit `.env` and add your Twelve Data API key:
   ```env
   MARKET_API_KEY=your_api_key_here
   MARKET_WS_ADDRESS=wss://ws.twelvedata.com/v1
   MARKET_REST_ADDRESS=https://api.twelvedata.com
   MARKET_REAL_TIME_PRICE_ROUTE=/quotes/price
   MARKET_EOD_ROUTE=/eod
   MARKET_QUOTE_ROUTE=/quote
   MARKET_INDICES_ROUTE=/indices
   MARKET_STOCKS_ROUTE=/stocks
   ```

3. **Install Trunk** (if not already installed)
   ```bash
   cargo install trunk
   ```

## Running

Run the application with the Trunk development server:

```bash
cd dashboard
trunk serve --open
```

The application will be available at `http://127.0.0.1:8080`

## Building for Production

```bash
cd dashboard
trunk build --release
```

The production build will be available in `dashboard/dist/`

## Project Structure

```
market_aggregator/
├── dashboard/              # Main Yew application
│   ├── src/
│   │   ├── common/        # Shared types and utilities
│   │   ├── components/    # UI components
│   │   ├── services/      # API and WebSocket services
│   │   └── main.rs        # Application entry point
│   ├── .env               # Environment variables (not in git)
│   └── Cargo.toml
├── Cargo.toml             # Workspace configuration
└── README.md
```

## Technologies

- **Yew** - Rust/WebAssembly framework for building web apps
- **Gloo** - Toolkit for Rust/WASM development
- **Twelve Data API** - Financial market data provider
- **WebSockets** - Real-time data streaming

## License

MIT OR Apache-2.0

## Author

Maksim Shubkin <mmshubkin@gmail.com>
