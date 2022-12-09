mod utils;
mod day8;

fn main() {
  // day7::day_7();
  /*let mut root = Node { name: "a".to_string(), children: vec![] };
  root.children.push(Node { name: "b".to_string(), children: vec![] });
  root.children.last_mut().unwrap().children.push(Node { name: "c".to_string(), children: vec![] });
  root.children.last_mut().unwrap().children.last_mut().unwrap().children.push(Node { name: "c".to_string(), children: vec![] });*/
  day8::day_8_part_2();
  //let a = vec![Node { name: "a".to_string(), children: vec![]}, Node { name: "b".to_string()}, Node { name: "c".to_string()} ];
  /*let mut stack: Vec<&Node> = vec![];
  //let b = &a[0];
  //stack.push(b);*/
}

/*struct Node {
  name: String,
  children: Vec<Node>
}*/

