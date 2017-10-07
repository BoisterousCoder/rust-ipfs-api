pub type BlockGetResponse = String;


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BlockPutResponse {
    pub key: String,
    pub size: isize,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BlockRmResponse {
    pub hash: String,
    pub error: String,
}


#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BlockStatResponse {
    pub key: String,
    pub size: isize,
}


#[cfg(test)]
mod tests {
    use super::BlockStatResponse;


    deserialize_test!(v0_block_stat_0, BlockStatResponse);
}
