use kademlia_dht::node::{Node, NodeInfo};
use kademlia_dht::protocol::Protocol;
use kademlia_dht::utils;

pub fn add_client_to_network(storage: u32, ram: u32, cpu_cores: u32) -> Protocol {
    let rootNodeInfo = NodeInfo {
        storage: 100,
        ram: 8,
        cpu_cores: 2,
        arch_images: 0,
        ip: "10.11.0.207".to_string(),
        wallet_address: "1".to_string(),
    };
    let nodeInfo = NodeInfo {
        storage,
        ram,
        cpu_cores,
        arch_images: 0,
        ip: utils::get_local_ip().unwrap(),
        wallet_address: "sup_nigga".to_string(),
    };

    let root = Node::new(rootNodeInfo.ip.clone(), 7999, rootNodeInfo.clone());

    let node = Node::new(utils::get_local_ip().unwrap(), 63001, nodeInfo.clone());
    Protocol::new(node.ip, node.port, node.info, Some(root.clone()))
}
