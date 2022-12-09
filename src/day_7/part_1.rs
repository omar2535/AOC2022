use std::{fs::File, io::{BufReader, BufRead}, rc::Rc, borrow::BorrowMut, cell::RefCell};

pub enum FileType {
    FILE,
    DIR,
}

pub struct Node<'a> {
    name: String,
    node_type: FileType,
    size: u64,
    parent: Option<Rc<&'a Node<'a>>>,
    children: RefCell<Vec<&'a Node<'a>>>
}

impl Node<'_> {
    fn get_child(self: &Self, node_name: &str) -> &Node {
        for child in self.children.borrow().iter() {
            if child.name == node_name {
                return child;
            }
        }
        panic!("NO");
    }

    fn add_child(self: &Self, new_node: &Node) {
        self.children.borrow_mut().push(new_node);
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
        children: RefCell::new(vec![])
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
                // skip a line
                
                // get the line range with ls output
                let mut range: usize = 0;
                i += 1;
                while i+range < lines.len() && !lines[i+range].contains(&CMD_STRING) {
                    range += 1;
                }

                for j in 0..range {
                    let ls_info: Vec<&str> = line.split(' ').collect::<Vec<&str>>();
                    let type_or_size: &str = ls_info[0];
                    let name: &str = ls_info[1];
                    let filetype: FileType;
                    let mut size: u64 = 0;

                    if type_or_size == "dir" {
                        filetype = FileType::DIR;
                    } else {
                        filetype = FileType::FILE;
                        size = type_or_size.parse().unwrap();
                    }

                    // create the new node
                    let newnode: Node = Node {
                        name: name.to_string(),
                        node_type: filetype,
                        size: size,
                        parent: Some(Rc::new(cur_node)),
                        children: RefCell::new(vec![])
                    };

                    // add this node to the current node's children
                    cur_node.add_child(&newnode);
                    
                }
            }            
        }


        
        i += 1;
        //if it's a cd, change current node depending on .. or <dir>
    }
}
