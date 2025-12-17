use telnyx_rs::{TelnyxClient, models::CreateAddressRequest};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TelnyxClient::builder().api_key("EXAMPLE").build()?;

    let request = CreateAddressRequest::builder()
        .street_address("311 W Superior St".to_string())
        .locality("Chicago".to_string())
        .country_code("US".to_string())
        .administrative_area("IL".to_string())             // Optional
        .postal_code("60654".to_string())                  // Optional
        .build();

    let address_created = client.addresses().create(request).await?;
    println!("Address found: {}", address_created.data.id);

    let address_found = client.addresses().get(&address_created.data.id.to_string()).await?;
    println!("Address found: {}", address_found.data.id);

    Ok(())
}