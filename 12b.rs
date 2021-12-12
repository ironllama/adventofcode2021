pub mod usrlib;

use std::collections::HashMap;

fn main() {
    let input_stuff = [
        // "start-A",
        // "start-b",
        // "A-c",
        // "A-b",
        // "b-d",
        // "A-end",
        // "b-end",

        // "dc-end",
        // "HN-start",
        // "start-kj",
        // "dc-start",
        // "dc-HN",
        // "LN-dc",
        // "HN-end",
        // "kj-sa",
        // "kj-HN",
        // "kj-dc",

        // "fs-end",
        // "he-DX",
        // "fs-he",
        // "start-DX",
        // "pj-DX",
        // "end-zg",
        // "zg-sl",
        // "zg-pj",
        // "pj-he",
        // "RW-he",
        // "fs-DX",
        // "pj-RW",
        // "zg-RW",
        // "start-pj",
        // "he-WI",
        // "zg-he",
        // "pj-fs",
        // "start-RW",

        "QR-da",
        "QR-end",
        "QR-al",
        "start-op",
        "zh-iw",
        "zh-start",
        "da-PF",
        "op-bj",
        "iw-QR",
        "end-HR",
        "bj-PF",
        "da-LY",
        "op-PF",
        "bj-iw",
        "end-da",
        "bj-zh",
        "HR-iw",
        "zh-op",
        "zh-PF",
        "HR-bj",
        "start-PF",
        "HR-da",
        "QR-bj",
    ];

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    // Utility function to help create a lookup table out of the input.
    fn add_node (in_name: String, in_edge: String, graph: &mut HashMap<String, Vec<String>>) {
        match graph.get_mut(&in_name) {
            Some(x) => if !x.contains(&in_edge) { x.push(in_edge.clone()) },
            None => {
                graph.insert(in_name.clone(), vec![in_edge.clone()]);
                // return;
            },
        };
        match graph.get_mut(&in_edge) {
            Some(x) => if !x.contains(&in_name) { x.push(in_name.clone()) },
            None => {
                graph.insert(in_edge.clone(), vec![in_name.clone()]);
                // return;
            },
        };
    }

    input_stuff.iter().for_each(|line| {
        let tokens: Vec<&str> = line.split('-').collect();
        add_node(tokens[0].to_string(), tokens[1].to_string(), &mut graph)
    });
    // println!("{:?}", graph);


    // Traverse the graph-ish thing! DFS-ish!
    fn visit (graph: &HashMap<String, Vec<String>>, in_node: &String, path: &Vec<String>, visited: &Vec<String>, all_paths: &mut Vec<String>, one_pass: &String) {
        // println!("VISITING: {}", in_node);

        let mut new_path = path.clone();
        new_path.push(in_node.to_string());

        let mut new_visited = visited.clone();
        if &in_node.to_uppercase() != in_node {
            new_visited.push(in_node.to_string());
        }

        if in_node == &"end".to_string() {
            all_paths.push(new_path.join(","));
        }
        else {
            let neighbors = graph.get(in_node).unwrap();
            // println!("PATH: {:?}\tVISITED: {:?}\t NEIGHBORS: {:?}", new_path, &new_visited, neighbors);
            neighbors.iter().for_each(|new_node| {
                if !new_visited.contains(new_node)
                {
                    visit(graph, new_node, &new_path, &new_visited, all_paths, one_pass);
                }
                else {  // Added to allow ONE lowercase room to be revisted ONCE.
                    if new_node != &"start".to_string()
                        && one_pass == &" ".to_string()
                    {
                        let new_one_pass = new_node.clone();
                        visit(graph, new_node, &new_path, &new_visited, all_paths, &new_one_pass);
                    }
                }
            });
        }
    }

    let mut all_paths: Vec<String> = vec![];  // To track all possible paths.
    let one_pass: String = " ".to_string();  // To track which lowercase room was visited twice.
    visit(&graph, &"start".to_string(), &vec![], &vec![], &mut all_paths, &one_pass);

    // println!("PATHS: {:?}", all_paths);
    println!("NUM PATHS: {}", all_paths.len());
}