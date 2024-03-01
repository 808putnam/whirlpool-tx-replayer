use anyhow::{anyhow, Result};
use dirs::home_dir;
use std::path::{Path};
use std::process::{Child, Stdio};
use std::io::prelude::*;
use std::fs::{self, File};
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::{Keypair, Signer};

pub const STARTUP_WAIT: i32 = 5000;
pub const SHUTDOWN_WAIT: i32 = 2000;

// Version of the docker image.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const DOCKER_BUILDER_VERSION: &str = VERSION;


// Builds, deploys, and tests all workspace programs in a single command.
#[allow(clippy::too_many_arguments)]
pub fn test(
    skip_local_validator: bool,
    detach: bool,
) -> Result<()> {
    run_test_suite(
        skip_local_validator,
        detach
    )?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn run_test_suite(
    skip_local_validator: bool,
    detach: bool,
) -> Result<()> {
    let mut validator_handle = None;
    if !skip_local_validator {
        validator_handle = Some(start_test_validator(false)?);
    }

    // Keep validator running if needed.
    if detach {
        println!("Local validator still running. Press Ctrl + C quit.");
        std::io::stdin().lock().lines().next().unwrap().unwrap();
    }

    // Check all errors and shut down.
    if let Some(mut child) = validator_handle {
        if let Err(err) = child.kill() {
            println!("Failed to kill subprocess {}: {}", child.id(), err);
        }
    }

    Ok(())
}

fn start_test_validator(test_log_stdout: bool
) -> Result<Child> {
    let (test_ledger_directory, test_ledger_log_filename) =
        test_validator_file_paths();

    // Start a validator for testing.
    let (test_validator_stdout, test_validator_stderr) = match test_log_stdout {
        true => {
            let test_validator_stdout_file = File::create(&test_ledger_log_filename)?;
            let test_validator_sterr_file = test_validator_stdout_file.try_clone()?;
            (
                Stdio::from(test_validator_stdout_file),
                Stdio::from(test_validator_sterr_file),
            )
        }
        false => (Stdio::inherit(), Stdio::inherit()),
    };

    let rpc_url = test_validator_rpc_url();

    let rpc_port = solana_sdk::rpc_port::DEFAULT_RPC_PORT;
    if !portpicker::is_free(rpc_port) {
        return Err(anyhow!(
            "Your configured rpc port: {rpc_port} is already in use"
        ));
    }
    let faucet_port = solana_faucet::faucet::FAUCET_PORT;
    if !portpicker::is_free(faucet_port) {
        return Err(anyhow!(
            "Your configured faucet port: {faucet_port} is already in use"
        ));
    }

    let mut validator_handle = std::process::Command::new("solana-test-validator")
        .arg("--ledger")
        .arg(test_ledger_directory)
        .arg("--mint")
        .arg(wallet_kp()?.pubkey().to_string())
        .stdout(test_validator_stdout)
        .stderr(test_validator_stderr)
        .spawn()
        .map_err(|e| anyhow::format_err!("{}", e.to_string()))?;

    // Wait for the validator to be ready.
    let client = create_client(rpc_url);
    let mut count = 0;
    let ms_wait = STARTUP_WAIT;
    while count < ms_wait {
        let r = client.get_latest_blockhash();
        if r.is_ok() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
        count += 1;
    }
    if count == ms_wait {
        eprintln!(
            "Unable to get latest blockhash. Test validator does not look started. Check {test_ledger_log_filename} for errors. Consider increasing [test.startup_wait] in Anchor.toml."
        );
        validator_handle.kill()?;
        std::process::exit(1);
    }
    Ok(validator_handle)
}

// Setup and return paths to the solana-test-validator ledger directory and log
// files given the configuration
fn test_validator_file_paths() -> (String, String) {

    let ledger_directory = ".anchor/test-ledger";

    if !Path::new(&ledger_directory).is_relative() {
        // Prevent absolute paths to avoid someone using / or similar, as the
        // directory gets removed
        eprintln!("Ledger directory {ledger_directory} must be relative");
        std::process::exit(1);
    }
    if Path::new(&ledger_directory).exists() {
        fs::remove_dir_all(ledger_directory).unwrap();
    }

    fs::create_dir_all(ledger_directory).unwrap();

    (
        ledger_directory.to_string(),
        format!("{ledger_directory}/test-ledger-log.txt"),
    )
}

// Return the URL that solana-test-validator should be running on given the
// configuration
fn test_validator_rpc_url() -> String {
        "http://127.0.0.1:8899".to_string()
}

/// Create a new [`RpcClient`] with `confirmed` commitment level instead of the default(finalized).
fn create_client<U: ToString>(url: U) -> RpcClient {
    RpcClient::new_with_commitment(url, CommitmentConfig::confirmed())
}

pub fn wallet_kp() -> Result<Keypair> {

    let default_wallet = ".config/solana/Ko64NSYFCqstSbQCHSxmTKrgTZURHVjaR2SDuxzREjr.json".to_string();
    let wallet_path = home_dir().unwrap().join(default_wallet).display().to_string();
    solana_sdk::signature::read_keypair_file(wallet_path)
        .map_err(|_| anyhow!("Unable to read keypair file"))
}
