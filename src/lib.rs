//! # Inventory Management
//! 
//! This crate is a port of an existing python project. It allows one to sync the current
//! quantities from a supply inventory csv file to a filtered down view they have created of their
//! own truncated "master inventory" (their own inventory)
//! 
//! ### This program will update the quantity in master inventory with the quantity found in supply inventory based for each part in master inventory
//! 
//! 
//! - The default filenames that program accepts are:  
//!    - "SupplyInventory.csv" for the _supply list_  
//!    - "MasterInventory.csv" for the _master list_   
//! - _You can set your own filenames but you must specify their name and location in command line arguments_
//!    - _See how to use command line arguments below_
//! 
//! 
//! - Both csv files must include the following columns:  
//! 
//!    | VenCode | PartNumber | TotalQty |  
//!    | ------- |:----------:|:--------:|  
//! 
//! ## Installation
//!
//! You can install this application in one of two ways.  Either clone the repo and build the
//! release version with cargo, or simply use cargo install:
//!
//! ```terminal
//! $ cargo install inventory-mgt
//! ```
//! 
//! ## Use
//!
//! To use this application, simply run it with the generate command to generate a new master csv
//! with updated quantity fields:
//!
//! ```terminal
//! $ inventory-mgt generate
//! ```
//!
//! Make sure you are in the root folder where your `SupplyInventory.csv` and `MasterInventory.csv`
//! files are. You can grab sample ones from the github repo.
//!
//! Optionally you can pass in custom filenames with flags after the `generate` command:
//!
//! ```terminal
//! $ inventory-mgt generate -m masterinv.csv -s supplyinv.csv
//! ```
//!
//! That's it! Enjoy!

#[macro_use]
extern crate structopt;

#[macro_use]
extern crate serde_derive;

use csv;

use structopt::StructOpt;
use std::error::Error;
use std::str;

use indexmap::map::IndexMap;

/**
 * Command Line Parsing
 */

#[derive(StructOpt, Debug)]
#[structopt(
    name = "inventorymgt",
    about = "Updates Master Inventory from Supply List CSV.",
    long_about = "You can use this terminal program to populate a new csv, products from master inventory and quantitties from the newest supply inventory csv."
)]
/// InventoryConfig provides a structure for structopt to take in commands.
pub enum InventoryConfig {
    #[structopt(name = "generate")]
    /// Generates a new csv file with updated quantities from the supply csv.
    Generate {
        /// Specifies the filename for master inventory csv and defaults to MasterInventory.csv.
        #[structopt(short = "m", long = "master", default_value = "MasterInventory.csv")]
        master_filename: String,

        /// specifies the filename for supply inventory csv and defaults to SupplyInventory.csv
        #[structopt(short = "s", long = "supply", default_value = "SupplyInventory.csv")]
        supply_filename: String,
    },
}

/**
 * CSV Structs
 */

/// Provides the structures for a part from the Master csv list
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MasterPart {
    pub ven_code: String,
    pub part_number: String,
    #[serde(rename = "SKU")]
    pub sku: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    pub total_qty: Option<i32>,
}

impl MasterPart {

    /// Updates total_qty with a supplied quantity, mutating the instance in place
    pub fn update_qty(&mut self, qty: i32) {
        self.total_qty = Some(qty);
    }
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct SupplyPart<'a> {
    ven_code: &'a str,
    part_number: &'a [u8],
    total_qty: i32,
}

/**
 * MasterCache & Methods
 */

/// Holds the master inventory in a hashmap, where the key is the ven_code, and the value is a
/// MasterPart struct.  This is for efficiency when searching through the master cache.
pub struct MasterCache {
    pub products: IndexMap<String, Vec<MasterPart>>
}

impl MasterCache {

    /// Takes in a filename for a MasterInventory csv file, and on success, returns
    /// a MasterCache struct instance.
    pub fn from(filename: &str) -> Result<MasterCache, Box<dyn Error>> {
        let mut rdr = csv::Reader::from_path(filename)?;
        let mut products: IndexMap<String, Vec<MasterPart>> = IndexMap::new();

        for result in rdr.deserialize() {
            let product: MasterPart = result?;
            let ven_code = product.ven_code.clone();

            if let Some(v_code) = products.get_mut(&ven_code) {
                v_code.push(product);
            } else {
                products.insert(ven_code, vec![product]);
            }
        }

        Ok(MasterCache {
            products,
        })
    }

    /// Writes a new csv file from a MasterCache instance
    pub fn write_csv(&self, filename: &'static str) -> Result<(), Box<dyn Error>> {
        let mut wtr = csv::Writer::from_path(filename)?;

        for products in self.products.values() {
            for product in products {
                wtr.serialize(product)?;
            }
        }

        wtr.flush()?;
        Ok(())
    }
}

/// Creates a new master csv file called "newmaster.csv" with the updated quantities, pulled
/// from the supply csv file
pub fn update_master(master_filename: String, supply_filename: String) -> Result<(), Box<dyn Error>> {
    let mut master_cache = MasterCache::from(&master_filename)?;

    let mut rdr = csv::Reader::from_path(supply_filename)?;
    let mut raw_record = csv::ByteRecord::new();
    let headers = rdr.byte_headers()?.clone();

    while rdr.read_byte_record(&mut raw_record)? {
        let product: SupplyPart = raw_record.deserialize(Some(&headers))?;
        let ven_code = product.ven_code;
        let product_qty = product.total_qty;
        
        if let Some(v_code) = master_cache.products.get_mut(ven_code) {
            
            // this ven_code is in our master_cache so let's see
            // if the product is there and update it's quantity
            if let Some(master_product) = v_code.iter_mut().find(|p| p.part_number == str::from_utf8(product.part_number).unwrap()) {
                master_product.update_qty(product_qty);
            }
        }
    }
    

    // lastly let's write the updated master supply list
    master_cache.write_csv("newmaster.csv")?;

    Ok(())
}

/// `run` will take in an InventoryConfig enum config (parsed in `main`) and execute the appropriate
/// program logic
pub fn run(config: InventoryConfig) -> Result<(), Box<dyn Error>> {
    match config {
        InventoryConfig::Generate { master_filename, supply_filename } => update_master(master_filename, supply_filename)?,
    }

    Ok(())
}
