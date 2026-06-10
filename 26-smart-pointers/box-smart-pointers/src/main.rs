fn main() {
    let my_box: Box<i32> = Box::new(100); // The Box itself is a Struct.
    println!("{}", *my_box);
    println!("{my_box}");
    println!("{}", my_box);
    println!("{:?}", my_box);

    let your_box: Box<i32> = my_box;
    println!("{your_box}");
}
