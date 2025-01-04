use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;
use tonic::{codec::CompressionEncoding, Request};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = {
        let mut client = GreeterClient::connect("http://[::1]:50051")
            .await?
            .accept_compressed(CompressionEncoding::Zstd)
            .send_compressed(CompressionEncoding::Zstd);

        let request = Request::new(HelloRequest {
            name: "Tonic".into(),
        });

        client.say_hello(request).await?
    };

    println!("RESPONSE={:?}", response);

    Ok(())
}
