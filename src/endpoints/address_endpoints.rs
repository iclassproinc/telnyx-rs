use crate::{client::TelnyxClient, error::TelnyxError, models::{Address, ApiListResponse, ApiResponse, CreateAddressRequest, ValidateAddressRequest, ValidateAddressResult, AddressAcceptSuggestionRequest, AddressAcceptSuggestionResult}};

/// API client for addresses
pub struct AddressApi<'a> {
    client: &'a TelnyxClient
}

impl<'a> AddressApi<'a> {
    pub(crate) fn new(client: &'a TelnyxClient) -> Self {
        Self { client }
    }

    /// List all addresses
    ///
    /// # Arguments
    ///
    /// * `params` - Optional pagination parameters
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use telnyx_rs::{TelnyxClient, models::ListAddressesParams};
    /// # async fn example(client: &TelnyxClient) -> Result<(), telnyx_rs::TelnyxError> {
    /// // List with defaults
    /// let addresses = client.addresses().list(None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self) -> Result<ApiListResponse<Address>, TelnyxError> {
        self.client.get("/addresses").await
    }

    /// Get an address by ID
    ///
    /// # Arguments
    ///
    /// * `id` - The address ID
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use telnyx_rs::TelnyxClient;
    /// # async fn example(client: &TelnyxClient) -> Result<(), telnyx_rs::TelnyxError> {
    /// let address = client.addresses().get("1234567890").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, id: &str) -> Result<ApiResponse<Address>, TelnyxError> {
        self.client.get(&format!(".address/{}", id)).await
    }

    // Create a new address
    ///
    /// # Arguments
    ///
    /// * `request` - The address creation request
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use telnyx_rs::{TelnyxClient, models::CreateAddressRequest};
    /// # async fn example(client: &TelnyxClient) -> Result<(), telnyx_rs::TelnyxError> {
    /// let request = CreateAddressRequest::builder()
    ///     .street_address("311 W Superior St")
    ///     .locality("Chicago")
    ///     .country_code("US")
    ///     .administrative_area("IL")
    ///     .postal_code("60654")
    ///     .first_name("John")
    ///     .last_name("Doe")
    ///     .build();
    ///
    /// let address = client.addresses().create(request).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(&self, request: CreateAddressRequest) -> Result<ApiResponse<Address>, TelnyxError> {
        self.client.post("/address", &request).await
    }

    /// Delete an address
    ///
    /// # Arguments
    ///
    /// * `id` - The address ID to delete
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use telnyx_rs::TelnyxClient;
    /// # async fn example(client: &TelnyxClient) -> Result<(), telnyx_rs::TelnyxError> {
    /// client.addresses().delete("1234567890").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, id: &str) -> Result<(), TelnyxError> {
        self.client.delete(&format!(".address/{}", id)).await
    }

    /// Validate an address for emergency services
    ///
    /// # Arguments
    ///
    /// * `request` - The address validation request
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use telnyx_rs::{TelnyxClient, models::ValidateAddressRequest};
    /// # async fn example(client: &TelnyxClient) -> Result<(), telnyx_rs::TelnyxError> {
    /// let request = ValidateAddressRequest::builder()
    ///     .street_address("311 W Superior St")
    ///     .postal_code("60654")
    ///     .country_code("US")
    ///     .build();
    ///
    /// let result = client.addresses().validate(request).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn validate(&self, request: ValidateAddressRequest) -> Result<ApiResponse<ValidateAddressResult>, TelnyxError> {
        self.client.post("/addresses/actions/validate", &request).await
    }

    /// Accept address suggestions for an address
    ///
    /// # Arguments
    ///
    /// * `id` - The address ID
    /// * `request` - The accept suggestion request
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use telnyx_rs::{TelnyxClient, models::AddressAcceptSuggestionRequest};
    /// # async fn example(client: &TelnyxClient) -> Result<(), telnyx_rs::TelnyxError> {
    /// let request = AddressAcceptSuggestionRequest::default();
    /// let result = client.addresses().accept_suggestions("addr_123", request).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn accept_suggestions(&self, id: &str, request: AddressAcceptSuggestionRequest) -> Result<ApiResponse<AddressAcceptSuggestionResult>, TelnyxError> {
        self.client.post(&format!("/addresses/{}/actions/accept_suggestions", id), &request).await
    }
}