use crate::schema::{Declaration, Definition};
use crate::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;
use std::io::Write;

impl BorshSerialize for Pubkey {
    fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.to_bytes().serialize(writer)
    }
}

impl BorshDeserialize for Pubkey {
    fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let arr = BorshDeserialize::deserialize(buf)?;
        Ok(Pubkey::new_from_array(arr))
    }
}

impl BorshSchema for Pubkey {
    fn add_definitions_recursively(_definitions: &mut HashMap<Declaration, Definition>) {}

    fn declaration() -> Declaration {
        "pubkey".to_string()
    }
}
