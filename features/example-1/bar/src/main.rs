fn main() {
    println!("Begin:");
    println!("> exec bar-tools::f() start ...");
    bar_tools::f();
    println!("> exec bar-tools::f() done.");
    println!("> exec libfoo::f() start ...");
    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    libfoo::f();
    println!("> exec libfoo::f() done.");
    println!("End.");
}
