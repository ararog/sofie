#[cfg(test)]
mod handler_tests {
    use http::HeaderValue;
    use hyper::StatusCode;
    use vetis::Response;

    #[tokio::test]
    async fn test_response_creation() {
        let response: Response = Response::builder()
            .status(StatusCode::OK)
            .text("Hello, World!");

        assert_eq!(
            response
                .into_inner()
                .status(),
            StatusCode::OK
        );
    }

    #[tokio::test]
    async fn test_json_response() {
        let json_data = r#"{"message": "Hello from Sophia", "status": "success"}"#;

        let response: Response = Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", HeaderValue::from_static("application/json"))
            .text(json_data);

        let inner_response = response.into_inner();

        assert_eq!(inner_response.status(), StatusCode::OK);
        assert_eq!(
            inner_response
                .headers()
                .get("Content-Type")
                .unwrap(),
            "application/json"
        );
    }

    #[tokio::test]
    async fn test_error_response() {
        let response: Response = Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .text("Internal Server Error");

        assert_eq!(
            response
                .into_inner()
                .status(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[tokio::test]
    async fn test_response_with_headers() {
        let response: Response = Response::builder()
            .status(StatusCode::OK)
            .header("X-Custom-Header", HeaderValue::from_static("test-value"))
            .header("Cache-Control", HeaderValue::from_static("no-cache"))
            .text("Response with headers");

        let inner_response = response.into_inner();

        assert_eq!(inner_response.status(), StatusCode::OK);
        assert_eq!(
            inner_response
                .headers()
                .get("X-Custom-Header")
                .unwrap(),
            "test-value"
        );
        assert_eq!(
            inner_response
                .headers()
                .get("Cache-Control")
                .unwrap(),
            "no-cache"
        );
    }

    #[tokio::test]
    async fn test_empty_response() {
        let response: Response = Response::builder()
            .status(StatusCode::NO_CONTENT)
            .text("");

        let inner_response = response.into_inner();

        assert_eq!(inner_response.status(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn test_different_status_codes() {
        let status_codes = vec![
            StatusCode::OK,
            StatusCode::CREATED,
            StatusCode::ACCEPTED,
            StatusCode::NO_CONTENT,
            StatusCode::BAD_REQUEST,
            StatusCode::UNAUTHORIZED,
            StatusCode::FORBIDDEN,
            StatusCode::NOT_FOUND,
            StatusCode::INTERNAL_SERVER_ERROR,
        ];

        for status in status_codes {
            let response: Response = Response::builder()
                .status(status)
                .text("Test response");

            let inner_response = response.into_inner();

            assert_eq!(inner_response.status(), status);
        }
    }

    #[tokio::test]
    async fn test_large_response() {
        let large_data = "x".repeat(100_000); // 100KB of data

        let response: Response = Response::builder()
            .status(StatusCode::OK)
            .header(
                "Content-Length",
                HeaderValue::from_str(
                    &large_data
                        .len()
                        .to_string(),
                )
                .unwrap(),
            )
            .text(&large_data);

        assert_eq!(
            response
                .into_inner()
                .status(),
            StatusCode::OK
        );
    }

    #[tokio::test]
    async fn test_async_operations() {
        let result = async {
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            "Async result"
        }
        .await;

        assert_eq!(result, "Async result");
    }

    #[tokio::test]
    async fn test_handler_function() {
        async fn test_handler() -> Response {
            Response::builder()
                .status(StatusCode::OK)
                .text("Handler response")
        }

        let response = test_handler().await;
        assert_eq!(
            response
                .into_inner()
                .status(),
            StatusCode::OK
        );
    }
}

#[cfg(test)]
mod handler_integration_tests {
    use http::HeaderValue;
    use hyper::StatusCode;
    use vetis::Response;

    #[tokio::test]
    async fn test_rest_api_patterns() {
        let success_response: Response = Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", HeaderValue::from_static("application/json"))
            .text(r#"{"status": "success", "data": {}}"#);
        assert_eq!(
            success_response
                .into_inner()
                .status(),
            StatusCode::OK
        );

        let created_response: Response = Response::builder()
            .status(StatusCode::CREATED)
            .header("Content-Type", HeaderValue::from_static("application/json"))
            .header("Location", HeaderValue::from_static("/api/users/123"))
            .text(r#"{"id": 123, "created": true}"#);
        assert_eq!(
            created_response
                .into_inner()
                .status(),
            StatusCode::CREATED
        );

        let error_response: Response = Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header("Content-Type", HeaderValue::from_static("application/json"))
            .text(r#"{"error": "Bad Request", "message": "Invalid input"}"#);
        assert_eq!(
            error_response
                .into_inner()
                .status(),
            StatusCode::BAD_REQUEST
        );

        let not_found_response: Response = Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header("Content-Type", HeaderValue::from_static("application/json"))
            .text(r#"{"error": "Not Found", "message": "Resource not found"}"#);
        assert_eq!(
            not_found_response
                .into_inner()
                .status(),
            StatusCode::NOT_FOUND
        );
    }

    #[tokio::test]
    async fn test_content_type_handling() {
        let content_types = vec![
            ("application/json", r#"{"key": "value"}"#),
            ("text/plain", "Plain text content"),
            ("text/html", "<html><body>Hello</body></html>"),
            ("application/xml", r#"<root><item>test</item></root>"#),
        ];

        for (content_type, body) in content_types {
            let response: Response = Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", HeaderValue::from_static(content_type))
                .text(body);

            let inner_response = response.into_inner();

            assert_eq!(inner_response.status(), StatusCode::OK);
            assert_eq!(
                inner_response
                    .headers()
                    .get("Content-Type")
                    .unwrap(),
                content_type
            );
        }
    }

    #[tokio::test]
    async fn test_response_chaining() {
        async fn process_request() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
            tokio::time::sleep(std::time::Duration::from_millis(1)).await;

            tokio::time::sleep(std::time::Duration::from_millis(1)).await;

            Ok("Processed data".to_string())
        }

        async fn create_response(data: String) -> Response {
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", HeaderValue::from_static("application/json"))
                .text(&format!(r#"{{"result": "{}"}}"#, data))
        }

        let data = process_request()
            .await
            .unwrap();
        let response: Response = create_response(data).await;

        assert_eq!(
            response
                .into_inner()
                .status(),
            StatusCode::OK
        );
    }
}
