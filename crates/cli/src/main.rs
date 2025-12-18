mod args;
mod runner;
mod output;
mod banner;
use banner::print_banner;

use anyhow::{Context, Result};
use clap::Parser;
use tracing_subscriber::{fmt, EnvFilter};

use args::{Cli, Commands};
use runner::run_scan;

use reqwest::Client;
use serde::Deserialize;
use std::env;

// ------------------------------------------------------
// NVD API STRUCTURES (Correct for NVD API 2.0)
// ------------------------------------------------------
#[derive(Debug, Deserialize)]
pub struct NvdResponse {
    #[serde(default)]
    pub vulnerabilities: Vec<VulnerabilityItem>,
}

#[derive(Debug, Deserialize)]
pub struct VulnerabilityItem {
    pub cve: Cve,
}

#[derive(Debug, Deserialize)]
pub struct Cve {
    pub id: String,

    #[serde(default)]
    pub descriptions: Vec<Description>,
}

#[derive(Debug, Deserialize)]
pub struct Description {
    pub lang: String,
    pub value: String,
}

// ------------------------------------------------------
// Fetch CVEs from NVD
// ------------------------------------------------------
pub async fn fetch_cves(cpe: &str, api_key: &str) -> Result<NvdResponse> {
    let url = "https://services.nvd.nist.gov/rest/json/cves/2.0";

    let client = Client::new();

    let parsed: NvdResponse = client
        .get(url)
        .query(&[
            ("cpeName", cpe),
            ("resultsPerPage", "5"),
        ])
        .header("apiKey", api_key)
        .header("Content-Type", "application/json")
        .send()
        .await
        .context("Failed to send request database")?
        .json()
        .await
        .context("Failed to parse database JSON")?;

    Ok(parsed)
}

// ------------------------------------------------------
// Main entry point
// ------------------------------------------------------
#[tokio::main]
async fn main() -> Result<()> {
    print_banner();
    println!("Vajra Vulnerability scanner is starting ...");
    let cli = Cli::parse();
    init_logging(cli.verbose);

    match cli.command {
        Commands::Scan {
            targets,
            ports,
            concurrency,
            rate_limit,
            timeout,
            banner_timeout,
            output_format,
            scan_type,
            preset,
        } => {
            // ---------------------------
            // Run your existing scanner
            // ---------------------------
            run_scan(
                targets.clone(),
                ports,
                concurrency,
                rate_limit,
                timeout,
                banner_timeout,
                output_format,
                preset,
                Some(scan_type),
            )
            .await?;

            // -------------------------------------------------
            // Fetch CVEs from NVD after scan
            // -------------------------------------------------
            println!("\n[+] Fetching CVEs from Databse...");

            // Load API key from environment variable
            let api_key = env::var("Api_key").context("Api-Key is not set properly set !!!")?;

            // Test CPE — real values later
            let cpe_target = "cpe:2.3:a:openssl:openssl:1.1.1:*:*:*:*:*:*:*";

            match fetch_cves(&cpe_target, &api_key).await {
                Ok(response) => {
                    // ===== Updated here to count vulnerabilities =====
                    let vuln_count = response.vulnerabilities.len();
                    println!("\n[+] Vulnerabilities Found: {}\n", vuln_count);

                    if vuln_count == 0 {
                        println!("[-] No CVEs found for this CPE");
                    } else {
                        for item in response.vulnerabilities {
                            println!("---------------------------------------------");
                            println!("CVE ID: {}\n", item.cve.id);

                            for desc in item.cve.descriptions {
                                if desc.lang == "en" {
                                    println!("  Description: {}\n", desc.value);
                                }
                            }
                            println!("---------------------------------------------");
                        }
                    }
                }
                Err(err) => {
                    eprintln!("[-] ERROR: Failed to fetch CVEs: {:?}\n", err);
                }
            }
        }
    }

    Ok(())
}

// ------------------------------------------------------
// Logging system
// ------------------------------------------------------
fn init_logging(verbose: u8) {
    if verbose == 0 {
        return; // No logging by default
    }

    let log_level = match verbose {
        1 => "info",
        _ => "debug",
    };

    let filter = EnvFilter::new(log_level);

    fmt()
        .with_env_filter(filter)
        .compact()
        .init();
    println!("\n");
}
