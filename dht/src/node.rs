use super::key::Key;
use std::fmt;
// use std::ce
use super::attributes::Query;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]

pub struct NodeInfo {
    // storage of the machine of the vm in GBs.
    pub storage: u32,

    // RAM requiremnt in GB
    pub ram: u32,

    // Number of cpu cores
    pub cpu_cores: u32,

    // THe name of the arch image. 
    pub arch_images: u32,

    pub ip: String,
    
}

impl NodeInfo {
    // Define a scoring function for NodeInfo compared to a Query
    pub fn score(&self, query: &Query) -> u32 {
        (self.storage.saturating_sub(query.storage)
            + self.ram.saturating_sub(query.ram)
            + self.cpu_cores.saturating_sub(query.cpu_cores)
            + self.arch_images.saturating_sub(query.arch_images))
    }
}

impl fmt::Display for NodeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Storage: {}GB, RAM: {}GB, CPU Cores: {}, Arch Image: {}, Ip : {}",
            self.storage, self.ram, self.cpu_cores, self.arch_images, self.ip
        )
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]

pub struct Node {
    pub ip: String,
    pub port: u16,
    pub id: Key,
    pub info : NodeInfo
}


impl Node {
    pub fn new(ip: String, port: u16, info: NodeInfo ) -> Self {
        let addr = format!("{}:{}", ip, port);
        let id = Key::new(addr);
        // let info = NodeInfo { storage, ram, cpu_cores, arch_images };
        Node { ip, port, id, info }
    }
    pub fn get_info(&self) -> String {
        let mut parsed_id = hex::encode(self.id.0);
        parsed_id = parsed_id.to_ascii_uppercase();

        format!("{}:{}:{}", self.ip, self.port, parsed_id)
    }

    pub fn get_addr(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}
