// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The location in Amazon S3 where build or script files are stored for access by Amazon GameLift Servers.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct S3Location {
    /// <p>An Amazon S3 bucket identifier. Thename of the S3 bucket.</p><note>
    /// <p>Amazon GameLift Servers doesn't support uploading from Amazon S3 buckets with names that contain a dot (.).</p>
    /// </note>
    pub bucket: ::std::option::Option<::std::string::String>,
    /// <p>The name of the zip file that contains the build files or script files.</p>
    pub key: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/s3-arn-format.html">ARN</a>) for an IAM role that allows Amazon GameLift Servers to access the S3 bucket.</p>
    pub role_arn: ::std::option::Option<::std::string::String>,
    /// <p>The version of the file, if object versioning is turned on for the bucket. Amazon GameLift Servers uses this information when retrieving files from an S3 bucket that you own. Use this parameter to specify a specific version of the file. If not set, the latest version of the file is retrieved.</p>
    pub object_version: ::std::option::Option<::std::string::String>,
}
impl S3Location {
    /// <p>An Amazon S3 bucket identifier. Thename of the S3 bucket.</p><note>
    /// <p>Amazon GameLift Servers doesn't support uploading from Amazon S3 buckets with names that contain a dot (.).</p>
    /// </note>
    pub fn bucket(&self) -> ::std::option::Option<&str> {
        self.bucket.as_deref()
    }
    /// <p>The name of the zip file that contains the build files or script files.</p>
    pub fn key(&self) -> ::std::option::Option<&str> {
        self.key.as_deref()
    }
    /// <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/s3-arn-format.html">ARN</a>) for an IAM role that allows Amazon GameLift Servers to access the S3 bucket.</p>
    pub fn role_arn(&self) -> ::std::option::Option<&str> {
        self.role_arn.as_deref()
    }
    /// <p>The version of the file, if object versioning is turned on for the bucket. Amazon GameLift Servers uses this information when retrieving files from an S3 bucket that you own. Use this parameter to specify a specific version of the file. If not set, the latest version of the file is retrieved.</p>
    pub fn object_version(&self) -> ::std::option::Option<&str> {
        self.object_version.as_deref()
    }
}
impl S3Location {
    /// Creates a new builder-style object to manufacture [`S3Location`](crate::types::S3Location).
    pub fn builder() -> crate::types::builders::S3LocationBuilder {
        crate::types::builders::S3LocationBuilder::default()
    }
}

/// A builder for [`S3Location`](crate::types::S3Location).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct S3LocationBuilder {
    pub(crate) bucket: ::std::option::Option<::std::string::String>,
    pub(crate) key: ::std::option::Option<::std::string::String>,
    pub(crate) role_arn: ::std::option::Option<::std::string::String>,
    pub(crate) object_version: ::std::option::Option<::std::string::String>,
}
impl S3LocationBuilder {
    /// <p>An Amazon S3 bucket identifier. Thename of the S3 bucket.</p><note>
    /// <p>Amazon GameLift Servers doesn't support uploading from Amazon S3 buckets with names that contain a dot (.).</p>
    /// </note>
    pub fn bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bucket = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An Amazon S3 bucket identifier. Thename of the S3 bucket.</p><note>
    /// <p>Amazon GameLift Servers doesn't support uploading from Amazon S3 buckets with names that contain a dot (.).</p>
    /// </note>
    pub fn set_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bucket = input;
        self
    }
    /// <p>An Amazon S3 bucket identifier. Thename of the S3 bucket.</p><note>
    /// <p>Amazon GameLift Servers doesn't support uploading from Amazon S3 buckets with names that contain a dot (.).</p>
    /// </note>
    pub fn get_bucket(&self) -> &::std::option::Option<::std::string::String> {
        &self.bucket
    }
    /// <p>The name of the zip file that contains the build files or script files.</p>
    pub fn key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the zip file that contains the build files or script files.</p>
    pub fn set_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key = input;
        self
    }
    /// <p>The name of the zip file that contains the build files or script files.</p>
    pub fn get_key(&self) -> &::std::option::Option<::std::string::String> {
        &self.key
    }
    /// <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/s3-arn-format.html">ARN</a>) for an IAM role that allows Amazon GameLift Servers to access the S3 bucket.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.role_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/s3-arn-format.html">ARN</a>) for an IAM role that allows Amazon GameLift Servers to access the S3 bucket.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.role_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (<a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/s3-arn-format.html">ARN</a>) for an IAM role that allows Amazon GameLift Servers to access the S3 bucket.</p>
    pub fn get_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.role_arn
    }
    /// <p>The version of the file, if object versioning is turned on for the bucket. Amazon GameLift Servers uses this information when retrieving files from an S3 bucket that you own. Use this parameter to specify a specific version of the file. If not set, the latest version of the file is retrieved.</p>
    pub fn object_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.object_version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The version of the file, if object versioning is turned on for the bucket. Amazon GameLift Servers uses this information when retrieving files from an S3 bucket that you own. Use this parameter to specify a specific version of the file. If not set, the latest version of the file is retrieved.</p>
    pub fn set_object_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.object_version = input;
        self
    }
    /// <p>The version of the file, if object versioning is turned on for the bucket. Amazon GameLift Servers uses this information when retrieving files from an S3 bucket that you own. Use this parameter to specify a specific version of the file. If not set, the latest version of the file is retrieved.</p>
    pub fn get_object_version(&self) -> &::std::option::Option<::std::string::String> {
        &self.object_version
    }
    /// Consumes the builder and constructs a [`S3Location`](crate::types::S3Location).
    pub fn build(self) -> crate::types::S3Location {
        crate::types::S3Location {
            bucket: self.bucket,
            key: self.key,
            role_arn: self.role_arn,
            object_version: self.object_version,
        }
    }
}
