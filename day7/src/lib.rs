use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

struct Folder {
    name: String,
    parent: Option<Rc<RefCell<Node>>>,
    children: Vec<Rc<RefCell<Node>>>,
}

struct File {
    name: String,
    parent: Rc<RefCell<Node>>,
    size: usize,
}

enum Node {
    Folder(Folder),
    File(File),
}

struct Session {
    root: Rc<RefCell<Node>>,
    node: Rc<RefCell<Node>>,
}

impl Folder {
    fn new(name: String, parent: Option<Rc<RefCell<Node>>>) -> Folder {
        Folder {
            name,
            parent,
            children: Vec::new(),
        }
    }

    fn root() -> Folder {
        Folder {
            name: "/".to_string(),
            parent: None,
            children: Vec::new(),
        }
    }
}

impl File {
    fn new(name: String, parent: Rc<RefCell<Node>>, size: usize) -> File {
        File { name, parent, size }
    }
}

impl Node {
    fn name(&self) -> String {
        match self {
            Node::Folder(folder) => folder.name.to_string(),
            Node::File(file) => file.name.to_string(),
        }
    }

    fn size(&self) -> usize {
        match self {
            Node::Folder(_) => self
                .children()
                .iter()
                .map(|child| child.borrow().size())
                .sum(),
            Node::File(file) => file.size,
        }
    }

    fn parent(&self) -> Option<Rc<RefCell<Node>>> {
        match self {
            Node::Folder(folder) => folder.parent.as_ref().map(Rc::clone),
            Node::File(file) => Some(Rc::clone(&file.parent)),
        }
    }

    fn children(&self) -> Vec<Rc<RefCell<Node>>> {
        match self {
            Node::Folder(folder) => folder.children.iter().map(Rc::clone).collect(),
            Node::File(_) => panic!("{}: Not a directory", self.name()),
        }
    }

    fn get_child(&self, name: &str) -> Rc<RefCell<Node>> {
        return Rc::clone(
            self.children()
                .iter()
                .find(|child| child.borrow().name() == name)
                .unwrap(),
        );
    }

    fn add_child(&mut self, node: Rc<RefCell<Node>>) {
        match self {
            Node::Folder(folder) => folder.children.push(node),
            Node::File(_) => panic!("{}: Not a directory", self.name()),
        }
    }

    fn traverse(&self, callback: &mut impl FnMut(&Node)) {
        callback(self);
        match self {
            Node::Folder(_) => {
                self.children().iter().for_each(|child| {
                    child.borrow().traverse(callback);
                });
            }
            Node::File(_) => {}
        }
    }

    fn _as_tree_str(&self, depth: usize) -> String {
        let mut output = format!("{}- {}", "  ".repeat(depth), self.name());
        return match self {
            Node::Folder(folder) => {
                output += " (dir)";
                folder.children.iter().fold(output, |output, child| {
                    output + "\n" + child.borrow()._as_tree_str(depth + 1).as_str()
                })
            }
            Node::File(_) => output + format!(" (file, size={})", self.size()).as_str(),
        };
    }
}

impl Session {
    fn new() -> Session {
        let root = Rc::new(RefCell::new(Node::Folder(Folder::root())));
        let node = Rc::clone(&root);
        Session { root, node }
    }

    fn from_output(output: String) -> Session {
        let mut commands_output = output.split("$ ");
        commands_output.next(); // Skip empty result before first `$`.

        let mut session = Session::new();

        for command_output in commands_output {
            let mut lines = command_output.lines();
            let command = lines.next().unwrap();

            if command.starts_with("cd") {
                let path = command.split(' ').nth(1).unwrap();
                session.cd(path)
            } else if command.starts_with("ls") {
                let list = lines.collect::<Vec<&str>>();
                session.ls(list)
            } else {
                panic!("Unsupported command: {}", command);
            }
        }

        session
    }

    fn cd(&mut self, path: &str) {
        self.node = match path {
            "/" => Rc::clone(&self.root),
            ".." => match Rc::clone(&self.node).borrow().parent() {
                Some(parent) => Rc::clone(&parent),
                None => return,
            },
            _ => Rc::clone(&self.node).borrow().get_child(path),
        }
    }

    fn ls(&self, list: Vec<&str>) {
        for line in list {
            let node = if line.starts_with("dir") {
                let name = line.split(' ').nth(1).unwrap().to_string();
                Node::Folder(Folder::new(name, Some(Rc::clone(&self.node))))
            } else {
                let mut parts = line.split(' ');
                let size = parts.next().unwrap().parse::<usize>().unwrap();
                let name = parts.next().unwrap().to_string();
                Node::File(File::new(name, Rc::clone(&self.node), size))
            };
            self.node
                .borrow_mut()
                .add_child(Rc::new(RefCell::new(node)));
        }
    }
}

pub fn sum_total_size_of_directories_up_to(filename: String, limit: usize) -> usize {
    let input = fs::read_to_string(filename).unwrap();
    let session = Session::from_output(input);
    let mut sum = 0;
    session
        .root
        .borrow()
        .traverse(&mut |node: &Node| match node {
            Node::Folder(_) => {
                let size = node.size();
                if size < limit {
                    sum += size;
                }
            }
            Node::File(_) => {}
        });
    sum
}

pub fn smallest_directory_to_free_up(filename: String, capacity: usize, to_free: usize) -> usize {
    let input = fs::read_to_string(filename).unwrap();
    let session = Session::from_output(input);
    let limit = to_free - (capacity - session.root.borrow().size());
    let mut smallest = session.root.borrow().size();
    session
        .root
        .borrow()
        .traverse(&mut |node: &Node| match node {
            Node::Folder(_) => {
                let size = node.size();
                if size >= limit && size < smallest {
                    smallest = size;
                }
            }
            Node::File(_) => {}
        });
    smallest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_parsing() {
        let input = fs::read_to_string("assets/example.txt").unwrap();
        let session = Session::from_output(input);
        assert_eq!(
            session.root.borrow()._as_tree_str(0),
            "
- / (dir)
  - a (dir)
    - e (dir)
      - i (file, size=584)
    - f (file, size=29116)
    - g (file, size=2557)
    - h.lst (file, size=62596)
  - b.txt (file, size=14848514)
  - c.dat (file, size=8504156)
  - d (dir)
    - j (file, size=4060174)
    - d.log (file, size=8033020)
    - d.ext (file, size=5626152)
    - k (file, size=7214296)
            "
            .trim()
        );
    }

    #[test]
    fn part1() {
        assert_eq!(
            sum_total_size_of_directories_up_to("assets/example.txt".to_string(), 100000),
            95437
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            smallest_directory_to_free_up("assets/example.txt".to_string(), 70000000, 30000000),
            24933642
        );
    }
}
