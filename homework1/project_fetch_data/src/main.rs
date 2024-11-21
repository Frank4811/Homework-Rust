use std::{fs::OpenOptions, io::Write};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::thread;
use std::time::Duration;

// Define the trait Pricing which requires implementing methods for fetching and saving prices
pub trait Pricing {
    fn fetch_price(&self) -> f32; // Method to fetch the price
    fn save_to_file(&self, price: f32); // Method to save the fetched price to a file
}

// Struct for Bitcoin, representing the JSON response for Bitcoin price
#[derive(Deserialize, Serialize)]
struct BitcoinPrice {
    bitcoin: PriceDetails, // The 'bitcoin' field contains the price details
}

// A helper struct for the price details (just the USD price in this case)
#[derive(Deserialize, Serialize)]
struct PriceDetails {
    usd: f32, // The price in USD
}

// Struct representing Bitcoin and its related data like API address and file name
pub struct Bitcoin {
    api_address: String,
    file_name: String,
}

// Implementation of methods for Bitcoin
impl Bitcoin {
    // Constructor to create a new Bitcoin instance with the provided API address and file name
    pub fn new(api_address: &str, file_name: &str) -> Self {
        Self {
            api_address: api_address.to_string(),
            file_name: file_name.to_string(),
        }
    }
}

// Implementing the Pricing trait for Bitcoin
impl Pricing for Bitcoin {
    // Fetching the price of Bitcoin from the CoinGecko API
    fn fetch_price(&self) -> f32 {
        let response = ureq::get(&self.api_address)
            .call()
            .unwrap()  // Make sure the request succeeds
            .into_string()
            .unwrap(); // Convert the response into a string
        
        println!("Raw response body: {}", response);  // For debugging

        // Deserialize the JSON response into the BitcoinPrice struct
        let price: BitcoinPrice = from_str(&response).unwrap();
        price.bitcoin.usd  // Extract the USD price from the response
    }

    // Save the fetched price to a file
    fn save_to_file(&self, price: f32) {
        let mut file = OpenOptions::new()
            .create(true) // Create the file if it doesn't exist
            .append(true) // Append to the file if it exists
            .open(&self.file_name)
            .unwrap();
        
        // Write the price to the file
        writeln!(file, "Bitcoin price: {}", price).unwrap();
        println!("Saved Bitcoin price to {}", self.file_name);
    }
}

// Struct for Ethereum, representing the JSON response for Ethereum price
#[derive(Deserialize, Serialize)]
struct EthereumPrice {
    ethereum: PriceDetails,  // The 'ethereum' field contains the price details
}

// Struct representing Ethereum and its related data
pub struct Ethereum {
    api_address: String,
    file_name: String,
}

// Implementation of methods for Ethereum
impl Ethereum {
    // Constructor to create a new Ethereum instance
    pub fn new(api_address: &str, file_name: &str) -> Self {
        Self {
            api_address: api_address.to_string(),
            file_name: file_name.to_string(),
        }
    }
}

// Implementing the Pricing trait for Ethereum
impl Pricing for Ethereum {
    // Fetching the price of Ethereum from the CoinGecko API
    fn fetch_price(&self) -> f32 {
        let response = ureq::get(&self.api_address)
            .call()
            .unwrap()
            .into_string()
            .unwrap();
        
        println!("Raw response body: {}", response);  // For debugging

        // Deserialize the JSON response into the EthereumPrice struct
        let price: EthereumPrice = from_str(&response).unwrap();
        price.ethereum.usd  // Extract the USD price from the response
    }

    // Save the fetched price to a file
    fn save_to_file(&self, price: f32) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_name)
            .unwrap();
        
        // Write the price to the file
        writeln!(file, "Ethereum price: {}", price).unwrap();
        println!("Saved Ethereum price to {}", self.file_name);
    }
}

// Struct for S&P 500 response with nested data structure
#[derive(Deserialize)]
struct Sp500Response {
    chart: Chart, // Contains the chart data
}

// Represents the chart containing multiple results
#[derive(Deserialize)]
struct Chart {
    result: Vec<ChartResult>, // A vector of ChartResult, each representing a different time period
}

// Represents each result in the chart, containing indicators and quotes
#[derive(Deserialize)]
struct ChartResult {
    indicators: Indicators, // Contains the price indicators for the S&P 500
}

// Represents the indicators for the S&P 500, specifically the quote data
#[derive(Deserialize)]
struct Indicators {
    quote: Vec<Quote>, // A vector of Quote, each representing a price at a specific time
}

// Represents a single quote, including the close price
#[derive(Deserialize)]
struct Quote {
    close: Vec<Option<f32>>, // Close price for the S&P 500, using Option to handle null values
}

// Struct for S&P 500, representing the API address and file name
pub struct Sp500 {
    api_address: String,
    file_name: String,
}

// Implementation of methods for S&P 500
impl Sp500 {
    // Constructor to create a new S&P 500 instance
    pub fn new(api_address: &str, file_name: &str) -> Self {
        Self {
            api_address: api_address.to_string(),
            file_name: file_name.to_string(),
        }
    }
}

// Implementing the Pricing trait for S&P 500
impl Pricing for Sp500 {
    // Fetch the latest price for the S&P 500 from the Yahoo Finance API
    fn fetch_price(&self) -> f32 {
        let response = ureq::get(&self.api_address)
            .call()
            .unwrap()
            .into_string()
            .unwrap();

        // Parse the JSON response
        let data: Sp500Response = serde_json::from_str(&response).unwrap();

        // Extract the latest closing price from the nested data structure
        let latest_price = data.chart.result[0]
            .indicators
            .quote[0]
            .close
            .iter()
            .filter_map(|&price| price) // Filter out `None` values
            .last()
            .expect("No valid closing prices found.");

        latest_price
    }

    // Save the fetched price to a file
    fn save_to_file(&self, price: f32) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_name)
            .unwrap();

        // Write the price to the file
        writeln!(file, "S&P 500 price: {}", price).unwrap();
        println!("Saved S&P 500 price to {}", self.file_name);
    }
}

// Main function that runs the application in a loop, fetching and saving prices
fn main() {
    // Create instances for Bitcoin, Ethereum, and S&P 500
    let bitcoin = Bitcoin::new("https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd", "bitcoin_price.txt");
    let ethereum = Ethereum::new("https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd", "ethereum_price.txt");
    let sp500 = Sp500::new("https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC?interval=1m&range=1d", "sp500_price.txt");

    loop {
        // Fetch the prices for Bitcoin, Ethereum, and S&P 500
        let bitcoin_price = bitcoin.fetch_price();
        let ethereum_price = ethereum.fetch_price();
        let sp500_price = sp500.fetch_price();

        // Save the fetched prices to their respective files
        bitcoin.save_to_file(bitcoin_price);
        ethereum.save_to_file(ethereum_price);
        sp500.save_to_file(sp500_price);

        // Optionally print the prices to the console
        println!("Bitcoin Price: {}", bitcoin_price);
        println!("Ethereum Price: {}", ethereum_price);
        println!("Sp500 Price: {}", sp500_price);

        // Sleep for 10 seconds before the next fetch
        thread::sleep(Duration::new(10, 0));
    }
}
