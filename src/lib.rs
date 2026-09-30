#![no_std]

extern crate alloc;

pub mod cli;
pub mod registry;
pub mod asset_parser;
pub mod codegen;
pub mod sql_bridge;

use alloc::string::ToString;
use cli::{VCli, VCommand};
use registry::VoltSqlRegistry;
use asset_parser::AssetParser;
use codegen::CodeGenerator;
use sql_bridge::SqlBridge;

/// Fungsi teras Volt yang bebas OS (100% Bare-Metal Compatible)
pub fn run_volt_engine() {
    let mut bridge = SqlBridge::new("sqlite://volt_local.db");
    if bridge.connect() {
        let _res = bridge.execute_raw_query("SELECT ip FROM volt_domains WHERE domain='shadow-rpg.volt'");
    }

    let mut sql_db = VoltSqlRegistry::new();
    sql_db.register_crate("shadow_core", "1.0.0", "hash_abc123");

    let cmd = VCommand::Build { target: "main.volt".to_string() };
    let _cli_out = VCli::execute(cmd);

    let fake_glb = b"glTF\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x80\x3f\x00\x00\x00\x40\x00\x00\x40\x40";
    let mesh_data = AssetParser::parse_glb_vertices(fake_glb).ok();
    
    let _bytecode = CodeGenerator::generate_binary("shadow-rpg.volt", mesh_data.as_ref());
}
