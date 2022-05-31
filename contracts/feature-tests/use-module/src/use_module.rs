#![no_std]

mod internal_mod_a;
mod internal_mod_b;
mod internal_mod_c;
mod internal_mod_d;
mod internal_mod_init;
mod only_owner_derived_mod;
mod only_owner_mod;

elrond_wasm::imports!();

/// Contract that tests that using modules works correctly.
/// Also provides testing for the most common modules:
/// - DnsModule
/// - FeaturesModule
/// - EsdtModule
/// - GovernanceModule
/// - PauseModule
#[elrond_wasm::contract]
pub trait UseModule:
    internal_mod_a::InternalModuleA
    + internal_mod_b::InternalModuleB
    + internal_mod_c::InternalModuleC
    + internal_mod_init::InternalModuleInit
    + only_owner_mod::OnlyOwnerModule
    + only_owner_derived_mod::OnlyOwnerDerivedModule
    + elrond_wasm_modules::dns::DnsModule
    + elrond_wasm_modules::esdt::EsdtModule
    + elrond_wasm_modules::features::FeaturesModule
    + elrond_wasm_modules::governance::GovernanceModule
    + elrond_wasm_modules::governance::governance_configurable::GovernanceConfigurablePropertiesModule
    + elrond_wasm_modules::pause::PauseModule
    + elrond_wasm_modules::staking::StakingModule
    + elrond_wasm_modules::default_issue_callbacks::DefaultIssueCallbacksModule
    + elrond_wasm_modules::transfer_role::transfer_proxy::TransferProxyModule
    + elrond_wasm_modules::transfer_role::transfer_destination::TransferDestinationModule
{
    /// Validates that the "featureName" feature is on.
    /// Uses the `feature_guard!` macro.
    #[endpoint(checkFeatureGuard)]
    fn check_feature_guard(&self) {
        self.check_feature_on(b"featureName", true);
    }

    #[endpoint(checkPause)]
    fn check_pause(&self) -> SCResult<bool> {
        Ok(self.is_paused())
    }

    #[payable("*")]
    #[endpoint(forwardPayments)]
    fn forward_payments(
        &self,
        dest: ManagedAddress,
        endpoint_name: ManagedBuffer,
        args: MultiValueEncoded<ManagedBuffer>,
    ) {
        let original_caller = self.blockchain().get_caller();
        let payments = self.call_value().all_esdt_transfers();
        if payments.is_empty() {
            return;
        }

        if !self.blockchain().is_smart_contract(&dest) {
            self.transfer_to_user(original_caller, dest, payments, endpoint_name);
        } else {
            let mut args_buffer = ManagedArgBuffer::new_empty();
            for arg in args {
                args_buffer.push_arg(arg);
            }

            self.transfer_to_contract_raw(
                original_caller,
                dest,
                payments,
                endpoint_name,
                args_buffer,
            );
        }
    }
}
