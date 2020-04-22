pub fn is_std() -> bool {
    #[cfg(feature = "std")]
    return true;
    #[cfg(not(feature = "std"))]
    return false;
}
