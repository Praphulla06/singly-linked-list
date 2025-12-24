use linked_list::List;
fn main() {
    let mut l = List::new();

    l.insert_at_head(100);
    l.insert_at_tail(200);
    l.insert_at_head(300);
    l.insert_at_tail(400);
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
