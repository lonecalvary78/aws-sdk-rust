// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the drift information for a resource that has been checked for drift. This includes actual and expected property values for resources in which CloudFormation has detected drift. Only resource properties explicitly defined in the stack template are checked for drift. For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/using-cfn-stack-drift.html">Detect unmanaged configuration changes to stacks and resources with drift detection</a>.</p>
/// <p>Resources that don't currently support drift detection can't be checked. For a list of resources that support drift detection, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/resource-import-supported-resources.html">Resource type support for imports and drift detection</a>.</p>
/// <p>Use <code>DetectStackResourceDrift</code> to detect drift on individual resources, or <code>DetectStackDrift</code> to detect drift on all resources in a given stack that support drift detection.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StackResourceDrift {
    /// <p>The ID of the stack.</p>
    pub stack_id: ::std::option::Option<::std::string::String>,
    /// <p>The logical name of the resource specified in the template.</p>
    pub logical_resource_id: ::std::option::Option<::std::string::String>,
    /// <p>The name or unique identifier that corresponds to a physical instance ID of a resource supported by CloudFormation.</p>
    pub physical_resource_id: ::std::option::Option<::std::string::String>,
    /// <p>Context information that enables CloudFormation to uniquely identify a resource. CloudFormation uses context key-value pairs in cases where a resource's logical and physical IDs aren't enough to uniquely identify that resource. Each context key-value pair specifies a unique resource that contains the targeted resource.</p>
    pub physical_resource_id_context: ::std::option::Option<::std::vec::Vec<crate::types::PhysicalResourceIdContextKeyValuePair>>,
    /// <p>The type of the resource.</p>
    pub resource_type: ::std::option::Option<::std::string::String>,
    /// <p>A JSON structure that contains the expected property values of the stack resource, as defined in the stack template and any values specified as template parameters.</p>
    /// <p>For resources whose <code>StackResourceDriftStatus</code> is <code>DELETED</code>, this structure will not be present.</p>
    pub expected_properties: ::std::option::Option<::std::string::String>,
    /// <p>A JSON structure that contains the actual property values of the stack resource.</p>
    /// <p>For resources whose <code>StackResourceDriftStatus</code> is <code>DELETED</code>, this structure will not be present.</p>
    pub actual_properties: ::std::option::Option<::std::string::String>,
    /// <p>A collection of the resource properties whose actual values differ from their expected values. These will be present only for resources whose <code>StackResourceDriftStatus</code> is <code>MODIFIED</code>.</p>
    pub property_differences: ::std::option::Option<::std::vec::Vec<crate::types::PropertyDifference>>,
    /// <p>Status of the resource's actual configuration compared to its expected configuration.</p>
    /// <ul>
    /// <li>
    /// <p><code>DELETED</code>: The resource differs from its expected template configuration because the resource has been deleted.</p></li>
    /// <li>
    /// <p><code>MODIFIED</code>: One or more resource properties differ from their expected values (as defined in the stack template and any values specified as template parameters).</p></li>
    /// <li>
    /// <p><code>IN_SYNC</code>: The resource's actual configuration matches its expected template configuration.</p></li>
    /// <li>
    /// <p><code>NOT_CHECKED</code>: CloudFormation does not currently return this value.</p></li>
    /// <li>
    /// <p><code>UNKNOWN</code>: CloudFormation could not run drift detection for the resource. See the <code>DriftStatusReason</code> for details.</p></li>
    /// </ul>
    pub stack_resource_drift_status: ::std::option::Option<crate::types::StackResourceDriftStatus>,
    /// <p>Time at which CloudFormation performed drift detection on the stack resource.</p>
    pub timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>Contains information about the module from which the resource was created, if the resource was created from a module included in the stack template.</p>
    pub module_info: ::std::option::Option<crate::types::ModuleInfo>,
    /// <p>The reason for the drift status.</p>
    pub drift_status_reason: ::std::option::Option<::std::string::String>,
}
impl StackResourceDrift {
    /// <p>The ID of the stack.</p>
    pub fn stack_id(&self) -> ::std::option::Option<&str> {
        self.stack_id.as_deref()
    }
    /// <p>The logical name of the resource specified in the template.</p>
    pub fn logical_resource_id(&self) -> ::std::option::Option<&str> {
        self.logical_resource_id.as_deref()
    }
    /// <p>The name or unique identifier that corresponds to a physical instance ID of a resource supported by CloudFormation.</p>
    pub fn physical_resource_id(&self) -> ::std::option::Option<&str> {
        self.physical_resource_id.as_deref()
    }
    /// <p>Context information that enables CloudFormation to uniquely identify a resource. CloudFormation uses context key-value pairs in cases where a resource's logical and physical IDs aren't enough to uniquely identify that resource. Each context key-value pair specifies a unique resource that contains the targeted resource.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.physical_resource_id_context.is_none()`.
    pub fn physical_resource_id_context(&self) -> &[crate::types::PhysicalResourceIdContextKeyValuePair] {
        self.physical_resource_id_context.as_deref().unwrap_or_default()
    }
    /// <p>The type of the resource.</p>
    pub fn resource_type(&self) -> ::std::option::Option<&str> {
        self.resource_type.as_deref()
    }
    /// <p>A JSON structure that contains the expected property values of the stack resource, as defined in the stack template and any values specified as template parameters.</p>
    /// <p>For resources whose <code>StackResourceDriftStatus</code> is <code>DELETED</code>, this structure will not be present.</p>
    pub fn expected_properties(&self) -> ::std::option::Option<&str> {
        self.expected_properties.as_deref()
    }
    /// <p>A JSON structure that contains the actual property values of the stack resource.</p>
    /// <p>For resources whose <code>StackResourceDriftStatus</code> is <code>DELETED</code>, this structure will not be present.</p>
    pub fn actual_properties(&self) -> ::std::option::Option<&str> {
        self.actual_properties.as_deref()
    }
    /// <p>A collection of the resource properties whose actual values differ from their expected values. These will be present only for resources whose <code>StackResourceDriftStatus</code> is <code>MODIFIED</code>.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.property_differences.is_none()`.
    pub fn property_differences(&self) -> &[crate::types::PropertyDifference] {
        self.property_differences.as_deref().unwrap_or_default()
    }
    /// <p>Status of the resource's actual configuration compared to its expected configuration.</p>
    /// <ul>
    /// <li>
    /// <p><code>DELETED</code>: The resource differs from its expected template configuration because the resource has been deleted.</p></li>
    /// <li>
    /// <p><code>MODIFIED</code>: One or more resource properties differ from their expected values (as defined in the stack template and any values specified as template parameters).</p></li>
    /// <li>
    /// <p><code>IN_SYNC</code>: The resource's actual configuration matches its expected template configuration.</p></li>
    /// <li>
    /// <p><code>NOT_CHECKED</code>: CloudFormation does not currently return this value.</p></li>
    /// <li>
    /// <p><code>UNKNOWN</code>: CloudFormation could not run drift detection for the resource. See the <code>DriftStatusReason</code> for details.</p></li>
    /// </ul>
    pub fn stack_resource_drift_status(&self) -> ::std::option::Option<&crate::types::StackResourceDriftStatus> {
        self.stack_resource_drift_status.as_ref()
    }
    /// <p>Time at which CloudFormation performed drift detection on the stack resource.</p>
    pub fn timestamp(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.timestamp.as_ref()
    }
    /// <p>Contains information about the module from which the resource was created, if the resource was created from a module included in the stack template.</p>
    pub fn module_info(&self) -> ::std::option::Option<&crate::types::ModuleInfo> {
        self.module_info.as_ref()
    }
    /// <p>The reason for the drift status.</p>
    pub fn drift_status_reason(&self) -> ::std::option::Option<&str> {
        self.drift_status_reason.as_deref()
    }
}
impl StackResourceDrift {
    /// Creates a new builder-style object to manufacture [`StackResourceDrift`](crate::types::StackResourceDrift).
    pub fn builder() -> crate::types::builders::StackResourceDriftBuilder {
        crate::types::builders::StackResourceDriftBuilder::default()
    }
}

/// A builder for [`StackResourceDrift`](crate::types::StackResourceDrift).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct StackResourceDriftBuilder {
    pub(crate) stack_id: ::std::option::Option<::std::string::String>,
    pub(crate) logical_resource_id: ::std::option::Option<::std::string::String>,
    pub(crate) physical_resource_id: ::std::option::Option<::std::string::String>,
    pub(crate) physical_resource_id_context: ::std::option::Option<::std::vec::Vec<crate::types::PhysicalResourceIdContextKeyValuePair>>,
    pub(crate) resource_type: ::std::option::Option<::std::string::String>,
    pub(crate) expected_properties: ::std::option::Option<::std::string::String>,
    pub(crate) actual_properties: ::std::option::Option<::std::string::String>,
    pub(crate) property_differences: ::std::option::Option<::std::vec::Vec<crate::types::PropertyDifference>>,
    pub(crate) stack_resource_drift_status: ::std::option::Option<crate::types::StackResourceDriftStatus>,
    pub(crate) timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) module_info: ::std::option::Option<crate::types::ModuleInfo>,
    pub(crate) drift_status_reason: ::std::option::Option<::std::string::String>,
}
impl StackResourceDriftBuilder {
    /// <p>The ID of the stack.</p>
    /// This field is required.
    pub fn stack_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.stack_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the stack.</p>
    pub fn set_stack_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.stack_id = input;
        self
    }
    /// <p>The ID of the stack.</p>
    pub fn get_stack_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.stack_id
    }
    /// <p>The logical name of the resource specified in the template.</p>
    /// This field is required.
    pub fn logical_resource_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.logical_resource_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The logical name of the resource specified in the template.</p>
    pub fn set_logical_resource_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.logical_resource_id = input;
        self
    }
    /// <p>The logical name of the resource specified in the template.</p>
    pub fn get_logical_resource_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.logical_resource_id
    }
    /// <p>The name or unique identifier that corresponds to a physical instance ID of a resource supported by CloudFormation.</p>
    pub fn physical_resource_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.physical_resource_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name or unique identifier that corresponds to a physical instance ID of a resource supported by CloudFormation.</p>
    pub fn set_physical_resource_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.physical_resource_id = input;
        self
    }
    /// <p>The name or unique identifier that corresponds to a physical instance ID of a resource supported by CloudFormation.</p>
    pub fn get_physical_resource_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.physical_resource_id
    }
    /// Appends an item to `physical_resource_id_context`.
    ///
    /// To override the contents of this collection use [`set_physical_resource_id_context`](Self::set_physical_resource_id_context).
    ///
    /// <p>Context information that enables CloudFormation to uniquely identify a resource. CloudFormation uses context key-value pairs in cases where a resource's logical and physical IDs aren't enough to uniquely identify that resource. Each context key-value pair specifies a unique resource that contains the targeted resource.</p>
    pub fn physical_resource_id_context(mut self, input: crate::types::PhysicalResourceIdContextKeyValuePair) -> Self {
        let mut v = self.physical_resource_id_context.unwrap_or_default();
        v.push(input);
        self.physical_resource_id_context = ::std::option::Option::Some(v);
        self
    }
    /// <p>Context information that enables CloudFormation to uniquely identify a resource. CloudFormation uses context key-value pairs in cases where a resource's logical and physical IDs aren't enough to uniquely identify that resource. Each context key-value pair specifies a unique resource that contains the targeted resource.</p>
    pub fn set_physical_resource_id_context(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::PhysicalResourceIdContextKeyValuePair>>,
    ) -> Self {
        self.physical_resource_id_context = input;
        self
    }
    /// <p>Context information that enables CloudFormation to uniquely identify a resource. CloudFormation uses context key-value pairs in cases where a resource's logical and physical IDs aren't enough to uniquely identify that resource. Each context key-value pair specifies a unique resource that contains the targeted resource.</p>
    pub fn get_physical_resource_id_context(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::PhysicalResourceIdContextKeyValuePair>> {
        &self.physical_resource_id_context
    }
    /// <p>The type of the resource.</p>
    /// This field is required.
    pub fn resource_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resource_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The type of the resource.</p>
    pub fn set_resource_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resource_type = input;
        self
    }
    /// <p>The type of the resource.</p>
    pub fn get_resource_type(&self) -> &::std::option::Option<::std::string::String> {
        &self.resource_type
    }
    /// <p>A JSON structure that contains the expected property values of the stack resource, as defined in the stack template and any values specified as template parameters.</p>
    /// <p>For resources whose <code>StackResourceDriftStatus</code> is <code>DELETED</code>, this structure will not be present.</p>
    pub fn expected_properties(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.expected_properties = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A JSON structure that contains the expected property values of the stack resource, as defined in the stack template and any values specified as template parameters.</p>
    /// <p>For resources whose <code>StackResourceDriftStatus</code> is <code>DELETED</code>, this structure will not be present.</p>
    pub fn set_expected_properties(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.expected_properties = input;
        self
    }
    /// <p>A JSON structure that contains the expected property values of the stack resource, as defined in the stack template and any values specified as template parameters.</p>
    /// <p>For resources whose <code>StackResourceDriftStatus</code> is <code>DELETED</code>, this structure will not be present.</p>
    pub fn get_expected_properties(&self) -> &::std::option::Option<::std::string::String> {
        &self.expected_properties
    }
    /// <p>A JSON structure that contains the actual property values of the stack resource.</p>
    /// <p>For resources whose <code>StackResourceDriftStatus</code> is <code>DELETED</code>, this structure will not be present.</p>
    pub fn actual_properties(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.actual_properties = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A JSON structure that contains the actual property values of the stack resource.</p>
    /// <p>For resources whose <code>StackResourceDriftStatus</code> is <code>DELETED</code>, this structure will not be present.</p>
    pub fn set_actual_properties(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.actual_properties = input;
        self
    }
    /// <p>A JSON structure that contains the actual property values of the stack resource.</p>
    /// <p>For resources whose <code>StackResourceDriftStatus</code> is <code>DELETED</code>, this structure will not be present.</p>
    pub fn get_actual_properties(&self) -> &::std::option::Option<::std::string::String> {
        &self.actual_properties
    }
    /// Appends an item to `property_differences`.
    ///
    /// To override the contents of this collection use [`set_property_differences`](Self::set_property_differences).
    ///
    /// <p>A collection of the resource properties whose actual values differ from their expected values. These will be present only for resources whose <code>StackResourceDriftStatus</code> is <code>MODIFIED</code>.</p>
    pub fn property_differences(mut self, input: crate::types::PropertyDifference) -> Self {
        let mut v = self.property_differences.unwrap_or_default();
        v.push(input);
        self.property_differences = ::std::option::Option::Some(v);
        self
    }
    /// <p>A collection of the resource properties whose actual values differ from their expected values. These will be present only for resources whose <code>StackResourceDriftStatus</code> is <code>MODIFIED</code>.</p>
    pub fn set_property_differences(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::PropertyDifference>>) -> Self {
        self.property_differences = input;
        self
    }
    /// <p>A collection of the resource properties whose actual values differ from their expected values. These will be present only for resources whose <code>StackResourceDriftStatus</code> is <code>MODIFIED</code>.</p>
    pub fn get_property_differences(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::PropertyDifference>> {
        &self.property_differences
    }
    /// <p>Status of the resource's actual configuration compared to its expected configuration.</p>
    /// <ul>
    /// <li>
    /// <p><code>DELETED</code>: The resource differs from its expected template configuration because the resource has been deleted.</p></li>
    /// <li>
    /// <p><code>MODIFIED</code>: One or more resource properties differ from their expected values (as defined in the stack template and any values specified as template parameters).</p></li>
    /// <li>
    /// <p><code>IN_SYNC</code>: The resource's actual configuration matches its expected template configuration.</p></li>
    /// <li>
    /// <p><code>NOT_CHECKED</code>: CloudFormation does not currently return this value.</p></li>
    /// <li>
    /// <p><code>UNKNOWN</code>: CloudFormation could not run drift detection for the resource. See the <code>DriftStatusReason</code> for details.</p></li>
    /// </ul>
    /// This field is required.
    pub fn stack_resource_drift_status(mut self, input: crate::types::StackResourceDriftStatus) -> Self {
        self.stack_resource_drift_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>Status of the resource's actual configuration compared to its expected configuration.</p>
    /// <ul>
    /// <li>
    /// <p><code>DELETED</code>: The resource differs from its expected template configuration because the resource has been deleted.</p></li>
    /// <li>
    /// <p><code>MODIFIED</code>: One or more resource properties differ from their expected values (as defined in the stack template and any values specified as template parameters).</p></li>
    /// <li>
    /// <p><code>IN_SYNC</code>: The resource's actual configuration matches its expected template configuration.</p></li>
    /// <li>
    /// <p><code>NOT_CHECKED</code>: CloudFormation does not currently return this value.</p></li>
    /// <li>
    /// <p><code>UNKNOWN</code>: CloudFormation could not run drift detection for the resource. See the <code>DriftStatusReason</code> for details.</p></li>
    /// </ul>
    pub fn set_stack_resource_drift_status(mut self, input: ::std::option::Option<crate::types::StackResourceDriftStatus>) -> Self {
        self.stack_resource_drift_status = input;
        self
    }
    /// <p>Status of the resource's actual configuration compared to its expected configuration.</p>
    /// <ul>
    /// <li>
    /// <p><code>DELETED</code>: The resource differs from its expected template configuration because the resource has been deleted.</p></li>
    /// <li>
    /// <p><code>MODIFIED</code>: One or more resource properties differ from their expected values (as defined in the stack template and any values specified as template parameters).</p></li>
    /// <li>
    /// <p><code>IN_SYNC</code>: The resource's actual configuration matches its expected template configuration.</p></li>
    /// <li>
    /// <p><code>NOT_CHECKED</code>: CloudFormation does not currently return this value.</p></li>
    /// <li>
    /// <p><code>UNKNOWN</code>: CloudFormation could not run drift detection for the resource. See the <code>DriftStatusReason</code> for details.</p></li>
    /// </ul>
    pub fn get_stack_resource_drift_status(&self) -> &::std::option::Option<crate::types::StackResourceDriftStatus> {
        &self.stack_resource_drift_status
    }
    /// <p>Time at which CloudFormation performed drift detection on the stack resource.</p>
    /// This field is required.
    pub fn timestamp(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.timestamp = ::std::option::Option::Some(input);
        self
    }
    /// <p>Time at which CloudFormation performed drift detection on the stack resource.</p>
    pub fn set_timestamp(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.timestamp = input;
        self
    }
    /// <p>Time at which CloudFormation performed drift detection on the stack resource.</p>
    pub fn get_timestamp(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.timestamp
    }
    /// <p>Contains information about the module from which the resource was created, if the resource was created from a module included in the stack template.</p>
    pub fn module_info(mut self, input: crate::types::ModuleInfo) -> Self {
        self.module_info = ::std::option::Option::Some(input);
        self
    }
    /// <p>Contains information about the module from which the resource was created, if the resource was created from a module included in the stack template.</p>
    pub fn set_module_info(mut self, input: ::std::option::Option<crate::types::ModuleInfo>) -> Self {
        self.module_info = input;
        self
    }
    /// <p>Contains information about the module from which the resource was created, if the resource was created from a module included in the stack template.</p>
    pub fn get_module_info(&self) -> &::std::option::Option<crate::types::ModuleInfo> {
        &self.module_info
    }
    /// <p>The reason for the drift status.</p>
    pub fn drift_status_reason(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.drift_status_reason = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The reason for the drift status.</p>
    pub fn set_drift_status_reason(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.drift_status_reason = input;
        self
    }
    /// <p>The reason for the drift status.</p>
    pub fn get_drift_status_reason(&self) -> &::std::option::Option<::std::string::String> {
        &self.drift_status_reason
    }
    /// Consumes the builder and constructs a [`StackResourceDrift`](crate::types::StackResourceDrift).
    pub fn build(self) -> crate::types::StackResourceDrift {
        crate::types::StackResourceDrift {
            stack_id: self.stack_id,
            logical_resource_id: self.logical_resource_id,
            physical_resource_id: self.physical_resource_id,
            physical_resource_id_context: self.physical_resource_id_context,
            resource_type: self.resource_type,
            expected_properties: self.expected_properties,
            actual_properties: self.actual_properties,
            property_differences: self.property_differences,
            stack_resource_drift_status: self.stack_resource_drift_status,
            timestamp: self.timestamp,
            module_info: self.module_info,
            drift_status_reason: self.drift_status_reason,
        }
    }
}
