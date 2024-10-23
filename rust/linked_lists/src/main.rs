// src/main.rs

mod singly_linked_node;

use singly_linked_node::Node;

fn main() {
   let mut list = Node::new(Some("Head".to_string()));

   list.append(Node::new(Some(" ".to_string())));
   list.append(Node::new(Some("This".to_string())));
   list.append(Node::new(Some("Is".to_string())));
   list.append(Node::new(Some("A".to_string())));
   list.append(Node::new(Some("Linked".to_string())));
   list.append(Node::new(Some("List".to_string())));

   list.print_forward();
}
