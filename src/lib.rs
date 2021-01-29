/*
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

extern crate exitcode;
extern crate futures;
extern crate reqwest;
extern crate xml;
extern crate serde_derive;

pub mod commands;
mod utils;
use utils::cache;

use std::env;
use xml::reader::XmlEvent;


pub use loco::{
    apis::configuration::{ApiKey, Configuration},
    apis::{assets_api, auth_api, Error, export_api},
    models::{Asset, Credentials, PingResponse},
};


pub use commands::{
    tags::TagsSubCommandEnum,
    TopLevel,
    SubCommandEnum,
    Platform::{Android, IOS},
    Keytype::{DENIED, MISSING, RO, RW},
    ApiCommand
};



