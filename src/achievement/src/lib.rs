pub mod ecdsa;

use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use ic_cdk::{query, update};
use std::convert::TryFrom;
use std::str::FromStr;

ic_cdk::export_candid!();