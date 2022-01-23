static mut UUID_COUNTER: usize = 0;

/// Generates a uid, which is us just a counter.
///
/// Will not return a uid of 0, because the counter is incremented before it is returned.
/// Also means the max value cannot be returned as this function will panic due to trying to
/// increment with overflow.
pub fn generate_uid() -> usize {
    unsafe {
        UUID_COUNTER += 1;
        UUID_COUNTER
    }
}