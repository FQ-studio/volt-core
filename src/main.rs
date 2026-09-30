mod cli;
mod registry;
mod asset_parser;
mod codegen;
mod sql_bridge;

use cli::{VCli, VCommand};
use registry::VoltSqlRegistry;
use asset_parser::AssetParser;
use codegen::CodeGenerator;
use sql_bridge::SqlBridge;

fn main() {
    println!("--- Volt Sovereign Ecosystem Engine ---");

    // 1. SQL Registry Test
    let mut bridge = SqlBridge::new("sqlite://volt_local.db");
    if bridge.connect() {
        let _res = bridge.execute_raw_query("SELECT ip FROM volt_domains WHERE domain='shadow-rpg.volt'");
    }

    let mut sql_db = VoltSqlRegistry::new();
    sql_db.register_crate("shadow_core", "1.0.0", "hash_abc123");

    // 2. CLI Execution
    let cmd = VCommand::Build { target: "main.volt".to_string() };
    let _cli_out = VCli::execute(cmd);

    // 3. Asset Parsing & Codegen
    let fake_glb = b"glTF\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00\x80\x3f\x00\x00\x00\x40\x00\x00\x40\x40";
    let mesh_data = AssetParser::parse_glb_vertices(fake_glb).ok();
    
    let _bytecode = CodeGenerator::generate_binary("shadow-rpg.volt", mesh_data.as_ref());
    
    println!("Volt Core built successfully!");
}
