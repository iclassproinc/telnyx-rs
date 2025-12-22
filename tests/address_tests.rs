mod common;

use telnyx_rs::models::{
    AddressAcceptSuggestionRequest, AddressValidationStatus, CreateAddressRequest,
    ValidateAddressRequest,
};
use wiremock::{
    Mock, ResponseTemplate,
    matchers::{bearer_token, body_json, method, path},
};

mod responses {
    use chrono::Utc;

    pub fn address_response(id: i64) -> serde_json::Value {
        serde_json::json!({
            "data": {
                "id": id,
                "record_type": "address",
                "customer_reference": null,
                "first_name": "John",
                "last_name": "Doe",
                "business_name": null,
                "phone_number": null,
                "street_address": "311 W Superior St",
                "extended_address": null,
                "locality": "Chicago",
                "administrative_area": "IL",
                "neighborhood": null,
                "borough": null,
                "postal_code": "60654",
                "country_code": "US",
                "address_book": false,
                "validate_address": false,
                "created_at": Utc::now().to_rfc3339(),
                "updated_at": Utc::now().to_rfc3339()
            }
        })
    }

    fn address_data(id: i64) -> serde_json::Value {
        address_response(id)["data"].clone()
    }

    pub fn address_list_response(ids: &[i64]) -> serde_json::Value {
        let addresses: Vec<serde_json::Value> = ids.iter().map(|id| address_data(*id)).collect();

        serde_json::json!({
            "data": addresses,
            "meta": {
                "total_pages": 1,
                "total_results": ids.len(),
                "page_number": 1,
                "page_size": 25
            }
        })
    }

    pub fn validation_response(valid: bool) -> serde_json::Value {
        serde_json::json!({
            "data": {
                "record_type": "address_validation",
                "result": if valid { "valid" } else { "invalid" },
                "suggested": {
                    "street_address": "311 W SUPERIOR ST",
                    "locality": "CHICAGO",
                    "administrative_area": "IL",
                    "postal_code": "60654-3554",
                    "country_code": "US"
                },
                "errors": []
            }
        })
    }

    pub fn accept_suggestion_response(id: &str) -> serde_json::Value {
        serde_json::json!({
            "data": {
                "accepted": true,
                "id": id
            }
        })
    }
}

#[tokio::test]
async fn create_address_sucess() {
    // Arrange
    let context = common::setup().await;

    let request = CreateAddressRequest::builder()
        .street_address("311 W Superior St".to_string())
        .locality("Chicago".to_string())
        .country_code("US".to_string())
        .administrative_area("IL".to_string())
        .postal_code("60654".to_string())
        .first_name("John".to_string())
        .last_name("Doe".to_string())
        .build();

    let expected_response = responses::address_response(123456);

    Mock::given(method("POST"))
        .and(path("/address"))
        .and(bearer_token("test-api-key"))
        .and(body_json(&request))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
        .expect(1)
        .mount(&context.server)
        .await;

    // Act
    let result = context.client.addresses().create(request).await;

    // Assert
    assert!(result.is_ok());
    let address = result.unwrap().data;
    assert_eq!(address.id, 123456);
    assert_eq!(address.street_address, "311 W Superior St");
    assert_eq!(address.locality, "Chicago");
    assert_eq!(address.administrative_area, Some("IL".to_string()));
    assert_eq!(address.postal_code, Some("60654".to_string()));
    assert_eq!(address.country_code, "US");
    assert_eq!(address.first_name, Some("John".to_string()));
    assert_eq!(address.last_name, Some("Doe".to_string()));
}

#[tokio::test]
async fn create_address_unauthorized() {
    // Arrange
    let ctx = common::setup().await;

    Mock::given(method("POST"))
        .and(path("/address"))
        .respond_with(ResponseTemplate::new(401))
        .expect(1)
        .mount(&ctx.server)
        .await;

    let request = CreateAddressRequest::builder()
        .street_address("311 W Superior St".to_string())
        .locality("Chicago".to_string())
        .country_code("US".to_string())
        .build();

    // Act
    let result = ctx.client.addresses().create(request).await;

    // Assert
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        telnyx_rs::TelnyxError::Api { status: 401, .. }
    ));
}

#[tokio::test]
async fn create_address_unprocessable() {
    // Arrange
    let ctx = common::setup().await;

    Mock::given(method("POST"))
        .and(path("/address"))
        .respond_with(ResponseTemplate::new(422))
        .expect(1)
        .mount(&ctx.server)
        .await;

    let request = CreateAddressRequest::builder()
        .street_address("Invalid".to_string())
        .locality("Nowhere".to_string())
        .country_code("XX".to_string())
        .build();

    // Act
    let result = ctx.client.addresses().create(request).await;

    // Assert
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        telnyx_rs::TelnyxError::Api { status: 422, .. }
    ));
}

#[tokio::test]
async fn get_address_unauthorized() {
    // Arrange
    let ctx = common::setup().await;

    Mock::given(method("GET"))
        .and(path(".address/123"))
        .respond_with(ResponseTemplate::new(401))
        .expect(1)
        .mount(&ctx.server)
        .await;

    // Act
    let result = ctx.client.addresses().get("123").await;

    // Assert
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        telnyx_rs::TelnyxError::Api { status: 401, .. }
    ));
}

#[tokio::test]
async fn get_address_not_found() {
    // Arrange
    let ctx = common::setup().await;

    Mock::given(method("GET"))
        .and(path(".address/nonexistent"))
        .respond_with(ResponseTemplate::new(404))
        .expect(1)
        .mount(&ctx.server)
        .await;

    // Act
    let result = ctx.client.addresses().get("nonexistent").await;


    // Assert
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        telnyx_rs::TelnyxError::Api { status: 404, .. }
    ));
}

#[tokio::test]
async fn list_addresses_success() {
    // Arrange
    let ctx = common::setup().await;

    let expected_response = responses::address_list_response(&[123, 456]);

    Mock::given(method("GET"))
        .and(path("/addresses"))
        .and(bearer_token("test-api-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
        .expect(1)
        .mount(&ctx.server)
        .await;

    // Act
    let result = ctx.client.addresses().list().await;

    // Assert
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.data.len(), 2);
    assert_eq!(response.data[0].id, 123);
    assert_eq!(response.data[1].id, 456);
    assert!(response.meta.is_some());
    let meta = response.meta.unwrap();
    assert_eq!(meta.total_results, 2);
    assert_eq!(meta.page_number, 1);
}

#[tokio::test]
async fn list_addresses_empty() {
    // Arrange
    let ctx = common::setup().await;

    let expected_response = responses::address_list_response(&[]);

    Mock::given(method("GET"))
        .and(path("/addresses"))
        .and(bearer_token("test-api-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
        .expect(1)
        .mount(&ctx.server)
        .await;

    // Act
    let result = ctx.client.addresses().list().await;

    // Assert
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response.data.is_empty());
    assert!(response.meta.is_some());
    let meta = response.meta.unwrap();
    assert_eq!(meta.total_results, 0);
}

#[tokio::test]
async fn list_addresses_unauthorized() {
    // Arrange
    let ctx = common::setup().await;

    Mock::given(method("GET"))
        .and(path("/addresses"))
        .respond_with(ResponseTemplate::new(401))
        .expect(1)
        .mount(&ctx.server)
        .await;

    // Act
    let result = ctx.client.addresses().list().await;

    // Assert
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        telnyx_rs::TelnyxError::Api { status: 401, .. }
    ));
}

#[tokio::test]
async fn delete_address_success() {
    // Arrange
    let ctx = common::setup().await;

    Mock::given(method("DELETE"))
        .and(path(".address/123"))
        .and(bearer_token("test-api-key"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&ctx.server)
        .await;

    // Act
    let result = ctx.client.addresses().delete("123").await;

    // Assert
    assert!(result.is_ok());
}

#[tokio::test]
async fn delete_address_unauthorized() {
    // Arrange
    let ctx = common::setup().await;

    Mock::given(method("DELETE"))
        .and(path(".address/123"))
        .respond_with(ResponseTemplate::new(401))
        .expect(1)
        .mount(&ctx.server)
        .await;

    // Act
    let result = ctx.client.addresses().delete("123").await;

    // Assert
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        telnyx_rs::TelnyxError::Api { status: 401, .. }
    ));
}

#[tokio::test]
async fn delete_address_not_found() {
    // Arrange
    let ctx = common::setup().await;

    Mock::given(method("DELETE"))
        .and(path(".address/nonexistent"))
        .respond_with(ResponseTemplate::new(404))
        .expect(1)
        .mount(&ctx.server)
        .await;

    // Act
    let result = ctx.client.addresses().delete("nonexistent").await;

    // Assert
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        telnyx_rs::TelnyxError::Api { status: 404, .. }
    ));
}

#[tokio::test]
async fn validate_address_valid() {
    // Arrange
    let ctx = common::setup().await;

    let request = ValidateAddressRequest::builder()
        .street_address("311 W Superior St".to_string())
        .postal_code("60654".to_string())
        .country_code("US".to_string())
        .build();

    let expected_response = responses::validation_response(true);

    Mock::given(method("POST"))
        .and(path("/addresses/actions/validate"))
        .and(bearer_token("test-api-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
        .expect(1)
        .mount(&ctx.server)
        .await;

    // Act
    let result = ctx.client.addresses().validate(request).await;

    // Assert
    assert!(result.is_ok());
    let validation = result.unwrap().data;
    assert_eq!(validation.result, AddressValidationStatus::Valid);
    assert_eq!(
        validation.suggested.street_address,
        Some("311 W SUPERIOR ST".to_string())
    );
}

#[tokio::test]
async fn validate_address_invalid() {
    // Arrange
    let ctx = common::setup().await;

    let request = ValidateAddressRequest::builder()
        .street_address("123 Fake St".to_string())
        .postal_code("00000".to_string())
        .country_code("US".to_string())
        .build();

    let expected_response = responses::validation_response(false);

    Mock::given(method("POST"))
        .and(path("/addresses/actions/validate"))
        .and(bearer_token("test-api-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
        .expect(1)
        .mount(&ctx.server)
        .await;

    // Act
    let result = ctx.client.addresses().validate(request).await;

    // Assert
    assert!(result.is_ok());
    let validation = result.unwrap().data;
    assert_eq!(validation.result, AddressValidationStatus::Invalid);
}

#[tokio::test]
async fn validate_address_unauthorized() {
    // Arrange
    let ctx = common::setup().await;

    Mock::given(method("POST"))
        .and(path("/addresses/actions/validate"))
        .respond_with(ResponseTemplate::new(401))
        .expect(1)
        .mount(&ctx.server)
        .await;

    let request = ValidateAddressRequest::builder()
        .street_address("311 W Superior St".to_string())
        .postal_code("60654".to_string())
        .country_code("US".to_string())
        .build();

    // Act
    let result = ctx.client.addresses().validate(request).await;

    // Assert
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        telnyx_rs::TelnyxError::Api { status: 401, .. }
    ));
}

#[tokio::test]
async fn validate_address_unprocessable() {
    // Arrange
    let ctx = common::setup().await;

    Mock::given(method("POST"))
        .and(path("/addresses/actions/validate"))
        .respond_with(ResponseTemplate::new(422))
        .expect(1)
        .mount(&ctx.server)
        .await;

    let request = ValidateAddressRequest::builder()
        .street_address("Invalid".to_string())
        .postal_code("00000".to_string())
        .country_code("XX".to_string())
        .build();

    // Act
    let result = ctx.client.addresses().validate(request).await;

    // Assert
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        telnyx_rs::TelnyxError::Api { status: 422, .. }
    ));
}

#[tokio::test]
async fn accept_suggestions_success() {
    // Arrange
    let ctx = common::setup().await;

    let expected_response = responses::accept_suggestion_response("addr_123");

    Mock::given(method("POST"))
        .and(path("/addresses/123/actions/accept_suggestions"))
        .and(bearer_token("test-api-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&expected_response))
        .expect(1)
        .mount(&ctx.server)
        .await;

    // Act
    let request = AddressAcceptSuggestionRequest::default();
    let result = ctx.client.addresses().accept_suggestions("123", request).await;

    // Assert
    assert!(result.is_ok());
    let response = result.unwrap().data;
    assert!(response.accepted);
    assert_eq!(response.id, Some("addr_123".to_string()));
}

#[tokio::test]
async fn accept_suggestions_unauthorized() {
    // Arrange
    let ctx = common::setup().await;

    Mock::given(method("POST"))
        .and(path("/addresses/123/actions/accept_suggestions"))
        .respond_with(ResponseTemplate::new(401))
        .expect(1)
        .mount(&ctx.server)
        .await;

    // Act
    let request = AddressAcceptSuggestionRequest::default();
    let result = ctx.client.addresses().accept_suggestions("123", request).await;

    // Assert
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        telnyx_rs::TelnyxError::Api { status: 401, .. }
    ));
}

#[tokio::test]
async fn accept_suggestions_not_found() {
    // Arrange
    let ctx = common::setup().await;

    Mock::given(method("POST"))
        .and(path("/addresses/nonexistent/actions/accept_suggestions"))
        .respond_with(ResponseTemplate::new(404))
        .expect(1)
        .mount(&ctx.server)
        .await;

    // Act
    let request = AddressAcceptSuggestionRequest::default();
    let result = ctx.client.addresses().accept_suggestions("nonexistent", request).await;

    // Assert
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        telnyx_rs::TelnyxError::Api { status: 404, .. }
    ));
}



#[tokio::test]
async fn accept_suggestions_unprocessable() {
    // Arrange
    let ctx = common::setup().await;

    Mock::given(method("POST"))
        .and(path("/addresses/123/actions/accept_suggestions"))
        .respond_with(ResponseTemplate::new(422))
        .expect(1)
        .mount(&ctx.server)
        .await;

    // Act
    let request = AddressAcceptSuggestionRequest::default();
    let result = ctx.client.addresses().accept_suggestions("123", request).await;

    // Assert
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        telnyx_rs::TelnyxError::Api { status: 422, .. }
    ));
}
