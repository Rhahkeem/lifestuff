use std::ffi::OsString;

use crate::currency::ENV_VAR_NAME;

/// Temporarily overrides the API endpoint and restores it, even during unwinding.
/// All tests reading or changing this variable must use the same `#[serial]` lock.
pub(crate) struct ApiEndpointGuard(Option<OsString>);

impl ApiEndpointGuard {
    pub(crate) fn new(value: Option<&str>) -> Self {
        let previous = std::env::var_os(ENV_VAR_NAME);
        // SAFETY: Callers hold the shared #[serial] lock for endpoint tests.
        unsafe {
            match value {
                Some(value) => std::env::set_var(ENV_VAR_NAME, value),
                None => std::env::remove_var(ENV_VAR_NAME),
            }
        }
        Self(previous)
    }
}

impl Drop for ApiEndpointGuard {
    fn drop(&mut self) {
        // SAFETY: The guard is dropped before the caller releases its #[serial] lock.
        unsafe {
            match &self.0 {
                Some(value) => std::env::set_var(ENV_VAR_NAME, value),
                None => std::env::remove_var(ENV_VAR_NAME),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn endpoint_is_restored_after_a_panic() {
        let previous = std::env::var_os(ENV_VAR_NAME);
        let result = std::panic::catch_unwind(|| {
            let _endpoint = ApiEndpointGuard::new(Some("https://temporary.example.com"));
            panic!("simulate a failing assertion");
        });

        assert!(result.is_err());
        assert_eq!(std::env::var_os(ENV_VAR_NAME), previous);
    }
}
