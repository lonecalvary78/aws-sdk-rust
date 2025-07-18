// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RevokeCertificateOutput {
    /// <p>The Amazon Resource Name (ARN) of the public or private certificate that was revoked.</p>
    pub certificate_arn: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl RevokeCertificateOutput {
    /// <p>The Amazon Resource Name (ARN) of the public or private certificate that was revoked.</p>
    pub fn certificate_arn(&self) -> ::std::option::Option<&str> {
        self.certificate_arn.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for RevokeCertificateOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl RevokeCertificateOutput {
    /// Creates a new builder-style object to manufacture [`RevokeCertificateOutput`](crate::operation::revoke_certificate::RevokeCertificateOutput).
    pub fn builder() -> crate::operation::revoke_certificate::builders::RevokeCertificateOutputBuilder {
        crate::operation::revoke_certificate::builders::RevokeCertificateOutputBuilder::default()
    }
}

/// A builder for [`RevokeCertificateOutput`](crate::operation::revoke_certificate::RevokeCertificateOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct RevokeCertificateOutputBuilder {
    pub(crate) certificate_arn: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl RevokeCertificateOutputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the public or private certificate that was revoked.</p>
    pub fn certificate_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.certificate_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the public or private certificate that was revoked.</p>
    pub fn set_certificate_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.certificate_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the public or private certificate that was revoked.</p>
    pub fn get_certificate_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.certificate_arn
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`RevokeCertificateOutput`](crate::operation::revoke_certificate::RevokeCertificateOutput).
    pub fn build(self) -> crate::operation::revoke_certificate::RevokeCertificateOutput {
        crate::operation::revoke_certificate::RevokeCertificateOutput {
            certificate_arn: self.certificate_arn,
            _request_id: self._request_id,
        }
    }
}
