use linked_list::List;
fn main() {
    let mut l = List::new();

    l.insert_at_head(100);
    l.insert_at_head(200);
    l.insert_at_head(300);
    l.insert_at_tail(400);
    l.insert_at_tail(500);
    l.insert_at_tail(600);
    l.insert_at_tail(700);
    l.insert_at_head(800);

    println!("{:#?}", l.traverse()); 
    
    l.delete_from_head();
    println!("{:#?}", l.traverse()); 
    
    l.delete_from_tail();
    println!("{:#?}", l.traverse()); 
}
