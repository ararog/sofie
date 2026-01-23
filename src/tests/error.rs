#[cfg(test)]
mod error_tests {
    use crate::errors::SofieError;
    use std::error::Error as StdError;

    #[test]
    fn test_sofie_error_display() {
        let error = SofieError::ServerStart("Failed to bind to port 8080".to_string());
        let display_str = format!("{}", error);
        assert_eq!(display_str, "Failed to start server: Failed to bind to port 8080");
    }

    #[test]
    fn test_sofie_error_debug() {
        let error = SofieError::ServerStart("Connection refused".to_string());
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("ServerStart"));
        assert!(debug_str.contains("Connection refused"));
    }

    #[test]
    fn test_sofie_error_source() {
        let error = SofieError::ServerStart("Test error".to_string());
        // SofieError doesn't implement source() since it doesn't wrap other errors
        // But we can test that it implements the Error trait
        assert!(<SofieError as StdError>::source(&error).is_none());
    }

    #[test]
    fn test_sofie_error_from_string() {
        let error_msg = "Port already in use".to_string();
        let error = SofieError::ServerStart(error_msg.clone());

        match error {
            SofieError::ServerStart(msg) => {
                assert_eq!(msg, error_msg);
            }
        }
    }

    #[test]
    fn test_sofie_error_clone() {
        let error = SofieError::ServerStart("Test error".to_string());
        let cloned_error = error.clone();
        assert_eq!(format!("{}", error), format!("{}", cloned_error));
    }

    #[test]
    fn test_sofie_error_partial_eq() {
        let error1 = SofieError::ServerStart("Same error".to_string());
        let error2 = SofieError::ServerStart("Same error".to_string());
        let error3 = SofieError::ServerStart("Different error".to_string());

        // SofieError derives PartialEq, so we can compare errors
        assert_eq!(error1, error2);
        assert_ne!(error1, error3);
    }

    #[test]
    fn test_sofie_error_send_sync() {
        // Test that SofieError implements Send and Sync
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<SofieError>();
    }
}

#[cfg(test)]
mod integration_error_tests {
    use crate::errors::SofieError;
    use std::error::Error as StdError;

    #[test]
    fn test_sofie_error_as_dyn_error() {
        let error = SofieError::ServerStart("Test error".to_string());
        let dyn_error: &dyn StdError = &error;

        assert_eq!(dyn_error.to_string(), "Failed to start server: Test error");
        assert!(dyn_error
            .source()
            .is_none());
    }

    #[test]
    fn test_sofie_error_chain() {
        let error = SofieError::ServerStart("Root cause error".to_string());

        // Test that we can use it in error handling chains
        let result: Result<(), SofieError> = Err(error);

        match result {
            Err(SofieError::ServerStart(msg)) => {
                assert_eq!(msg, "Root cause error");
            }
            _ => panic!("Expected SofieError::ServerStart"),
        }
    }
}
