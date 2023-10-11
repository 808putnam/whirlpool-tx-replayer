use std::borrow::BorrowMut;
use std::{fs::File, collections::BTreeMap};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use csv;
use serde_derive::{Deserialize, Serialize};
use serde::de;
use serde_json;
use base64::{Engine, prelude::BASE64_STANDARD};

use mysql::*;
use mysql::prelude::*;

mod errors;
mod decoded_instructions;
mod util_database_io;
mod util_file_io;

use decoded_instructions::{from_json, DecodedWhirlpoolInstruction};

#[derive(Debug, PartialEq, Eq)]
struct Slot {
    slot: u64,
    block_height: u64,
    block_time: i64,
}


fn main() {
    let url = "mysql://root:password@localhost:3306/localtest";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let start_snapshot_file = "data/whirlpool-snapshot-215135999.csv.gz";

    // TODO: protect account_map (stop using HashMap directly)
    let account_map = util_file_io::load_from_snapshot_file(&start_snapshot_file.to_string());
    println!("loaded {} accounts", account_map.len());

    let selected_slots = conn.query_map(
        "SELECT slot, blockHeight, blockTime FROM slots WHERE slot > 215135999 ORDER BY slot ASC LIMIT 10",
        |(slot, block_height, block_time)| {
            Slot {
                slot,
                block_height,
                block_time,
            }
        },
    ).unwrap();

    for slot in selected_slots {
        println!("processing slot = {:?} ...", slot);

        let txid_start = slot.slot << 24;
        let txid_end = ((slot.slot + 1) << 24) - 1;
        println!("  start: {:?}, end: {:?}", txid_start, txid_end);

        let ixs_in_slot = util_database_io::fetch_ixs_in_slot(slot.slot, &mut conn);

        for ix in ixs_in_slot {
            match ix.ix {
            decoded_instructions::DecodedWhirlpoolInstruction::Swap(detail) => {
                println!("    {:?}", detail);
            },
            decoded_instructions::DecodedWhirlpoolInstruction::IncreaseLiquidity(detail) => {
                println!("    {:?}", detail);
            },
            _ => {},
            }
        }
    }
}
