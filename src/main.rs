use std::path::Path;

fn main() {
    let c_drive = Path::new("C:");
    println!("{}", c_drive.join("/").to_str().expect("valid Unicode"));
    println!("{}", c_drive.join(r"\").to_str().expect("valid Unicode"));
}
