#[cfg(test)]
mod error_tests {
    use crate::errors::SophiaError;
    use std::error::Error as StdError;

    #[test]
    fn test_sophia_error_display() {
        let error = SophiaError::ServerStart("Failed to bind to port 8080".to_string());
        let display_str = format!("{}", error);
        assert_eq!(display_str, "Failed to start server: Failed to bind to port 8080");
    }

    #[test]
    fn test_sophia_error_debug() {
        let error = SophiaError::ServerStart("Connection refused".to_string());
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("ServerStart"));
        assert!(debug_str.contains("Connection refused"));
    }

    #[test]
    fn test_sophia_error_source() {
        let error = SophiaError::ServerStart("Test error".to_string());
        // SophiaError doesn't implement source() since it doesn't wrap other errors
        // But we can test that it implements the Error trait
        assert!(<SophiaError as StdError>::source(&error).is_none());
    }

    #[test]
    fn test_sophia_error_from_string() {
        let error_msg = "Port already in use".to_string();
        let error = SophiaError::ServerStart(error_msg.clone());

        match error {
            SophiaError::ServerStart(msg) => {
                assert_eq!(msg, error_msg);
            }
        }
    }

    #[test]
    fn test_sophia_error_clone() {
        let error = SophiaError::ServerStart("Test error".to_string());
        let cloned_error = error.clone();
        assert_eq!(format!("{}", error), format!("{}", cloned_error));
    }

    #[test]
    fn test_sophia_error_partial_eq() {
        let error1 = SophiaError::ServerStart("Same error".to_string());
        let error2 = SophiaError::ServerStart("Same error".to_string());
        let error3 = SophiaError::ServerStart("Different error".to_string());

        // SophiaError derives PartialEq, so we can compare errors
        assert_eq!(error1, error2);
        assert_ne!(error1, error3);
    }

    #[test]
    fn test_sophia_error_send_sync() {
        // Test that SophiaError implements Send and Sync
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<SophiaError>();
    }
}

#[cfg(test)]
mod integration_error_tests {
    use crate::errors::SophiaError;
    use std::error::Error as StdError;

    #[test]
    fn test_sophia_error_as_dyn_error() {
        let error = SophiaError::ServerStart("Test error".to_string());
        let dyn_error: &dyn StdError = &error;

        assert_eq!(dyn_error.to_string(), "Failed to start server: Test error");
        assert!(dyn_error
            .source()
            .is_none());
    }

    #[test]
    fn test_sophia_error_chain() {
        let error = SophiaError::ServerStart("Root cause error".to_string());

        // Test that we can use it in error handling chains
        let result: Result<(), SophiaError> = Err(error);

        match result {
            Err(SophiaError::ServerStart(msg)) => {
                assert_eq!(msg, "Root cause error");
            }
            _ => panic!("Expected SophiaError::ServerStart"),
        }
    }
}
