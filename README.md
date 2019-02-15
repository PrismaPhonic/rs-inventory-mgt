## Inventory Management
[![Build Status](https://travis-ci.org/PrismaPhonic/rs-inventory-mgt.svg?branch=master)](https://travis-ci.org/PrismaPhonic/rs-inventory-mgt)
[![crates.io](http://meritbadge.herokuapp.com/inventory-mgt)](https://crates.io/crates/inventory-mgt)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Released API docs](https://docs.rs/inventory-mgt/badge.svg)](https://docs.rs/inventory-mgt)

This crate is a port of an existing python project. It allows one to sync the current
quantities from a supply inventory csv file to a filtered down view they have created of their
own truncated "master inventory" (their own inventory)

For examples of master inventory and supply inventory csv files, look in the
root directory of this repo.

### This program will update the quantity in master inventory with the quantity found in supply inventory based for each part in master inventory


- The default filenames that program accepts are:
   - "SupplyInventory.csv" for the _supply list_
   - "MasterInventory.csv" for the _master list_
- _You can set your own filenames but you must specify their name and location in command line arguments_
   - _See how to use command line arguments below_


- Both csv files must include the following columns:

   | VenCode | PartNumber | TotalQty |
   | ------- |:----------:|:--------:|

### Installation

You can install this application in one of two ways.  Either clone the repo and build the
release version with cargo, or simply use cargo install:

```terminal
$ cargo install inventory-mgt
```

### Use

To use this application, simply run it with the generate command to generate a new master csv
with updated quantity fields:

```terminal
$ inventory-mgt generate
```

Make sure you are in the root folder where your `SupplyInventory.csv` and `MasterInventory.csv`
files are. You can grab sample ones from the github repo.

Optionally you can pass in custom filenames with flags after the `generate` command:

```terminal
$ inventory-mgt generate -m masterinv.csv -s supplyinv.csv
```

That's it! Enjoy!
