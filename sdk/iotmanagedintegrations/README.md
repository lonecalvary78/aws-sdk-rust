# aws-sdk-iotmanagedintegrations

Managed integrations is a feature of AWS IoT Device Management that enables developers to quickly build innovative IoT solutions. Customers can use managed integrations to automate device setup workflows and support interoperability across many devices, regardless of device vendor or connectivity protocol. This allows developers to use a single user-interface to control, manage, and operate a range of devices.

## Getting Started

> Examples are available for many services and operations, check out the
> [examples folder in GitHub](https://github.com/awslabs/aws-sdk-rust/tree/main/examples).

The SDK provides one crate per AWS service. You must add [Tokio](https://crates.io/crates/tokio)
as a dependency within your Rust project to execute asynchronous code. To add `aws-sdk-iotmanagedintegrations` to
your project, add the following to your **Cargo.toml** file:

```toml
[dependencies]
aws-config = { version = "1.1.7", features = ["behavior-version-latest"] }
aws-sdk-iotmanagedintegrations = "1.18.0"
tokio = { version = "1", features = ["full"] }
```

Then in code, a client can be created with the following:

```rust,no_run
use aws_sdk_iotmanagedintegrations as iotmanagedintegrations;

#[::tokio::main]
async fn main() -> Result<(), iotmanagedintegrations::Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_iotmanagedintegrations::Client::new(&config);

    // ... make some calls with the client

    Ok(())
}
```

See the [client documentation](https://docs.rs/aws-sdk-iotmanagedintegrations/latest/aws_sdk_iotmanagedintegrations/client/struct.Client.html)
for information on what calls can be made, and the inputs and outputs for each of those calls.

## Using the SDK

Until the SDK is released, we will be adding information about using the SDK to the
[Developer Guide](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/welcome.html). Feel free to suggest
additional sections for the guide by opening an issue and describing what you are trying to do.

## Getting Help

* [GitHub discussions](https://github.com/awslabs/aws-sdk-rust/discussions) - For ideas, RFCs & general questions
* [GitHub issues](https://github.com/awslabs/aws-sdk-rust/issues/new/choose) - For bug reports & feature requests
* [Generated Docs (latest version)](https://awslabs.github.io/aws-sdk-rust/)
* [Usage examples](https://github.com/awslabs/aws-sdk-rust/tree/main/examples)

## License

This project is licensed under the Apache-2.0 License.

