use std::{fmt::Debug, ops::Sub, time::{ SystemTime, UNIX_EPOCH }};
use sha2::{Digest, Sha256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Block {
  pub index: usize,
  pub timestamp: String,
  pub proof: usize,
  pub previous_hash: String
}

pub struct  Blockchain {
  pub chain: Vec<Block>
}

impl Blockchain {
    pub fn new() -> Self {
      let mut bc = Self {
        chain: vec![]
      };
      bc.create_block(1, String::from("0"));

      bc
    }

    pub fn create_block(&mut self, proof: usize, previous_hash: String) -> &Block {
      let block = Block {
        index: self.chain.len() + 1,
        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis().to_string(),
        proof,
        previous_hash
      };

      self.chain.push(block);

      self.get_previous_block()
    }

    pub fn get_previous_block(&self) -> &Block {
      self.chain.last().unwrap()
    }

    pub fn proof_of_work(&self, previous_proof: usize) -> usize {
      let mut new_proof: usize = 1;
      let mut check_proof = false;
      let mut hash_operation: String;

      while check_proof == false {
        let diff = new_proof.wrapping_pow(2).wrapping_sub(previous_proof.wrapping_pow(2));
        hash_operation = format!("{:x}", Sha256::digest(&diff.to_be_bytes()));

        if &hash_operation[..4] == "0000" {
          check_proof = true;
        } else {
          new_proof += 1;
        }
      }

      new_proof
    }

    pub fn hash(&self, block: &Block) -> String {
      let encoded_block = serde_json::to_string(block).unwrap();
      let mut hasher = Sha256::new();
      hasher.update(encoded_block);
      
      format!("{:x}", hasher.finalize())
    }

    pub fn is_chain_valid(&self, chain: &Vec<Block>) -> bool {
      let mut previous_block = &chain[0];
      let mut block_index = 1;

      while block_index < chain.len() {
        let block = &chain[block_index];

        if block.previous_hash != self.hash(previous_block) {
          return false;
        }

        let previous_proof = previous_block.proof;
        let proof = block.proof;
        let diff = proof.wrapping_pow(2).wrapping_sub(previous_proof.wrapping_pow(2));
        let hash_operation = format!("{:x}", Sha256::digest(&diff.to_be_bytes()));

        if &hash_operation[..4] != "0000" {
          return false;
        }

        previous_block = block;
        block_index += 1;
      }

      true
    }
}