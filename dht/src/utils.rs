use super::node::Node;
use super::protocol::Protocol;

use std::fs::create_dir_all;
use std::io::Write;
use std::net::UdpSocket;

use super::network;
use super::routing::{KBucket, NodeAndDistance};

#[derive(Debug)]
pub enum ChannelPayload {
    Request((network::Request, Node)),
    Response(network::Response),
    NoData,
}

pub fn get_local_ip() -> Option<String> {
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(_) => return None,
    };

    match socket.connect("8.8.8.8:80") {
        Ok(()) => (),
        Err(_) => return None,
    };

    match socket.local_addr() {
        Ok(addr) => return Some(addr.ip().to_string()),
        Err(_) => return None,
    };
}

pub fn make_req_get_res(
    rpc: &network::Rpc,
    req: network::Request,
    dst: Node,
) -> Option<network::Response> {
    rpc.make_request(req, dst)
        .recv()
        .expect("[FAILED] Utils::make_req_get_res --> Failed to receive response through channel")
}

pub fn dump_interface_state(interface: &Protocol, path: &str) {
    create_dir_all("dumps")
        .expect("[FAILED] Utils::dump_interface_state --> Unable to create dumps dir");

    let rt = interface
        .routes
        .lock()
        .expect("[FAILED] Utils::dump_interface_state --> Failed to acquire mutex on Routes");
    let st = interface
        .store_tuples
        .lock()
        .expect("[FAILED] Utils::dump_interface_state --> Failed to acquire mutex on Store");

    let flattened: Vec<&KBucket> = rt.kbuckets.iter().collect();

    let mut parsed_buckets = vec![];
    for kb in flattened {
        for n in &kb.nodes {
            let kbucket = serde_json::json!({
                "nodes": {
                    "ip": n.ip,
                    "port": n.port,
                    "id": format!("{:?}", n.id),
                },
                "size": kb.size,
            });
            parsed_buckets.push(kbucket);
        }
    }

    let mut parsed_store = vec![];
    // parse store
    for (k, v) in &*st {
        let obj = serde_json::json!({ k: v });
        parsed_store.push(obj);
    }

    let json = serde_json::json!({
        "node": {
            "ip": interface.node.ip,
            "port": interface.node.port,
            "id": format!("{:?}", interface.node.id),
        },
        "routes": {
            "node": {
                "ip": rt.node.ip,
                "port": rt.node.port,
                "id": format!("{:?}", interface.node.id),
            },
            "kbuckets": parsed_buckets,
        },
        "store": parsed_store,
        "rpc": {
            "socket": format!("{:?}", interface.rpc.socket),
            "pending": format!("{:?}", interface.rpc.pending.lock().unwrap()),
            "node": {
                "ip": interface.rpc.node.ip,
                "port": interface.rpc.node.port,
                "id": format!("{:?}", interface.rpc.node.id),
            },
        }
    });

    // write to json file
    let mut file = std::fs::File::create(path)
        .expect("[FAILED] Utils::dump_interface_state --> Unable to create dump file");
    file.write_all(&json.to_string().as_bytes())
        .expect("[FAILED] Utils::dump_interface_state --> Unable to write to dump file");

    // write also to a .plantuml file
    let mut diagram = std::fs::File::create(format!("{}.plantuml", path))
        .expect("[FAILED] Utils::dump_interface_state --> Unable to create dump file");
    diagram
        .write_all("@startjson\n".to_string().as_bytes())
        .expect("[FAILED] Utils::dump_interface_state --> Unable to write to dump file");

    diagram
        .write_all(&json.to_string().as_bytes())
        .expect("[FAILED] Utils::dump_interface_state --> Unable to write to dump file");

    diagram
        .write_all("\n@endjson".to_string().as_bytes())
        .expect("[FAILED] Utils::dump_interface_state --> Unable to write to dump file");
}

pub fn dump_node_and_distance(
    entries: &Vec<NodeAndDistance>,
    target: &super::key::Key,
    path: &str,
) {
    let mut parsed = vec![];

    for e in entries {
        parsed.push(serde_json::json!({
            "node": {
                "ip": e.0.ip,
                "port": e.0.port,
                "id": format!("{:?}", e.0.id),
            },
            "distance": format!("{:?}", e.1),
        }))
    }

    let json = serde_json::json!({
        "target": format!("{:?}", target),
        "found": parsed,
    });

    let mut file = std::fs::File::create(path)
        .expect("[FAILED] Utils::dump_node_and_distance --> Unable to create dump file");
    file.write_all(&json.to_string().as_bytes())
        .expect("[FAILED] Utils::dump_node_and_distance --> Unable to write to dump file");
}


