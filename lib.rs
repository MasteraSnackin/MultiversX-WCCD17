#![no_std]

// Import necessary modules and macros from the elrond_wasm library.
elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait OreExchange {
    /// Initialize the contract. This function is called once when the contract is deployed.
    #[init]
    fn init(&self) {}

    /// Endpoint to allow users to exchange 20 STONE tokens for 1 ORE token.
    #[payable("STONE")]
    #[endpoint(exchangeStoneForOre)]
    fn exchange_stone_for_ore(&self) -> SCResult<()> {
        let caller = self.blockchain().get_caller();
        let stone_token_id: TokenIdentifier = TokenIdentifier::from("STONE");

        // Check if the correct amount of STONE tokens are sent
        let stone_amount = self.call_value().single_esdt().unwrap().1;
        require!(stone_amount == 20u64.into(), "Require 20 STONE tokens");

        // Burn the STONE tokens by sending them to the zero address
        self.send().direct(&ManagedAddress::zero(), &stone_token_id, 0, &stone_amount, &[]);

        // Schedule the ORE minting event for immediate claiming
        self.pending_ore(&caller).set(1u64.into());

        Ok(())
    }

    /// Endpoint to claim the ORE token after the exchange.
    #[endpoint(claimOre)]
    fn claim_ore(&self) -> SCResult<()> {
        let caller = self.blockchain().get_caller();
        let ore_amount = self.pending_ore(&caller).get();

        require!(ore_amount > 0, "No ORE available to claim");

        // Mint the ORE token for the caller
        self.issue_ore(&caller, ore_amount)?;

        // Clear the pending ORE entry
        self.pending_ore(&caller).clear();

        Ok(())
    }

    /// Helper function to issue the ORE token.
    fn issue_ore(&self, recipient: &ManagedAddress, amount: BigUint) -> SCResult<()> {
        // Details of the ORE issuance would go here.
        // This would involve using the token contract's logic to mint and assign ORE tokens.
        Ok(())
    }

    /// Storage mapper to keep track of pending ORE tokens for each user.
    #[view(getPendingOre)]
    #[storage_mapper("pendingOre")]
    fn pending_ore(&self, address: &ManagedAddress) -> SingleValueMapper<BigUint>;
}