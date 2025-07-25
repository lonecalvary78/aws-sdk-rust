// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_instance_types::_describe_instance_types_output::DescribeInstanceTypesOutputBuilder;

pub use crate::operation::describe_instance_types::_describe_instance_types_input::DescribeInstanceTypesInputBuilder;

impl crate::operation::describe_instance_types::builders::DescribeInstanceTypesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_instance_types::DescribeInstanceTypesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_instance_types::DescribeInstanceTypesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_instance_types();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeInstanceTypes`.
///
/// <p>Describes the specified instance types. By default, all instance types for the current Region are described. Alternatively, you can filter the results.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeInstanceTypesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_instance_types::builders::DescribeInstanceTypesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_instance_types::DescribeInstanceTypesOutput,
        crate::operation::describe_instance_types::DescribeInstanceTypesError,
    > for DescribeInstanceTypesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_instance_types::DescribeInstanceTypesOutput,
            crate::operation::describe_instance_types::DescribeInstanceTypesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeInstanceTypesFluentBuilder {
    /// Creates a new `DescribeInstanceTypesFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeInstanceTypes as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_instance_types::builders::DescribeInstanceTypesInputBuilder {
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
        crate::operation::describe_instance_types::DescribeInstanceTypesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_instance_types::DescribeInstanceTypesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_instance_types::DescribeInstanceTypes::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_instance_types::DescribeInstanceTypes::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_instance_types::DescribeInstanceTypesOutput,
        crate::operation::describe_instance_types::DescribeInstanceTypesError,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::describe_instance_types::paginator::DescribeInstanceTypesPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::describe_instance_types::paginator::DescribeInstanceTypesPaginator {
        crate::operation::describe_instance_types::paginator::DescribeInstanceTypesPaginator::new(self.handle, self.inner)
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
    ///
    /// Appends an item to `InstanceTypes`.
    ///
    /// To override the contents of this collection use [`set_instance_types`](Self::set_instance_types).
    ///
    /// <p>The instance types.</p>
    pub fn instance_types(mut self, input: crate::types::InstanceType) -> Self {
        self.inner = self.inner.instance_types(input);
        self
    }
    /// <p>The instance types.</p>
    pub fn set_instance_types(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::InstanceType>>) -> Self {
        self.inner = self.inner.set_instance_types(input);
        self
    }
    /// <p>The instance types.</p>
    pub fn get_instance_types(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::InstanceType>> {
        self.inner.get_instance_types()
    }
    ///
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>One or more filters. Filter names and values are case-sensitive.</p>
    /// <ul>
    /// <li>
    /// <p><code>auto-recovery-supported</code> - Indicates whether Amazon CloudWatch action based recovery is supported (<code>true</code> | <code>false</code>).</p></li>
    /// <li>
    /// <p><code>bare-metal</code> - Indicates whether it is a bare metal instance type (<code>true</code> | <code>false</code>).</p></li>
    /// <li>
    /// <p><code>burstable-performance-supported</code> - Indicates whether the instance type is a burstable performance T instance type (<code>true</code> | <code>false</code>).</p></li>
    /// <li>
    /// <p><code>current-generation</code> - Indicates whether this instance type is the latest generation instance type of an instance family (<code>true</code> | <code>false</code>).</p></li>
    /// <li>
    /// <p><code>dedicated-hosts-supported</code> - Indicates whether the instance type supports Dedicated Hosts. (<code>true</code> | <code>false</code>)</p></li>
    /// <li>
    /// <p><code>ebs-info.ebs-optimized-info.baseline-bandwidth-in-mbps</code> - The baseline bandwidth performance for an EBS-optimized instance type, in Mbps.</p></li>
    /// <li>
    /// <p><code>ebs-info.ebs-optimized-info.baseline-iops</code> - The baseline input/output storage operations per second for an EBS-optimized instance type.</p></li>
    /// <li>
    /// <p><code>ebs-info.ebs-optimized-info.baseline-throughput-in-mbps</code> - The baseline throughput performance for an EBS-optimized instance type, in MB/s.</p></li>
    /// <li>
    /// <p><code>ebs-info.ebs-optimized-info.maximum-bandwidth-in-mbps</code> - The maximum bandwidth performance for an EBS-optimized instance type, in Mbps.</p></li>
    /// <li>
    /// <p><code>ebs-info.ebs-optimized-info.maximum-iops</code> - The maximum input/output storage operations per second for an EBS-optimized instance type.</p></li>
    /// <li>
    /// <p><code>ebs-info.ebs-optimized-info.maximum-throughput-in-mbps</code> - The maximum throughput performance for an EBS-optimized instance type, in MB/s.</p></li>
    /// <li>
    /// <p><code>ebs-info.ebs-optimized-support</code> - Indicates whether the instance type is EBS-optimized (<code>supported</code> | <code>unsupported</code> | <code>default</code>).</p></li>
    /// <li>
    /// <p><code>ebs-info.encryption-support</code> - Indicates whether EBS encryption is supported (<code>supported</code> | <code>unsupported</code>).</p></li>
    /// <li>
    /// <p><code>ebs-info.nvme-support</code> - Indicates whether non-volatile memory express (NVMe) is supported for EBS volumes (<code>required</code> | <code>supported</code> | <code>unsupported</code>).</p></li>
    /// <li>
    /// <p><code>free-tier-eligible</code> - A Boolean that indicates whether this instance type can be used under the Amazon Web Services Free Tier (<code>true</code> | <code>false</code>).</p></li>
    /// <li>
    /// <p><code>hibernation-supported</code> - Indicates whether On-Demand hibernation is supported (<code>true</code> | <code>false</code>).</p></li>
    /// <li>
    /// <p><code>hypervisor</code> - The hypervisor (<code>nitro</code> | <code>xen</code>).</p></li>
    /// <li>
    /// <p><code>instance-storage-info.disk.count</code> - The number of local disks.</p></li>
    /// <li>
    /// <p><code>instance-storage-info.disk.size-in-gb</code> - The storage size of each instance storage disk, in GB.</p></li>
    /// <li>
    /// <p><code>instance-storage-info.disk.type</code> - The storage technology for the local instance storage disks (<code>hdd</code> | <code>ssd</code>).</p></li>
    /// <li>
    /// <p><code>instance-storage-info.encryption-support</code> - Indicates whether data is encrypted at rest (<code>required</code> | <code>supported</code> | <code>unsupported</code>).</p></li>
    /// <li>
    /// <p><code>instance-storage-info.nvme-support</code> - Indicates whether non-volatile memory express (NVMe) is supported for instance store (<code>required</code> | <code>supported</code> | <code>unsupported</code>).</p></li>
    /// <li>
    /// <p><code>instance-storage-info.total-size-in-gb</code> - The total amount of storage available from all local instance storage, in GB.</p></li>
    /// <li>
    /// <p><code>instance-storage-supported</code> - Indicates whether the instance type has local instance storage (<code>true</code> | <code>false</code>).</p></li>
    /// <li>
    /// <p><code>instance-type</code> - The instance type (for example <code>c5.2xlarge</code> or c5*).</p></li>
    /// <li>
    /// <p><code>memory-info.size-in-mib</code> - The memory size.</p></li>
    /// <li>
    /// <p><code>network-info.bandwidth-weightings</code> - For instances that support bandwidth weighting to boost performance (<code>default</code>, <code>vpc-1</code>, <code>ebs-1</code>).</p></li>
    /// <li>
    /// <p><code>network-info.efa-info.maximum-efa-interfaces</code> - The maximum number of Elastic Fabric Adapters (EFAs) per instance.</p></li>
    /// <li>
    /// <p><code>network-info.efa-supported</code> - Indicates whether the instance type supports Elastic Fabric Adapter (EFA) (<code>true</code> | <code>false</code>).</p></li>
    /// <li>
    /// <p><code>network-info.ena-support</code> - Indicates whether Elastic Network Adapter (ENA) is supported or required (<code>required</code> | <code>supported</code> | <code>unsupported</code>).</p></li>
    /// <li>
    /// <p><code>network-info.flexible-ena-queues-support</code> - Indicates whether an instance supports flexible ENA queues (<code>supported</code> | <code>unsupported</code>).</p></li>
    /// <li>
    /// <p><code>network-info.encryption-in-transit-supported</code> - Indicates whether the instance type automatically encrypts in-transit traffic between instances (<code>true</code> | <code>false</code>).</p></li>
    /// <li>
    /// <p><code>network-info.ipv4-addresses-per-interface</code> - The maximum number of private IPv4 addresses per network interface.</p></li>
    /// <li>
    /// <p><code>network-info.ipv6-addresses-per-interface</code> - The maximum number of private IPv6 addresses per network interface.</p></li>
    /// <li>
    /// <p><code>network-info.ipv6-supported</code> - Indicates whether the instance type supports IPv6 (<code>true</code> | <code>false</code>).</p></li>
    /// <li>
    /// <p><code>network-info.maximum-network-cards</code> - The maximum number of network cards per instance.</p></li>
    /// <li>
    /// <p><code>network-info.maximum-network-interfaces</code> - The maximum number of network interfaces per instance.</p></li>
    /// <li>
    /// <p><code>network-info.network-performance</code> - The network performance (for example, "25 Gigabit").</p></li>
    /// <li>
    /// <p><code>nitro-enclaves-support</code> - Indicates whether Nitro Enclaves is supported (<code>supported</code> | <code>unsupported</code>).</p></li>
    /// <li>
    /// <p><code>nitro-tpm-support</code> - Indicates whether NitroTPM is supported (<code>supported</code> | <code>unsupported</code>).</p></li>
    /// <li>
    /// <p><code>nitro-tpm-info.supported-versions</code> - The supported NitroTPM version (<code>2.0</code>).</p></li>
    /// <li>
    /// <p><code>processor-info.supported-architecture</code> - The CPU architecture (<code>arm64</code> | <code>i386</code> | <code>x86_64</code>).</p></li>
    /// <li>
    /// <p><code>processor-info.sustained-clock-speed-in-ghz</code> - The CPU clock speed, in GHz.</p></li>
    /// <li>
    /// <p><code>processor-info.supported-features</code> - The supported CPU features (<code>amd-sev-snp</code>).</p></li>
    /// <li>
    /// <p><code>reboot-migration-support</code> - Indicates whether enabling reboot migration is supported (<code>supported</code> | <code>unsupported</code>).</p></li>
    /// <li>
    /// <p><code>supported-boot-mode</code> - The boot mode (<code>legacy-bios</code> | <code>uefi</code>).</p></li>
    /// <li>
    /// <p><code>supported-root-device-type</code> - The root device type (<code>ebs</code> | <code>instance-store</code>).</p></li>
    /// <li>
    /// <p><code>supported-usage-class</code> - The usage class (<code>on-demand</code> | <code>spot</code> | <code>capacity-block</code>).</p></li>
    /// <li>
    /// <p><code>supported-virtualization-type</code> - The virtualization type (<code>hvm</code> | <code>paravirtual</code>).</p></li>
    /// <li>
    /// <p><code>vcpu-info.default-cores</code> - The default number of cores for the instance type.</p></li>
    /// <li>
    /// <p><code>vcpu-info.default-threads-per-core</code> - The default number of threads per core for the instance type.</p></li>
    /// <li>
    /// <p><code>vcpu-info.default-vcpus</code> - The default number of vCPUs for the instance type.</p></li>
    /// <li>
    /// <p><code>vcpu-info.valid-cores</code> - The number of cores that can be configured for the instance type.</p></li>
    /// <li>
    /// <p><code>vcpu-info.valid-threads-per-core</code> - The number of threads per core that can be configured for the instance type. For example, "1" or "1,2".</p></li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>One or more filters. Filter names and values are case-sensitive.</p>
    /// <ul>
    /// <li>
    /// <p><code>auto-recovery-supported</code> - Indicates whether Amazon CloudWatch action based recovery is supported (<code>true</code> | <code>false</code>).</p></li>
    /// <li>
    /// <p><code>bare-metal</code> - Indicates whether it is a bare metal instance type (<code>true</code> | <code>false</code>).</p></li>
    /// <li>
    /// <p><code>burstable-performance-supported</code> - Indicates whether the instance type is a burstable performance T instance type (<code>true</code> | <code>false</code>).</p></li>
    /// <li>
    /// <p><code>current-generation</code> - Indicates whether this instance type is the latest generation instance type of an instance family (<code>true</code> | <code>false</code>).</p></li>
    /// <li>
    /// <p><code>dedicated-hosts-supported</code> - Indicates whether the instance type supports Dedicated Hosts. (<code>true</code> | <code>false</code>)</p></li>
    /// <li>
    /// <p><code>ebs-info.ebs-optimized-info.baseline-bandwidth-in-mbps</code> - The baseline bandwidth performance for an EBS-optimized instance type, in Mbps.</p></li>
    /// <li>
    /// <p><code>ebs-info.ebs-optimized-info.baseline-iops</code> - The baseline input/output storage operations per second for an EBS-optimized instance type.</p></li>
    /// <li>
    /// <p><code>ebs-info.ebs-optimized-info.baseline-throughput-in-mbps</code> - The baseline throughput performance for an EBS-optimized instance type, in MB/s.</p></li>
    /// <li>
    /// <p><code>ebs-info.ebs-optimized-info.maximum-bandwidth-in-mbps</code> - The maximum bandwidth performance for an EBS-optimized instance type, in Mbps.</p></li>
    /// <li>
    /// <p><code>ebs-info.ebs-optimized-info.maximum-iops</code> - The maximum input/output storage operations per second for an EBS-optimized instance type.</p></li>
    /// <li>
    /// <p><code>ebs-info.ebs-optimized-info.maximum-throughput-in-mbps</code> - The maximum throughput performance for an EBS-optimized instance type, in MB/s.</p></li>
    /// <li>
    /// <p><code>ebs-info.ebs-optimized-support</code> - Indicates whether the instance type is EBS-optimized (<code>supported</code> | <code>unsupported</code> | <code>default</code>).</p></li>
    /// <li>
    /// <p><code>ebs-info.encryption-support</code> - Indicates whether EBS encryption is supported (<code>supported</code> | <code>unsupported</code>).</p></li>
    /// <li>
    /// <p><code>ebs-info.nvme-support</code> - Indicates whether non-volatile memory express (NVMe) is supported for EBS volumes (<code>required</code> | <code>supported</code> | <code>unsupported</code>).</p></li>
    /// <li>
    /// <p><code>free-tier-eligible</code> - A Boolean that indicates whether this instance type can be used under the Amazon Web Services Free Tier (<code>true</code> | <code>false</code>).</p></li>
    /// <li>
    /// <p><code>hibernation-supported</code> - Indicates whether On-Demand hibernation is supported (<code>true</code> | <code>false</code>).</p></li>
    /// <li>
    /// <p><code>hypervisor</code> - The hypervisor (<code>nitro</code> | <code>xen</code>).</p></li>
    /// <li>
    /// <p><code>instance-storage-info.disk.count</code> - The number of local disks.</p></li>
    /// <li>
    /// <p><code>instance-storage-info.disk.size-in-gb</code> - The storage size of each instance storage disk, in GB.</p></li>
    /// <li>
    /// <p><code>instance-storage-info.disk.type</code> - The storage technology for the local instance storage disks (<code>hdd</code> | <code>ssd</code>).</p></li>
    /// <li>
    /// <p><code>instance-storage-info.encryption-support</code> - Indicates whether data is encrypted at rest (<code>required</code> | <code>supported</code> | <code>unsupported</code>).</p></li>
    /// <li>
    /// <p><code>instance-storage-info.nvme-support</code> - Indicates whether non-volatile memory express (NVMe) is supported for instance store (<code>required</code> | <code>supported</code> | <code>unsupported</code>).</p></li>
    /// <li>
    /// <p><code>instance-storage-info.total-size-in-gb</code> - The total amount of storage available from all local instance storage, in GB.</p></li>
    /// <li>
    /// <p><code>instance-storage-supported</code> - Indicates whether the instance type has local instance storage (<code>true</code> | <code>false</code>).</p></li>
    /// <li>
    /// <p><code>instance-type</code> - The instance type (for example <code>c5.2xlarge</code> or c5*).</p></li>
    /// <li>
    /// <p><code>memory-info.size-in-mib</code> - The memory size.</p></li>
    /// <li>
    /// <p><code>network-info.bandwidth-weightings</code> - For instances that support bandwidth weighting to boost performance (<code>default</code>, <code>vpc-1</code>, <code>ebs-1</code>).</p></li>
    /// <li>
    /// <p><code>network-info.efa-info.maximum-efa-interfaces</code> - The maximum number of Elastic Fabric Adapters (EFAs) per instance.</p></li>
    /// <li>
    /// <p><code>network-info.efa-supported</code> - Indicates whether the instance type supports Elastic Fabric Adapter (EFA) (<code>true</code> | <code>false</code>).</p></li>
    /// <li>
    /// <p><code>network-info.ena-support</code> - Indicates whether Elastic Network Adapter (ENA) is supported or required (<code>required</code> | <code>supported</code> | <code>unsupported</code>).</p></li>
    /// <li>
    /// <p><code>network-info.flexible-ena-queues-support</code> - Indicates whether an instance supports flexible ENA queues (<code>supported</code> | <code>unsupported</code>).</p></li>
    /// <li>
    /// <p><code>network-info.encryption-in-transit-supported</code> - Indicates whether the instance type automatically encrypts in-transit traffic between instances (<code>true</code> | <code>false</code>).</p></li>
    /// <li>
    /// <p><code>network-info.ipv4-addresses-per-interface</code> - The maximum number of private IPv4 addresses per network interface.</p></li>
    /// <li>
    /// <p><code>network-info.ipv6-addresses-per-interface</code> - The maximum number of private IPv6 addresses per network interface.</p></li>
    /// <li>
    /// <p><code>network-info.ipv6-supported</code> - Indicates whether the instance type supports IPv6 (<code>true</code> | <code>false</code>).</p></li>
    /// <li>
    /// <p><code>network-info.maximum-network-cards</code> - The maximum number of network cards per instance.</p></li>
    /// <li>
    /// <p><code>network-info.maximum-network-interfaces</code> - The maximum number of network interfaces per instance.</p></li>
    /// <li>
    /// <p><code>network-info.network-performance</code> - The network performance (for example, "25 Gigabit").</p></li>
    /// <li>
    /// <p><code>nitro-enclaves-support</code> - Indicates whether Nitro Enclaves is supported (<code>supported</code> | <code>unsupported</code>).</p></li>
    /// <li>
    /// <p><code>nitro-tpm-support</code> - Indicates whether NitroTPM is supported (<code>supported</code> | <code>unsupported</code>).</p></li>
    /// <li>
    /// <p><code>nitro-tpm-info.supported-versions</code> - The supported NitroTPM version (<code>2.0</code>).</p></li>
    /// <li>
    /// <p><code>processor-info.supported-architecture</code> - The CPU architecture (<code>arm64</code> | <code>i386</code> | <code>x86_64</code>).</p></li>
    /// <li>
    /// <p><code>processor-info.sustained-clock-speed-in-ghz</code> - The CPU clock speed, in GHz.</p></li>
    /// <li>
    /// <p><code>processor-info.supported-features</code> - The supported CPU features (<code>amd-sev-snp</code>).</p></li>
    /// <li>
    /// <p><code>reboot-migration-support</code> - Indicates whether enabling reboot migration is supported (<code>supported</code> | <code>unsupported</code>).</p></li>
    /// <li>
    /// <p><code>supported-boot-mode</code> - The boot mode (<code>legacy-bios</code> | <code>uefi</code>).</p></li>
    /// <li>
    /// <p><code>supported-root-device-type</code> - The root device type (<code>ebs</code> | <code>instance-store</code>).</p></li>
    /// <li>
    /// <p><code>supported-usage-class</code> - The usage class (<code>on-demand</code> | <code>spot</code> | <code>capacity-block</code>).</p></li>
    /// <li>
    /// <p><code>supported-virtualization-type</code> - The virtualization type (<code>hvm</code> | <code>paravirtual</code>).</p></li>
    /// <li>
    /// <p><code>vcpu-info.default-cores</code> - The default number of cores for the instance type.</p></li>
    /// <li>
    /// <p><code>vcpu-info.default-threads-per-core</code> - The default number of threads per core for the instance type.</p></li>
    /// <li>
    /// <p><code>vcpu-info.default-vcpus</code> - The default number of vCPUs for the instance type.</p></li>
    /// <li>
    /// <p><code>vcpu-info.valid-cores</code> - The number of cores that can be configured for the instance type.</p></li>
    /// <li>
    /// <p><code>vcpu-info.valid-threads-per-core</code> - The number of threads per core that can be configured for the instance type. For example, "1" or "1,2".</p></li>
    /// </ul>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>One or more filters. Filter names and values are case-sensitive.</p>
    /// <ul>
    /// <li>
    /// <p><code>auto-recovery-supported</code> - Indicates whether Amazon CloudWatch action based recovery is supported (<code>true</code> | <code>false</code>).</p></li>
    /// <li>
    /// <p><code>bare-metal</code> - Indicates whether it is a bare metal instance type (<code>true</code> | <code>false</code>).</p></li>
    /// <li>
    /// <p><code>burstable-performance-supported</code> - Indicates whether the instance type is a burstable performance T instance type (<code>true</code> | <code>false</code>).</p></li>
    /// <li>
    /// <p><code>current-generation</code> - Indicates whether this instance type is the latest generation instance type of an instance family (<code>true</code> | <code>false</code>).</p></li>
    /// <li>
    /// <p><code>dedicated-hosts-supported</code> - Indicates whether the instance type supports Dedicated Hosts. (<code>true</code> | <code>false</code>)</p></li>
    /// <li>
    /// <p><code>ebs-info.ebs-optimized-info.baseline-bandwidth-in-mbps</code> - The baseline bandwidth performance for an EBS-optimized instance type, in Mbps.</p></li>
    /// <li>
    /// <p><code>ebs-info.ebs-optimized-info.baseline-iops</code> - The baseline input/output storage operations per second for an EBS-optimized instance type.</p></li>
    /// <li>
    /// <p><code>ebs-info.ebs-optimized-info.baseline-throughput-in-mbps</code> - The baseline throughput performance for an EBS-optimized instance type, in MB/s.</p></li>
    /// <li>
    /// <p><code>ebs-info.ebs-optimized-info.maximum-bandwidth-in-mbps</code> - The maximum bandwidth performance for an EBS-optimized instance type, in Mbps.</p></li>
    /// <li>
    /// <p><code>ebs-info.ebs-optimized-info.maximum-iops</code> - The maximum input/output storage operations per second for an EBS-optimized instance type.</p></li>
    /// <li>
    /// <p><code>ebs-info.ebs-optimized-info.maximum-throughput-in-mbps</code> - The maximum throughput performance for an EBS-optimized instance type, in MB/s.</p></li>
    /// <li>
    /// <p><code>ebs-info.ebs-optimized-support</code> - Indicates whether the instance type is EBS-optimized (<code>supported</code> | <code>unsupported</code> | <code>default</code>).</p></li>
    /// <li>
    /// <p><code>ebs-info.encryption-support</code> - Indicates whether EBS encryption is supported (<code>supported</code> | <code>unsupported</code>).</p></li>
    /// <li>
    /// <p><code>ebs-info.nvme-support</code> - Indicates whether non-volatile memory express (NVMe) is supported for EBS volumes (<code>required</code> | <code>supported</code> | <code>unsupported</code>).</p></li>
    /// <li>
    /// <p><code>free-tier-eligible</code> - A Boolean that indicates whether this instance type can be used under the Amazon Web Services Free Tier (<code>true</code> | <code>false</code>).</p></li>
    /// <li>
    /// <p><code>hibernation-supported</code> - Indicates whether On-Demand hibernation is supported (<code>true</code> | <code>false</code>).</p></li>
    /// <li>
    /// <p><code>hypervisor</code> - The hypervisor (<code>nitro</code> | <code>xen</code>).</p></li>
    /// <li>
    /// <p><code>instance-storage-info.disk.count</code> - The number of local disks.</p></li>
    /// <li>
    /// <p><code>instance-storage-info.disk.size-in-gb</code> - The storage size of each instance storage disk, in GB.</p></li>
    /// <li>
    /// <p><code>instance-storage-info.disk.type</code> - The storage technology for the local instance storage disks (<code>hdd</code> | <code>ssd</code>).</p></li>
    /// <li>
    /// <p><code>instance-storage-info.encryption-support</code> - Indicates whether data is encrypted at rest (<code>required</code> | <code>supported</code> | <code>unsupported</code>).</p></li>
    /// <li>
    /// <p><code>instance-storage-info.nvme-support</code> - Indicates whether non-volatile memory express (NVMe) is supported for instance store (<code>required</code> | <code>supported</code> | <code>unsupported</code>).</p></li>
    /// <li>
    /// <p><code>instance-storage-info.total-size-in-gb</code> - The total amount of storage available from all local instance storage, in GB.</p></li>
    /// <li>
    /// <p><code>instance-storage-supported</code> - Indicates whether the instance type has local instance storage (<code>true</code> | <code>false</code>).</p></li>
    /// <li>
    /// <p><code>instance-type</code> - The instance type (for example <code>c5.2xlarge</code> or c5*).</p></li>
    /// <li>
    /// <p><code>memory-info.size-in-mib</code> - The memory size.</p></li>
    /// <li>
    /// <p><code>network-info.bandwidth-weightings</code> - For instances that support bandwidth weighting to boost performance (<code>default</code>, <code>vpc-1</code>, <code>ebs-1</code>).</p></li>
    /// <li>
    /// <p><code>network-info.efa-info.maximum-efa-interfaces</code> - The maximum number of Elastic Fabric Adapters (EFAs) per instance.</p></li>
    /// <li>
    /// <p><code>network-info.efa-supported</code> - Indicates whether the instance type supports Elastic Fabric Adapter (EFA) (<code>true</code> | <code>false</code>).</p></li>
    /// <li>
    /// <p><code>network-info.ena-support</code> - Indicates whether Elastic Network Adapter (ENA) is supported or required (<code>required</code> | <code>supported</code> | <code>unsupported</code>).</p></li>
    /// <li>
    /// <p><code>network-info.flexible-ena-queues-support</code> - Indicates whether an instance supports flexible ENA queues (<code>supported</code> | <code>unsupported</code>).</p></li>
    /// <li>
    /// <p><code>network-info.encryption-in-transit-supported</code> - Indicates whether the instance type automatically encrypts in-transit traffic between instances (<code>true</code> | <code>false</code>).</p></li>
    /// <li>
    /// <p><code>network-info.ipv4-addresses-per-interface</code> - The maximum number of private IPv4 addresses per network interface.</p></li>
    /// <li>
    /// <p><code>network-info.ipv6-addresses-per-interface</code> - The maximum number of private IPv6 addresses per network interface.</p></li>
    /// <li>
    /// <p><code>network-info.ipv6-supported</code> - Indicates whether the instance type supports IPv6 (<code>true</code> | <code>false</code>).</p></li>
    /// <li>
    /// <p><code>network-info.maximum-network-cards</code> - The maximum number of network cards per instance.</p></li>
    /// <li>
    /// <p><code>network-info.maximum-network-interfaces</code> - The maximum number of network interfaces per instance.</p></li>
    /// <li>
    /// <p><code>network-info.network-performance</code> - The network performance (for example, "25 Gigabit").</p></li>
    /// <li>
    /// <p><code>nitro-enclaves-support</code> - Indicates whether Nitro Enclaves is supported (<code>supported</code> | <code>unsupported</code>).</p></li>
    /// <li>
    /// <p><code>nitro-tpm-support</code> - Indicates whether NitroTPM is supported (<code>supported</code> | <code>unsupported</code>).</p></li>
    /// <li>
    /// <p><code>nitro-tpm-info.supported-versions</code> - The supported NitroTPM version (<code>2.0</code>).</p></li>
    /// <li>
    /// <p><code>processor-info.supported-architecture</code> - The CPU architecture (<code>arm64</code> | <code>i386</code> | <code>x86_64</code>).</p></li>
    /// <li>
    /// <p><code>processor-info.sustained-clock-speed-in-ghz</code> - The CPU clock speed, in GHz.</p></li>
    /// <li>
    /// <p><code>processor-info.supported-features</code> - The supported CPU features (<code>amd-sev-snp</code>).</p></li>
    /// <li>
    /// <p><code>reboot-migration-support</code> - Indicates whether enabling reboot migration is supported (<code>supported</code> | <code>unsupported</code>).</p></li>
    /// <li>
    /// <p><code>supported-boot-mode</code> - The boot mode (<code>legacy-bios</code> | <code>uefi</code>).</p></li>
    /// <li>
    /// <p><code>supported-root-device-type</code> - The root device type (<code>ebs</code> | <code>instance-store</code>).</p></li>
    /// <li>
    /// <p><code>supported-usage-class</code> - The usage class (<code>on-demand</code> | <code>spot</code> | <code>capacity-block</code>).</p></li>
    /// <li>
    /// <p><code>supported-virtualization-type</code> - The virtualization type (<code>hvm</code> | <code>paravirtual</code>).</p></li>
    /// <li>
    /// <p><code>vcpu-info.default-cores</code> - The default number of cores for the instance type.</p></li>
    /// <li>
    /// <p><code>vcpu-info.default-threads-per-core</code> - The default number of threads per core for the instance type.</p></li>
    /// <li>
    /// <p><code>vcpu-info.default-vcpus</code> - The default number of vCPUs for the instance type.</p></li>
    /// <li>
    /// <p><code>vcpu-info.valid-cores</code> - The number of cores that can be configured for the instance type.</p></li>
    /// <li>
    /// <p><code>vcpu-info.valid-threads-per-core</code> - The number of threads per core that can be configured for the instance type. For example, "1" or "1,2".</p></li>
    /// </ul>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Filter>> {
        self.inner.get_filters()
    }
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
