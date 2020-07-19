use regex::Regex;
use tbd::project_path;
use serde::Serialize;
use serde_json;

#[derive(Serialize)]
struct Node {
    id: String
}

#[derive(Serialize)]
struct Link {
    source: String,
    target: String
}

fn main() {
    let re = Regex::new(r":\d+").unwrap();

    let src = std::fs::read_to_string(project_path("data/messages/galaxy.txt")).unwrap();
    let mut links: Vec<Link> = Vec::new();
    let mut nodes: Vec<Node> = Vec::new();
    for line in src.split_terminator('\n') {
        let parts: Vec<&str> = line.split(" = ").collect();
        nodes.push(Node {id: parts[0].to_string()});
        for cap in re.find_iter(parts[1]) {
            links.push(Link {source: cap.as_str().to_string(), target: parts[0].to_string()});
            //println!("{} -> {}", parts[0], cap.as_str())
        }
    }
    let links_json = serde_json::to_string(&links).unwrap();
    let nodes_json = serde_json::to_string(&nodes).unwrap();
    println!("{{\"nodes\":{},\"links\":{}}}", nodes_json, links_json);
}
