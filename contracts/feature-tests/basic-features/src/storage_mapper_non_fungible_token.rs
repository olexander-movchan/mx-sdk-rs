elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct RgbColor {
    r: u8,
    g: u8,
    b: u8,
}

#[elrond_wasm::module]
pub trait NonFungibleTokenMapperFeatures:
    elrond_wasm_modules::default_issue_callbacks::DefaultIssueCallbacksModule
{
    #[endpoint]
    fn mapper_nft_set_token_id(&self, token_id: TokenIdentifier) {
        self.non_fungible_token_mapper().set_token_id(&token_id);
    }

    #[endpoint]
    fn mapper_nft_create(
        &self,
        amount: BigUint,
        attributes: RgbColor,
    ) -> EsdtTokenPayment<Self::Api> {
        self.non_fungible_token_mapper()
            .nft_create(amount, &attributes)
    }

    #[endpoint]
    fn mapper_nft_create_and_send(
        &self,
        to: ManagedAddress,
        amount: BigUint,
        attributes: RgbColor,
    ) -> EsdtTokenPayment<Self::Api> {
        self.non_fungible_token_mapper()
            .nft_create_and_send(&to, amount, &attributes)
    }

    #[endpoint]
    fn mapper_nft_add_quantity(
        &self,
        token_nonce: u64,
        amount: BigUint,
    ) -> EsdtTokenPayment<Self::Api> {
        self.non_fungible_token_mapper()
            .nft_add_quantity(token_nonce, amount)
    }

    #[endpoint]
    fn mapper_nft_add_quantity_and_send(
        &self,
        to: ManagedAddress,
        token_nonce: u64,
        amount: BigUint,
    ) -> EsdtTokenPayment<Self::Api> {
        self.non_fungible_token_mapper()
            .nft_add_quantity_and_send(&to, token_nonce, amount)
    }

    #[endpoint]
    fn mapper_nft_burn(&self, token_nonce: u64, amount: BigUint) {
        self.non_fungible_token_mapper()
            .nft_burn(token_nonce, &amount);
    }

    #[endpoint]
    fn mapper_nft_get_balance(&self, token_nonce: u64) -> BigUint {
        self.non_fungible_token_mapper().get_balance(token_nonce)
    }

    #[endpoint]
    fn mapper_get_token_attributes(&self, token_nonce: u64) -> RgbColor {
        self.non_fungible_token_mapper()
            .get_token_attributes(token_nonce)
    }

    #[view(getNonFungibleTokenId)]
    #[storage_mapper("nonFungibleTokenMapper")]
    fn non_fungible_token_mapper(&self) -> NonFungibleTokenMapper<Self::Api>;
}