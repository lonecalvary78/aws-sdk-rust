// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::sign::_sign_output::SignOutputBuilder;

pub use crate::operation::sign::_sign_input::SignInputBuilder;

impl crate::operation::sign::builders::SignInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::sign::SignOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::sign::SignError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.sign();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `Sign`.
///
/// <p>Creates a <a href="https://en.wikipedia.org/wiki/Digital_signature">digital signature</a> for a message or message digest by using the private key in an asymmetric signing KMS key. To verify the signature, use the <code>Verify</code> operation, or use the public key in the same asymmetric KMS key outside of KMS. For information about asymmetric KMS keys, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symmetric-asymmetric.html">Asymmetric KMS keys</a> in the <i>Key Management Service Developer Guide</i>.</p>
/// <p>Digital signatures are generated and verified by using asymmetric key pair, such as an RSA, ECC, or ML-DSA pair that is represented by an asymmetric KMS key. The key owner (or an authorized user) uses their private key to sign a message. Anyone with the public key can verify that the message was signed with that particular private key and that the message hasn't changed since it was signed.</p>
/// <p>To use the <code>Sign</code> operation, provide the following information:</p>
/// <ul>
/// <li>
/// <p>Use the <code>KeyId</code> parameter to identify an asymmetric KMS key with a <code>KeyUsage</code> value of <code>SIGN_VERIFY</code>. To get the <code>KeyUsage</code> value of a KMS key, use the <code>DescribeKey</code> operation. The caller must have <code>kms:Sign</code> permission on the KMS key.</p></li>
/// <li>
/// <p>Use the <code>Message</code> parameter to specify the message or message digest to sign. You can submit messages of up to 4096 bytes. To sign a larger message, generate a hash digest of the message, and then provide the hash digest in the <code>Message</code> parameter. To indicate whether the message is a full message, a digest, or an ML-DSA EXTERNAL_MU, use the <code>MessageType</code> parameter.</p></li>
/// <li>
/// <p>Choose a signing algorithm that is compatible with the KMS key.</p></li>
/// </ul><important>
/// <p>When signing a message, be sure to record the KMS key and the signing algorithm. This information is required to verify the signature.</p>
/// </important> <note>
/// <p>Best practices recommend that you limit the time during which any signature is effective. This deters an attack where the actor uses a signed message to establish validity repeatedly or long after the message is superseded. Signatures do not include a timestamp, but you can include a timestamp in the signed message to help you detect when its time to refresh the signature.</p>
/// </note>
/// <p>To verify the signature that this operation generates, use the <code>Verify</code> operation. Or use the <code>GetPublicKey</code> operation to download the public key and then use the public key to verify the signature outside of KMS.</p>
/// <p>The KMS key that you use for this operation must be in a compatible key state. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key states of KMS keys</a> in the <i>Key Management Service Developer Guide</i>.</p>
/// <p><b>Cross-account use</b>: Yes. To perform this operation with a KMS key in a different Amazon Web Services account, specify the key ARN or alias ARN in the value of the <code>KeyId</code> parameter.</p>
/// <p><b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:Sign</a> (key policy)</p>
/// <p><b>Related operations</b>: <code>Verify</code></p>
/// <p><b>Eventual consistency</b>: The KMS API follows an eventual consistency model. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/accessing-kms.html#programming-eventual-consistency">KMS eventual consistency</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct SignFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::sign::builders::SignInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl crate::client::customize::internal::CustomizableSend<crate::operation::sign::SignOutput, crate::operation::sign::SignError>
    for SignFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<crate::operation::sign::SignOutput, crate::operation::sign::SignError>,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl SignFluentBuilder {
    /// Creates a new `SignFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the Sign as a reference.
    pub fn as_input(&self) -> &crate::operation::sign::builders::SignInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::sign::SignOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::sign::SignError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::sign::Sign::operation_runtime_plugins(self.handle.runtime_plugins.clone(), &self.handle.conf, self.config_override);
        crate::operation::sign::Sign::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<crate::operation::sign::SignOutput, crate::operation::sign::SignError, Self> {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
        self.set_config_override(::std::option::Option::Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>Identifies an asymmetric KMS key. KMS uses the private key in the asymmetric KMS key to sign the message. The <code>KeyUsage</code> type of the KMS key must be <code>SIGN_VERIFY</code>. To find the <code>KeyUsage</code> of a KMS key, use the <code>DescribeKey</code> operation.</p>
    /// <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN or alias ARN.</p>
    /// <p>For example:</p>
    /// <ul>
    /// <li>
    /// <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>
    /// <li>
    /// <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>
    /// <li>
    /// <p>Alias name: <code>alias/ExampleAlias</code></p></li>
    /// <li>
    /// <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code></p></li>
    /// </ul>
    /// <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>. To get the alias name and alias ARN, use <code>ListAliases</code>.</p>
    pub fn key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.key_id(input.into());
        self
    }
    /// <p>Identifies an asymmetric KMS key. KMS uses the private key in the asymmetric KMS key to sign the message. The <code>KeyUsage</code> type of the KMS key must be <code>SIGN_VERIFY</code>. To find the <code>KeyUsage</code> of a KMS key, use the <code>DescribeKey</code> operation.</p>
    /// <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN or alias ARN.</p>
    /// <p>For example:</p>
    /// <ul>
    /// <li>
    /// <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>
    /// <li>
    /// <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>
    /// <li>
    /// <p>Alias name: <code>alias/ExampleAlias</code></p></li>
    /// <li>
    /// <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code></p></li>
    /// </ul>
    /// <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>. To get the alias name and alias ARN, use <code>ListAliases</code>.</p>
    pub fn set_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_key_id(input);
        self
    }
    /// <p>Identifies an asymmetric KMS key. KMS uses the private key in the asymmetric KMS key to sign the message. The <code>KeyUsage</code> type of the KMS key must be <code>SIGN_VERIFY</code>. To find the <code>KeyUsage</code> of a KMS key, use the <code>DescribeKey</code> operation.</p>
    /// <p>To specify a KMS key, use its key ID, key ARN, alias name, or alias ARN. When using an alias name, prefix it with <code>"alias/"</code>. To specify a KMS key in a different Amazon Web Services account, you must use the key ARN or alias ARN.</p>
    /// <p>For example:</p>
    /// <ul>
    /// <li>
    /// <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>
    /// <li>
    /// <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>
    /// <li>
    /// <p>Alias name: <code>alias/ExampleAlias</code></p></li>
    /// <li>
    /// <p>Alias ARN: <code>arn:aws:kms:us-east-2:111122223333:alias/ExampleAlias</code></p></li>
    /// </ul>
    /// <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>. To get the alias name and alias ARN, use <code>ListAliases</code>.</p>
    pub fn get_key_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_key_id()
    }
    /// <p>Specifies the message or message digest to sign. Messages can be 0-4096 bytes. To sign a larger message, provide a message digest.</p>
    /// <p>If you provide a message digest, use the <code>DIGEST</code> value of <code>MessageType</code> to prevent the digest from being hashed again while signing.</p>
    pub fn message(mut self, input: ::aws_smithy_types::Blob) -> Self {
        self.inner = self.inner.message(input);
        self
    }
    /// <p>Specifies the message or message digest to sign. Messages can be 0-4096 bytes. To sign a larger message, provide a message digest.</p>
    /// <p>If you provide a message digest, use the <code>DIGEST</code> value of <code>MessageType</code> to prevent the digest from being hashed again while signing.</p>
    pub fn set_message(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.inner = self.inner.set_message(input);
        self
    }
    /// <p>Specifies the message or message digest to sign. Messages can be 0-4096 bytes. To sign a larger message, provide a message digest.</p>
    /// <p>If you provide a message digest, use the <code>DIGEST</code> value of <code>MessageType</code> to prevent the digest from being hashed again while signing.</p>
    pub fn get_message(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        self.inner.get_message()
    }
    /// <p>Tells KMS whether the value of the <code>Message</code> parameter should be hashed as part of the signing algorithm. Use <code>RAW</code> for unhashed messages; use <code>DIGEST</code> for message digests, which are already hashed; use <code>EXTERNAL_MU</code> for 64-byte representative μ used in ML-DSA signing as defined in NIST FIPS 204 Section 6.2.</p>
    /// <p>When the value of <code>MessageType</code> is <code>RAW</code>, KMS uses the standard signing algorithm, which begins with a hash function. When the value is <code>DIGEST</code>, KMS skips the hashing step in the signing algorithm. When the value is <code>EXTERNAL_MU</code> KMS skips the concatenated hashing of the public key hash and the message done in the ML-DSA signing algorithm.</p><important>
    /// <p>Use the <code>DIGEST</code> or <code>EXTERNAL_MU</code> value only when the value of the <code>Message</code> parameter is a message digest. If you use the <code>DIGEST</code> value with an unhashed message, the security of the signing operation can be compromised.</p>
    /// </important>
    /// <p>When the value of <code>MessageType</code> is <code>DIGEST</code>, the length of the <code>Message</code> value must match the length of hashed messages for the specified signing algorithm.</p>
    /// <p>When the value of <code>MessageType</code> is <code>EXTERNAL_MU</code> the length of the <code>Message</code> value must be 64 bytes.</p>
    /// <p>You can submit a message digest and omit the <code>MessageType</code> or specify <code>RAW</code> so the digest is hashed again while signing. However, this can cause verification failures when verifying with a system that assumes a single hash.</p>
    /// <p>The hashing algorithm that <code>Sign</code> uses is based on the <code>SigningAlgorithm</code> value.</p>
    /// <ul>
    /// <li>
    /// <p>Signing algorithms that end in SHA_256 use the SHA_256 hashing algorithm.</p></li>
    /// <li>
    /// <p>Signing algorithms that end in SHA_384 use the SHA_384 hashing algorithm.</p></li>
    /// <li>
    /// <p>Signing algorithms that end in SHA_512 use the SHA_512 hashing algorithm.</p></li>
    /// <li>
    /// <p>Signing algorithms that end in SHAKE_256 use the SHAKE_256 hashing algorithm.</p></li>
    /// <li>
    /// <p>SM2DSA uses the SM3 hashing algorithm. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/offline-operations.html#key-spec-sm-offline-verification">Offline verification with SM2 key pairs</a>.</p></li>
    /// </ul>
    pub fn message_type(mut self, input: crate::types::MessageType) -> Self {
        self.inner = self.inner.message_type(input);
        self
    }
    /// <p>Tells KMS whether the value of the <code>Message</code> parameter should be hashed as part of the signing algorithm. Use <code>RAW</code> for unhashed messages; use <code>DIGEST</code> for message digests, which are already hashed; use <code>EXTERNAL_MU</code> for 64-byte representative μ used in ML-DSA signing as defined in NIST FIPS 204 Section 6.2.</p>
    /// <p>When the value of <code>MessageType</code> is <code>RAW</code>, KMS uses the standard signing algorithm, which begins with a hash function. When the value is <code>DIGEST</code>, KMS skips the hashing step in the signing algorithm. When the value is <code>EXTERNAL_MU</code> KMS skips the concatenated hashing of the public key hash and the message done in the ML-DSA signing algorithm.</p><important>
    /// <p>Use the <code>DIGEST</code> or <code>EXTERNAL_MU</code> value only when the value of the <code>Message</code> parameter is a message digest. If you use the <code>DIGEST</code> value with an unhashed message, the security of the signing operation can be compromised.</p>
    /// </important>
    /// <p>When the value of <code>MessageType</code> is <code>DIGEST</code>, the length of the <code>Message</code> value must match the length of hashed messages for the specified signing algorithm.</p>
    /// <p>When the value of <code>MessageType</code> is <code>EXTERNAL_MU</code> the length of the <code>Message</code> value must be 64 bytes.</p>
    /// <p>You can submit a message digest and omit the <code>MessageType</code> or specify <code>RAW</code> so the digest is hashed again while signing. However, this can cause verification failures when verifying with a system that assumes a single hash.</p>
    /// <p>The hashing algorithm that <code>Sign</code> uses is based on the <code>SigningAlgorithm</code> value.</p>
    /// <ul>
    /// <li>
    /// <p>Signing algorithms that end in SHA_256 use the SHA_256 hashing algorithm.</p></li>
    /// <li>
    /// <p>Signing algorithms that end in SHA_384 use the SHA_384 hashing algorithm.</p></li>
    /// <li>
    /// <p>Signing algorithms that end in SHA_512 use the SHA_512 hashing algorithm.</p></li>
    /// <li>
    /// <p>Signing algorithms that end in SHAKE_256 use the SHAKE_256 hashing algorithm.</p></li>
    /// <li>
    /// <p>SM2DSA uses the SM3 hashing algorithm. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/offline-operations.html#key-spec-sm-offline-verification">Offline verification with SM2 key pairs</a>.</p></li>
    /// </ul>
    pub fn set_message_type(mut self, input: ::std::option::Option<crate::types::MessageType>) -> Self {
        self.inner = self.inner.set_message_type(input);
        self
    }
    /// <p>Tells KMS whether the value of the <code>Message</code> parameter should be hashed as part of the signing algorithm. Use <code>RAW</code> for unhashed messages; use <code>DIGEST</code> for message digests, which are already hashed; use <code>EXTERNAL_MU</code> for 64-byte representative μ used in ML-DSA signing as defined in NIST FIPS 204 Section 6.2.</p>
    /// <p>When the value of <code>MessageType</code> is <code>RAW</code>, KMS uses the standard signing algorithm, which begins with a hash function. When the value is <code>DIGEST</code>, KMS skips the hashing step in the signing algorithm. When the value is <code>EXTERNAL_MU</code> KMS skips the concatenated hashing of the public key hash and the message done in the ML-DSA signing algorithm.</p><important>
    /// <p>Use the <code>DIGEST</code> or <code>EXTERNAL_MU</code> value only when the value of the <code>Message</code> parameter is a message digest. If you use the <code>DIGEST</code> value with an unhashed message, the security of the signing operation can be compromised.</p>
    /// </important>
    /// <p>When the value of <code>MessageType</code> is <code>DIGEST</code>, the length of the <code>Message</code> value must match the length of hashed messages for the specified signing algorithm.</p>
    /// <p>When the value of <code>MessageType</code> is <code>EXTERNAL_MU</code> the length of the <code>Message</code> value must be 64 bytes.</p>
    /// <p>You can submit a message digest and omit the <code>MessageType</code> or specify <code>RAW</code> so the digest is hashed again while signing. However, this can cause verification failures when verifying with a system that assumes a single hash.</p>
    /// <p>The hashing algorithm that <code>Sign</code> uses is based on the <code>SigningAlgorithm</code> value.</p>
    /// <ul>
    /// <li>
    /// <p>Signing algorithms that end in SHA_256 use the SHA_256 hashing algorithm.</p></li>
    /// <li>
    /// <p>Signing algorithms that end in SHA_384 use the SHA_384 hashing algorithm.</p></li>
    /// <li>
    /// <p>Signing algorithms that end in SHA_512 use the SHA_512 hashing algorithm.</p></li>
    /// <li>
    /// <p>Signing algorithms that end in SHAKE_256 use the SHAKE_256 hashing algorithm.</p></li>
    /// <li>
    /// <p>SM2DSA uses the SM3 hashing algorithm. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/offline-operations.html#key-spec-sm-offline-verification">Offline verification with SM2 key pairs</a>.</p></li>
    /// </ul>
    pub fn get_message_type(&self) -> &::std::option::Option<crate::types::MessageType> {
        self.inner.get_message_type()
    }
    ///
    /// Appends an item to `GrantTokens`.
    ///
    /// To override the contents of this collection use [`set_grant_tokens`](Self::set_grant_tokens).
    ///
    /// <p>A list of grant tokens.</p>
    /// <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/using-grant-token.html">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p>
    pub fn grant_tokens(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.grant_tokens(input.into());
        self
    }
    /// <p>A list of grant tokens.</p>
    /// <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/using-grant-token.html">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p>
    pub fn set_grant_tokens(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_grant_tokens(input);
        self
    }
    /// <p>A list of grant tokens.</p>
    /// <p>Use a grant token when your permission to call this operation comes from a new grant that has not yet achieved <i>eventual consistency</i>. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/grants.html#grant_token">Grant token</a> and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/using-grant-token.html">Using a grant token</a> in the <i>Key Management Service Developer Guide</i>.</p>
    pub fn get_grant_tokens(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_grant_tokens()
    }
    /// <p>Specifies the signing algorithm to use when signing the message.</p>
    /// <p>Choose an algorithm that is compatible with the type and size of the specified asymmetric KMS key. When signing with RSA key pairs, RSASSA-PSS algorithms are preferred. We include RSASSA-PKCS1-v1_5 algorithms for compatibility with existing applications.</p>
    pub fn signing_algorithm(mut self, input: crate::types::SigningAlgorithmSpec) -> Self {
        self.inner = self.inner.signing_algorithm(input);
        self
    }
    /// <p>Specifies the signing algorithm to use when signing the message.</p>
    /// <p>Choose an algorithm that is compatible with the type and size of the specified asymmetric KMS key. When signing with RSA key pairs, RSASSA-PSS algorithms are preferred. We include RSASSA-PKCS1-v1_5 algorithms for compatibility with existing applications.</p>
    pub fn set_signing_algorithm(mut self, input: ::std::option::Option<crate::types::SigningAlgorithmSpec>) -> Self {
        self.inner = self.inner.set_signing_algorithm(input);
        self
    }
    /// <p>Specifies the signing algorithm to use when signing the message.</p>
    /// <p>Choose an algorithm that is compatible with the type and size of the specified asymmetric KMS key. When signing with RSA key pairs, RSASSA-PSS algorithms are preferred. We include RSASSA-PKCS1-v1_5 algorithms for compatibility with existing applications.</p>
    pub fn get_signing_algorithm(&self) -> &::std::option::Option<crate::types::SigningAlgorithmSpec> {
        self.inner.get_signing_algorithm()
    }
    /// <p>Checks if your request will succeed. <code>DryRun</code> is an optional parameter.</p>
    /// <p>To learn more about how to use this parameter, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/testing-permissions.html">Testing your permissions</a> in the <i>Key Management Service Developer Guide</i>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks if your request will succeed. <code>DryRun</code> is an optional parameter.</p>
    /// <p>To learn more about how to use this parameter, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/testing-permissions.html">Testing your permissions</a> in the <i>Key Management Service Developer Guide</i>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Checks if your request will succeed. <code>DryRun</code> is an optional parameter.</p>
    /// <p>To learn more about how to use this parameter, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/testing-permissions.html">Testing your permissions</a> in the <i>Key Management Service Developer Guide</i>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
}
