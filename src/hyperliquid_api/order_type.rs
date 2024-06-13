// use ethers::{
//     contract::{Eip712, EthAbiType},
//     types::{H256, U256},
// };
// use serde::{Deserialize, Serialize};

// #[derive(Debug, Clone, Serialize, Deserialize, EthAbiType, Eip712)]
// #[eip712(
//     name = "Market",
//     version = "1",
//     chain_id = 1337,
//     verifying_contract = "0x0000000000000000000000000000000000000000"
// )]
// pub struct Market {
//     #[eip712(type = "string")]
//     pub grouping: String,
//     #[eip712(type = "Order[]")]
//     pub orders: Vec<Order>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, EthAbiType, Eip712)]
// pub struct Order {
//     #[eip712(type = "uint256")]
//     pub a: U256,
//     #[eip712(type = "bool")]
//     pub b: bool,
//     #[eip712(type = "string")]
//     pub p: String,
//     #[eip712(type = "string")]
//     pub s: String,
//     #[eip712(type = "bool")]
//     pub r: bool,
//     #[eip712(type = "string")]
//     pub t: String,
//     #[eip712(type = "string", optional = true)]
//     pub c: Option<String>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, EthAbiType, Eip712)]
// pub struct Limit {
//     #[eip712(type = "string")]
//     pub tif: String, // "Alo", "Ioc", or "Gtc"
// }

// #[derive(Debug, Clone, Serialize, Deserialize, EthAbiType, Eip712)]
// pub struct Trigger {
//     #[eip712(type = "bool")]
//     pub is_market: bool,
//     #[eip712(type = "string")]
//     pub trigger_px: String,
//     #[eip712(type = "string")]
//     pub tpsl: String, // "tp" or "sl"
// }
