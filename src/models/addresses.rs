use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use bon::{Builder};

/// Address list and detail object
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Address {
    /// Uniquely identifies the address.
    pub id: i64,
    /// Idenftifies the type of the resource.
    pub record_type: String,
    /// A customer reference string for customer look ups.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer_reference: Option<String>,
    /// The first name associated with the address. An address must have either a first last name or a business name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// The last name associated with the address. An address must have either a first last name or a business name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// The business name associated with the address. An address must have either a first last name or a business name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_name: Option<String>,
    /// The phone number associated with the address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// The primary street address information about the address.
    pub street_address: String,
    /// Additional street address information about the address such as, but not limited to, unit number or apartment number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extended_address: Option<String>,
    /// The locality of the address. For US addresses, this corresponds to the city of the address.
    pub locality: String,
    /// The locality of the address. For US addresses, this corresponds to the state of the address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub administrative_area: Option<String>,
    /// The neighborhood of the address. This field is not used for addresses in the US but is used for some international addresses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub neighborhood: Option<String>,
    /// The borough of the address. This field is not used for addresses in the US but is used for some international addresses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub borough: Option<String>,
    /// The postal code of the address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// The two-character (ISO 3166-1 alpha-2) country code of the address.
    pub country_code: String,
    /// Indicates whether or not the address should be considered part of your list of addresses that appear for regular use.
    #[serde(default)]
    pub address_book: bool,
    /// Indicates whether or not the address should be validated for emergency use upon creation or not. This should be left with the default value of true unless you have used the /addresses/actions/validate endpoint to validate the address separately prior to creation. If an address is not validated for emergency use upon creation and it is not valid, it will not be able to be used for emergency services.
    #[serde(default)]
    pub validate_address: bool,
    /// ISO 8601 formatted date indicating when the resource was created.
    pub created_at: DateTime<Utc>,
    /// ISO 8601 formatted date indicating when the resource was updated.
    pub updated_at: DateTime<Utc>
}

/// A request to create a new address
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
pub struct CreateAddressRequest {
    /// A customer reference string for customer look ups.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer_reference: Option<String>,
    /// The first name associated with the address. An address must have either a first last name or a business name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// The last name associated with the address. An address must have either a first last name or a business name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// The business name associated with the address. An address must have either a first last name or a business name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_name: Option<String>,
    /// The phone number associated with the address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// The primary street address information about the address.
    pub street_address: String,
    /// Additional street address information about the address such as, but not limited to, unit number or apartment number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extended_address: Option<String>,
    /// The locality of the address. For US addresses, this corresponds to the city of the address.
    pub locality: String,
    /// The locality of the address. For US addresses, this corresponds to the state of the address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub administrative_area: Option<String>,
    /// The neighborhood of the address. This field is not used for addresses in the US but is used for some international addresses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub neighborhood: Option<String>,
    /// The borough of the address. This field is not used for addresses in the US but is used for some international addresses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub borough: Option<String>,
    /// The postal code of the address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// The two-character (ISO 3166-1 alpha-2) country code of the address.
    pub country_code: String,
    /// Indicates whether or not the address should be considered part of your list of addresses that appear for regular use.
    #[serde(default)]
    #[builder(default)]
    pub address_book: bool,
    /// Indicates whether or not the address should be validated for emergency use upon creation or not. This should be left with the default value of true unless you have used the /addresses/actions/validate endpoint to validate the address separately prior to creation. If an address is not validated for emergency use upon creation and it is not valid, it will not be able to be used for emergency services.
    #[serde(default)]
    #[builder(default)]
    pub validate_address: bool
}

/// A request to validate an address for emergecy services
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
pub struct ValidateAddressRequest {
    /// The primary street address information about the address.
    pub street_address: String,
    /// Additional street address information about the address such as, but not limited to, unit number or apartment number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extended_address: Option<String>,
    /// The locality of the address. For US addresses, this corresponds to the city of the address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    /// The locality of the address. For US addresses, this corresponds to the state of the address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub administrative_area: Option<String>,
    /// The postal code of the address.
    pub postal_code: String,
    /// The two-character (ISO 3166-1 alpha-2) country code of the address.
    pub country_code: String
}

/// The result of address validation
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ValidateAddressResult {
     /// Idenftifies the type of the resource.
     pub record_type: String,
     /// Indicates whether an address is valid or invalid.
     pub result: AddressValidationStatus,
     /// Provides normalized address when available.
     pub suggested: ValidateAddressField,
    /// Validation errors if any
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<AddressValidationError>
}

/// Normalized validated address field
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ValidateAddressField {
    /// The primary street address information about the address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_address: Option<String>,
    /// Additional street address information about the address such as, but not limited to, unit number or apartment number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_address: Option<String>,
    /// The locality of the address. For US addresses, this corresponds to the city of the address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    /// The locality of the address. For US addresses, this corresponds to the state of the address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrative_area: Option<String>,
    /// The postal code of the address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// The two-character (ISO 3166-1 alpha-2) country code of the address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
}

/// Address API error object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressValidationError {
    /// Error code
    pub code: String,
    /// Error title
    pub title: String,
    /// Detailed error description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// Source location of the error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ErrorSource>,
}

/// Source location of an API error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorSource {
    /// Indicates which query parameter caused the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter: Option<String>,
    /// JSON pointer (RFC6901) to the offending entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer: Option<String>,
}

/// Indicates whether an address is valid or invalid, with an unknown fallback
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum AddressValidationStatus {
    /// Address is valid
    Valid,
    /// Address is invalid
    Invalid,
    /// Fallback if neither value can be bound
    #[default]
    #[serde(other)]
    Unknown,
}

/// Request to accept this address suggestion as the new emergency address
#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
pub struct AddressAcceptSuggestionRequest {
    /// The ID of the address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AddressAcceptSuggestionResult {
    /// Indicates if the address suggestions are accepted.
    #[serde(default)]
    pub accepted: bool,
    /// The UUID of the location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>
}