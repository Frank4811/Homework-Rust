# Crypto and Stock Price Fetcher

This project fetches the latest prices of Bitcoin, Ethereum, and the S&P 500 Index. It uses APIs to retrieve data in real-time and saves it to files for later analysis or review.

## Features
- Fetches live Bitcoin and Ethereum prices from the CoinGecko API.
- Retrieves the latest S&P 500 Index price using Yahoo Finance's API.
- Saves fetched prices to separate text files:
  - `bitcoin_price.txt`
  - `ethereum_price.txt`
  - `sp500_price.txt`
- Regularly updates prices every 10 seconds.
- Outputs prices to the console for real-time monitoring.

## Requirements
- Rust programming language installed on your system.
- An internet connection for API access.
- The following Rust crates:
  - `serde` for data serialization.
  - `serde_json` for JSON parsing.
  - `ureq` for HTTP requests.

## Files Created
- bitcoin_price.txt: Stores the latest Bitcoin prices.
- ethereum_price.txt: Stores the latest Ethereum prices.
- sp500_price.txt: Stores the latest S&P 500 prices.
- Each line in these files represents a price fetched at a specific time.

## Code Overview
- Bitcoin and Ethereum Structs
- Fetch and process live cryptocurrency prices using the CoinGecko API.
- Sp500 Struct
- Fetches and processes the latest S&P 500 Index price using Yahoo Finance's API.
- Pricing Trait
- Defines methods for fetching and saving prices, ensuring consistency across structs.
- Main Function
- Continuously fetches and saves prices in 10-second intervals.