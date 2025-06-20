// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_bucket::_create_bucket_output::CreateBucketOutputBuilder;

pub use crate::operation::create_bucket::_create_bucket_input::CreateBucketInputBuilder;

impl crate::operation::create_bucket::builders::CreateBucketInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_bucket::CreateBucketOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_bucket::CreateBucketError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_bucket();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateBucket`.
///
/// <important>
/// <p>End of support notice: Beginning October 1, 2025, Amazon S3 will discontinue support for creating new Email Grantee Access Control Lists (ACL). Email Grantee ACLs created prior to this date will continue to work and remain accessible through the Amazon Web Services Management Console, Command Line Interface (CLI), SDKs, and REST API. However, you will no longer be able to create new Email Grantee ACLs.</p>
/// <p>This change affects the following Amazon Web Services Regions: US East (N. Virginia) Region, US West (N. California) Region, US West (Oregon) Region, Asia Pacific (Singapore) Region, Asia Pacific (Sydney) Region, Asia Pacific (Tokyo) Region, Europe (Ireland) Region, and South America (São Paulo) Region.</p>
/// </important> <important>
/// <p>End of support notice: Beginning October 1, 2025, Amazon S3 will stop returning <code>DisplayName</code>. Update your applications to use canonical IDs (unique identifier for Amazon Web Services accounts), Amazon Web Services account ID (12 digit identifier) or IAM ARNs (full resource naming) as a direct replacement of <code>DisplayName</code>.</p>
/// <p>This change affects the following Amazon Web Services Regions: US East (N. Virginia) Region, US West (N. California) Region, US West (Oregon) Region, Asia Pacific (Singapore) Region, Asia Pacific (Sydney) Region, Asia Pacific (Tokyo) Region, Europe (Ireland) Region, and South America (São Paulo) Region.</p>
/// </important> <note>
/// <p>This action creates an Amazon S3 bucket. To create an Amazon S3 on Outposts bucket, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_control_CreateBucket.html"> <code>CreateBucket</code> </a>.</p>
/// </note>
/// <p>Creates a new S3 bucket. To create a bucket, you must set up Amazon S3 and have a valid Amazon Web Services Access Key ID to authenticate requests. Anonymous requests are never allowed to create buckets. By creating the bucket, you become the bucket owner.</p>
/// <p>There are two types of buckets: general purpose buckets and directory buckets. For more information about these bucket types, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/creating-buckets-s3.html">Creating, configuring, and working with Amazon S3 buckets</a> in the <i>Amazon S3 User Guide</i>.</p><note>
/// <ul>
/// <li>
/// <p><b>General purpose buckets</b> - If you send your <code>CreateBucket</code> request to the <code>s3.amazonaws.com</code> global endpoint, the request goes to the <code>us-east-1</code> Region. So the signature calculations in Signature Version 4 must use <code>us-east-1</code> as the Region, even if the location constraint in the request specifies another Region where the bucket is to be created. If you create a bucket in a Region other than US East (N. Virginia), your application must be able to handle 307 redirect. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/VirtualHosting.html">Virtual hosting of buckets</a> in the <i>Amazon S3 User Guide</i>.</p></li>
/// <li>
/// <p><b>Directory buckets </b> - For directory buckets, you must make requests for this API operation to the Regional endpoint. These endpoints support path-style requests in the format <code>https://s3express-control.<i>region-code</i>.amazonaws.com/<i>bucket-name</i> </code>. Virtual-hosted-style requests aren't supported. For more information about endpoints in Availability Zones, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/endpoint-directory-buckets-AZ.html">Regional and Zonal endpoints for directory buckets in Availability Zones</a> in the <i>Amazon S3 User Guide</i>. For more information about endpoints in Local Zones, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-lzs-for-directory-buckets.html">Concepts for directory buckets in Local Zones</a> in the <i>Amazon S3 User Guide</i>.</p></li>
/// </ul>
/// </note>
/// <dl>
/// <dt>
/// Permissions
/// </dt>
/// <dd>
/// <ul>
/// <li>
/// <p><b>General purpose bucket permissions</b> - In addition to the <code>s3:CreateBucket</code> permission, the following permissions are required in a policy when your <code>CreateBucket</code> request includes specific headers:</p>
/// <ul>
/// <li>
/// <p><b>Access control lists (ACLs)</b> - In your <code>CreateBucket</code> request, if you specify an access control list (ACL) and set it to <code>public-read</code>, <code>public-read-write</code>, <code>authenticated-read</code>, or if you explicitly specify any other custom ACLs, both <code>s3:CreateBucket</code> and <code>s3:PutBucketAcl</code> permissions are required. In your <code>CreateBucket</code> request, if you set the ACL to <code>private</code>, or if you don't specify any ACLs, only the <code>s3:CreateBucket</code> permission is required.</p></li>
/// <li>
/// <p><b>Object Lock</b> - In your <code>CreateBucket</code> request, if you set <code>x-amz-bucket-object-lock-enabled</code> to true, the <code>s3:PutBucketObjectLockConfiguration</code> and <code>s3:PutBucketVersioning</code> permissions are required.</p></li>
/// <li>
/// <p><b>S3 Object Ownership</b> - If your <code>CreateBucket</code> request includes the <code>x-amz-object-ownership</code> header, then the <code>s3:PutBucketOwnershipControls</code> permission is required.</p><important>
/// <p>To set an ACL on a bucket as part of a <code>CreateBucket</code> request, you must explicitly set S3 Object Ownership for the bucket to a different value than the default, <code>BucketOwnerEnforced</code>. Additionally, if your desired bucket ACL grants public access, you must first create the bucket (without the bucket ACL) and then explicitly disable Block Public Access on the bucket before using <code>PutBucketAcl</code> to set the ACL. If you try to create a bucket with a public ACL, the request will fail.</p>
/// <p>For the majority of modern use cases in S3, we recommend that you keep all Block Public Access settings enabled and keep ACLs disabled. If you would like to share data with users outside of your account, you can use bucket policies as needed. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/about-object-ownership.html">Controlling ownership of objects and disabling ACLs for your bucket </a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/access-control-block-public-access.html">Blocking public access to your Amazon S3 storage </a> in the <i>Amazon S3 User Guide</i>.</p>
/// </important></li>
/// <li>
/// <p><b>S3 Block Public Access</b> - If your specific use case requires granting public access to your S3 resources, you can disable Block Public Access. Specifically, you can create a new bucket with Block Public Access enabled, then separately call the <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeletePublicAccessBlock.html"> <code>DeletePublicAccessBlock</code> </a> API. To use this operation, you must have the <code>s3:PutBucketPublicAccessBlock</code> permission. For more information about S3 Block Public Access, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/access-control-block-public-access.html">Blocking public access to your Amazon S3 storage </a> in the <i>Amazon S3 User Guide</i>.</p></li>
/// </ul></li>
/// <li>
/// <p><b>Directory bucket permissions</b> - You must have the <code>s3express:CreateBucket</code> permission in an IAM identity-based policy instead of a bucket policy. Cross-account access to this API operation isn't supported. This operation can only be performed by the Amazon Web Services account that owns the resource. For more information about directory bucket policies and permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-express-security-iam.html">Amazon Web Services Identity and Access Management (IAM) for S3 Express One Zone</a> in the <i>Amazon S3 User Guide</i>.</p><important>
/// <p>The permissions for ACLs, Object Lock, S3 Object Ownership, and S3 Block Public Access are not supported for directory buckets. For directory buckets, all Block Public Access settings are enabled at the bucket level and S3 Object Ownership is set to Bucket owner enforced (ACLs disabled). These settings can't be modified.</p>
/// <p>For more information about permissions for creating and working with directory buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/directory-buckets-overview.html">Directory buckets</a> in the <i>Amazon S3 User Guide</i>. For more information about supported S3 features for directory buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-express-one-zone.html#s3-express-features">Features of S3 Express One Zone</a> in the <i>Amazon S3 User Guide</i>.</p>
/// </important></li>
/// </ul>
/// </dd>
/// <dt>
/// HTTP Host header syntax
/// </dt>
/// <dd>
/// <p><b>Directory buckets </b> - The HTTP Host header syntax is <code>s3express-control.<i>region-code</i>.amazonaws.com</code>.</p>
/// </dd>
/// </dl>
/// <p>The following operations are related to <code>CreateBucket</code>:</p>
/// <ul>
/// <li>
/// <p><a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutObject.html">PutObject</a></p></li>
/// <li>
/// <p><a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucket.html">DeleteBucket</a></p></li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateBucketFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_bucket::builders::CreateBucketInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_bucket::CreateBucketOutput,
        crate::operation::create_bucket::CreateBucketError,
    > for CreateBucketFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_bucket::CreateBucketOutput,
            crate::operation::create_bucket::CreateBucketError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateBucketFluentBuilder {
    /// Creates a new `CreateBucketFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateBucket as a reference.
    pub fn as_input(&self) -> &crate::operation::create_bucket::builders::CreateBucketInputBuilder {
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
        crate::operation::create_bucket::CreateBucketOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_bucket::CreateBucketError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_bucket::CreateBucket::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_bucket::CreateBucket::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_bucket::CreateBucketOutput,
        crate::operation::create_bucket::CreateBucketError,
        Self,
    > {
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
    /// <p>The canned ACL to apply to the bucket.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn acl(mut self, input: crate::types::BucketCannedAcl) -> Self {
        self.inner = self.inner.acl(input);
        self
    }
    /// <p>The canned ACL to apply to the bucket.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn set_acl(mut self, input: ::std::option::Option<crate::types::BucketCannedAcl>) -> Self {
        self.inner = self.inner.set_acl(input);
        self
    }
    /// <p>The canned ACL to apply to the bucket.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn get_acl(&self) -> &::std::option::Option<crate::types::BucketCannedAcl> {
        self.inner.get_acl()
    }
    /// <p>The name of the bucket to create.</p>
    /// <p><b>General purpose buckets</b> - For information about bucket naming restrictions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html">Bucket naming rules</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p><b>Directory buckets </b> - When you use this operation with a directory bucket, you must use path-style requests in the format <code>https://s3express-control.<i>region-code</i>.amazonaws.com/<i>bucket-name</i> </code>. Virtual-hosted-style requests aren't supported. Directory bucket names must be unique in the chosen Zone (Availability Zone or Local Zone). Bucket names must also follow the format <code> <i>bucket-base-name</i>--<i>zone-id</i>--x-s3</code> (for example, <code> <i>DOC-EXAMPLE-BUCKET</i>--<i>usw2-az1</i>--x-s3</code>). For information about bucket naming restrictions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/directory-bucket-naming-rules.html">Directory bucket naming rules</a> in the <i>Amazon S3 User Guide</i></p>
    pub fn bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bucket(input.into());
        self
    }
    /// <p>The name of the bucket to create.</p>
    /// <p><b>General purpose buckets</b> - For information about bucket naming restrictions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html">Bucket naming rules</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p><b>Directory buckets </b> - When you use this operation with a directory bucket, you must use path-style requests in the format <code>https://s3express-control.<i>region-code</i>.amazonaws.com/<i>bucket-name</i> </code>. Virtual-hosted-style requests aren't supported. Directory bucket names must be unique in the chosen Zone (Availability Zone or Local Zone). Bucket names must also follow the format <code> <i>bucket-base-name</i>--<i>zone-id</i>--x-s3</code> (for example, <code> <i>DOC-EXAMPLE-BUCKET</i>--<i>usw2-az1</i>--x-s3</code>). For information about bucket naming restrictions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/directory-bucket-naming-rules.html">Directory bucket naming rules</a> in the <i>Amazon S3 User Guide</i></p>
    pub fn set_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bucket(input);
        self
    }
    /// <p>The name of the bucket to create.</p>
    /// <p><b>General purpose buckets</b> - For information about bucket naming restrictions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html">Bucket naming rules</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p><b>Directory buckets </b> - When you use this operation with a directory bucket, you must use path-style requests in the format <code>https://s3express-control.<i>region-code</i>.amazonaws.com/<i>bucket-name</i> </code>. Virtual-hosted-style requests aren't supported. Directory bucket names must be unique in the chosen Zone (Availability Zone or Local Zone). Bucket names must also follow the format <code> <i>bucket-base-name</i>--<i>zone-id</i>--x-s3</code> (for example, <code> <i>DOC-EXAMPLE-BUCKET</i>--<i>usw2-az1</i>--x-s3</code>). For information about bucket naming restrictions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/directory-bucket-naming-rules.html">Directory bucket naming rules</a> in the <i>Amazon S3 User Guide</i></p>
    pub fn get_bucket(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bucket()
    }
    /// <p>The configuration information for the bucket.</p>
    pub fn create_bucket_configuration(mut self, input: crate::types::CreateBucketConfiguration) -> Self {
        self.inner = self.inner.create_bucket_configuration(input);
        self
    }
    /// <p>The configuration information for the bucket.</p>
    pub fn set_create_bucket_configuration(mut self, input: ::std::option::Option<crate::types::CreateBucketConfiguration>) -> Self {
        self.inner = self.inner.set_create_bucket_configuration(input);
        self
    }
    /// <p>The configuration information for the bucket.</p>
    pub fn get_create_bucket_configuration(&self) -> &::std::option::Option<crate::types::CreateBucketConfiguration> {
        self.inner.get_create_bucket_configuration()
    }
    /// <p>Allows grantee the read, write, read ACP, and write ACP permissions on the bucket.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn grant_full_control(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.grant_full_control(input.into());
        self
    }
    /// <p>Allows grantee the read, write, read ACP, and write ACP permissions on the bucket.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn set_grant_full_control(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_grant_full_control(input);
        self
    }
    /// <p>Allows grantee the read, write, read ACP, and write ACP permissions on the bucket.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn get_grant_full_control(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_grant_full_control()
    }
    /// <p>Allows grantee to list the objects in the bucket.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn grant_read(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.grant_read(input.into());
        self
    }
    /// <p>Allows grantee to list the objects in the bucket.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn set_grant_read(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_grant_read(input);
        self
    }
    /// <p>Allows grantee to list the objects in the bucket.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn get_grant_read(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_grant_read()
    }
    /// <p>Allows grantee to read the bucket ACL.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn grant_read_acp(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.grant_read_acp(input.into());
        self
    }
    /// <p>Allows grantee to read the bucket ACL.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn set_grant_read_acp(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_grant_read_acp(input);
        self
    }
    /// <p>Allows grantee to read the bucket ACL.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn get_grant_read_acp(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_grant_read_acp()
    }
    /// <p>Allows grantee to create new objects in the bucket.</p>
    /// <p>For the bucket and object owners of existing objects, also allows deletions and overwrites of those objects.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn grant_write(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.grant_write(input.into());
        self
    }
    /// <p>Allows grantee to create new objects in the bucket.</p>
    /// <p>For the bucket and object owners of existing objects, also allows deletions and overwrites of those objects.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn set_grant_write(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_grant_write(input);
        self
    }
    /// <p>Allows grantee to create new objects in the bucket.</p>
    /// <p>For the bucket and object owners of existing objects, also allows deletions and overwrites of those objects.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn get_grant_write(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_grant_write()
    }
    /// <p>Allows grantee to write the ACL for the applicable bucket.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn grant_write_acp(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.grant_write_acp(input.into());
        self
    }
    /// <p>Allows grantee to write the ACL for the applicable bucket.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn set_grant_write_acp(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_grant_write_acp(input);
        self
    }
    /// <p>Allows grantee to write the ACL for the applicable bucket.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn get_grant_write_acp(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_grant_write_acp()
    }
    /// <p>Specifies whether you want S3 Object Lock to be enabled for the new bucket.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn object_lock_enabled_for_bucket(mut self, input: bool) -> Self {
        self.inner = self.inner.object_lock_enabled_for_bucket(input);
        self
    }
    /// <p>Specifies whether you want S3 Object Lock to be enabled for the new bucket.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn set_object_lock_enabled_for_bucket(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_object_lock_enabled_for_bucket(input);
        self
    }
    /// <p>Specifies whether you want S3 Object Lock to be enabled for the new bucket.</p><note>
    /// <p>This functionality is not supported for directory buckets.</p>
    /// </note>
    pub fn get_object_lock_enabled_for_bucket(&self) -> &::std::option::Option<bool> {
        self.inner.get_object_lock_enabled_for_bucket()
    }
    /// <p>The container element for object ownership for a bucket's ownership controls.</p>
    /// <p><code>BucketOwnerPreferred</code> - Objects uploaded to the bucket change ownership to the bucket owner if the objects are uploaded with the <code>bucket-owner-full-control</code> canned ACL.</p>
    /// <p><code>ObjectWriter</code> - The uploading account will own the object if the object is uploaded with the <code>bucket-owner-full-control</code> canned ACL.</p>
    /// <p><code>BucketOwnerEnforced</code> - Access control lists (ACLs) are disabled and no longer affect permissions. The bucket owner automatically owns and has full control over every object in the bucket. The bucket only accepts PUT requests that don't specify an ACL or specify bucket owner full control ACLs (such as the predefined <code>bucket-owner-full-control</code> canned ACL or a custom ACL in XML format that grants the same permissions).</p>
    /// <p>By default, <code>ObjectOwnership</code> is set to <code>BucketOwnerEnforced</code> and ACLs are disabled. We recommend keeping ACLs disabled, except in uncommon use cases where you must control access for each object individually. For more information about S3 Object Ownership, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/about-object-ownership.html">Controlling ownership of objects and disabling ACLs for your bucket</a> in the <i>Amazon S3 User Guide</i>.</p><note>
    /// <p>This functionality is not supported for directory buckets. Directory buckets use the bucket owner enforced setting for S3 Object Ownership.</p>
    /// </note>
    pub fn object_ownership(mut self, input: crate::types::ObjectOwnership) -> Self {
        self.inner = self.inner.object_ownership(input);
        self
    }
    /// <p>The container element for object ownership for a bucket's ownership controls.</p>
    /// <p><code>BucketOwnerPreferred</code> - Objects uploaded to the bucket change ownership to the bucket owner if the objects are uploaded with the <code>bucket-owner-full-control</code> canned ACL.</p>
    /// <p><code>ObjectWriter</code> - The uploading account will own the object if the object is uploaded with the <code>bucket-owner-full-control</code> canned ACL.</p>
    /// <p><code>BucketOwnerEnforced</code> - Access control lists (ACLs) are disabled and no longer affect permissions. The bucket owner automatically owns and has full control over every object in the bucket. The bucket only accepts PUT requests that don't specify an ACL or specify bucket owner full control ACLs (such as the predefined <code>bucket-owner-full-control</code> canned ACL or a custom ACL in XML format that grants the same permissions).</p>
    /// <p>By default, <code>ObjectOwnership</code> is set to <code>BucketOwnerEnforced</code> and ACLs are disabled. We recommend keeping ACLs disabled, except in uncommon use cases where you must control access for each object individually. For more information about S3 Object Ownership, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/about-object-ownership.html">Controlling ownership of objects and disabling ACLs for your bucket</a> in the <i>Amazon S3 User Guide</i>.</p><note>
    /// <p>This functionality is not supported for directory buckets. Directory buckets use the bucket owner enforced setting for S3 Object Ownership.</p>
    /// </note>
    pub fn set_object_ownership(mut self, input: ::std::option::Option<crate::types::ObjectOwnership>) -> Self {
        self.inner = self.inner.set_object_ownership(input);
        self
    }
    /// <p>The container element for object ownership for a bucket's ownership controls.</p>
    /// <p><code>BucketOwnerPreferred</code> - Objects uploaded to the bucket change ownership to the bucket owner if the objects are uploaded with the <code>bucket-owner-full-control</code> canned ACL.</p>
    /// <p><code>ObjectWriter</code> - The uploading account will own the object if the object is uploaded with the <code>bucket-owner-full-control</code> canned ACL.</p>
    /// <p><code>BucketOwnerEnforced</code> - Access control lists (ACLs) are disabled and no longer affect permissions. The bucket owner automatically owns and has full control over every object in the bucket. The bucket only accepts PUT requests that don't specify an ACL or specify bucket owner full control ACLs (such as the predefined <code>bucket-owner-full-control</code> canned ACL or a custom ACL in XML format that grants the same permissions).</p>
    /// <p>By default, <code>ObjectOwnership</code> is set to <code>BucketOwnerEnforced</code> and ACLs are disabled. We recommend keeping ACLs disabled, except in uncommon use cases where you must control access for each object individually. For more information about S3 Object Ownership, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/about-object-ownership.html">Controlling ownership of objects and disabling ACLs for your bucket</a> in the <i>Amazon S3 User Guide</i>.</p><note>
    /// <p>This functionality is not supported for directory buckets. Directory buckets use the bucket owner enforced setting for S3 Object Ownership.</p>
    /// </note>
    pub fn get_object_ownership(&self) -> &::std::option::Option<crate::types::ObjectOwnership> {
        self.inner.get_object_ownership()
    }
}
