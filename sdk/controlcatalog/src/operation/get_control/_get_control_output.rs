// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetControlOutput {
    /// <p>The Amazon Resource Name (ARN) of the control.</p>
    pub arn: ::std::string::String,
    /// <p>The display name of the control.</p>
    pub name: ::std::string::String,
    /// <p>A description of what the control does.</p>
    pub description: ::std::string::String,
    /// <p>A term that identifies the control's functional behavior. One of <code>Preventive</code>, <code>Deteictive</code>, <code>Proactive</code></p>
    pub behavior: crate::types::ControlBehavior,
    /// <p>Returns information about the control, including the scope of the control, if enabled, and the Regions in which the control currently is available for deployment.</p>
    /// <p>If you are applying controls through an Amazon Web Services Control Tower landing zone environment, remember that the values returned in the <code>RegionConfiguration</code> API operation are not related to the governed Regions in your landing zone. For example, if you are governing Regions <code>A</code>,<code>B</code>,and <code>C</code> while the control is available in Regions <code>A</code>, <code>B</code>, C<code>,</code> and <code>D</code>, you'd see a response with <code>DeployableRegions</code> of <code>A</code>, <code>B</code>, <code>C</code>, and <code>D</code> for a control with <code>REGIONAL</code> scope, even though you may not intend to deploy the control in Region <code>D</code>, because you do not govern it through your landing zone.</p>
    pub region_configuration: ::std::option::Option<crate::types::RegionConfiguration>,
    _request_id: Option<String>,
}
impl GetControlOutput {
    /// <p>The Amazon Resource Name (ARN) of the control.</p>
    pub fn arn(&self) -> &str {
        use std::ops::Deref;
        self.arn.deref()
    }
    /// <p>The display name of the control.</p>
    pub fn name(&self) -> &str {
        use std::ops::Deref;
        self.name.deref()
    }
    /// <p>A description of what the control does.</p>
    pub fn description(&self) -> &str {
        use std::ops::Deref;
        self.description.deref()
    }
    /// <p>A term that identifies the control's functional behavior. One of <code>Preventive</code>, <code>Deteictive</code>, <code>Proactive</code></p>
    pub fn behavior(&self) -> &crate::types::ControlBehavior {
        &self.behavior
    }
    /// <p>Returns information about the control, including the scope of the control, if enabled, and the Regions in which the control currently is available for deployment.</p>
    /// <p>If you are applying controls through an Amazon Web Services Control Tower landing zone environment, remember that the values returned in the <code>RegionConfiguration</code> API operation are not related to the governed Regions in your landing zone. For example, if you are governing Regions <code>A</code>,<code>B</code>,and <code>C</code> while the control is available in Regions <code>A</code>, <code>B</code>, C<code>,</code> and <code>D</code>, you'd see a response with <code>DeployableRegions</code> of <code>A</code>, <code>B</code>, <code>C</code>, and <code>D</code> for a control with <code>REGIONAL</code> scope, even though you may not intend to deploy the control in Region <code>D</code>, because you do not govern it through your landing zone.</p>
    pub fn region_configuration(&self) -> ::std::option::Option<&crate::types::RegionConfiguration> {
        self.region_configuration.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for GetControlOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetControlOutput {
    /// Creates a new builder-style object to manufacture [`GetControlOutput`](crate::operation::get_control::GetControlOutput).
    pub fn builder() -> crate::operation::get_control::builders::GetControlOutputBuilder {
        crate::operation::get_control::builders::GetControlOutputBuilder::default()
    }
}

/// A builder for [`GetControlOutput`](crate::operation::get_control::GetControlOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetControlOutputBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) behavior: ::std::option::Option<crate::types::ControlBehavior>,
    pub(crate) region_configuration: ::std::option::Option<crate::types::RegionConfiguration>,
    _request_id: Option<String>,
}
impl GetControlOutputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the control.</p>
    /// This field is required.
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the control.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the control.</p>
    pub fn get_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.arn
    }
    /// <p>The display name of the control.</p>
    /// This field is required.
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The display name of the control.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The display name of the control.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// <p>A description of what the control does.</p>
    /// This field is required.
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A description of what the control does.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>A description of what the control does.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }
    /// <p>A term that identifies the control's functional behavior. One of <code>Preventive</code>, <code>Deteictive</code>, <code>Proactive</code></p>
    /// This field is required.
    pub fn behavior(mut self, input: crate::types::ControlBehavior) -> Self {
        self.behavior = ::std::option::Option::Some(input);
        self
    }
    /// <p>A term that identifies the control's functional behavior. One of <code>Preventive</code>, <code>Deteictive</code>, <code>Proactive</code></p>
    pub fn set_behavior(mut self, input: ::std::option::Option<crate::types::ControlBehavior>) -> Self {
        self.behavior = input;
        self
    }
    /// <p>A term that identifies the control's functional behavior. One of <code>Preventive</code>, <code>Deteictive</code>, <code>Proactive</code></p>
    pub fn get_behavior(&self) -> &::std::option::Option<crate::types::ControlBehavior> {
        &self.behavior
    }
    /// <p>Returns information about the control, including the scope of the control, if enabled, and the Regions in which the control currently is available for deployment.</p>
    /// <p>If you are applying controls through an Amazon Web Services Control Tower landing zone environment, remember that the values returned in the <code>RegionConfiguration</code> API operation are not related to the governed Regions in your landing zone. For example, if you are governing Regions <code>A</code>,<code>B</code>,and <code>C</code> while the control is available in Regions <code>A</code>, <code>B</code>, C<code>,</code> and <code>D</code>, you'd see a response with <code>DeployableRegions</code> of <code>A</code>, <code>B</code>, <code>C</code>, and <code>D</code> for a control with <code>REGIONAL</code> scope, even though you may not intend to deploy the control in Region <code>D</code>, because you do not govern it through your landing zone.</p>
    /// This field is required.
    pub fn region_configuration(mut self, input: crate::types::RegionConfiguration) -> Self {
        self.region_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>Returns information about the control, including the scope of the control, if enabled, and the Regions in which the control currently is available for deployment.</p>
    /// <p>If you are applying controls through an Amazon Web Services Control Tower landing zone environment, remember that the values returned in the <code>RegionConfiguration</code> API operation are not related to the governed Regions in your landing zone. For example, if you are governing Regions <code>A</code>,<code>B</code>,and <code>C</code> while the control is available in Regions <code>A</code>, <code>B</code>, C<code>,</code> and <code>D</code>, you'd see a response with <code>DeployableRegions</code> of <code>A</code>, <code>B</code>, <code>C</code>, and <code>D</code> for a control with <code>REGIONAL</code> scope, even though you may not intend to deploy the control in Region <code>D</code>, because you do not govern it through your landing zone.</p>
    pub fn set_region_configuration(mut self, input: ::std::option::Option<crate::types::RegionConfiguration>) -> Self {
        self.region_configuration = input;
        self
    }
    /// <p>Returns information about the control, including the scope of the control, if enabled, and the Regions in which the control currently is available for deployment.</p>
    /// <p>If you are applying controls through an Amazon Web Services Control Tower landing zone environment, remember that the values returned in the <code>RegionConfiguration</code> API operation are not related to the governed Regions in your landing zone. For example, if you are governing Regions <code>A</code>,<code>B</code>,and <code>C</code> while the control is available in Regions <code>A</code>, <code>B</code>, C<code>,</code> and <code>D</code>, you'd see a response with <code>DeployableRegions</code> of <code>A</code>, <code>B</code>, <code>C</code>, and <code>D</code> for a control with <code>REGIONAL</code> scope, even though you may not intend to deploy the control in Region <code>D</code>, because you do not govern it through your landing zone.</p>
    pub fn get_region_configuration(&self) -> &::std::option::Option<crate::types::RegionConfiguration> {
        &self.region_configuration
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetControlOutput`](crate::operation::get_control::GetControlOutput).
    /// This method will fail if any of the following fields are not set:
    /// - [`arn`](crate::operation::get_control::builders::GetControlOutputBuilder::arn)
    /// - [`name`](crate::operation::get_control::builders::GetControlOutputBuilder::name)
    /// - [`description`](crate::operation::get_control::builders::GetControlOutputBuilder::description)
    /// - [`behavior`](crate::operation::get_control::builders::GetControlOutputBuilder::behavior)
    pub fn build(self) -> ::std::result::Result<crate::operation::get_control::GetControlOutput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::get_control::GetControlOutput {
            arn: self.arn.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "arn",
                    "arn was not specified but it is required when building GetControlOutput",
                )
            })?,
            name: self.name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "name",
                    "name was not specified but it is required when building GetControlOutput",
                )
            })?,
            description: self.description.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "description",
                    "description was not specified but it is required when building GetControlOutput",
                )
            })?,
            behavior: self.behavior.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "behavior",
                    "behavior was not specified but it is required when building GetControlOutput",
                )
            })?,
            region_configuration: self.region_configuration,
            _request_id: self._request_id,
        })
    }
}
