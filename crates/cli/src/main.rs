//Updation : tWO MODES are added regular mode for only CVE and interesting keywords  , INsane Verbose mode Every single details about...  date : 1/6/2026
//Both of them need api_key ; 

mod args;
mod runner;
mod output;
mod banner;
use banner::print_banner;
use chrono::Utc;

use anyhow::{Context, Result};
use clap::Parser;
use tracing_subscriber::{fmt, EnvFilter};

use args::{Cli, Commands};
use runner::run_scan;

use reqwest::Client;
use serde::Deserialize;
use std::env;
use tokio::time::{sleep,Duration};
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
pub async fn fetch_cves(cpe: &str,api_key: &str) -> Result<NvdResponse> {
    // Time window: last 30 days
    let end_date = Utc::now();
    let start_date = end_date - chrono::Duration::days(30);

    let url = "https://services.nvd.nist.gov/rest/json/cves/2.0";
    let client = Client::new();

    // Famous + interesting keywords since 2020 ans so on date 1/5/26 
    let keywords = vec![
        // 2020
        "SolarWinds supply chain breach",
        "Microsoft Exchange Server vulnerabilities",
        "Zoom security flaws",
        "Ransomware Ryuk Maze",
        "Privilege escalation Linux kernel",
        // 2021
        "Log4Shell Log4j remote code execution",
        "Kaseya VSA ransomware",
        "Colonial Pipeline attack",
        "ProxyShell Exchange exploit",
        "Zero-day Chrome browser",
        // 2022
        "Spring4Shell Java RCE",
        "Follina Microsoft Office exploit",
        "Confluence RCE Atlassian",
        "OpenSSL critical vulnerability",
        "CISA KEV catalog exploits",
        // 2023
        "MOVEit Transfer data breach",
        "Barracuda ESG zero-day",
        "Citrix Bleed vulnerability",
        "Fortinet SSL VPN exploit",
        "Windows SmartScreen bypass",
        // 2024
        "XZ Utils backdoor Linux",
        "Ivanti VPN zero-day",
        "Cisco IOS XE implant",
        "Okta breach identity compromise",
        "AI-powered phishing exploits",
        // 2025 (OWASP Top 10)
        "Broken Access Control",
        "Security Misconfiguration",
        "Software Supply Chain Failures",
        "Cryptographic Failures",
        "Injection attacks",
        "Authentication Failures",
        "Insecure Design",
        "Data Integrity Failures",
        "Logging and Alerting Failures",
        "Exceptional Condition Mishandling",
    ];

    // Collect all vulnerabilities here
    let mut all_vulns: Vec<VulnerabilityItem> = Vec::new();

    for kw in keywords {
        let resp = client
            .get(url)
            .query(&[
                ("resultsPerPage", "50".to_string()),
                ("keywordSearch", kw.to_string()),
                ("pubStartDate", start_date.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()),
                ("pubEndDate", end_date.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()),
            ])
            .header("apiKey", api_key)
            .header("User-Agent", "vajra-scanner/1.0")
            .send()
            .await
            .context("Failed to send request to NVD")?;

        if !resp.status().is_success() {
            return Err(anyhow::anyhow!("NVD HTTP {}", resp.status()));
        }

        let parsed: NvdResponse = resp
            .json()
            .await
            .context("Failed to parse NVD JSON")?;

        // Merge vulnerabilities into one vector
        all_vulns.extend(parsed.vulnerabilities);
        if api_key.is_empty(){
        sleep(Duration::from_secs(30)).await
        }

    }

    // Return combined response
    Ok(NvdResponse {
        vulnerabilities: all_vulns,
    })
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
            depth, // <-- depth flag integrated
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
            if depth{
           println!("\n[+] Going Insane VERBOSE mode...");
            }
            else {
                println!("\n[+] Regular Mode..");
            }
            //let api_key = String::new(); // API key disabled FOR (dev mode)
            
            // if api_key.is_empty(){ // if somebody doesn't have api_key 
            // println!("\n[+] Vulnerability scan is still going on please wait ...\n [Register API key for faster Results]");
            //  }

             // Load API key from environment variable, default to empty string if not set
                let api_key = env::var("nvd_api").unwrap_or_else(|_| String::new());

                if api_key.is_empty() {
                    println!("\n[+] No API key detected. Running in slow mode (rate-limited)...");
                    println!("[!] Register an NVD API key for faster results.");
                } else {
                    println!("\n[+] API key detected ");
                }


            
            let _cpe_target = "cpe:2.3:a:openssl:openssl:1.1.1:*:*:*:*:*:*:*";

            match fetch_cves("",&api_key).await {
                Ok(response) => {
                    // ===== Updated here to count vulnerabilities =====
                    let vuln_count = response.vulnerabilities.len();
                    println!("\n[+] Vulnerabilities Found: {}\n", vuln_count);

                    if vuln_count == 0 {
                        println!("[-] No CVEs found for this CPE");
                        // fallback list of interesting CVEs
                        println!("[*] Showing recent interesting CVEs since 2020:");
                        for kw in ["Log4Shell", "Spring4Shell", "XZ Utils backdoor", "ProxyShell"] {
                            println!("  - {}", kw);
                        }
                    } else {
                        for item in response.vulnerabilities {
                            println!("---------------------------------------------");
                            println!("CVE ID: {}\n", item.cve.id);

                            if depth {
                                // full verbose mode
                                for desc in item.cve.descriptions {
                                    if desc.lang == "en" {
                                        println!("  Description: {}\n", desc.value);
                                        print_wrapped(&desc.value, 40 , " ");
                                    }
                                }
                            } else {
                                // short mode: just first Intersting aspects of vulnerabilties
                                if let Some(desc) = item.cve.descriptions.iter().find(|d| d.lang == "en") {
                                    let short = desc.value.split('.').next().unwrap_or(&desc.value);
                                    println!("  Summary: {}", short);
                                }
                            }

                            println!("----------------------------------------------------");
                        }
                        println!("\n-------------------------------------Vajra Made of LOVE------------------------------------------\n");
                    }
                }
                Err(err) => {
                    eprintln!("[-] ERROR: Failed to fetch CVEs: {:?}\n", err);//Falls back to this if api is not set ;
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

pub fn print_wrapped(text: &str, width: usize, indent: &str) {
    let mut count = 0;

    print!("{}", indent);

    for ch in text.chars() {
        if count > 0 && count % width == 0 {
            println!();
            print!("{}", indent);
        }

        print!("{}", ch);
        count += 1;
    }

    println!();
}

//THese are crucial for no old or outdated data but rn it's not working we will get into there 
//.header("User-Agent", "vajra-scanner/1.0")

// ("pubStartDate", "2020-01-01T00:00:00.000Z"),
//("pubEndDate", "2025-12-22T00:00:00.000Z"),
