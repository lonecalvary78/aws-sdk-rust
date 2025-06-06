// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateBrandPublishedVersion`](crate::operation::update_brand_published_version::builders::UpdateBrandPublishedVersionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`aws_account_id(impl Into<String>)`](crate::operation::update_brand_published_version::builders::UpdateBrandPublishedVersionFluentBuilder::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::operation::update_brand_published_version::builders::UpdateBrandPublishedVersionFluentBuilder::set_aws_account_id):<br>required: **true**<br><p>The ID of the Amazon Web Services account that owns the brand.</p><br>
    ///   - [`brand_id(impl Into<String>)`](crate::operation::update_brand_published_version::builders::UpdateBrandPublishedVersionFluentBuilder::brand_id) / [`set_brand_id(Option<String>)`](crate::operation::update_brand_published_version::builders::UpdateBrandPublishedVersionFluentBuilder::set_brand_id):<br>required: **true**<br><p>The ID of the Amazon QuickSight brand.</p><br>
    ///   - [`version_id(impl Into<String>)`](crate::operation::update_brand_published_version::builders::UpdateBrandPublishedVersionFluentBuilder::version_id) / [`set_version_id(Option<String>)`](crate::operation::update_brand_published_version::builders::UpdateBrandPublishedVersionFluentBuilder::set_version_id):<br>required: **true**<br><p>The ID of the published version.</p><br>
    /// - On success, responds with [`UpdateBrandPublishedVersionOutput`](crate::operation::update_brand_published_version::UpdateBrandPublishedVersionOutput) with field(s):
    ///   - [`request_id(Option<String>)`](crate::operation::update_brand_published_version::UpdateBrandPublishedVersionOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
    ///   - [`version_id(Option<String>)`](crate::operation::update_brand_published_version::UpdateBrandPublishedVersionOutput::version_id): <p>The ID of the published version.</p>
    /// - On failure, responds with [`SdkError<UpdateBrandPublishedVersionError>`](crate::operation::update_brand_published_version::UpdateBrandPublishedVersionError)
    pub fn update_brand_published_version(
        &self,
    ) -> crate::operation::update_brand_published_version::builders::UpdateBrandPublishedVersionFluentBuilder {
        crate::operation::update_brand_published_version::builders::UpdateBrandPublishedVersionFluentBuilder::new(self.handle.clone())
    }
}
