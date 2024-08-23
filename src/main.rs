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

    println!("{}", Path::new(r"\").join(r"\.\").to_str().expect("valid Unicode"));
    println!("{}", Path::new(r"\\.\").is_absolute());
}
