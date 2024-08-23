use std::path::Path;

fn main() {
    let drive = Path::new("C:");
    let slash = Path::new("/");
    let backslash = Path::new(r"\");
    let drive_slash = drive.join(slash);
    let drive_backslash = drive.join(backslash);

    for path in [drive, slash, backslash, drive_slash.as_path(), drive_backslash.as_path()] {
        let path_str = path.to_str().expect("valid Unicode");
        println!("{:3}  relative? {:5}  absolute? {}", path_str, path.is_relative(), path.is_absolute());
    }
}

// fn main() {
//     let drive = Path::new("C:");
//     for sep in ["/", r"\"] {
//         let joined = drive.join(sep);


//         println!("{}  {}", joined.to_str().expect("valid Unicode"), joined.is_absolute());
//     }

//     let q = c_drive.join("/");
//     let r = c_drive.join(r"\");
//     println("{}")
//     println!("{}", c_drive.join("/").to_str().expect("valid Unicode"));
//     println!("{}", c_drive.join(r"\").to_str().expect("valid Unicode"));
// }
