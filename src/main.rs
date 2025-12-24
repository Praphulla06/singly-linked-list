use linked_list::List;
fn main() {
    let mut l = List::new();

    l.insert_at_head(10.10);
    l.insert_at_tail(20.20);
    l.insert_at_head(30.30);
    l.insert_at_tail(40.40);
    match l.traverse() {
        Ok(list) => println!("{:#?}", list),
        Err(e) => println!("{}", e),
    }

    if let Err(e) = l.delete_from_head() {
        println!("{}", e);
    }
    match l.traverse() {
        Ok(list) => println!("{:#?}", list),
        Err(e) => println!("{}", e),
    }

    if let Err(e) = l.delete_from_tail() {
        println!("{}", e);
    }
    match l.traverse() {
        Ok(list) => println!("{:#?}", list),
        Err(e) => println!("{}", e),
    }
}
