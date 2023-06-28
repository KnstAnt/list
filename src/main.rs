mod single_linked_list;
use crate::single_linked_list::List; 

#[derive(Debug)]
pub struct MyType {
    text: String,
}

impl MyType {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}

impl Drop for MyType {
    fn drop (&mut self) {
        println!("drop {}", self.text);
    }
}

fn main() {     
    let mut my_list = List::new();

    my_list.push(MyType::new("1".to_string()));

    dbg!(&my_list);

    my_list.push(MyType::new("2".to_string())); 
    
    dbg!(&my_list);

    my_list.iter().for_each(|v| println!("{:?}", v) );

    dbg!(my_list.pop());
//   my_list.pop_back();
    dbg!(&my_list);   


/*      
    let data = "test".to_string();

    let ptr = Box::new(data);

    let mut ptr_rc = Rc::new(ptr);

    dbg!(&ptr_rc);

    let ptr_rc2 = Rc::clone(&ptr_rc);  

    dbg!(&ptr_rc2);

    (*(*ptr_rc)) = " ".to_string();  */
}
