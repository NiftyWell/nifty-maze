#![no_std]

multiversx_sc::imports!();

// Simple text constant.
const PING: &[u8] = b"ping";

#[multiversx_sc::contract]
pub trait PingProxy {

    #[init]
    fn init(&self) {}

    // Ping proxy will call the ping method in the caller contract once the caller contract calls the pong method in the ping proxy.
    #[endpoint(pong)]
    fn pong(
        &self,
    ) {
        require!(
            self.blockchain().get_caller() == self.caller_address().get(),
            "Unauthorized call."
        );
        let caller_address = self.caller_address().get();

        let arg_buffer = ManagedArgBuffer::new();
        self.send_raw().async_call_raw(
            &caller_address,
            &BigUint::zero(),
            &ManagedBuffer::from(PING),
            &arg_buffer,
        );
    }

    #[only_owner]
    #[endpoint(setCaller)]
    #[allow(clippy::too_many_arguments)]
    fn set_caller(
        &self,
        caller: ManagedAddress,
    ) -> SCResult<()> {
        self.caller_address().set(caller);
        Ok(())
    }

    // Contract address allowed to call the ping proxy. 
    #[view(getCallerAddress)]
    #[storage_mapper("callerAddress")]
    fn caller_address(&self) -> SingleValueMapper<ManagedAddress>;
}
