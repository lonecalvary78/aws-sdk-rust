# aws-sdk-qconnect

  - [Amazon Q actions](https://docs.aws.amazon.com/connect/latest/APIReference/API_Operations_Amazon_Q_Connect.html)
  - [Amazon Q data types](https://docs.aws.amazon.com/connect/latest/APIReference/API_Types_Amazon_Q_Connect.html)

Amazon Q in Connect is a generative AI customer service assistant. It is an LLM-enhanced evolution of Amazon Connect Wisdom that delivers real-time recommendations to help contact center agents resolve customer issues quickly and accurately.

Amazon Q in Connect automatically detects customer intent during calls and chats using conversational analytics and natural language understanding (NLU). It then provides agents with immediate, real-time generative responses and suggested actions, and links to relevant documents and articles. Agents can also query Amazon Q in Connect directly using natural language or keywords to answer customer requests.

Use the Amazon Q in Connect APIs to create an assistant and a knowledge base, for example, or manage content by uploading custom files.

For more information, see [Use Amazon Q in Connect for generative AI powered agent assistance in real-time](https://docs.aws.amazon.com/connect/latest/adminguide/amazon-q-connect.html) in the _Amazon Connect Administrator Guide_.

## Getting Started

> Examples are available for many services and operations, check out the
> [examples folder in GitHub](https://github.com/awslabs/aws-sdk-rust/tree/main/examples).

The SDK provides one crate per AWS service. You must add [Tokio](https://crates.io/crates/tokio)
as a dependency within your Rust project to execute asynchronous code. To add `aws-sdk-qconnect` to
your project, add the following to your **Cargo.toml** file:

```toml
[dependencies]
aws-config = { version = "1.1.7", features = ["behavior-version-latest"] }
aws-sdk-qconnect = "1.86.0"
tokio = { version = "1", features = ["full"] }
```

Then in code, a client can be created with the following:

```rust,no_run
use aws_sdk_qconnect as qconnect;

#[::tokio::main]
async fn main() -> Result<(), qconnect::Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_qconnect::Client::new(&config);

    // ... make some calls with the client

    Ok(())
}
```

See the [client documentation](https://docs.rs/aws-sdk-qconnect/latest/aws_sdk_qconnect/client/struct.Client.html)
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

