use std::{fs::File, io::{BufReader, BufRead}, rc::Rc, borrow::{BorrowMut, Borrow}, cell::{RefCell, Ref}, collections::HashMap, hash::Hash};

pub enum FileType {
    FILE,
    DIR,
}

pub struct Node {
    name: String,
    node_type: FileType,
    size: u64,
    self_idx: usize,
    parent: usize,
    children: RefCell<Vec<usize>>
}

impl Node {
    fn get_child(self: &Self, node_name: String, node_list: &Vec<Node>) -> Option<usize> {
        for index in self.children.borrow().iter() {
            if node_list[*index].name == node_name {
                return Some(*index);
            }
        }
        None
    }
}

pub fn get_nodelist() -> Vec<Node> {
    let file_result = File::open("./data/day_7.txt");
    let file = file_result.unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();   
    
    let CMD_STRING = String::from("$");
    let CD_STRING = String::from("cd");

    let mut node_list: Vec<Node> = Vec::new();
    let mut cur_path: Vec<String> = vec![String::from("/")];

    let root_node = Node {
        name: cur_path[0].clone(),
        node_type: FileType::DIR,
        self_idx: 0,
        size: 0,
        parent: 0,
        children: RefCell::new(vec![])
    };
    node_list.push(root_node);

    let mut i: usize = 0;
    while i < lines.len() {
        let line: &String = &lines[i];
        if line.contains(&CMD_STRING) {
            if line.contains(&CD_STRING) {
                let cd_dir: &str = line.split(' ').collect::<Vec<&str>>()[2];
                if cd_dir == ".." {
                    if cur_path.len() == 1 {
                        // nothing
                    } else {
                        cur_path.pop();
                    }
                } else {
                    cur_path.push(cd_dir.to_string());
                }
            }
        }

        if !line.contains(&CMD_STRING) {
            let ls_info: Vec<&str> = line.split(' ').collect::<Vec<&str>>();
            let type_or_size: &str = ls_info[0];
            let name: String = cur_path.join("/") + ls_info[1];
            let filetype: FileType;
            let mut size: u64 = 0;

            if type_or_size == "dir" {
                filetype = FileType::DIR;
            } else {
                filetype = FileType::FILE;
                size = type_or_size.parse().unwrap();
            }

            // create the new node
            let new_node: Node = Node {
                name: name,
                node_type: filetype,
                self_idx: node_list.len(),
                size: size,
                parent: 0,
                children: RefCell::new(vec![])
            };

            // add this node to children
            // can't do this since references to the current node list will be invalidated
            node_list.push(new_node);
        }
        i += 1;
    }

    return node_list;
}

pub fn get_node_index(nodelist: &Vec<Node>, name: String) -> usize {
    println!("Get node index for: {}", name);
    for (index, node) in nodelist.iter().enumerate() {
        if node.name == name {
            return index;
        }
    }
    panic!("Can't find node");
}

fn update_sums(node: Node, ) -> u64{
    if node.size != 0 {
        return node.size;
    } else {
        let mut sum: u64 = 0;
        for child_node in node.children.borrow().iter() {
            sum += update_sums(child_node);
        }
        return sum;
    }
}

pub fn part_1() {
    let CD_STRING = String::from("cd");
    let LS_STRING = String::from("ls");
    let CMD_STRING = String::from("$");
    let node_list: Vec<Node> = get_nodelist();
    let mut parent_mappings: HashMap<usize, usize> = HashMap::new();
    let mut cur_path: Vec<String> = vec![String::from("/")];


    let root_node: &Node = &node_list[0];
    let mut cur_node = root_node;

    // start parsing lines
    let file_result = File::open("./data/day_7.txt");
    let file = file_result.unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    let mut i: usize = 0;
    while i < lines.len() {
        let line: &String = &lines[i];
        println!("{}", line);

        if line.contains(&CMD_STRING) {
            if line.contains(&CD_STRING) {
                let cd_dir: &str = line.split(' ').collect::<Vec<&str>>()[2];
                if cd_dir == ".." {
                    let parent_idx = parent_mappings.get(&cur_node.self_idx).unwrap();
                    cur_node = &node_list[*parent_idx];
                    // update path
                    if cur_path.len() == 1 {
                        // nothing
                    } else {
                        cur_path.pop();
                    }
                } else {
                    let child_name: String = cur_path.join("/") + cd_dir;
                    cur_node = &node_list[cur_node.get_child(child_name, &node_list).unwrap()];
                    cur_path.push(cd_dir.to_string());
                }
            } else if line.contains(&LS_STRING) {                
                // get the line range with ls output
                let mut range: usize = 0;
                i += 1;
                while i+range < lines.len() && !lines[i+range].contains(&CMD_STRING) {
                    range += 1;
                }

                for j in 0..range {
                    // find in node list
                    // add the node index to the children of the current node
                    let listing_line = &lines[i+j];
                    let ls_info: Vec<&str> = listing_line.split(' ').collect::<Vec<&str>>();
                    let name: String = cur_path.join("/") + ls_info[1];

                    let idx: usize = get_node_index(&node_list, name);
                    parent_mappings.insert(idx, cur_node.self_idx);
                    cur_node.children.borrow_mut().push(idx);
                }
                i += range-1;
            }
        }
        i += 1;
    }

    // Here, we have a good grpah now (hopefully)
}
