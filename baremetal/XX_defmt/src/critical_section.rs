#[cfg(not(feature = "std"))] // needed for `cargo test --features std`

mod no_std {
    // This is a type alias for the enabled `restore-state-*` feature.
    // For example, it is `bool` if you enable `restore-state-bool`.
    use critical_section::RawRestoreState;

    struct MyCriticalSection;
    critical_section::set_impl!(MyCriticalSection);

    unsafe impl critical_section::Impl for MyCriticalSection {
        unsafe fn acquire() -> RawRestoreState {
            // TODO
        }

        unsafe fn release(token: RawRestoreState) {
            // TODO
        }
    }
}
