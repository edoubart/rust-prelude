#[derive(Debug)]
//enum LinkedList {
//    Empty,
//    Node {
//        value: i32,
//        next: LinkedList, // error[E0072]: recursive type `LinkedList` has infinite size
//    }
//}

//enum LinkedList {
//    Empty,
//    Node {
//        value: i32,
//        next: Box<LinkedList>, // Doesn't store a nested LinkedList, but a pointer through the Box instead.
//    }
//}

// With Generic
//enum LinkedList<T> {
//    Empty,
//    Node {
//        value: T,
//        next: Box<LinkedList<T>>, // Doesn't store a nested LinkedList, but a pointer through the Box instead.
//    }
//}

// Using Box
enum LinkedListUsingBox<T> {
    Empty,
    Node {
        value: T,
        next: Box<LinkedListUsingBox<T>>,
    }
}

// Using Reference
//enum LinkedListUsingReference<T> {
//    Empty,
//    Node {
//        value: T,
//        next: &LinkedListUsingReference<T>, // error[E0106]: missing lifetime specifier
//    }
//}

//enum LinkedListUsingReference<'a, T> {
//    Empty,
//    Node {
//        value: T,
//        next: &'a LinkedListUsingReference<'a, T>,
//    }
//}

// Helpers
//fn create_list<'a>() -> LinkedListUsingReference<'a, i32> {
//    let second_node = LinkedListUsingReference::Node {
//        value: 2,
//        next: &LinkedListUsingReference::Empty,
//    };
//
//    let first_node = LinkedListUsingReference::Node {
//        value: 1,
//        next: &second_node, // error[E0515]: cannot return value referencing local variable `second_node`
//    };
//
//    first_node
//}

fn create_list() -> LinkedListUsingBox<i32> {
    let second_node: LinkedListUsingBox<i32> = LinkedListUsingBox::Node {
        value: 2,
        next: Box::new(LinkedListUsingBox::Empty),
    };

    let first_node: LinkedListUsingBox<i32> = LinkedListUsingBox::Node {
        value: 1,
        next: Box::new(second_node),
    };

    first_node
}

fn main() {
    //let list = LinkedList::Node {
    //    value: 1,
    //    next: Box::new(LinkedList::Empty),
    //};

    //let list = LinkedList::Node {
    //    value: 1,
    //    next: Box::new(LinkedList::Node {
    //        value: 2,
    //        next: Box::new(LinkedList::Node {
    //            value: 3,
    //            next: Box::new(LinkedList::Empty),
    //        }),
    //    }),
    //};

    //let list = LinkedList::Node {
    //    value: 1.3,
    //    next: Box::new(LinkedList::Node {
    //        value: 2.6,
    //        next: Box::new(LinkedList::Node {
    //            value: 3.9,
    //            next: Box::new(LinkedList::Empty),
    //        }),
    //    }),
    //};

    //println!("{list:#?}");

    //let im_with_you: LinkedList<String> = LinkedList::Node {
    //    value: String::from("I'm With You"),
    //    next: Box::new(LinkedList::Empty),
    //};

    //let sk8er_boi: LinkedList<String> = LinkedList::Node {
    //    value: String::from("Sk8er Boi"),
    //    next: Box::new(im_with_you),
    //};

    //let complicated: LinkedList<String> = LinkedList::Node {
    //    value: String::from("Complicated"),
    //    next: Box::new(sk8er_boi),
    //};

    //println!("{complicated:#?}");

    //let second_node = LinkedListUsingReference::Node {
    //    value: 2,
    //    next: &LinkedListUsingReference::Empty,
    //};

    //let first_node = LinkedListUsingReference::Node {
    //    value: 1,
    //    next: &second_node,
    //};

    //drop(first_node);

    ////println!("{first_node:#?}");
    //println!("{second_node:#?}");

    //let second_node: LinkedListUsingBox<i32> = LinkedListUsingBox::Node {
    //    value: 2,
    //    next: Box::new(LinkedListUsingBox::Empty),
    //};

    //let first_node: LinkedListUsingBox<i32> = LinkedListUsingBox::Node {
    //    value: 1,
    //    next: Box::new(second_node),
    //};

    //println!("{first_node:?}");
    //println!("{second_node:#?}"); // error[E0382]: borrow of moved value: `second_node`

    let list: LinkedListUsingBox<i32> = create_list();
    println!("{list:#?}");
}
