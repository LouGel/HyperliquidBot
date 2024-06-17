use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Meta {
    pub universe: Vec<AssetMeta>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetMeta {
    pub name: String,
    pub sz_decimals: Option<u32>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub name: String,
    pub sz_decimals: u8,
    pub wei_decimals: u8,
    pub index: u64,
    pub token_id: String,
    pub is_canonical: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SpotMeta {
    pub universe: Vec<AssetMeta>,
    pub tokens: Vec<Token>,
}
