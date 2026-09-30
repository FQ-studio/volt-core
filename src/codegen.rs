use alloc::vec::Vec;
use alloc::string::String;
use crate::asset_parser::Mesh3D;

pub struct CodeGenerator;

impl CodeGenerator {
    /// Menjana header dan bytecode biner .vbin untuk VoltBrowser
    pub fn generate_binary(domain: &str, mesh: Option<&Mesh3D>) -> Vec<u8> {
        let mut bytecode = Vec::new();

        // 1. Magic Header VOLT (.vbin format)
        bytecode.extend_from_slice(b"VOLT_BIN_V1");

        // 2. Encode Domain Metadata Length & String
        let domain_bytes = domain.as_bytes();
        bytecode.push(domain_bytes.len() as u8);
        bytecode.extend_from_slice(domain_bytes);

        // 3. Serialize 3D Mesh Vertices (jika ada)
        if let Some(m) = mesh {
            bytecode.push(0x3D); // Marker Section 3D Mesh
            let vertex_count = m.vertices.len() as u32;
            bytecode.extend_from_slice(&vertex_count.to_le_bytes());

            for v in &m.vertices {
                bytecode.extend_from_slice(&v.x.to_le_bytes());
                bytecode.extend_from_slice(&v.y.to_le_bytes());
                bytecode.extend_from_slice(&v.z.to_le_bytes());
            }
        } else {
            bytecode.push(0x00); // Tiada Aset 3D
        }

        bytecode
    }
}
