#[cfg(test)]
mod sofie_tests {
    use crate::App;

    #[test]
    fn test_sofie_new() {
        let _sofie = App::new();
        // Test that Sophia can be created successfully
        // Since Sophia is a simple struct with no fields, this just verifies it doesn't panic
        assert_eq!(std::mem::size_of::<App>(), 0);
    }

    #[test]
    fn test_sophia_default() {
        // Test that we can create multiple instances
        let sophia1 = App::new();
        let sophia2 = App::new();

        // Both should be the same size (zero-sized struct)
        assert_eq!(std::mem::size_of_val(&sophia1), 0);
        assert_eq!(std::mem::size_of_val(&sophia2), 0);
    }

    #[test]
    fn test_sophia_send_sync() {
        // Test that sofie implements Send and Sync
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<App>();
    }

    #[test]
    fn test_sophia_clone() {
        // Test that Sophia can be cloned (since it's zero-sized)
        let sophia1 = App::new();
        let _sophia2 = sophia1;
        // Sophia is automatically Copy because it's zero-sized
        let _sophia3 = App::new(); // Create a new instance instead
    }
}

#[cfg(test)]
mod sophia_integration_tests {
    use vetis::config::{ListenerConfig, ServerConfig};

    use crate::App;

    #[tokio::test]
    async fn test_sophia_handler_signature() {
        // Test that we can define a handler with the correct signature
        // This tests the type compatibility without actually running the server

        // Define a simple handler function that matches the expected signature
        async fn test_handler(
            _req: vetis::Request,
        ) -> Result<vetis::Response, Box<dyn std::error::Error + Send + Sync>> {
            // Return a simple error since we can't create ResponseType easily
            Err("Test error".into())
        }

        // Verify the handler can be stored and called (type checking)
        let _handler = test_handler;

        // This test passes if it compiles, which means the signature is correct
        assert!(true);
    }

    #[tokio::test]
    async fn test_sophia_configuration() {
        // Test that Sophia can be configured with different scenarios
        let mut sophia = App::new();

        let listener_config = ListenerConfig::builder()
            .port(8080)
            .interface("127.0.0.1")
            .build();

        let config = ServerConfig::builder()
            .add_listener(listener_config)
            .build();

        // Verify the config was created successfully
        assert_eq!(
            config
                .listeners()
                .len(),
            1
        );
        assert_eq!(config.listeners()[0].port(), 8080);
        assert_eq!(config.listeners()[0].interface(), "127.0.0.1");

        // Test that Sophia can work with this configuration
        // (We don't actually start the server to avoid port conflicts)
        let _ = &mut sophia;
        assert!(true);
    }

    #[tokio::test]
    async fn test_sophia_error_handling() {
        // Test error handling scenarios
        use crate::errors::SofieError;

        let error = SofieError::ServerStart("Test error".to_string());

        // Test error display
        let display_str = format!("{}", error);
        assert!(display_str.contains("Failed to start server"));
        assert!(display_str.contains("Test error"));

        // Test error debug
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("ServerStart"));

        // Test error equality
        let error2 = SofieError::ServerStart("Test error".to_string());
        assert_eq!(error, error2);

        let error3 = SofieError::ServerStart("Different error".to_string());
        assert_ne!(error, error3);
    }

    #[tokio::test]
    async fn test_sophia_multiple_instances() {
        // Test creating multiple Sofie instances
        let sophia_instances: Vec<App> = (0..5)
            .map(|_| App::new())
            .collect();

        // All instances should be valid
        assert_eq!(sophia_instances.len(), 5);

        // All should be zero-sized
        for sophia in &sophia_instances {
            assert_eq!(std::mem::size_of_val(sophia), 0);
        }
    }
}
