#[cfg(any(target_os = "windows", target_os = "macos"))]
compile_error!("this crate should be only compile under Linux");

pub fn f() {
    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    println!("> libfoo under linux");
    #[cfg(any(target_os = "windows", target_os = "macos"))]
    println!("> libfoo under windows or macos");
    #[cfg(feature = "feature-a")]
    println!("> libfoo::feature-a enabled");
    #[cfg(feature = "feature-b")]
    println!("> libfoo::feature-b enabled");
    #[cfg(feature = "feature-c")]
    println!("> libfoo::feature-c enabled");
}
