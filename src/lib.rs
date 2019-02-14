#[macro_use]
extern crate structopt;

#[macro_use]
extern crate serde_derive;

use csv;
use std::io;
use std::process;

use structopt::StructOpt;
use std::error::Error;

use std::collections::{HashMap, HashSet};

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
        #[structopt(short = "m", long = "master", default_value = "SupplyInventory.csv")]
        supply_filename: String,
    },
}

/**
 * CSV Structs
 */

#[derive(Debug, Deserialize)]
pub struct MasterPart {
    pub VenCode: String,
    pub PartNumber: i32,
    pub SKU: String,
    pub TotalQty: i32,
}

impl MasterPart {

    /// Updates TotalQty with a supplied quantity, mutating the instance in place
    pub fn update_qty(&mut self, qty: i32) {
        self.TotalQty = qty;
    }
}

#[derive(Debug, Deserialize)]
struct SupplyPart {
    VenCode: String,
    PartNumber: i32,
    LongDescription: String,
    JobberPrice: f32,
    Fedexable: bool,
    ExeterQty: i32,
    TotalQty: i32,
    UpCharge: f32,
    AAIACode: String,
}

/**
 * MasterCache & Methods
 */

/// Holds the master inventory in a hashmap, where the key is the VenCode, and the value is a
/// MasterPart struct.  This is for efficiency when searching through the master cache.
pub struct MasterCache {
    pub products: HashMap<String, MasterPart>
}

impl MasterCache {

    /// Takes in a filename for a MasterInventory csv file, and on success, returns
    /// a MasterCache struct instance.
    pub fn from(filename: &str) -> Result<MasterCache, Box<dyn Error>> {
        let mut rdr = csv::Reader::from_path(filename)?;
        let mut products = HashMap::new();

        for result in rdr.deserialize() {
            let product: MasterPart = result?;
            let ven_code = product.VenCode.clone();
            
            products.insert(ven_code, product);
        }

        Ok(MasterCache {
            products,
        })
    }

    pub fn write_csv(&self) -> Result<(), Box<dyn Error>> {


        Ok(())
    }
}

/// `run` will take in an InventoryConfig enum config (parsed in `main`) and execute the appropriate
/// program logic
pub fn run(config: InventoryConfig) -> Result<(), Box<dyn Error>> {
    match config {
        InventoryConfig::Generate { master_filename, supply_filename } => update_master(master_filename, supply_filename)?,
    }

    Ok(())
}

pub fn update_master(master_filename: String, supply_filename: String) -> Result<(), Box<dyn Error>> {
    let mut master_cache = MasterCache::from(&master_filename)?.products;
    let mut no_ven_codes = HashSet::new();

    let mut rdr = csv::Reader::from_path(supply_filename)?;

    for result in rdr.deserialize() {
        let product: SupplyPart = result?;
        let ven_code = &product.VenCode;
        let product_qty = product.TotalQty;

        // continue looping if we've already seen this ven_code, and it's not in our 
        // master_cache
        if no_ven_codes.contains(ven_code) { continue };
        
        if let Some(master_product) = master_cache.get_mut(ven_code) {
            master_product.update_qty(product_qty);
        } else {
            no_ven_codes.insert(ven_code);
        }
    }


    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
