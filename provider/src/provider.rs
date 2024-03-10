use kademlia_dht::attributes::{
    self, get_attribute_key, get_equal_arm_image_enums, get_higher_ram_enums,
    get_higher_storage_enums, get_higher_virtual_cpu_enums, get_virtual_cpu_enum, Query, Storage,
};
use kademlia_dht::node::{self, Node, NodeInfo};
use kademlia_dht::protocol::Protocol;
use kademlia_dht::utils;
use std::error::Error;
use std::{thread, time};

pub fn add_provider_to_network(storage: u32, ram: u32, cpu_cores: u32) {
    let rootNodeInfo = NodeInfo {
        storage: 100,
        ram: 8,
        cpu_cores: 2,
        arch_images: 0,
        ip: "10.11.1.59".to_string(),
    };
    let nodeInfo = NodeInfo {
        storage,
        ram,
        cpu_cores,
        arch_images: 0,
        ip: utils::get_local_ip().unwrap(),
    };

    let root = Node::new(rootNodeInfo.ip, 7999, rootNodeInfo.clone());

    let node = Node::new(utils::get_local_ip().unwrap(), 63001, nodeInfo.clone());
    let interface = Protocol::new(node.ip, node.port, node.info, Some(root.clone()));
    interface.put_attributes("storage".to_string(), interface.node.info.storage);
    interface.put_attributes("ram".to_string(), interface.node.info.ram);
    interface.put_attributes("virtual_cpu".to_string(), interface.node.info.cpu_cores);
    interface.put_attributes("arm_image".to_string(), interface.node.info.arch_images);
    interface.put_tuple(interface.node.id, interface.node.info);

    let ten_millis = time::Duration::from_millis(10);
    loop {
        thread::sleep(ten_millis);
    }
}
