
use crate::attributes::get_attribute_key;
use crate::attributes::get_attribute_key_from_enum;
use crate::attributes::DHTValueStruct;
use crate::attributes::Query;
use crate::key::Key;
use attributes::{ get_equal_arm_image_enums, get_higher_ram_enums, get_higher_storage_enums, get_higher_virtual_cpu_enums, get_virtual_cpu_enum,  Storage};
use std::sync::{Arc, Mutex};
use std::thread;
// use crate::node::NodeInfo;

use super::network;
use super::node::Node;
use super::node::NodeInfo;

use super::routing;
use super::utils;
use super::attributes;
use std::rc::Rc;
use std::cell::RefCell;

use crossbeam_channel;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::sync::mpsc;

#[derive(Debug,Clone)]
pub struct SharedBucket {
    pub bucket: HashSet<NodeInfo>,
}

impl SharedBucket {
    pub fn new() -> Self {
        SharedBucket {
            bucket: HashSet::new(),
        }
    }

    pub fn add_to_bucket(&mut self, node_info: NodeInfo) {
        self.bucket.insert(node_info);
    }
}

#[derive(Debug, Clone)]
pub struct Protocol {
    pub routes: Arc<Mutex<routing::RoutingTable>>,
    pub store_tuples: Arc<Mutex<HashMap<Key, NodeInfo>>>,
    pub store_attributes: Arc<Mutex<HashMap<Key, Vec<DHTValueStruct>>>>,
    pub rpc: Arc<network::Rpc>,
    pub node: Node,
}

pub fn is_node_greater_than_query(node_info: NodeInfo, query: Query) -> bool {
    // Check if storage, ram, and cpu_cores in the query are greater than the corresponding values in node_info
    let storage_condition = query.storage <= node_info.storage;
    let ram_condition = query.ram <= node_info.ram;
    let cpu_cores_condition = query.cpu_cores <= node_info.cpu_cores;

    // Check if arch_images in the query matches the arch_images in node_info
    let arch_images_condition = query.arch_images == node_info.arch_images;

    // Return true if all conditions are met
    storage_condition && ram_condition && cpu_cores_condition && arch_images_condition
}

impl Protocol {
    pub fn new(ip: String, port: u16, info : NodeInfo, bootstrap: Option<Node>) -> Self {
        
        let node = Node::new(ip, port, info);

        // channel used for a 2-way communication with the Routing Table module
        let (rt_channel_sender, rt_channel_receiver) = crossbeam_channel::unbounded();

        let routes = routing::RoutingTable::new(
            node.clone(),
            bootstrap,
            rt_channel_sender.clone(),
            rt_channel_receiver.clone(),
        );

        // 1-way channel to communicate with the Network module
        let (rpc_channel_sender, rpc_channel_receiver) = mpsc::channel();

        let rpc = network::Rpc::new(node.clone());
        network::Rpc::open(rpc.clone(), rpc_channel_sender);

        let protocol = Self {
            routes: Arc::new(Mutex::new(routes)),
            store_tuples: Arc::new(Mutex::new(HashMap::new())),
            store_attributes:  Arc::new(Mutex::new(HashMap::new())),
            rpc: Arc::new(rpc),
            node: node.clone(),
        };

        protocol.clone().requests_handler(rpc_channel_receiver);
        protocol
            .clone()
            .rt_forwarder(rt_channel_sender, rt_channel_receiver);

        // performing node lookup on ourselves
        protocol.nodes_lookup(&node.id);

        // // republishing <key, value> pairs every hour
        // let protocol_clone = protocol.clone();
        // std::thread::spawn(move || {
        //     std::thread::sleep(std::time::Duration::from_secs(60 * 60));
        //     protocol_clone.republish();
        // });

        protocol
    }

    // fn republish(&self) {
    //     let st = self
    //         .store
    //         .lock()
    //         .expect("[FAILED] Protocol::republish --> Failed to acquire mutex on Store");
    //     for (key, value) in &*st {
    //         self.put(key.to_string(), value.to_string());
    //     }
    // }

    // forwards upcoming requests (only Pings at the moment) from the Routing table
    fn rt_forwarder(
        self,
        sender: crossbeam_channel::Sender<utils::ChannelPayload>,
        receiver: crossbeam_channel::Receiver<utils::ChannelPayload>,
    ) {
        std::thread::spawn(move || {
            for req in receiver.iter() {
                let protocol = self.clone();
                let sender_clone = sender.clone();

                std::thread::spawn(move || match req {
                    utils::ChannelPayload::Request(payload) => match payload.0 {
                        network::Request::Ping => {
                            let success = protocol.ping(payload.1);
                            if success {
                                if let Err(_) = sender_clone
                                    .send(utils::ChannelPayload::Response(network::Response::Ping))
                                {
                                    eprintln!("[FAILED] Protocol::rt_forwared --> Receiver is dead, closing channel");
                                }
                            } else {
                                if let Err(_) = sender_clone.send(utils::ChannelPayload::NoData) {
                                    eprintln!("[FAILED] Protocol::rt_forwared --> Receiver is dead, closing channel");
                                }
                            }
                        }
                        _ => {
                            unimplemented!();
                        }
                    },
                    utils::ChannelPayload::Response(_) => {
                        eprintln!("[FAILED] Protocol::rt_forwarder --> Received a Response instead of a Request")
                    }
                    utils::ChannelPayload::NoData => {
                        eprintln!("[FAILED] Protocol::rt_forwarder --> Received a NoData instead of a Request")
                    }
                });
            }
        });
    }

    // handles requests by crafting responses and sending them
    fn requests_handler(self, receiver: mpsc::Receiver<network::ReqWrapper>) {
        std::thread::spawn(move || {
            for req in receiver.iter() {
                let protocol = self.clone();

                std::thread::spawn(move || {
                    let res = protocol.craft_res(req);
                    protocol.reply(res);
                });
            }
        });
    }

    fn craft_res(&self, req: network::ReqWrapper) -> (network::Response, network::ReqWrapper) {
        let mut routes = self
            .routes
            .lock()
            .expect("[FAILED] Protocol::craft_res --> Failed to acquire mutex on Routes");

        // must craft node object because ReqWrapper contains only the src string addr
        let split = req.src.split(":");
        let parsed: Vec<&str> = split.collect();
        
        let nodeInfo  = NodeInfo { storage:100, ram:8, cpu_cores:2, arch_images:0 };
        let src_node = Node::new(
            parsed[0].to_string(),
            parsed[1]
                .parse::<u16>()
                .expect("[FAILED] Protocol::craft_res --> Failed to parse Node port from address"),
                nodeInfo
        );
        routes.update(src_node);
        drop(routes);

        match req.payload {
            network::Request::Ping => (network::Response::Ping, req),
            network::Request::Store_Tuple(ref k, ref v) => {
                // ref is used to borrow k and v, which are the contents of req

                let mut store = self
                    .store_tuples
                    .lock()
                    .expect("[FAILED] Protocol::craft_res --> Failed to acquire mutex on Store");
                store.insert((*k).clone(), (*v).clone());

                (network::Response::Ping, req)
            },
            network::Request::Store_Attribute(ref k, ref v) => {
                // ref is used to borrow k and v, which are the contents of req
                // dbg!((*k).clone());
                // dbg!((*v).clone());

                let mut store = self
                    .store_attributes
                    .lock()
                    .expect("[FAILED] Protocol::craft_res --> Failed to acquire mutex on Store");
                store
                .entry((*k).clone())
                .or_insert_with(Vec::new)
                .push((*v).clone());

                (network::Response::Ping, req)
            },
            network::Request::FindNode(ref id) => {
                let routes = self
                    .routes
                    .lock()
                    .expect("[FAILED] Protocol::craft_res --> Failed to acquire mutex on Routes");

                let result = routes.get_closest_nodes(id, super::K_PARAM);

                (network::Response::FindNode(result), req)
            }
            network::Request::FindValue(ref k) => {
                let key = k;
                let store = self
                    .store_tuples
                    .lock()
                    .expect("[FAILED] Protocol::craft_res --> Failed to acquire mutex on Store");

                let val = store.get(k);

                match val {
                    Some(v) => (
                        network::Response::FindValue(routing::FindValueResult::Value(
                            (*v).clone(),
                        )),
                        req,
                    ),
                    None => {
                        let routes = self.routes.lock().expect(
                            "[FAILED] Protocol::craft_res --> Failed to acquire mutex on Routes",
                        );
                        (
                            network::Response::FindValue(routing::FindValueResult::Nodes(
                                routes.get_closest_nodes(&key, super::K_PARAM),
                            )),
                            req,
                        )
                    }
                }
            }
            network::Request::FindValueAttr(ref k) => {
                let key = k;
                let store = self
                    .store_attributes
                    .lock()
                    .expect("[FAILED] Protocol::craft_res --> Failed to acquire mutex on Store");

                let val = store.get(k);

                match val {
                    Some(v) => (
                        network::Response::FindValueAttr(routing::FindValueResultAttribute::Value(
                            (*v).clone(),
                        )),
                        req,
                    ),
                    None => {
                        let routes = self.routes.lock().expect(
                            "[FAILED] Protocol::craft_res --> Failed to acquire mutex on Routes",
                        );
                        (
                            network::Response::FindValueAttr(routing::FindValueResultAttribute::Nodes(
                                routes.get_closest_nodes(&key, super::K_PARAM),
                            )),
                            req,
                        )
                    }
                }
            }
            }
        }

    fn reply(&self, packet_details: (network::Response, network::ReqWrapper)) {
        let msg = network::RpcMessage {
            token: packet_details.1.token,
            src: self.node.get_addr(),
            dst: packet_details.1.src,
            msg: network::Message::Response(packet_details.0),
        };

        self.rpc.send_msg(&msg);
    }

    pub fn ping(&self, dst: Node) -> bool {
        let res = utils::make_req_get_res(&self.rpc, network::Request::Ping, dst.clone());

        let mut routes = self
            .routes
            .lock()
            .expect("[FAILED] Protocol::ping --> Failed to acquire lock on Routes");

        if let Some(network::Response::Ping) = res {
            routes.update(dst);
            true
        } else {
            eprintln!(
                "[WARNING] Protocol::Ping --> No response, removing contact from routing table"
            );
            routes.remove(&dst);
            false
        }
    }

    pub fn store_tuples(&self, dst: Node, key: Key, val:NodeInfo) -> bool {
        let res =
            utils::make_req_get_res(&self.rpc, network::Request::Store_Tuple(key, val), dst.clone());

        // since we get a ping, update our routing table
        let mut routes = self
            .routes
            .lock()
            .expect("[FAILED] Protocol::store --> Failed to acquire mutex on Routes");
        if let Some(network::Response::Ping) = res {
            routes.update(dst);
            true
        } else {
            routes.remove(&dst);
            false
        }
    }

    pub fn store_attributes(&self, dst: Node, key: Key, val:DHTValueStruct) -> bool {
        // dbg!(key.clone());
        // dbg!(val.clone());
        let res =
            utils::make_req_get_res(&self.rpc, network::Request::Store_Attribute(key, val), dst.clone());

        // since we get a ping, update our routing table
        let mut routes = self
            .routes
            .lock()
            .expect("[FAILED] Protocol::store --> Failed to acquire mutex on Routes");
        if let Some(network::Response::Ping) = res {
            routes.update(dst);
            true
        } else {
            routes.remove(&dst);
            false
        }
    }

    pub fn find_node(
        &self,
        dst: Node,
        id: super::key::Key,
    ) -> Option<Vec<routing::NodeAndDistance>> {
        let res = utils::make_req_get_res(&self.rpc, network::Request::FindNode(id), dst.clone());

        let mut routes = self
            .routes
            .lock()
            .expect("[FAILED] Protocol::find_node --> Failed to acquire mutex on Routes");
        if let Some(network::Response::FindNode(entries)) = res {
            routes.update(dst);
            Some(entries)
        } else {
            routes.remove(&dst);
            None
        }
    }

    pub fn find_value(&self, dst: Node, k: Key) -> Option<routing::FindValueResult> {
        let res = utils::make_req_get_res(&self.rpc, network::Request::FindValue(k), dst.clone());

        let mut routes = self
            .routes
            .lock()
            .expect("[FAILED] Protocol::find_value --> Failed to acquire mutex on Routes");

        if let Some(network::Response::FindValue(val)) = res {
            routes.update(dst);
            Some(val)
        } else {
            routes.remove(&dst);
            None
        }
    }

    pub fn find_value_attributes(&self, dst: Node, k: Key) -> Option<routing::FindValueResultAttribute> {
        // dbg!(dst.clone());
        // dbg!(k.clone());
        // dbg!("there");
        let res = utils::make_req_get_res(&self.rpc, network::Request::FindValueAttr(k), dst.clone());

        let mut routes = self
            .routes
            .lock()
            .expect("[FAILED] Protocol::find_value --> Failed to acquire mutex on Routes");

        if let Some(network::Response::FindValueAttr(val)) = res {
            routes.update(dst);
            Some(val)
        } else {
            routes.remove(&dst);
            None
        }
    }

    pub fn nodes_lookup(&self, id: &super::key::Key) -> Vec<routing::NodeAndDistance> {
        let mut ret: Vec<routing::NodeAndDistance> = Vec::new();

        // nodes visited
        let mut queried = HashSet::new();
        let routes = self
            .routes
            .lock()
            .expect("[FAILED] Protocol::nodes_lookup --> Failed to acquire mutex on Routes");

        // nodes to visit
        let mut to_query = BinaryHeap::from(routes.get_closest_nodes(id, super::K_PARAM));
        drop(routes);

        for entry in &to_query {
            queried.insert(entry.clone());
        }

        while !to_query.is_empty() {
            // threads joins
            let mut joins: Vec<std::thread::JoinHandle<Option<Vec<routing::NodeAndDistance>>>> =
                Vec::new();
            // outgoing queries
            let mut queries: Vec<routing::NodeAndDistance> = Vec::new();
            let mut results: Vec<Option<Vec<routing::NodeAndDistance>>> = Vec::new();

            for _ in 0..super::ALPHA {
                match to_query.pop() {
                    Some(entry) => {
                        queries.push(entry);
                    }
                    None => {
                        break;
                    }
                }
            }

            for &routing::NodeAndDistance(ref node, _) in &queries {
                let n = node.clone();
                let id_clone = id.clone();
                let protocol_clone = self.clone();

                joins.push(std::thread::spawn(move || {
                    protocol_clone.find_node(n, id_clone)
                }));
            }

            for j in joins {
                results.push(j.join().expect(
                    "[FAILED] Protocol::nodes_lookup --> Failed to join thread while visiting nodes",
                ));
            }

            for (result, query) in results.into_iter().zip(queries) {
                if let Some(entries) = result {
                    ret.push(query);

                    for entry in entries {
                        if queried.insert(entry.clone()) {
                            to_query.push(entry);
                        }
                    }
                }
            }
        }

        ret.sort_by(|a, b| a.1.cmp(&b.1));
        ret.truncate(super::K_PARAM);

        ret
    }

    pub fn value_lookup_tuples(&self, k: Key) -> (Option<NodeInfo>, Vec<routing::NodeAndDistance>) {
        // NOTE: k and key are two different things, one is a string used to search for the corresponding value while the other is a key::Key

        let mut ret: Vec<routing::NodeAndDistance> = Vec::new();
        let key = k;
        let mut queried = HashSet::new();

        let routes = self
            .routes
            .lock()
            .expect("[FAILED] Protocol::value_lookup --> Failed to acquire mutex on Routes");
        let mut to_query = BinaryHeap::from(routes.get_closest_nodes(&key, super::K_PARAM));
        drop(routes);

        for entry in &to_query {
            queried.insert(entry.clone());
        }

        while !to_query.is_empty() {
            let mut joins: Vec<std::thread::JoinHandle<Option<routing::FindValueResult>>> =
                Vec::new();
            let mut queries: Vec<routing::NodeAndDistance> = Vec::new();
            let mut results: Vec<Option<routing::FindValueResult>> = Vec::new();

            for _ in 0..super::ALPHA {
                match to_query.pop() {
                    Some(entry) => {
                        queries.push(entry);
                    }
                    None => {
                        break;
                    }
                }
            }

            for &routing::NodeAndDistance(ref n, _) in &queries {
                let k_clone = key.clone();
                let node = n.clone();
                let protocol = self.clone();

                joins.push(std::thread::spawn(move || {
                    protocol.find_value(node, k_clone)
                }));
            }

            for j in joins {
                results.push(j.join().expect("[FAILED] Protocol::value_lookup --> Failed to join thread while searching for value"));
            }

            for (result, query) in results.into_iter().zip(queries) {
                if let Some(find_value_result) = result {
                    match find_value_result {
                        routing::FindValueResult::Nodes(entries) => {
                            // we didn't get the value we looked for
                            ret.push(query);
                            for entry in entries {
                                if queried.insert(entry.clone()) {
                                    to_query.push(entry);
                                }
                            }
                        }

                        routing::FindValueResult::Value(val) => {
                            ret.sort_by(|a, b| a.1.cmp(&b.1));
                            ret.truncate(super::K_PARAM);

                            return (Some(val), ret);
                        }
                    }
                }
            }
        }
        ret.sort_by(|a, b| a.1.cmp(&b.1));
        ret.truncate(super::K_PARAM);
        (None, ret)
    }

    pub fn value_lookup_attributes(&self, k: Key) -> (Option<Vec<DHTValueStruct>>, Vec<routing::NodeAndDistance>) {
        // NOTE: k and key are two different things, one is a string used to search for the corresponding value while the other is a key::Key
        // dbg!("here");
        // dbg!(k);
        let mut ret: Vec<routing::NodeAndDistance> = Vec::new();
        let key = k;
        let mut queried = HashSet::new();

        let routes = self
            .routes
            .lock()
            .expect("[FAILED] Protocol::value_lookup --> Failed to acquire mutex on Routes");
        let mut to_query = BinaryHeap::from(routes.get_closest_nodes(&key, super::K_PARAM));
        drop(routes);

        for entry in &to_query {
            queried.insert(entry.clone());
        }

        while !to_query.is_empty() {
            let mut joins: Vec<std::thread::JoinHandle<Option<routing::FindValueResultAttribute>>> =
                Vec::new();
            let mut queries: Vec<routing::NodeAndDistance> = Vec::new();
            let mut results: Vec<Option<routing::FindValueResultAttribute>> = Vec::new();

            for _ in 0..super::ALPHA {
                match to_query.pop() {
                    Some(entry) => {
                        queries.push(entry);
                    }
                    None => {
                        break;
                    }
                }
            }

            for &routing::NodeAndDistance(ref n, _) in &queries {
                let k_clone = key.clone();
                let node = n.clone();
                let protocol = self.clone();

                joins.push(std::thread::spawn(move || {
                    protocol.find_value_attributes(node, k_clone)
                }));
            }

            for j in joins {
                results.push(j.join().expect("[FAILED] Protocol::value_lookup --> Failed to join thread while searching for value"));
            }

            for (result, query) in results.into_iter().zip(queries) {
                if let Some(find_value_result) = result {
                    match find_value_result {
                        routing::FindValueResultAttribute::Nodes(entries) => {
                            // we didn't get the value we looked for
                            ret.push(query);
                            for entry in entries {
                                if queried.insert(entry.clone()) {
                                    to_query.push(entry);
                                }
                            }
                        }

                        routing::FindValueResultAttribute::Value(val) => {
                            ret.sort_by(|a, b| a.1.cmp(&b.1));
                            ret.truncate(super::K_PARAM);

                            return (Some(val), ret);
                        }
                    }
                }
            }
        }
        ret.sort_by(|a, b| a.1.cmp(&b.1));
        ret.truncate(super::K_PARAM);
        (None, ret)
    }

    pub fn put_attributes(&self, attribute: String, value: u32) {
        // dbg!(attribute.clone());
        let key = get_attribute_key(attribute, value);
        let candidates = self.nodes_lookup(&key);
        // dbg!(candidates.clone());
        let dht_value = DHTValueStruct { key: self.node.id, value:value};
        // dbg!(dht_value.clone());
        for routing::NodeAndDistance(node, _) in candidates {
            let protocol_clone = self.clone();
            let k_clone = key.clone();
            let v_clone = dht_value.clone();

            std::thread::spawn(move || {
                protocol_clone.store_attributes(node, k_clone, v_clone);
            });
        }
    }

    pub fn put_tuple(&self, key : Key, value : NodeInfo) {
        let candidates = self.nodes_lookup(&key);

        for routing::NodeAndDistance(node, _) in candidates {
            let protocol_clone = self.clone();
            let k_clone = key.clone();
            let v_clone = value.clone();

            std::thread::spawn(move || {
                protocol_clone.store_tuples(node, k_clone, v_clone);
            });
        }
    }

    pub fn get_tuple(&self, k: Key) -> Option<NodeInfo> {
        let (val, mut nodes) = self.value_lookup_tuples(k.clone());

        val.map(|v| {
            // if let Some(routing::NodeAndDistance(target, _)) = nodes.pop() {
            //     self.store_tuples(target, k, v.clone());
            // } else {
            //     self.store_tuples(self.node.clone(), k, v.clone());
            // }

            v
        })
    }

    pub fn get_attribute(&self, k: Key) -> Option<Vec<DHTValueStruct>> {
        // dbg!(k.clone());
        let (val, mut nodes) = self.value_lookup_attributes(k.clone());
        // dbg!(val)
        val.map(|v| {
            // if let Some(routing::NodeAndDistance(target, _)) = nodes.pop() {
            //     self.store_tuples(target, k, v.clone());
            // } else {
            //     self.store_tuples(self.node.clone(), k, v.clone());
            // }
            v
        })
    }
    pub fn get_best_fit(&self, query : Query)  {
        dbg!("here");
        // debug(que)
        let higher_storage = get_higher_storage_enums(query.storage);
        let higher_ram = get_higher_ram_enums(query.ram);
        let higher_cpu_cores = get_higher_virtual_cpu_enums(query.cpu_cores);
        let higher_archs_images = get_equal_arm_image_enums(query.arch_images);

        let mut shared_bucket = SharedBucket::new();

        for storage in higher_storage {
            let current_key = get_attribute_key_from_enum("storage".to_string(),storage);
            let (val, mut nodes) = self.value_lookup_attributes(current_key.clone());
            if let Some(value) = val {
                dbg!("Value is: {:?}", value.clone());
                for nodes in value {
                    let (node_info, mut nodes) = self.value_lookup_tuples(nodes.key.clone());
                    dbg!("jhjh");
                    dbg!(node_info);

                    if let Some(node_info) = node_info.clone() {
                        if is_node_greater_than_query(node_info, query.clone()) {
                            dbg!("Achintya");
                            shared_bucket.add_to_bucket(node_info.clone());
                        }
                    } else {
                        // Handle the case when node_info is None
                    }                
                }

            } else {
                // dbg!("Value is None");
            }
        }

        // handles.push(handle);
        // dbg!(shared_bucket_clone);


        // let shared_bucket_ram = Arc::clone(&shared_bucket);
        // let handle_ram = thread::spawn(move || {
        for ram in higher_ram {
            let current_key = get_attribute_key_from_enum("ram".to_string(),ram);
            let (val, mut nodes) = self.value_lookup_attributes(current_key.clone());
            if let Some(value) = val {
                // dbg!("Value is: {:?}", value.clone());
                for nodes in value {
                    let (node_info, mut nodes) = self.value_lookup_tuples(nodes.key.clone());
                    if let Some(node_info) = node_info.clone() {
                        if is_node_greater_than_query(node_info, query.clone()) {
                            // Your code here
                            shared_bucket.add_to_bucket(node_info.clone());

                        }
                    } else {
                        // Handle the case when node_info is None
                    }                
                }

            } else {
                // dbg!("Value is None");
            }
        }
        // });
        // handles.push(handle_ram);


        // dbg!("virtual_cpu");
        // let shared_bucket_cpu = Arc::clone(&shared_bucket);
        // let handle_cpu = thread::spawn(move || {
        for cpu in higher_cpu_cores {
            let current_key = get_attribute_key_from_enum("virtual_cpu".to_string(),cpu);
            let (val, mut nodes) = self.value_lookup_attributes(current_key.clone());
            if let Some(value) = val {
                // dbg!("Value is: {:?}", value.clone());
                for nodes in value {
                    let (node_info, mut nodes) = self.value_lookup_tuples(nodes.key.clone());
                    if let Some(node_info) = node_info.clone() {
                        if is_node_greater_than_query(node_info, query.clone()) {
                            // Your code here
                        shared_bucket.add_to_bucket(node_info.clone());

                        }
                    } else {
                        // Handle the case when node_info is None
                    }                
                }

            } else {
                // dbg!("Value is None");
            }
        }
        // });
        // handles.push(handle_cpu);

        // dbg!("arch_image");
        // let shared_bucket_arch_image = Arc::clone(&shared_bucket);
        // let handle_arch_image = thread::spawn(move || {
        for archs in higher_archs_images {
            let current_key = get_attribute_key_from_enum("arm_image".to_string(),archs);
            // dbg!(current_key.clone());
            let (val, mut nodes) = self.value_lookup_attributes(current_key.clone());
            // dbg!(val);
            if let Some(value) = val {
                // dbg!("Value is: {:?}", value.clone());
                for nodes in value {
                    let (node_info, mut nodes) = self.value_lookup_tuples(nodes.key.clone());
                    if let Some(node_info) = node_info.clone() {
                        if is_node_greater_than_query(node_info, query.clone()) {
                            // Your code here
                        shared_bucket.add_to_bucket(node_info.clone());

                        }
                    } else {
                        // Handle the case when node_info is None
                    }                
                }
            } else {
                // dbg!("Value is None");
            }
        }
        // });
    dbg!(shared_bucket.clone());

        let best_fit = shared_bucket.bucket
        .iter()
        .min_by_key(|node_info| node_info.score(&query));

        match best_fit {
            Some(node_info) => {
                println!("Best fit: {:?}", node_info);
                println!("Score: {}", node_info.score(&query));
            }
            None => {
                println!("No suitable fit found.");
            }
        }

    }
}
