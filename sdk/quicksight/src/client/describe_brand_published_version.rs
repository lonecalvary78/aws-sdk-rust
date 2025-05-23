// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeBrandPublishedVersion`](crate::operation::describe_brand_published_version::builders::DescribeBrandPublishedVersionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`aws_account_id(impl Into<String>)`](crate::operation::describe_brand_published_version::builders::DescribeBrandPublishedVersionFluentBuilder::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::operation::describe_brand_published_version::builders::DescribeBrandPublishedVersionFluentBuilder::set_aws_account_id):<br>required: **true**<br><p>The ID of the Amazon Web Services account that owns the brand.</p><br>
    ///   - [`brand_id(impl Into<String>)`](crate::operation::describe_brand_published_version::builders::DescribeBrandPublishedVersionFluentBuilder::brand_id) / [`set_brand_id(Option<String>)`](crate::operation::describe_brand_published_version::builders::DescribeBrandPublishedVersionFluentBuilder::set_brand_id):<br>required: **true**<br><p>The ID of the Amazon QuickSight brand.</p><br>
    /// - On success, responds with [`DescribeBrandPublishedVersionOutput`](crate::operation::describe_brand_published_version::DescribeBrandPublishedVersionOutput) with field(s):
    ///   - [`request_id(Option<String>)`](crate::operation::describe_brand_published_version::DescribeBrandPublishedVersionOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
    ///   - [`brand_detail(Option<BrandDetail>)`](crate::operation::describe_brand_published_version::DescribeBrandPublishedVersionOutput::brand_detail): <p>The details of the brand.</p>
    ///   - [`brand_definition(Option<BrandDefinition>)`](crate::operation::describe_brand_published_version::DescribeBrandPublishedVersionOutput::brand_definition): <p>The definition of the brand.</p>
    /// - On failure, responds with [`SdkError<DescribeBrandPublishedVersionError>`](crate::operation::describe_brand_published_version::DescribeBrandPublishedVersionError)
    pub fn describe_brand_published_version(
        &self,
    ) -> crate::operation::describe_brand_published_version::builders::DescribeBrandPublishedVersionFluentBuilder {
        crate::operation::describe_brand_published_version::builders::DescribeBrandPublishedVersionFluentBuilder::new(self.handle.clone())
    }
}
