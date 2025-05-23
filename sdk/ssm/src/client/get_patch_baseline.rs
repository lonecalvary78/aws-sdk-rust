// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetPatchBaseline`](crate::operation::get_patch_baseline::builders::GetPatchBaselineFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`baseline_id(impl Into<String>)`](crate::operation::get_patch_baseline::builders::GetPatchBaselineFluentBuilder::baseline_id) / [`set_baseline_id(Option<String>)`](crate::operation::get_patch_baseline::builders::GetPatchBaselineFluentBuilder::set_baseline_id):<br>required: **true**<br><p>The ID of the patch baseline to retrieve.</p><note>  <p>To retrieve information about an Amazon Web Services managed patch baseline, specify the full Amazon Resource Name (ARN) of the baseline. For example, for the baseline <code>AWS-AmazonLinuxDefaultPatchBaseline</code>, specify <code>arn:aws:ssm:us-east-2:733109147000:patchbaseline/pb-0e392de35e7c563b7</code> instead of <code>pb-0e392de35e7c563b7</code>.</p> </note><br>
    /// - On success, responds with [`GetPatchBaselineOutput`](crate::operation::get_patch_baseline::GetPatchBaselineOutput) with field(s):
    ///   - [`baseline_id(Option<String>)`](crate::operation::get_patch_baseline::GetPatchBaselineOutput::baseline_id): <p>The ID of the retrieved patch baseline.</p>
    ///   - [`name(Option<String>)`](crate::operation::get_patch_baseline::GetPatchBaselineOutput::name): <p>The name of the patch baseline.</p>
    ///   - [`operating_system(Option<OperatingSystem>)`](crate::operation::get_patch_baseline::GetPatchBaselineOutput::operating_system): <p>Returns the operating system specified for the patch baseline.</p>
    ///   - [`global_filters(Option<PatchFilterGroup>)`](crate::operation::get_patch_baseline::GetPatchBaselineOutput::global_filters): <p>A set of global filters used to exclude patches from the baseline.</p>
    ///   - [`approval_rules(Option<PatchRuleGroup>)`](crate::operation::get_patch_baseline::GetPatchBaselineOutput::approval_rules): <p>A set of rules used to include patches in the baseline.</p>
    ///   - [`approved_patches(Option<Vec::<String>>)`](crate::operation::get_patch_baseline::GetPatchBaselineOutput::approved_patches): <p>A list of explicitly approved patches for the baseline.</p>
    ///   - [`approved_patches_compliance_level(Option<PatchComplianceLevel>)`](crate::operation::get_patch_baseline::GetPatchBaselineOutput::approved_patches_compliance_level): <p>Returns the specified compliance severity level for approved patches in the patch baseline.</p>
    ///   - [`approved_patches_enable_non_security(Option<bool>)`](crate::operation::get_patch_baseline::GetPatchBaselineOutput::approved_patches_enable_non_security): <p>Indicates whether the list of approved patches includes non-security updates that should be applied to the managed nodes. The default value is <code>false</code>. Applies to Linux managed nodes only.</p>
    ///   - [`rejected_patches(Option<Vec::<String>>)`](crate::operation::get_patch_baseline::GetPatchBaselineOutput::rejected_patches): <p>A list of explicitly rejected patches for the baseline.</p>
    ///   - [`rejected_patches_action(Option<PatchAction>)`](crate::operation::get_patch_baseline::GetPatchBaselineOutput::rejected_patches_action): <p>The action specified to take on patches included in the <code>RejectedPatches</code> list. A patch can be allowed only if it is a dependency of another package, or blocked entirely along with packages that include it as a dependency.</p>
    ///   - [`patch_groups(Option<Vec::<String>>)`](crate::operation::get_patch_baseline::GetPatchBaselineOutput::patch_groups): <p>Patch groups included in the patch baseline.</p>
    ///   - [`created_date(Option<DateTime>)`](crate::operation::get_patch_baseline::GetPatchBaselineOutput::created_date): <p>The date the patch baseline was created.</p>
    ///   - [`modified_date(Option<DateTime>)`](crate::operation::get_patch_baseline::GetPatchBaselineOutput::modified_date): <p>The date the patch baseline was last modified.</p>
    ///   - [`description(Option<String>)`](crate::operation::get_patch_baseline::GetPatchBaselineOutput::description): <p>A description of the patch baseline.</p>
    ///   - [`sources(Option<Vec::<PatchSource>>)`](crate::operation::get_patch_baseline::GetPatchBaselineOutput::sources): <p>Information about the patches to use to update the managed nodes, including target operating systems and source repositories. Applies to Linux managed nodes only.</p>
    ///   - [`available_security_updates_compliance_status(Option<PatchComplianceStatus>)`](crate::operation::get_patch_baseline::GetPatchBaselineOutput::available_security_updates_compliance_status): <p>Indicates the compliance status of managed nodes for which security-related patches are available but were not approved. This preference is specified when the <code>CreatePatchBaseline</code> or <code>UpdatePatchBaseline</code> commands are run.</p> <p>Applies to Windows Server managed nodes only.</p>
    /// - On failure, responds with [`SdkError<GetPatchBaselineError>`](crate::operation::get_patch_baseline::GetPatchBaselineError)
    pub fn get_patch_baseline(&self) -> crate::operation::get_patch_baseline::builders::GetPatchBaselineFluentBuilder {
        crate::operation::get_patch_baseline::builders::GetPatchBaselineFluentBuilder::new(self.handle.clone())
    }
}
