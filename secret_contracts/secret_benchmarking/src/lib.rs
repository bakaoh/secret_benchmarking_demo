// Built-In Attributes
#![no_std]

// Imports
extern crate eng_wasm;
extern crate eng_wasm_derive;

use eng_wasm::*;
use eng_wasm_derive::pub_interface;
use std::collections::HashMap;

// Encrypted state keys
static DATASET_COLLECTION: &str = "dataset_collection";
static NAME_LIST: &str = "name_list";

// Public struct Contract which will consist of private and public-facing secret contract functions
pub struct Contract;

// Private functions accessible only by the secret contract
impl Contract {
    fn get_dataset_collection() -> HashMap<String, Vec<U256>> {
        read_state!(DATASET_COLLECTION).unwrap_or_default()
    }
}

// Public trait defining public-facing secret contract functions
#[pub_interface]
pub trait ContractInterface{
    fn construct();
    fn submit_dataset(name: String, dataset: Vec<U256>);
    fn submit_quote(name: String, quote: U256) -> U256;
    fn get_name_list() -> String;
}

// Implementation of the public-facing secret contract functions defined in the ContractInterface
// trait implementation for the Contract struct above
impl ContractInterface for Contract {
    #[no_mangle]
    fn construct() {
        write_state!(NAME_LIST => String::new());
    }

    #[no_mangle]
    fn submit_dataset(name: String, dataset: Vec<U256>) {
        let mut collection = Self::get_dataset_collection();
        assert!(!collection.contains_key(&name), "dataset name is dupplicated");
        let mut namelist = Self::get_name_list();
        collection.insert(name.clone(), dataset);
        namelist.push_str(name.as_str());
        namelist.push(',');
        write_state!(DATASET_COLLECTION => collection);
        write_state!(NAME_LIST => namelist);        
    }

    #[no_mangle]
    fn submit_quote(name: String, quote: U256) -> U256 {
        let collection = Self::get_dataset_collection();
        let dataset = collection.get(&name).expect("dataset name is not found");        
        let gt_count = dataset.iter().filter(|x| **x <= quote).count();
        let percentile = (gt_count as f64) * 100.0 / (dataset.len() as f64);
        U256::from(percentile.round() as i64)
    }

    #[no_mangle]
    fn get_name_list() -> String {
        read_state!(NAME_LIST).unwrap_or_default()
    }
}