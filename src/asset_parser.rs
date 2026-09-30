use alloc::vec::Vec;

#[derive(Debug, Clone, Copy)]
pub struct Vertex3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Mesh3D {
    pub vertices: Vec<Vertex3D>,
}

pub struct AssetParser;

impl AssetParser {
    /// Mengekstrak vertex float 32-bit dari fail biner GLB/FBX
    pub fn parse_glb_vertices(raw_bytes: &[u8]) -> Result<Mesh3D, &'static str> {
        // Menyemak Magic Header GLB (0x46546C67 -> "glTF")
        if raw_bytes.len() < 12 || &raw_bytes[0..4] != b"glTF" {
            return Err("Error: Format fail 3D tidak sah. Magic header glTF tidak dijumpai.");
        }

        let mut vertices = Vec::new();
        
        // Ekstrak data titik vertex (simulasi pemetaan byte float)
        let mut i = 12;
        while i + 12 <= raw_bytes.len() {
            let x = f32::from_le_bytes([raw_bytes[i], raw_bytes[i+1], raw_bytes[i+2], raw_bytes[i+3]]);
            let y = f32::from_le_bytes([raw_bytes[i+4], raw_bytes[i+5], raw_bytes[i+6], raw_bytes[i+7]]);
            let z = f32::from_le_bytes([raw_bytes[i+8], raw_bytes[i+9], raw_bytes[i+10], raw_bytes[i+11]]);
            
            vertices.push(Vertex3D { x, y, z });
            i += 12;
        }

        Ok(Mesh3D { vertices })
    }
}
