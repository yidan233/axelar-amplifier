use axelar_wasm_std::address::AddressFormat;
use axelar_wasm_std::msg_id::MessageIdFormat;
use axelar_wasm_std::{nonempty, MajorityThreshold};
use cosmwasm_schema::cw_serde;
use router_api::ChainName;

#[cw_serde]
pub struct InstantiateMsg {
    /// Address that can call all messages of unrestricted governance permission level, like UpdateVotingThreshold.
    /// It can execute messages that bypasses verification checks to rescue the contract if it got into an otherwise unrecoverable state due to external forces.
    /// On mainnet it should match the address of the Cosmos governance module.
    pub governance_address: nonempty::String,
    /// Service registry contract address on axelar.
    pub service_registry_address: nonempty::String,
    /// Name of service in the service registry for which verifiers are registered.
    pub service_name: nonempty::String,
    /// Axelar's gateway contract address on the source chain
    pub source_gateway_address: nonempty::String,
    /// Threshold of weighted votes required for voting to be considered complete for a particular message
    pub voting_threshold: MajorityThreshold,
    /// The number of blocks after which a poll expires
    pub block_expiry: nonempty::Uint64,
    /// The number of blocks to wait for on the source chain before considering a transaction final
    pub confirmation_height: u64,
    /// Name of the source chain
    pub source_chain: ChainName,
    /// Rewards contract address on axelar.
    pub rewards_address: nonempty::String,
    /// Format that incoming messages should use for the id field of CrossChainId
    pub msg_id_format: MessageIdFormat,
    pub address_format: AddressFormat,
}
