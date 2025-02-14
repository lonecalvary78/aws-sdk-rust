// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An alternate list of prioritized locations for use with a game session queue. When this property is included in a <a href="https://docs.aws.amazon.com/gamelift/latest/apireference/API_StartGameSessionPlacement.html">StartGameSessionPlacement</a> request, this list overrides the queue's default location prioritization, as defined in the queue's <a href="gamelift/latest/apireference/API_PriorityConfiguration.html">PriorityConfiguration</a> setting (<i>LocationOrder</i>). This property overrides the queue's default priority list for individual placement requests only. Use this property only with queues that have a <code>PriorityConfiguration</code> setting that prioritizes first.</p><note>
/// <p>A priority configuration override list does not override a queue's FilterConfiguration setting, if the queue has one. Filter configurations are used to limit placements to a subset of the locations in a queue's destinations. If the override list includes a location that's not included in the FilterConfiguration allowed list, Amazon GameLift won't attempt to place a game session there.</p>
/// </note>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PriorityConfigurationOverride {
    /// <p>Instructions for how to use the override list if the first round of placement attempts fails. The first round is a failure if Amazon GameLift searches all listed locations, in all of the queue's destinations, without finding an available hosting resource for a new game session. Valid strategies include:</p>
    /// <ul>
    /// <li>
    /// <p><code>DEFAULT_AFTER_SINGLE_PASS</code> -- After the first round of placement attempts, discard the override list and use the queue's default location priority list. Continue to use the queue's default list until the placement request times out.</p></li>
    /// <li>
    /// <p><code>NONE</code> -- Continue to use the override list for all rounds of placement attempts until the placement request times out.</p></li>
    /// </ul>
    pub placement_fallback_strategy: ::std::option::Option<crate::types::PlacementFallbackStrategy>,
    /// <p>A prioritized list of hosting locations. The list can include Amazon Web Services Regions (such as <code>us-west-2</code>), local zones, and custom locations (for Anywhere fleets). Each location must be listed only once. For details, see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-regions.html">Amazon GameLift service locations.</a></p>
    pub location_order: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl PriorityConfigurationOverride {
    /// <p>Instructions for how to use the override list if the first round of placement attempts fails. The first round is a failure if Amazon GameLift searches all listed locations, in all of the queue's destinations, without finding an available hosting resource for a new game session. Valid strategies include:</p>
    /// <ul>
    /// <li>
    /// <p><code>DEFAULT_AFTER_SINGLE_PASS</code> -- After the first round of placement attempts, discard the override list and use the queue's default location priority list. Continue to use the queue's default list until the placement request times out.</p></li>
    /// <li>
    /// <p><code>NONE</code> -- Continue to use the override list for all rounds of placement attempts until the placement request times out.</p></li>
    /// </ul>
    pub fn placement_fallback_strategy(&self) -> ::std::option::Option<&crate::types::PlacementFallbackStrategy> {
        self.placement_fallback_strategy.as_ref()
    }
    /// <p>A prioritized list of hosting locations. The list can include Amazon Web Services Regions (such as <code>us-west-2</code>), local zones, and custom locations (for Anywhere fleets). Each location must be listed only once. For details, see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-regions.html">Amazon GameLift service locations.</a></p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.location_order.is_none()`.
    pub fn location_order(&self) -> &[::std::string::String] {
        self.location_order.as_deref().unwrap_or_default()
    }
}
impl PriorityConfigurationOverride {
    /// Creates a new builder-style object to manufacture [`PriorityConfigurationOverride`](crate::types::PriorityConfigurationOverride).
    pub fn builder() -> crate::types::builders::PriorityConfigurationOverrideBuilder {
        crate::types::builders::PriorityConfigurationOverrideBuilder::default()
    }
}

/// A builder for [`PriorityConfigurationOverride`](crate::types::PriorityConfigurationOverride).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct PriorityConfigurationOverrideBuilder {
    pub(crate) placement_fallback_strategy: ::std::option::Option<crate::types::PlacementFallbackStrategy>,
    pub(crate) location_order: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl PriorityConfigurationOverrideBuilder {
    /// <p>Instructions for how to use the override list if the first round of placement attempts fails. The first round is a failure if Amazon GameLift searches all listed locations, in all of the queue's destinations, without finding an available hosting resource for a new game session. Valid strategies include:</p>
    /// <ul>
    /// <li>
    /// <p><code>DEFAULT_AFTER_SINGLE_PASS</code> -- After the first round of placement attempts, discard the override list and use the queue's default location priority list. Continue to use the queue's default list until the placement request times out.</p></li>
    /// <li>
    /// <p><code>NONE</code> -- Continue to use the override list for all rounds of placement attempts until the placement request times out.</p></li>
    /// </ul>
    pub fn placement_fallback_strategy(mut self, input: crate::types::PlacementFallbackStrategy) -> Self {
        self.placement_fallback_strategy = ::std::option::Option::Some(input);
        self
    }
    /// <p>Instructions for how to use the override list if the first round of placement attempts fails. The first round is a failure if Amazon GameLift searches all listed locations, in all of the queue's destinations, without finding an available hosting resource for a new game session. Valid strategies include:</p>
    /// <ul>
    /// <li>
    /// <p><code>DEFAULT_AFTER_SINGLE_PASS</code> -- After the first round of placement attempts, discard the override list and use the queue's default location priority list. Continue to use the queue's default list until the placement request times out.</p></li>
    /// <li>
    /// <p><code>NONE</code> -- Continue to use the override list for all rounds of placement attempts until the placement request times out.</p></li>
    /// </ul>
    pub fn set_placement_fallback_strategy(mut self, input: ::std::option::Option<crate::types::PlacementFallbackStrategy>) -> Self {
        self.placement_fallback_strategy = input;
        self
    }
    /// <p>Instructions for how to use the override list if the first round of placement attempts fails. The first round is a failure if Amazon GameLift searches all listed locations, in all of the queue's destinations, without finding an available hosting resource for a new game session. Valid strategies include:</p>
    /// <ul>
    /// <li>
    /// <p><code>DEFAULT_AFTER_SINGLE_PASS</code> -- After the first round of placement attempts, discard the override list and use the queue's default location priority list. Continue to use the queue's default list until the placement request times out.</p></li>
    /// <li>
    /// <p><code>NONE</code> -- Continue to use the override list for all rounds of placement attempts until the placement request times out.</p></li>
    /// </ul>
    pub fn get_placement_fallback_strategy(&self) -> &::std::option::Option<crate::types::PlacementFallbackStrategy> {
        &self.placement_fallback_strategy
    }
    /// Appends an item to `location_order`.
    ///
    /// To override the contents of this collection use [`set_location_order`](Self::set_location_order).
    ///
    /// <p>A prioritized list of hosting locations. The list can include Amazon Web Services Regions (such as <code>us-west-2</code>), local zones, and custom locations (for Anywhere fleets). Each location must be listed only once. For details, see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-regions.html">Amazon GameLift service locations.</a></p>
    pub fn location_order(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.location_order.unwrap_or_default();
        v.push(input.into());
        self.location_order = ::std::option::Option::Some(v);
        self
    }
    /// <p>A prioritized list of hosting locations. The list can include Amazon Web Services Regions (such as <code>us-west-2</code>), local zones, and custom locations (for Anywhere fleets). Each location must be listed only once. For details, see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-regions.html">Amazon GameLift service locations.</a></p>
    pub fn set_location_order(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.location_order = input;
        self
    }
    /// <p>A prioritized list of hosting locations. The list can include Amazon Web Services Regions (such as <code>us-west-2</code>), local zones, and custom locations (for Anywhere fleets). Each location must be listed only once. For details, see <a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-regions.html">Amazon GameLift service locations.</a></p>
    pub fn get_location_order(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.location_order
    }
    /// Consumes the builder and constructs a [`PriorityConfigurationOverride`](crate::types::PriorityConfigurationOverride).
    pub fn build(self) -> crate::types::PriorityConfigurationOverride {
        crate::types::PriorityConfigurationOverride {
            placement_fallback_strategy: self.placement_fallback_strategy,
            location_order: self.location_order,
        }
    }
}
