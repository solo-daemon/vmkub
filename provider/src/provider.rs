
use kademlia_dht::node::{ Node, NodeInfo};
use kademlia_dht::protocol::Protocol;
use kademlia_dht::utils;
use std::{thread, time};

pub fn add_provider_to_network(storage: u32, ram: u32, cpu_cores: u32, wallet_addr: String) {
    let rootNodeInfo = NodeInfo {
        storage: 100,
        ram: 8,
        cpu_cores: 2,
        arch_images: 0,
        ip: "10.11.1.59".to_string(),
        wallet_address : "1".to_string(),
    };
    let nodeInfo = NodeInfo {
        storage,
        ram,
        cpu_cores,
        arch_images: 0,
        ip: utils::get_local_ip().unwrap(),
        wallet_address : wallet_addr
    };

    let root = Node::new(rootNodeInfo.ip.clone(), 7999, rootNodeInfo.clone());

    let node = Node::new(utils::get_local_ip().unwrap(), 63001, nodeInfo.clone());
    let interface = Protocol::new(node.ip, node.port, node.info, Some(root.clone()));
    interface.put_attributes("storage".to_string(), interface.node.info.storage);
    interface.put_attributes("ram".to_string(), interface.node.info.ram);
    interface.put_attributes("virtual_cpu".to_string(), interface.node.info.cpu_cores);
    interface.put_attributes("arm_image".to_string(), interface.node.info.arch_images);
    interface.put_tuple(interface.node.id, interface.node.info.clone());

    let ten_millis = time::Duration::from_millis(10);
    loop {
        thread::sleep(ten_millis);
    }
}
