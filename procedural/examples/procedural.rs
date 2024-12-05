use procedural::html;

fn main() {
    // Function like procedural macro
    //
    let b = html!(1 < 2);
    println!("b = {b:?}");
}
