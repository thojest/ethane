// TODO: eventually remove ethereum_types?
// TODO: eventually use Serde Serialize, or stay with Display?
use ethereum_types::Address;
use ethereum_types::H256;
use hex::ToHex;
use std::fmt::{Display, Formatter, Result as FmtResult};

mod geth;

pub trait RemoteProcedures {
    const ID: &'static str = "_ID_";
    const PARAMS: &'static str = "_PARAMS_";
    const METHOD: &'static str = "_METHOD_";
    const CMD: &'static str =
        r#"{"jsonrpc":"2.0","method":"_METHOD_","params":[_PARAMS_],"id":_ID_}"#;

    fn net_version(id: u32) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "net_version")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, "")
    }

    fn net_peer_count(id: u32) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "net_peerCount")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, "")
    }

    fn net_listening(id: u32) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "net_listening")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, "")
    }

    fn eth_protocol_version(id: u32) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_protocol_version")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, "")
    }

    fn eth_syncing(id: u32) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_syncing")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, "")
    }

    fn eth_coinbase(id: u32) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_coinbase")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, "")
    }

    fn eth_mining(id: u32) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_mining")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, "")
    }

    fn eth_hashrate(id: u32) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_hashrate")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, "")
    }

    fn eth_gas_price(id: u32) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_gasPrice")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, "")
    }

    fn eth_accounts(id: u32) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_accounts")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, "")
    }

    fn eth_block_number(id: u32) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_blockNumber")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, "")
    }

    fn eth_get_balance(id: u32, address: Address, block_param: BlockParameter) -> String {
        let params: String = vec![address.to_string(), block_param.to_string()].join(", ");

        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_getBalance")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &params)
    }

    fn eth_get_storage_at(
        id: u32,
        address: Address,
        storage_pos: u32,
        block_param: BlockParameter,
    ) -> String {
        let params: String = vec![
            address.to_string(),
            format!("{:#x}", storage_pos),
            block_param.to_string(),
        ]
        .join(", ");

        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_getStorageAt")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &params)
    }
    fn eth_get_transaction_count(id: u32, address: Address, block_param: BlockParameter) -> String {
        let params: String = vec![address.to_string(), block_param.to_string()].join(", ");

        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_getTransactionCount")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &params)
    }

    fn eth_get_block_transaction_count_by_hash(id: u32, block_hash: H256) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_getBlockTransactionCountByHash")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &block_hash.to_string())
    }

    fn eth_get_block_transaction_count_by_number(id: u32, block_param: BlockParameter) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_getBlockTransactionCountByNumber")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &block_param.to_string())
    }

    fn eth_get_uncle_count_by_block_hash(id: u32, block_hash: H256) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_getUncleCountByBlockHash")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &block_hash.to_string())
    }

    fn eth_get_uncle_count_by_block_number(id: u32, block_param: BlockParameter) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_getUncleCountByBlockNumber")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &block_param.to_string())
    }

    fn eth_get_code(id: u32, address: Address, block_param: BlockParameter) -> String {
        let params: String = vec![address.to_string(), block_param.to_string()].join(", ");

        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_getCode")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &params)
    }

    fn eth_sign(id: u32, address: Address, bytes: Bytes) -> String {
        let params: String = vec![address.to_string(), bytes.to_string()].join(", ");

        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_sign")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &params)
    }

    fn eth_sign_transaction(id: u32, transaction: Transaction) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_signTransaction")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &transaction.to_string())
    }

    fn eth_send_transaction(id: u32, transaction: Transaction) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_sendTransaction")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &transaction.to_string())
    }

    fn eth_send_raw_transaction(id: u32, raw_transaction: Bytes) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_sendRawTransaction")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &raw_transaction.to_string())
    }

    fn eth_call(id: u32, transaction: Transaction, block_param: BlockParameter) -> String {
        let params: String = vec![transaction.to_string(), block_param.to_string()].join(", ");

        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_call")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &params.to_string())
    }

    fn eth_estimate_gas(id: u32, transaction: Transaction, block_param: BlockParameter) -> String {
        let params: String = vec![transaction.to_string(), block_param.to_string()].join(", ");

        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_estimateGas")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &params.to_string())
    }

    fn eth_get_block_by_hash(id: u32, block_hash: H256, full_transactions: bool) -> String {
        let params: String = vec![block_hash.to_string(), full_transactions.to_string()].join(", ");

        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_getBlockByHash")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &params.to_string())
    }

    fn eth_get_block_by_number(
        id: u32,
        block_param: BlockParameter,
        full_transactions: bool,
    ) -> String {
        let params: String =
            vec![block_param.to_string(), full_transactions.to_string()].join(", ");

        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_getBlockByNumber")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &params.to_string())
    }

    fn eth_get_transaction_by_hash(id: u32, transaction_hash: H256) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_getTransactionByHash")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &transaction_hash.to_string())
    }

    fn eth_get_transaction_by_block_hash_and_index(
        id: u32,
        block_hash: H256,
        index_position: u32,
    ) -> String {
        let params: String =
            vec![block_hash.to_string(), format!("{:#x}", index_position)].join(", ");

        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_getTransactionByBlockHashAndIndex")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &params.to_string())
    }

    fn eth_get_transaction_by_block_number_and_index(
        id: u32,
        block_param: BlockParameter,
        index_position: u32,
    ) -> String {
        let params: String =
            vec![block_param.to_string(), format!("{:#x}", index_position)].join(", ");

        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_getTransactionByBlockNumberAndIndex")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &params.to_string())
    }

    fn eth_get_transaction_receipt(id: u32, transaction_hash: H256) -> String {
        String::from(Self::CMD)
            .replace(Self::METHOD, "eth_getTransactionReceipt")
            .replace(Self::ID, &id.to_string())
            .replace(Self::PARAMS, &transaction_hash.to_string())
    }

    fn eth_getUncleByBlockHashAndIndex() -> &'static str;
    fn eth_getUncleByBlockNumberAndIndex() -> &'static str;
    fn eth_getCompilers() -> &'static str;
    fn eth_compileLLL() -> &'static str;
    fn eth_compileSolidity() -> &'static str;
    fn eth_compileSerpent() -> &'static str;
    fn eth_newFilter() -> &'static str;
    fn eth_newBlockFilter() -> &'static str;
    fn eth_newPendingTransactionFilter() -> &'static str;
    fn eth_uninstallFilter() -> &'static str;
    fn eth_getFilterChanges() -> &'static str;
    fn eth_getFilterLogs() -> &'static str;
    fn eth_getLogs() -> &'static str;
    fn eth_getWork() -> &'static str;
    fn eth_submitWork() -> &'static str;
    fn eth_submitHashrate() -> &'static str;
}

pub enum BlockParameter {
    LATEST,
    EARLIEST,
    PENDING,
    CUSTOM(u32),
}

impl Display for BlockParameter {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let block_param = match *self {
            BlockParameter::LATEST => String::from("latest"),
            BlockParameter::EARLIEST => String::from("earliest"),
            BlockParameter::PENDING => String::from("pending"),
            BlockParameter::CUSTOM(num) => format!("{:#x}", num),
        };
        write!(f, "{}", block_param)
    }
}

pub struct Bytes(Vec<u8>);

impl Display for Bytes {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let inner_hex: String = self.0.encode_hex();
        let hex = String::from("0x") + &inner_hex;
        write!(f, "{}", hex)
    }
}

pub struct Transaction {
    from: Address,
    to: Address,
    gas: u32,
    gas_price: u32,
    value: u32,
    data: Bytes,
    nonce: u32,
}

impl Display for Transaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_for_block_parameter() {
        assert_eq!(BlockParameter::CUSTOM(0).to_string(), "0x0");
        assert_eq!(BlockParameter::CUSTOM(17).to_string(), "0x11");
        assert_eq!(BlockParameter::CUSTOM(256).to_string(), "0x100");
    }

    #[test]
    fn test_display_for_bytes() {
        let bytes = Bytes(vec![0, 1, 122, 4]);
        assert_eq!("0x00017a04", bytes.to_string());
    }
}
