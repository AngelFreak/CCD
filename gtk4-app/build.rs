fn main() {
    // Compile GLib resources if we add them later
    // This is a placeholder for future resource compilation
    println!("cargo:rerun-if-changed=resources/");
}
