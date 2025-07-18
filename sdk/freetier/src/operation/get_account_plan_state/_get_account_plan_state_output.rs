// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetAccountPlanStateOutput {
    /// <p>A unique identifier that identifies the account.</p>
    pub account_id: ::std::string::String,
    /// <p>The plan type for the account.</p>
    pub account_plan_type: crate::types::AccountPlanType,
    /// <p>The current status for the account plan.</p>
    pub account_plan_status: crate::types::AccountPlanStatus,
    /// <p>The amount of credits remaining for the account.</p>
    pub account_plan_remaining_credits: ::std::option::Option<crate::types::MonetaryAmount>,
    /// <p>The timestamp for when the current account plan expires.</p>
    pub account_plan_expiration_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    _request_id: Option<String>,
}
impl GetAccountPlanStateOutput {
    /// <p>A unique identifier that identifies the account.</p>
    pub fn account_id(&self) -> &str {
        use std::ops::Deref;
        self.account_id.deref()
    }
    /// <p>The plan type for the account.</p>
    pub fn account_plan_type(&self) -> &crate::types::AccountPlanType {
        &self.account_plan_type
    }
    /// <p>The current status for the account plan.</p>
    pub fn account_plan_status(&self) -> &crate::types::AccountPlanStatus {
        &self.account_plan_status
    }
    /// <p>The amount of credits remaining for the account.</p>
    pub fn account_plan_remaining_credits(&self) -> ::std::option::Option<&crate::types::MonetaryAmount> {
        self.account_plan_remaining_credits.as_ref()
    }
    /// <p>The timestamp for when the current account plan expires.</p>
    pub fn account_plan_expiration_date(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.account_plan_expiration_date.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for GetAccountPlanStateOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetAccountPlanStateOutput {
    /// Creates a new builder-style object to manufacture [`GetAccountPlanStateOutput`](crate::operation::get_account_plan_state::GetAccountPlanStateOutput).
    pub fn builder() -> crate::operation::get_account_plan_state::builders::GetAccountPlanStateOutputBuilder {
        crate::operation::get_account_plan_state::builders::GetAccountPlanStateOutputBuilder::default()
    }
}

/// A builder for [`GetAccountPlanStateOutput`](crate::operation::get_account_plan_state::GetAccountPlanStateOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetAccountPlanStateOutputBuilder {
    pub(crate) account_id: ::std::option::Option<::std::string::String>,
    pub(crate) account_plan_type: ::std::option::Option<crate::types::AccountPlanType>,
    pub(crate) account_plan_status: ::std::option::Option<crate::types::AccountPlanStatus>,
    pub(crate) account_plan_remaining_credits: ::std::option::Option<crate::types::MonetaryAmount>,
    pub(crate) account_plan_expiration_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    _request_id: Option<String>,
}
impl GetAccountPlanStateOutputBuilder {
    /// <p>A unique identifier that identifies the account.</p>
    /// This field is required.
    pub fn account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.account_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique identifier that identifies the account.</p>
    pub fn set_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.account_id = input;
        self
    }
    /// <p>A unique identifier that identifies the account.</p>
    pub fn get_account_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.account_id
    }
    /// <p>The plan type for the account.</p>
    /// This field is required.
    pub fn account_plan_type(mut self, input: crate::types::AccountPlanType) -> Self {
        self.account_plan_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The plan type for the account.</p>
    pub fn set_account_plan_type(mut self, input: ::std::option::Option<crate::types::AccountPlanType>) -> Self {
        self.account_plan_type = input;
        self
    }
    /// <p>The plan type for the account.</p>
    pub fn get_account_plan_type(&self) -> &::std::option::Option<crate::types::AccountPlanType> {
        &self.account_plan_type
    }
    /// <p>The current status for the account plan.</p>
    /// This field is required.
    pub fn account_plan_status(mut self, input: crate::types::AccountPlanStatus) -> Self {
        self.account_plan_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The current status for the account plan.</p>
    pub fn set_account_plan_status(mut self, input: ::std::option::Option<crate::types::AccountPlanStatus>) -> Self {
        self.account_plan_status = input;
        self
    }
    /// <p>The current status for the account plan.</p>
    pub fn get_account_plan_status(&self) -> &::std::option::Option<crate::types::AccountPlanStatus> {
        &self.account_plan_status
    }
    /// <p>The amount of credits remaining for the account.</p>
    pub fn account_plan_remaining_credits(mut self, input: crate::types::MonetaryAmount) -> Self {
        self.account_plan_remaining_credits = ::std::option::Option::Some(input);
        self
    }
    /// <p>The amount of credits remaining for the account.</p>
    pub fn set_account_plan_remaining_credits(mut self, input: ::std::option::Option<crate::types::MonetaryAmount>) -> Self {
        self.account_plan_remaining_credits = input;
        self
    }
    /// <p>The amount of credits remaining for the account.</p>
    pub fn get_account_plan_remaining_credits(&self) -> &::std::option::Option<crate::types::MonetaryAmount> {
        &self.account_plan_remaining_credits
    }
    /// <p>The timestamp for when the current account plan expires.</p>
    pub fn account_plan_expiration_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.account_plan_expiration_date = ::std::option::Option::Some(input);
        self
    }
    /// <p>The timestamp for when the current account plan expires.</p>
    pub fn set_account_plan_expiration_date(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.account_plan_expiration_date = input;
        self
    }
    /// <p>The timestamp for when the current account plan expires.</p>
    pub fn get_account_plan_expiration_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.account_plan_expiration_date
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetAccountPlanStateOutput`](crate::operation::get_account_plan_state::GetAccountPlanStateOutput).
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](crate::operation::get_account_plan_state::builders::GetAccountPlanStateOutputBuilder::account_id)
    /// - [`account_plan_type`](crate::operation::get_account_plan_state::builders::GetAccountPlanStateOutputBuilder::account_plan_type)
    /// - [`account_plan_status`](crate::operation::get_account_plan_state::builders::GetAccountPlanStateOutputBuilder::account_plan_status)
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::get_account_plan_state::GetAccountPlanStateOutput, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::get_account_plan_state::GetAccountPlanStateOutput {
            account_id: self.account_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "account_id",
                    "account_id was not specified but it is required when building GetAccountPlanStateOutput",
                )
            })?,
            account_plan_type: self.account_plan_type.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "account_plan_type",
                    "account_plan_type was not specified but it is required when building GetAccountPlanStateOutput",
                )
            })?,
            account_plan_status: self.account_plan_status.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "account_plan_status",
                    "account_plan_status was not specified but it is required when building GetAccountPlanStateOutput",
                )
            })?,
            account_plan_remaining_credits: self.account_plan_remaining_credits,
            account_plan_expiration_date: self.account_plan_expiration_date,
            _request_id: self._request_id,
        })
    }
}
