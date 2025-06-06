// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum Error {
    /// <p>A service resource associated with the request could not be found. The resource might not be specified correctly, or it may have a <code>state</code> of <code>DELETED</code>.</p>
    ResourceNotFoundException(crate::types::error::ResourceNotFoundException),
    /// <p>The request doesn't comply with IAM tag policy. Correct your request and then retry it.</p>
    TagPolicyException(crate::types::error::TagPolicyException),
    /// <p>The <code>CreateEnvironmentHost</code> operation couldn't be performed because the service is throttling requests. This exception is thrown when the <code>CreateEnvironmentHost</code> request exceeds concurrency of 1 transaction per second (TPS).</p>
    ThrottlingException(crate::types::error::ThrottlingException),
    /// <p>A service resource associated with the request has more than 200 tags.</p>
    TooManyTagsException(crate::types::error::TooManyTagsException),
    /// <p>The input fails to satisfy the specified constraints. You will see this exception if invalid inputs are provided for any of the Amazon EVS environment operations, or if a list operation is performed on an environment resource that is still initializing.</p>
    ValidationException(crate::types::error::ValidationException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
    variable wildcard pattern and check `.code()`:
     \
    &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
     \
    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-Error) for what information is available for the error.")]
    Unhandled(crate::error::sealed_unhandled::Unhandled),
}
impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::TagPolicyException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::TooManyTagsException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(_) => {
                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
                    write!(f, "unhandled error ({code})")
                } else {
                    f.write_str("unhandled error")
                }
            }
        }
    }
}
impl From<::aws_smithy_types::error::operation::BuildError> for Error {
    fn from(value: ::aws_smithy_types::error::operation::BuildError) -> Self {
        Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source: value.into(),
            meta: ::std::default::Default::default(),
        })
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for Error {
    fn meta(&self) -> &::aws_smithy_types::error::metadata::ErrorMetadata {
        match self {
            Self::ResourceNotFoundException(inner) => inner.meta(),
            Self::TagPolicyException(inner) => inner.meta(),
            Self::ThrottlingException(inner) => inner.meta(),
            Self::TooManyTagsException(inner) => inner.meta(),
            Self::ValidationException(inner) => inner.meta(),
            Self::Unhandled(inner) => &inner.meta,
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::create_environment::CreateEnvironmentError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::create_environment::CreateEnvironmentError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::create_environment::CreateEnvironmentError> for Error {
    fn from(err: crate::operation::create_environment::CreateEnvironmentError) -> Self {
        match err {
            crate::operation::create_environment::CreateEnvironmentError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::create_environment::CreateEnvironmentError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::create_environment_host::CreateEnvironmentHostError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::create_environment_host::CreateEnvironmentHostError, R>,
    ) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::create_environment_host::CreateEnvironmentHostError> for Error {
    fn from(err: crate::operation::create_environment_host::CreateEnvironmentHostError) -> Self {
        match err {
            crate::operation::create_environment_host::CreateEnvironmentHostError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::create_environment_host::CreateEnvironmentHostError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::create_environment_host::CreateEnvironmentHostError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::delete_environment::DeleteEnvironmentError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::delete_environment::DeleteEnvironmentError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::delete_environment::DeleteEnvironmentError> for Error {
    fn from(err: crate::operation::delete_environment::DeleteEnvironmentError) -> Self {
        match err {
            crate::operation::delete_environment::DeleteEnvironmentError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::delete_environment::DeleteEnvironmentError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::delete_environment::DeleteEnvironmentError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::delete_environment_host::DeleteEnvironmentHostError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::delete_environment_host::DeleteEnvironmentHostError, R>,
    ) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::delete_environment_host::DeleteEnvironmentHostError> for Error {
    fn from(err: crate::operation::delete_environment_host::DeleteEnvironmentHostError) -> Self {
        match err {
            crate::operation::delete_environment_host::DeleteEnvironmentHostError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::delete_environment_host::DeleteEnvironmentHostError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::delete_environment_host::DeleteEnvironmentHostError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_environment::GetEnvironmentError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_environment::GetEnvironmentError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::get_environment::GetEnvironmentError> for Error {
    fn from(err: crate::operation::get_environment::GetEnvironmentError) -> Self {
        match err {
            crate::operation::get_environment::GetEnvironmentError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::get_environment::GetEnvironmentError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::get_environment::GetEnvironmentError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_environment_hosts::ListEnvironmentHostsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_environment_hosts::ListEnvironmentHostsError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::list_environment_hosts::ListEnvironmentHostsError> for Error {
    fn from(err: crate::operation::list_environment_hosts::ListEnvironmentHostsError) -> Self {
        match err {
            crate::operation::list_environment_hosts::ListEnvironmentHostsError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::list_environment_hosts::ListEnvironmentHostsError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::list_environment_hosts::ListEnvironmentHostsError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_environments::ListEnvironmentsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_environments::ListEnvironmentsError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::list_environments::ListEnvironmentsError> for Error {
    fn from(err: crate::operation::list_environments::ListEnvironmentsError) -> Self {
        match err {
            crate::operation::list_environments::ListEnvironmentsError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::list_environments::ListEnvironmentsError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_environment_vlans::ListEnvironmentVlansError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_environment_vlans::ListEnvironmentVlansError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::list_environment_vlans::ListEnvironmentVlansError> for Error {
    fn from(err: crate::operation::list_environment_vlans::ListEnvironmentVlansError) -> Self {
        match err {
            crate::operation::list_environment_vlans::ListEnvironmentVlansError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::list_environment_vlans::ListEnvironmentVlansError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::list_environment_vlans::ListEnvironmentVlansError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_tags_for_resource::ListTagsForResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_tags_for_resource::ListTagsForResourceError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::list_tags_for_resource::ListTagsForResourceError> for Error {
    fn from(err: crate::operation::list_tags_for_resource::ListTagsForResourceError) -> Self {
        match err {
            crate::operation::list_tags_for_resource::ListTagsForResourceError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::list_tags_for_resource::ListTagsForResourceError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::tag_resource::TagResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::tag_resource::TagResourceError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::tag_resource::TagResourceError> for Error {
    fn from(err: crate::operation::tag_resource::TagResourceError) -> Self {
        match err {
            crate::operation::tag_resource::TagResourceError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::tag_resource::TagResourceError::TagPolicyException(inner) => Error::TagPolicyException(inner),
            crate::operation::tag_resource::TagResourceError::TooManyTagsException(inner) => Error::TooManyTagsException(inner),
            crate::operation::tag_resource::TagResourceError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::untag_resource::UntagResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::untag_resource::UntagResourceError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::untag_resource::UntagResourceError> for Error {
    fn from(err: crate::operation::untag_resource::UntagResourceError) -> Self {
        match err {
            crate::operation::untag_resource::UntagResourceError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::untag_resource::UntagResourceError::TagPolicyException(inner) => Error::TagPolicyException(inner),
            crate::operation::untag_resource::UntagResourceError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl ::std::error::Error for Error {
    fn source(&self) -> std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Error::ResourceNotFoundException(inner) => inner.source(),
            Error::TagPolicyException(inner) => inner.source(),
            Error::ThrottlingException(inner) => inner.source(),
            Error::TooManyTagsException(inner) => inner.source(),
            Error::ValidationException(inner) => inner.source(),
            Error::Unhandled(inner) => ::std::option::Option::Some(&*inner.source),
        }
    }
}
impl ::aws_types::request_id::RequestId for Error {
    fn request_id(&self) -> Option<&str> {
        match self {
            Self::ResourceNotFoundException(e) => e.request_id(),
            Self::TagPolicyException(e) => e.request_id(),
            Self::ThrottlingException(e) => e.request_id(),
            Self::TooManyTagsException(e) => e.request_id(),
            Self::ValidationException(e) => e.request_id(),
            Self::Unhandled(e) => e.meta.request_id(),
        }
    }
}
