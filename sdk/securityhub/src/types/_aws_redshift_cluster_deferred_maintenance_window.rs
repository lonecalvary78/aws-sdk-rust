// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A time windows during which maintenance was deferred for an Amazon Redshift cluster.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AwsRedshiftClusterDeferredMaintenanceWindow {
    /// <p>The end of the time window for which maintenance was deferred.</p>
    /// <p>This field accepts only the specified formats. Timestamps can end with <code>Z</code> or <code>("+" / "-") time-hour \[":" time-minute\]</code>. The time-secfrac after seconds is limited to a maximum of 9 digits. The offset is bounded by +/-18:00. Here are valid timestamp formats with examples:</p>
    /// <ul>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SSZ</code> (for example, <code>2019-01-31T23:00:00Z</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS.mmmmmmmmmZ</code> (for example, <code>2019-01-31T23:00:00.123456789Z</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS+HH:MM</code> (for example, <code>2024-01-04T15:25:10+17:59</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS-HHMM</code> (for example, <code>2024-01-04T15:25:10-1759</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS.mmmmmmmmm+HH:MM</code> (for example, <code>2024-01-04T15:25:10.123456789+17:59</code>)</p></li>
    /// </ul>
    pub defer_maintenance_end_time: ::std::option::Option<::std::string::String>,
    /// <p>The identifier of the maintenance window.</p>
    pub defer_maintenance_identifier: ::std::option::Option<::std::string::String>,
    /// <p>The start of the time window for which maintenance was deferred.</p>
    /// <p>This field accepts only the specified formats. Timestamps can end with <code>Z</code> or <code>("+" / "-") time-hour \[":" time-minute\]</code>. The time-secfrac after seconds is limited to a maximum of 9 digits. The offset is bounded by +/-18:00. Here are valid timestamp formats with examples:</p>
    /// <ul>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SSZ</code> (for example, <code>2019-01-31T23:00:00Z</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS.mmmmmmmmmZ</code> (for example, <code>2019-01-31T23:00:00.123456789Z</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS+HH:MM</code> (for example, <code>2024-01-04T15:25:10+17:59</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS-HHMM</code> (for example, <code>2024-01-04T15:25:10-1759</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS.mmmmmmmmm+HH:MM</code> (for example, <code>2024-01-04T15:25:10.123456789+17:59</code>)</p></li>
    /// </ul>
    pub defer_maintenance_start_time: ::std::option::Option<::std::string::String>,
}
impl AwsRedshiftClusterDeferredMaintenanceWindow {
    /// <p>The end of the time window for which maintenance was deferred.</p>
    /// <p>This field accepts only the specified formats. Timestamps can end with <code>Z</code> or <code>("+" / "-") time-hour \[":" time-minute\]</code>. The time-secfrac after seconds is limited to a maximum of 9 digits. The offset is bounded by +/-18:00. Here are valid timestamp formats with examples:</p>
    /// <ul>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SSZ</code> (for example, <code>2019-01-31T23:00:00Z</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS.mmmmmmmmmZ</code> (for example, <code>2019-01-31T23:00:00.123456789Z</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS+HH:MM</code> (for example, <code>2024-01-04T15:25:10+17:59</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS-HHMM</code> (for example, <code>2024-01-04T15:25:10-1759</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS.mmmmmmmmm+HH:MM</code> (for example, <code>2024-01-04T15:25:10.123456789+17:59</code>)</p></li>
    /// </ul>
    pub fn defer_maintenance_end_time(&self) -> ::std::option::Option<&str> {
        self.defer_maintenance_end_time.as_deref()
    }
    /// <p>The identifier of the maintenance window.</p>
    pub fn defer_maintenance_identifier(&self) -> ::std::option::Option<&str> {
        self.defer_maintenance_identifier.as_deref()
    }
    /// <p>The start of the time window for which maintenance was deferred.</p>
    /// <p>This field accepts only the specified formats. Timestamps can end with <code>Z</code> or <code>("+" / "-") time-hour \[":" time-minute\]</code>. The time-secfrac after seconds is limited to a maximum of 9 digits. The offset is bounded by +/-18:00. Here are valid timestamp formats with examples:</p>
    /// <ul>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SSZ</code> (for example, <code>2019-01-31T23:00:00Z</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS.mmmmmmmmmZ</code> (for example, <code>2019-01-31T23:00:00.123456789Z</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS+HH:MM</code> (for example, <code>2024-01-04T15:25:10+17:59</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS-HHMM</code> (for example, <code>2024-01-04T15:25:10-1759</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS.mmmmmmmmm+HH:MM</code> (for example, <code>2024-01-04T15:25:10.123456789+17:59</code>)</p></li>
    /// </ul>
    pub fn defer_maintenance_start_time(&self) -> ::std::option::Option<&str> {
        self.defer_maintenance_start_time.as_deref()
    }
}
impl AwsRedshiftClusterDeferredMaintenanceWindow {
    /// Creates a new builder-style object to manufacture [`AwsRedshiftClusterDeferredMaintenanceWindow`](crate::types::AwsRedshiftClusterDeferredMaintenanceWindow).
    pub fn builder() -> crate::types::builders::AwsRedshiftClusterDeferredMaintenanceWindowBuilder {
        crate::types::builders::AwsRedshiftClusterDeferredMaintenanceWindowBuilder::default()
    }
}

/// A builder for [`AwsRedshiftClusterDeferredMaintenanceWindow`](crate::types::AwsRedshiftClusterDeferredMaintenanceWindow).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct AwsRedshiftClusterDeferredMaintenanceWindowBuilder {
    pub(crate) defer_maintenance_end_time: ::std::option::Option<::std::string::String>,
    pub(crate) defer_maintenance_identifier: ::std::option::Option<::std::string::String>,
    pub(crate) defer_maintenance_start_time: ::std::option::Option<::std::string::String>,
}
impl AwsRedshiftClusterDeferredMaintenanceWindowBuilder {
    /// <p>The end of the time window for which maintenance was deferred.</p>
    /// <p>This field accepts only the specified formats. Timestamps can end with <code>Z</code> or <code>("+" / "-") time-hour \[":" time-minute\]</code>. The time-secfrac after seconds is limited to a maximum of 9 digits. The offset is bounded by +/-18:00. Here are valid timestamp formats with examples:</p>
    /// <ul>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SSZ</code> (for example, <code>2019-01-31T23:00:00Z</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS.mmmmmmmmmZ</code> (for example, <code>2019-01-31T23:00:00.123456789Z</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS+HH:MM</code> (for example, <code>2024-01-04T15:25:10+17:59</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS-HHMM</code> (for example, <code>2024-01-04T15:25:10-1759</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS.mmmmmmmmm+HH:MM</code> (for example, <code>2024-01-04T15:25:10.123456789+17:59</code>)</p></li>
    /// </ul>
    pub fn defer_maintenance_end_time(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.defer_maintenance_end_time = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The end of the time window for which maintenance was deferred.</p>
    /// <p>This field accepts only the specified formats. Timestamps can end with <code>Z</code> or <code>("+" / "-") time-hour \[":" time-minute\]</code>. The time-secfrac after seconds is limited to a maximum of 9 digits. The offset is bounded by +/-18:00. Here are valid timestamp formats with examples:</p>
    /// <ul>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SSZ</code> (for example, <code>2019-01-31T23:00:00Z</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS.mmmmmmmmmZ</code> (for example, <code>2019-01-31T23:00:00.123456789Z</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS+HH:MM</code> (for example, <code>2024-01-04T15:25:10+17:59</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS-HHMM</code> (for example, <code>2024-01-04T15:25:10-1759</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS.mmmmmmmmm+HH:MM</code> (for example, <code>2024-01-04T15:25:10.123456789+17:59</code>)</p></li>
    /// </ul>
    pub fn set_defer_maintenance_end_time(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.defer_maintenance_end_time = input;
        self
    }
    /// <p>The end of the time window for which maintenance was deferred.</p>
    /// <p>This field accepts only the specified formats. Timestamps can end with <code>Z</code> or <code>("+" / "-") time-hour \[":" time-minute\]</code>. The time-secfrac after seconds is limited to a maximum of 9 digits. The offset is bounded by +/-18:00. Here are valid timestamp formats with examples:</p>
    /// <ul>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SSZ</code> (for example, <code>2019-01-31T23:00:00Z</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS.mmmmmmmmmZ</code> (for example, <code>2019-01-31T23:00:00.123456789Z</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS+HH:MM</code> (for example, <code>2024-01-04T15:25:10+17:59</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS-HHMM</code> (for example, <code>2024-01-04T15:25:10-1759</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS.mmmmmmmmm+HH:MM</code> (for example, <code>2024-01-04T15:25:10.123456789+17:59</code>)</p></li>
    /// </ul>
    pub fn get_defer_maintenance_end_time(&self) -> &::std::option::Option<::std::string::String> {
        &self.defer_maintenance_end_time
    }
    /// <p>The identifier of the maintenance window.</p>
    pub fn defer_maintenance_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.defer_maintenance_identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the maintenance window.</p>
    pub fn set_defer_maintenance_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.defer_maintenance_identifier = input;
        self
    }
    /// <p>The identifier of the maintenance window.</p>
    pub fn get_defer_maintenance_identifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.defer_maintenance_identifier
    }
    /// <p>The start of the time window for which maintenance was deferred.</p>
    /// <p>This field accepts only the specified formats. Timestamps can end with <code>Z</code> or <code>("+" / "-") time-hour \[":" time-minute\]</code>. The time-secfrac after seconds is limited to a maximum of 9 digits. The offset is bounded by +/-18:00. Here are valid timestamp formats with examples:</p>
    /// <ul>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SSZ</code> (for example, <code>2019-01-31T23:00:00Z</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS.mmmmmmmmmZ</code> (for example, <code>2019-01-31T23:00:00.123456789Z</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS+HH:MM</code> (for example, <code>2024-01-04T15:25:10+17:59</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS-HHMM</code> (for example, <code>2024-01-04T15:25:10-1759</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS.mmmmmmmmm+HH:MM</code> (for example, <code>2024-01-04T15:25:10.123456789+17:59</code>)</p></li>
    /// </ul>
    pub fn defer_maintenance_start_time(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.defer_maintenance_start_time = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The start of the time window for which maintenance was deferred.</p>
    /// <p>This field accepts only the specified formats. Timestamps can end with <code>Z</code> or <code>("+" / "-") time-hour \[":" time-minute\]</code>. The time-secfrac after seconds is limited to a maximum of 9 digits. The offset is bounded by +/-18:00. Here are valid timestamp formats with examples:</p>
    /// <ul>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SSZ</code> (for example, <code>2019-01-31T23:00:00Z</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS.mmmmmmmmmZ</code> (for example, <code>2019-01-31T23:00:00.123456789Z</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS+HH:MM</code> (for example, <code>2024-01-04T15:25:10+17:59</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS-HHMM</code> (for example, <code>2024-01-04T15:25:10-1759</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS.mmmmmmmmm+HH:MM</code> (for example, <code>2024-01-04T15:25:10.123456789+17:59</code>)</p></li>
    /// </ul>
    pub fn set_defer_maintenance_start_time(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.defer_maintenance_start_time = input;
        self
    }
    /// <p>The start of the time window for which maintenance was deferred.</p>
    /// <p>This field accepts only the specified formats. Timestamps can end with <code>Z</code> or <code>("+" / "-") time-hour \[":" time-minute\]</code>. The time-secfrac after seconds is limited to a maximum of 9 digits. The offset is bounded by +/-18:00. Here are valid timestamp formats with examples:</p>
    /// <ul>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SSZ</code> (for example, <code>2019-01-31T23:00:00Z</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS.mmmmmmmmmZ</code> (for example, <code>2019-01-31T23:00:00.123456789Z</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS+HH:MM</code> (for example, <code>2024-01-04T15:25:10+17:59</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS-HHMM</code> (for example, <code>2024-01-04T15:25:10-1759</code>)</p></li>
    /// <li>
    /// <p><code>YYYY-MM-DDTHH:MM:SS.mmmmmmmmm+HH:MM</code> (for example, <code>2024-01-04T15:25:10.123456789+17:59</code>)</p></li>
    /// </ul>
    pub fn get_defer_maintenance_start_time(&self) -> &::std::option::Option<::std::string::String> {
        &self.defer_maintenance_start_time
    }
    /// Consumes the builder and constructs a [`AwsRedshiftClusterDeferredMaintenanceWindow`](crate::types::AwsRedshiftClusterDeferredMaintenanceWindow).
    pub fn build(self) -> crate::types::AwsRedshiftClusterDeferredMaintenanceWindow {
        crate::types::AwsRedshiftClusterDeferredMaintenanceWindow {
            defer_maintenance_end_time: self.defer_maintenance_end_time,
            defer_maintenance_identifier: self.defer_maintenance_identifier,
            defer_maintenance_start_time: self.defer_maintenance_start_time,
        }
    }
}
