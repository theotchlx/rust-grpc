use proto::admin_client::AdminClient;
use std::error::Error;

pub mod proto {
	tonic::include_proto!("calculator");
}

use tonic::{metadata::MetadataValue, transport::Channel, Request};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let token: MetadataValue<_> = "Bearer some-secret-token".parse()?;

	let channel = Channel::from_static("http://[::1]:50051").connect().await?;
	let mut client = AdminClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    });

	let req = proto::GetCountRequest {};
	let request = tonic::Request::new(req);

	let response = client.get_request_count(request).await?;

	println!("Response: {:?}", response.into_inner().count);

	Ok(())
}
