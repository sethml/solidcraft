use solidcraft_smlib::{BooleanOp, Brep};
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let (Ok(x), Ok(y)) = (Brep::read(a[1].as_ref(), false), Brep::read(a[2].as_ref(), false)) else {
        eprintln!("read failed");
        return;
    };
    for op in [BooleanOp::Subtract, BooleanOp::Union, BooleanOp::Intersect] {
        match x.boolean(&y, op) {
            Ok(r) => println!("{op:?}: ok, {}", if r.is_some() { "a body" } else { "empty" }),
            Err(e) => println!("{op:?}: {e}"),
        }
    }
}
