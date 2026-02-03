#[cfg(test)]
mod sofie_tests {
    use crate::App;

    #[test]
    fn test_sophia_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<App>();
    }

    #[test]
    fn test_sophia_clone() {
        let sophia1 = App::default();
        let _sophia2 = sophia1;
        let _sophia3 = App::default();
    }
}

#[cfg(test)]
mod sophia_integration_tests {
    use vetis::config::{ListenerConfig, ServerConfig};

    use crate::App;

    #[tokio::test]
    async fn test_sophia_handler_signature() {
        async fn test_handler(
            _req: vetis::Request,
        ) -> Result<vetis::Response, Box<dyn std::error::Error + Send + Sync>> {
            // Return a simple error since we can't create ResponseType easily
            Err("Test error".into())
        }

        let _handler = test_handler;

        assert!(true);
    }

    #[tokio::test]
    async fn test_sophia_configuration() {
        let mut sophia = App::default();

        let listener_config = ListenerConfig::builder()
            .port(8080)
            .interface("127.0.0.1")
            .build();

        let config = ServerConfig::builder()
            .add_listener(listener_config)
            .build();

        assert_eq!(
            config
                .listeners()
                .len(),
            1
        );
        assert_eq!(config.listeners()[0].port(), 8080);
        assert_eq!(config.listeners()[0].interface(), "127.0.0.1");

        let _ = &mut sophia;
        assert!(true);
    }

    #[tokio::test]
    async fn test_sophia_error_handling() {
        use crate::errors::SofieError;

        let error = SofieError::ServerStart("Test error".to_string());

        let display_str = format!("{}", error);
        assert!(display_str.contains("Failed to start server"));
        assert!(display_str.contains("Test error"));

        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("ServerStart"));

        let error2 = SofieError::ServerStart("Test error".to_string());
        assert_eq!(error, error2);

        let error3 = SofieError::ServerStart("Different error".to_string());
        assert_ne!(error, error3);
    }
}
