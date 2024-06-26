// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ModifyTrustStore`](crate::operation::modify_trust_store::builders::ModifyTrustStoreFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`trust_store_arn(impl Into<String>)`](crate::operation::modify_trust_store::builders::ModifyTrustStoreFluentBuilder::trust_store_arn) / [`set_trust_store_arn(Option<String>)`](crate::operation::modify_trust_store::builders::ModifyTrustStoreFluentBuilder::set_trust_store_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the trust store.</p><br>
    ///   - [`ca_certificates_bundle_s3_bucket(impl Into<String>)`](crate::operation::modify_trust_store::builders::ModifyTrustStoreFluentBuilder::ca_certificates_bundle_s3_bucket) / [`set_ca_certificates_bundle_s3_bucket(Option<String>)`](crate::operation::modify_trust_store::builders::ModifyTrustStoreFluentBuilder::set_ca_certificates_bundle_s3_bucket):<br>required: **true**<br><p>The Amazon S3 bucket for the ca certificates bundle.</p><br>
    ///   - [`ca_certificates_bundle_s3_key(impl Into<String>)`](crate::operation::modify_trust_store::builders::ModifyTrustStoreFluentBuilder::ca_certificates_bundle_s3_key) / [`set_ca_certificates_bundle_s3_key(Option<String>)`](crate::operation::modify_trust_store::builders::ModifyTrustStoreFluentBuilder::set_ca_certificates_bundle_s3_key):<br>required: **true**<br><p>The Amazon S3 path for the ca certificates bundle.</p><br>
    ///   - [`ca_certificates_bundle_s3_object_version(impl Into<String>)`](crate::operation::modify_trust_store::builders::ModifyTrustStoreFluentBuilder::ca_certificates_bundle_s3_object_version) / [`set_ca_certificates_bundle_s3_object_version(Option<String>)`](crate::operation::modify_trust_store::builders::ModifyTrustStoreFluentBuilder::set_ca_certificates_bundle_s3_object_version):<br>required: **false**<br><p>The Amazon S3 object version for the ca certificates bundle. If undefined the current version is used.</p><br>
    /// - On success, responds with [`ModifyTrustStoreOutput`](crate::operation::modify_trust_store::ModifyTrustStoreOutput) with field(s):
    ///   - [`trust_stores(Option<Vec::<TrustStore>>)`](crate::operation::modify_trust_store::ModifyTrustStoreOutput::trust_stores): <p>Information about the modified trust store.</p>
    /// - On failure, responds with [`SdkError<ModifyTrustStoreError>`](crate::operation::modify_trust_store::ModifyTrustStoreError)
    pub fn modify_trust_store(&self) -> crate::operation::modify_trust_store::builders::ModifyTrustStoreFluentBuilder {
        crate::operation::modify_trust_store::builders::ModifyTrustStoreFluentBuilder::new(self.handle.clone())
    }
}
