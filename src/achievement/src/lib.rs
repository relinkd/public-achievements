#![doc = include_str!("../README.md")]

pub mod ecdsa;
pub mod storable;
pub mod access;
pub mod state;
pub mod logic;

use candid::Principal;
use storable::*;

ic_cdk::export_candid!();