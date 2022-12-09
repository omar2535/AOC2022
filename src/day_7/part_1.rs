use std::{fs::File, io::{BufReader, BufRead}};

pub enum FileType {
    FILE,
    DIR,
}

pub struct Node {
    name: String,
    node_type: FileType,
    size: u64,
    parent: Option<Box<Node>>,
    children: Vec<Node>
}

impl Node {
    fn get_child(self: &Self, node_name: &str) -> &Node {
        for child in self.children.iter() {
            if child.name == node_name {
                return child;
            }
        }
        panic!("NO");
    }

    fn get_parent(self: &Self) -> &Node {
        return match &self.parent {
            Some(parent) => &parent,
            None => self
        }
    }
}

pub fn part_1() {
    let CD_STRING = String::from("cd");
    let LS_STRING = String::from("ls");
    let CMD_STRING = String::from("$");


    let root_node = &Node {
        name: String::from("/"),
        node_type: FileType::DIR,
        size: 0,
        parent: None,
        children: vec![]
    };

    let mut cur_node: &Node = root_node;

    // start parsing lines
    let file_result = File::open("./data/day_7.txt");
    let file = file_result.unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    let mut i: usize = 0;
    while i < lines.len() {
        let line: &String = &lines[i];

        if line.contains(&CMD_STRING) {
            if line.contains(&CD_STRING) {
                let cd_dir: &str = line.split(' ').collect::<Vec<&str>>()[2];
                if cd_dir == ".." {
                    cur_node = cur_node.get_parent();
                } else {
                    cur_node = cur_node.get_child(cd_dir);
                }
            } else if line.contains(&LS_STRING) {
                
                
            }            
        }


        
        i += 1;
        //if it's a cd, change current node depending on .. or <dir>
    }
}
