use declarative::string;

fn main() {
    // Declarative macro
    //
    let a = string! {"Hello World", UC};
    println!("a = {a:?}");
}
