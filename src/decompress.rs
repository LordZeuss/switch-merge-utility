use std::process::Command;

pub fn decompress(files: Vec<String>) {
    // Join the files into a single string, with each file separated by a space
    let files_joined = files.join(" ");

    let output = Command::new("nsz")
        .arg("-D")
        .arg(files_joined) // Pass the joined file paths
        .output()
        .expect("Failed to decompress files!");

    if !output.status.success() {
        eprintln!("Decompression failed: {:?}", String::from_utf8_lossy(&output.stderr));
    } else {
        println!("Decompression successful: {:?}", String::from_utf8_lossy(&output.stdout));
    }
}
