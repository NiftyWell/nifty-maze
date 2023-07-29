# PingProxy Smart Contract Documentation

The PingProxy smart contract is a simple proxy contract based on the Rust programming language for the multichain blockchain platform, formerly known as the Elrond blockchain. It acts as a proxy to call the `ping` method in the caller contract once the caller contract calls the `pong` method in the ping proxy.

## Constants

### PING
- `const PING: &[u8]`: A simple text constant representing the message "ping".

## Functions

### init
- `init(&self)`: Initialization function for the PingProxy contract. No arguments are required.

### pong
- `pong(&self)`: Function used to call the `ping` method in the caller contract once the caller contract calls the `pong` method in the ping proxy. This function requires that the caller address matches the allowed caller address set using the `set_caller` method.

### set_caller
- `set_caller(&self, caller: ManagedAddress) -> SCResult<()>`: Function used by the contract owner to set the contract address allowed to call the ping proxy.
  - `caller`: The address of the contract allowed to call the ping proxy.
  - Returns: An `SCResult` indicating the success or failure of setting the allowed caller address.

### View Functions

#### getCallerAddress
- `getCallerAddress(&self) -> SingleValueMapper<ManagedAddress>`: View function used to retrieve the address of the contract allowed to call the ping proxy.

## Storage Mappers

### caller_address
- `caller_address(&self) -> SingleValueMapper<ManagedAddress>`: Storage mapper used to store the address of the contract allowed to call the ping proxy.

