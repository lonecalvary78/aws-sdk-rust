// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about the restore for the table.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AwsDynamoDbTableRestoreSummary {
    /// <p>The ARN of the source backup from which the table was restored.</p>
    pub source_backup_arn: ::std::option::Option<::std::string::String>,
    /// <p>The ARN of the source table for the backup.</p>
    pub source_table_arn: ::std::option::Option<::std::string::String>,
    /// <p>Indicates the point in time that the table was restored to.</p>
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
    pub restore_date_time: ::std::option::Option<::std::string::String>,
    /// <p>Whether a restore is currently in progress.</p>
    pub restore_in_progress: ::std::option::Option<bool>,
}
impl AwsDynamoDbTableRestoreSummary {
    /// <p>The ARN of the source backup from which the table was restored.</p>
    pub fn source_backup_arn(&self) -> ::std::option::Option<&str> {
        self.source_backup_arn.as_deref()
    }
    /// <p>The ARN of the source table for the backup.</p>
    pub fn source_table_arn(&self) -> ::std::option::Option<&str> {
        self.source_table_arn.as_deref()
    }
    /// <p>Indicates the point in time that the table was restored to.</p>
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
    pub fn restore_date_time(&self) -> ::std::option::Option<&str> {
        self.restore_date_time.as_deref()
    }
    /// <p>Whether a restore is currently in progress.</p>
    pub fn restore_in_progress(&self) -> ::std::option::Option<bool> {
        self.restore_in_progress
    }
}
impl AwsDynamoDbTableRestoreSummary {
    /// Creates a new builder-style object to manufacture [`AwsDynamoDbTableRestoreSummary`](crate::types::AwsDynamoDbTableRestoreSummary).
    pub fn builder() -> crate::types::builders::AwsDynamoDbTableRestoreSummaryBuilder {
        crate::types::builders::AwsDynamoDbTableRestoreSummaryBuilder::default()
    }
}

/// A builder for [`AwsDynamoDbTableRestoreSummary`](crate::types::AwsDynamoDbTableRestoreSummary).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct AwsDynamoDbTableRestoreSummaryBuilder {
    pub(crate) source_backup_arn: ::std::option::Option<::std::string::String>,
    pub(crate) source_table_arn: ::std::option::Option<::std::string::String>,
    pub(crate) restore_date_time: ::std::option::Option<::std::string::String>,
    pub(crate) restore_in_progress: ::std::option::Option<bool>,
}
impl AwsDynamoDbTableRestoreSummaryBuilder {
    /// <p>The ARN of the source backup from which the table was restored.</p>
    pub fn source_backup_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.source_backup_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the source backup from which the table was restored.</p>
    pub fn set_source_backup_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.source_backup_arn = input;
        self
    }
    /// <p>The ARN of the source backup from which the table was restored.</p>
    pub fn get_source_backup_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.source_backup_arn
    }
    /// <p>The ARN of the source table for the backup.</p>
    pub fn source_table_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.source_table_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the source table for the backup.</p>
    pub fn set_source_table_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.source_table_arn = input;
        self
    }
    /// <p>The ARN of the source table for the backup.</p>
    pub fn get_source_table_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.source_table_arn
    }
    /// <p>Indicates the point in time that the table was restored to.</p>
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
    pub fn restore_date_time(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.restore_date_time = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Indicates the point in time that the table was restored to.</p>
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
    pub fn set_restore_date_time(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.restore_date_time = input;
        self
    }
    /// <p>Indicates the point in time that the table was restored to.</p>
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
    pub fn get_restore_date_time(&self) -> &::std::option::Option<::std::string::String> {
        &self.restore_date_time
    }
    /// <p>Whether a restore is currently in progress.</p>
    pub fn restore_in_progress(mut self, input: bool) -> Self {
        self.restore_in_progress = ::std::option::Option::Some(input);
        self
    }
    /// <p>Whether a restore is currently in progress.</p>
    pub fn set_restore_in_progress(mut self, input: ::std::option::Option<bool>) -> Self {
        self.restore_in_progress = input;
        self
    }
    /// <p>Whether a restore is currently in progress.</p>
    pub fn get_restore_in_progress(&self) -> &::std::option::Option<bool> {
        &self.restore_in_progress
    }
    /// Consumes the builder and constructs a [`AwsDynamoDbTableRestoreSummary`](crate::types::AwsDynamoDbTableRestoreSummary).
    pub fn build(self) -> crate::types::AwsDynamoDbTableRestoreSummary {
        crate::types::AwsDynamoDbTableRestoreSummary {
            source_backup_arn: self.source_backup_arn,
            source_table_arn: self.source_table_arn,
            restore_date_time: self.restore_date_time,
            restore_in_progress: self.restore_in_progress,
        }
    }
}
