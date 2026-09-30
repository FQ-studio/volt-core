use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct SqlDomainRecord {
    pub domain_name: String,
    pub target_ip: String,
    pub active: bool,
}

#[derive(Debug, Clone)]
pub struct SqlCrateRecord {
    pub name: String,
    pub version: String,
    pub sha256_hash: String,
}

pub struct VoltSqlRegistry {
    pub domains: Vec<SqlDomainRecord>,
    pub crates: Vec<SqlCrateRecord>,
}

impl VoltSqlRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            domains: Vec::new(),
            crates: Vec::new(),
        };
        
        registry.register_domain("voltMirror.volt", "127.0.0.1");
        registry.register_domain("shadow-rpg.volt", "192.168.1.100");
        registry
    }

    pub fn register_domain(&mut self, name: &str, ip: &str) {
        self.domains.push(SqlDomainRecord {
            domain_name: name.to_string(),
            target_ip: ip.to_string(),
            active: true,
        });
    }

    // Fungsi yang hilang sebelum ini:
    pub fn register_crate(&mut self, name: &str, version: &str, hash: &str) {
        self.crates.push(SqlCrateRecord {
            name: name.to_string(),
            version: version.to_string(),
            sha256_hash: hash.to_string(),
        });
    }

    pub fn query_domain(&self, domain: &str) -> Option<String> {
        for record in &self.domains {
            if record.domain_name == domain && record.active {
                return Some(record.target_ip.clone());
            }
        }
        None
    }
}
