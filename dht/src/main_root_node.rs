extern crate kademlia_dht;
use kademlia_dht::node::{self, Node, NodeInfo};
use kademlia_dht::protocol::Protocol;
use kademlia_dht::utils;
use kademlia_dht::attributes::{self, get_attribute_key, get_equal_arm_image_enums, get_higher_ram_enums, get_higher_storage_enums, get_higher_virtual_cpu_enums, get_virtual_cpu_enum, Query, Storage};
use tokio::time::{sleep, Duration};
const BIG_TEST: bool = true;

// be careful with the net size, for example my computer can't spawn too many threads
// messages may also exceed the buffer size used for streaming (see issue #1)
const NET_SIZE: usize = 5;

async fn test_big_net() {
	let mut interfaces: Vec<Protocol> = Vec::with_capacity(NET_SIZE);
	let mut base_port = 8000;
	let nodeInfo  = NodeInfo { storage : 100 , ram:8, cpu_cores:2, arch_images:0, ip : utils::get_local_ip().unwrap()};

	let root = Node::new(utils::get_local_ip().unwrap(), 7999,nodeInfo.clone());
	dbg!(root.clone());
	let root_interface = Protocol::new(root.ip.clone(), root.port.clone(), root.info.clone(), None);

	// root_interface.put("MAIN_KEY".to_owned(), "MAIN_VALUE".to_owned());

	for i in 0..(NET_SIZE - 1) {
		let nodeInfo  = NodeInfo { storage : ((i+1)*20) as u32, ram: (i+1) as u32, cpu_cores:(i+1) as u32, arch_images:0, ip: utils::get_local_ip().unwrap()};
		let node = Node::new(utils::get_local_ip().unwrap(), base_port, nodeInfo.clone());

		interfaces.push(Protocol::new(node.ip, node.port, node.info, Some(root.clone())));
		println!(
			"[+] Created interface for index: {} on port: {}",
			i, base_port
		);

		base_port += 1;
	}

	for (index, interface) in interfaces.iter().enumerate() {
		println!("{}",interface.node.info.storage);
		println!("[+] Putting <key, value> pair for index: {}", index);

		// attributes ka keys hai yeh
		let key = get_attribute_key("storage".to_string(), interface.node.info.storage);
		println!(" storage {:?}", key);
		interface.put_attributes("storage".to_string(), interface.node.info.storage);
		interface.put_attributes("ram".to_string(), interface.node.info.ram);
		interface.put_attributes("virtual_cpu".to_string(), interface.node.info.cpu_cores);
		interface.put_attributes("arm_image".to_string(), interface.node.info.arch_images);

		// let key_2 = get_attribute_key("ram".to_string(), interface.node.info.ram);
		// println!("ram key {:?}", key_2);

		// let key_3 = get_attribute_key("virtual_cpu".to_string(), interface.node.info.cpu_cores);
		// println!(" cpu_cores {:?}", key_3);

		// let key_4 = get_attribute_key("arm_image".to_string(), interface.node.info.arch_images);
		// println!(" arch_image {:?}", key_4);
		
		// yeh hoga tumahara nodes struct ka keys 
		interface.put_tuple(interface.node.id, interface.node.info.clone());
		
		// interface.put(format!("key_{}", index), format!("value_{}", index));
	}

	// ideally ek keval client wala ko chalana chaiye
	// for (index, interface) in interfaces.iter().enumerate() {
	// 	if index == 0 {
	// 		dbg!("uio");
	// 		let req = Query { storage : 40, ram : 1, cpu_cores : 1, arch_images: 0};
	// 		interface.get_best_fit(req);
	// 	}
	// }

	    // Introduce a loop to keep the program alive
	loop {
		// Add some delay to avoid busy-waiting
		sleep(Duration::from_secs(5)).await;
	}
	
}
// async fn main() {
//     if BIG_TEST {
//         test_big_net().await;
//     }
// }
#[tokio::main]
async fn main() {
	if BIG_TEST {
		test_big_net().await;
	}
	// } else {
	// 	let node0 = Node::new(utils::get_local_ip().unwrap(), 1337, 100, 8, 2,0);
	// 	println!("[+] Created node0: {:?}", node0);

	// 	let node1 = Node::new(utils::get_local_ip().unwrap(), 1338, 100, 8, 2,0);
	// 	println!("[+] Created node1: {:?}", node1);

	// 	let node2 = Node::new(utils::get_local_ip().unwrap(), 1339, 100, 8, 2,0);
	// 	println!("[+] Created node2: {:?}", node2);

	// 	let interface0 = Protocol::new(node0.ip.clone(), node0.port.clone(), node0.info.storage.clone(), node0.info.ram.clone(), node0.info.cpu_cores.clone(), node0.info.arch_images.clone(), None);
	// 	println!("[+] Initialized Kademlia Protocol for node0 (interface0)");

	// 	let interface1 = Protocol::new(node1.ip.clone(), node1.port.clone(),  node1.info.storage.clone(), node1.info.ram.clone(), node1.info.cpu_cores.clone(), node1.info.arch_images.clone(),Some(node0.clone()));
	// 	println!("[+] Initialized Kademlia Protocol for node1 (interface1)");

	// 	let interface2 = Protocol::new(node2.ip.clone(), node2.port.clone(), node2.info.storage.clone(), node2.info.ram.clone(), node2.info.cpu_cores.clone(), node2.info.arch_images.clone(),Some(node0.clone()));
	// 	println!("[+] Initialized Kademlia Protocol for node2 (interface2)");

	// 	println!("\n--------------------------------------");
	// 	println!("Calling Kademlia API");

	// 	interface0.put("some_key".to_owned(), "some_value".to_owned());
	// 	println!("\t[*] node0 > called PUT for key: 'some_key' and value: 'some_value'");

	// 	let get_res = interface2.get("some_key".to_owned());
	// 	println!("\t[*] node2 > called GET on key: 'some_key'");
	// 	println!("\t\t[+] Extracted: {:?}", get_res);
	// 	println!("--------------------------------------\n");

	// 	utils::dump_interface_state(&interface0, "dumps/interface0.json");
	// 	utils::dump_interface_state(&interface1, "dumps/interface1.json");
	// 	utils::dump_interface_state(&interface2, "dumps/interface2.json");
	// 	println!("[*] Dumped protocol states for node0, node1 and node2. Check out the 'dumps' folder for a complete tracelog");
	// 	println!("Exiting...");
	// }
}