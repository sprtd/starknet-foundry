use super::cheat_execution_info::{
    BlockInfoMockOperations, CheatArguments, ExecutionInfoMockOperations, Operation,
};
use crate::state::CheatSpan;
use crate::CheatnetState;
use starknet_api::core::ContractAddress;

impl CheatnetState {
    pub fn warp(&mut self, contract_address: ContractAddress, timestamp: u64, span: CheatSpan) {
        self.cheat_execution_info(ExecutionInfoMockOperations {
            block_info: BlockInfoMockOperations {
                block_timestamp: Operation::Start(CheatArguments {
                    value: timestamp,
                    span,
                    target: contract_address,
                }),
                ..Default::default()
            },
            ..Default::default()
        });
    }

    pub fn start_warp(&mut self, contract_address: ContractAddress, timestamp: u64) {
        self.warp(contract_address, timestamp, CheatSpan::Indefinite);
    }

    pub fn stop_warp(&mut self, contract_address: ContractAddress) {
        self.cheat_execution_info(ExecutionInfoMockOperations {
            block_info: BlockInfoMockOperations {
                block_timestamp: Operation::Stop(contract_address),
                ..Default::default()
            },
            ..Default::default()
        });
    }
}
