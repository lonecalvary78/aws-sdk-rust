// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn access_denied_exception_correct_errors(
    mut builder: crate::types::error::builders::AccessDeniedExceptionBuilder,
) -> crate::types::error::builders::AccessDeniedExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn conflict_exception_correct_errors(
    mut builder: crate::types::error::builders::ConflictExceptionBuilder,
) -> crate::types::error::builders::ConflictExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    if builder.resource_id.is_none() {
        builder.resource_id = Some(Default::default())
    }
    if builder.resource_type.is_none() {
        builder.resource_type = Some(Default::default())
    }
    builder
}

pub(crate) fn internal_server_exception_correct_errors(
    mut builder: crate::types::error::builders::InternalServerExceptionBuilder,
) -> crate::types::error::builders::InternalServerExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn resource_not_found_exception_correct_errors(
    mut builder: crate::types::error::builders::ResourceNotFoundExceptionBuilder,
) -> crate::types::error::builders::ResourceNotFoundExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    if builder.resource_id.is_none() {
        builder.resource_id = Some(Default::default())
    }
    if builder.resource_type.is_none() {
        builder.resource_type = Some(Default::default())
    }
    builder
}

pub(crate) fn service_quota_exceeded_exception_correct_errors(
    mut builder: crate::types::error::builders::ServiceQuotaExceededExceptionBuilder,
) -> crate::types::error::builders::ServiceQuotaExceededExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    if builder.resource_id.is_none() {
        builder.resource_id = Some(Default::default())
    }
    if builder.resource_type.is_none() {
        builder.resource_type = Some(Default::default())
    }
    if builder.service_code.is_none() {
        builder.service_code = Some(Default::default())
    }
    if builder.quota_code.is_none() {
        builder.quota_code = Some(Default::default())
    }
    builder
}

pub(crate) fn throttling_exception_correct_errors(
    mut builder: crate::types::error::builders::ThrottlingExceptionBuilder,
) -> crate::types::error::builders::ThrottlingExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn validation_exception_correct_errors(
    mut builder: crate::types::error::builders::ValidationExceptionBuilder,
) -> crate::types::error::builders::ValidationExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    if builder.reason.is_none() {
        builder.reason = "no value was set".parse::<crate::types::ValidationExceptionReason>().ok()
    }
    builder
}

pub(crate) fn list_customer_agreements_output_output_correct_errors(
    mut builder: crate::operation::list_customer_agreements::builders::ListCustomerAgreementsOutputBuilder,
) -> crate::operation::list_customer_agreements::builders::ListCustomerAgreementsOutputBuilder {
    if builder.customer_agreements.is_none() {
        builder.customer_agreements = Some(Default::default())
    }
    builder
}

pub(crate) fn validation_exception_field_correct_errors(
    mut builder: crate::types::builders::ValidationExceptionFieldBuilder,
) -> crate::types::builders::ValidationExceptionFieldBuilder {
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}
