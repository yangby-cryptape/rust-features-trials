pub fn f() {
    println!("> run  bar-tools::f() begin...");
    #[cfg(feature = "feature-c")]
    libfoo::f();
    println!("> run  bar-tools::f() end.");
}
