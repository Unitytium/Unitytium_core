pub mod nodeservice;
pub use nodeservice::Node;


#[cfg(test)]
mod tests {
    use crate::nodeservice;
    #[test]
    fn create_node_test() {
        let node = nodeservice::Node::create(0,String::from("127.0.0.1"), String::from("dskfj8ujenwjnjdskfndsfhsdjfn"));
        print!("Node Create {}",node.to_String())
    }
}
