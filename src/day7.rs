use std::{cell::RefCell, rc::{Rc, Weak}};

pub fn day_7() {
  let root = execute();
  let mut result = vec![];
  root.sizes_collect(&mut result, |total_size, self_size, collection| {
    if total_size <= 100000 && self_size == 0 {
      collection.push(total_size);
    }
  });
  let sum: u32 = result.iter().sum();
  println!("Result is {}", sum);
}

pub fn day_7_part_2() {
  let root = execute();
  let mut sizes = vec![];
  let total_size = root.sizes_collect(&mut sizes, |total_size, _, collection| {
    collection.push(total_size);
  });
  sizes.sort();

  let needed_size = 30000000 - (70000000 - total_size);
  let result = sizes.iter().find(|size| **size >= needed_size).unwrap();
  println!("Result is {}", result);
}

fn execute() -> Rc<TreeNode> {
  let root = TreeNode::new();
  let mut curr_node = &mut Rc::clone(&root);

  if let Ok(lines) = super::utils::read_lines("input7.txt") {
    for line in lines {
      if let Ok(line) = line {
        let line: Vec<&str> = line.split_whitespace().filter_map(|line| Some(line)).collect();
         match line[0] {
          "$" => command(line[1..].to_vec(), &mut curr_node),
          "dir" => add_dir(line[1].to_string(), curr_node),
          _ => add_file(line, curr_node)
        }
      }
    }
  }

  root
}

fn command(command: Vec<&str>, curr: &mut Rc<TreeNode>) {
  match command[0] {
    "ls" => return,
    "cd" => {
      if command[1] == ".." {
        *curr = curr.parent.upgrade().unwrap()
      } else if command[1] != "/" {
        let dir = command[1].to_owned();
        let node = Rc::clone(curr.children.borrow_mut().iter_mut().find(|node| node.name == dir).unwrap());
        *curr = node;
      }
    },
    _ => println!("Unknown command")
  }
}

fn add_dir(dir: String, curr: &Rc<TreeNode>) {
  curr.add_child(dir, 0);
}

fn add_file(file: Vec<&str>, curr: &Rc<TreeNode>) {
  let filename = file[1].to_string();
  let size = file[0].parse().unwrap();
  curr.add_child(filename, size);
}

type NodeVec = RefCell<Vec<Rc<TreeNode>>>;
type CollectFn = fn(u32, u32, &mut Vec<u32>);

#[derive(Debug)]
struct TreeNode {
  parent: Weak<TreeNode>,
  children: NodeVec,
  name: String,
  size: u32
}

impl TreeNode {
  fn new() -> Rc<TreeNode> {
    let new_node = Rc::new(TreeNode { parent: Weak::new(), children: RefCell::new(vec![]), name: "/".to_string(), size: 0 });
    new_node
  }
  
  fn add_child(self: &Rc<Self>, name: String, size: u32) {
    let new_node = Rc::new(TreeNode { parent: Rc::downgrade(self), children: RefCell::new(vec![]), name, size });
    self.children.borrow_mut().push(new_node);
  }

  fn sizes_collect(&self, collection: &mut Vec<u32>, collect_fn: CollectFn) -> u32 {
    let mut sizes: Vec<u32> = self.children.borrow().iter()
    .map(|child| child.sizes_collect(collection, collect_fn))
    .collect();
    sizes.push(self.size);

    let size = sizes.iter().sum();
    collect_fn(size, self.size, collection);
    size
  }
}