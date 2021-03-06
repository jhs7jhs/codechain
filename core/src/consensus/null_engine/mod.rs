// Copyright 2018-2019 Kodebox, Inc.
// This file is part of CodeChain.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

mod params;

use ctypes::machine::{Header, LiveBlock, Transactions, WithBalances};

use self::params::NullEngineParams;
use super::ConsensusEngine;
use crate::consensus::EngineType;
use crate::SignedTransaction;

/// An engine which does not provide any consensus mechanism and does not seal blocks.
pub struct NullEngine<M> {
    params: NullEngineParams,
    machine: M,
}

impl<M> NullEngine<M> {
    /// Returns new instance of NullEngine with default VM Factory
    pub fn new(params: NullEngineParams, machine: M) -> Self {
        NullEngine {
            params,
            machine,
        }
    }
}

impl<M: Default> Default for NullEngine<M> {
    fn default() -> Self {
        Self::new(Default::default(), Default::default())
    }
}

impl<M: WithBalances> ConsensusEngine<M> for NullEngine<M>
where
    M::LiveBlock: Transactions<Transaction = SignedTransaction>,
{
    fn name(&self) -> &str {
        "NullEngine"
    }

    fn machine(&self) -> &M {
        &self.machine
    }

    fn engine_type(&self) -> EngineType {
        EngineType::Solo
    }

    fn verify_local_seal(&self, _header: &M::Header) -> Result<(), M::Error> {
        Ok(())
    }

    fn on_close_block(&self, block: &mut M::LiveBlock) -> Result<(), M::Error> {
        let author = *LiveBlock::header(&*block).author();
        let total_reward = self.block_reward(block.header().number())
            + self.block_fee(Box::new(block.transactions().to_owned().into_iter().map(Into::into)));
        self.machine.add_balance(block, &author, total_reward)
    }

    fn block_reward(&self, _block_number: u64) -> u64 {
        self.params.block_reward
    }

    fn recommended_confirmation(&self) -> u32 {
        1
    }
}
