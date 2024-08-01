use bitcoincore_rpc::bitcoin::block::Header;
use bitcoincore_rpc::bitcoin::hashes::Hash;
use bitcoincore_rpc::bitcoin::BlockHash;
use bitcoincore_rpc::RpcApi;
use btc_types::hash::H256;

use crate::merkle_tools;

use crate::config::Config;

#[derive(Debug)]
pub struct Client {
    inner: bitcoincore_rpc::Client,
}

impl Client {
    pub fn new(config: &Config) -> Self {
        let inner = bitcoincore_rpc::Client::new(
            &config.bitcoin.endpoint,
            bitcoincore_rpc::Auth::UserPass(
                config.bitcoin.node_user.clone(),
                config.bitcoin.node_password.clone(),
            ),
        )
        .expect("failed to create a bitcoin client");

        Self { inner }
    }

    #[allow(dead_code)]
    pub fn get_best_block_hash(&self) -> BlockHash {
        self.inner.get_best_block_hash().unwrap()
    }

    pub fn get_block_count(&self) -> u64 {
        self.inner.get_block_count().unwrap()
    }

    pub fn get_block_hash(&self, height: u64) -> BlockHash {
        self.inner.get_block_hash(height).unwrap()
    }

    pub fn get_block_header(&self, block_hash: &BlockHash) -> Header {
        self.inner.get_block_header(block_hash).unwrap()
    }

    pub fn get_block_header_by_height(&self, height: u64) -> Header {
        let block_hash = self.get_block_hash(height);
        self.get_block_header(&block_hash)
    }

    pub fn get_block(&self, block_hash: &BlockHash) -> bitcoincore_rpc::bitcoin::Block {
        self.inner.get_block(block_hash).unwrap()
    }

    pub fn get_block_by_height(&self, height: u64) -> bitcoincore_rpc::bitcoin::Block {
        let block_hash = self.get_block_hash(height);
        self.get_block(&block_hash)
    }

    pub fn compute_merkle_proof(
        block: &bitcoincore_rpc::bitcoin::Block,
        transaction_position: usize,
    ) -> Vec<merkle_tools::H256> {
        let mut transactions: Vec<H256> = block
            .txdata
            .iter()
            .map(|tx| merkle_tools::H256(tx.txid().to_byte_array()))
            .collect();
        for i in 0..transactions.len() {
            transactions[i].0.reverse();
        }
        merkle_tools::merkle_proof_calculator(transactions, transaction_position)
    }
}
