extern crate alloc;

use embedded_alloc::Heap;
use core::sync::atomic::{AtomicBool, Ordering};

#[global_allocator]
static ALLOCATOR: Heap = Heap::empty();
static ALLOC_INITIALIZED: AtomicBool = AtomicBool::new(false);

mod cli;
mod registry;
mod asset_parser;
mod codegen;
mod sql_bridge;

use alloc::string::ToString;
use cli::{VCli, VCommand};
use registry::VoltSqlRegistry;
use asset_parser::AssetParser;
use codegen::CodeGenerator;
use sql_bridge::SqlBridge;

fn init_heap() {
    if !ALLOC_INITIALIZED.swap(true, Ordering::SeqCst) {
        const HEAP_SIZE: usize = 1024 * 1024; // 1 MB Heap
        static mut HEAP_MEM: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
        #[allow(static_mut_refs)]
        unsafe { ALLOCATOR.init(HEAP_MEM.as_mut_ptr() as usize, HEAP_SIZE) };
    }
}

fn main() {
    init_heap();

    // Isolated scope: Semua alokasi Vec/String di-drop otomatis saat keluar kurung kurawal
    {
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
}
