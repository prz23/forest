// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use super::client::Filecoin;
use auth::*;
use jsonrpc_v2::Error as JsonRpcError;
use jsonrpsee::raw::RawClient;
use jsonrpsee::transport::http::HttpTransportClient as HTC;

/// Creates a new JWT Token
pub async fn auth_new(client: &mut RawClient<HTC>, perm: String) -> Result<String, JsonRpcError> {
    let ret: String = match perm.as_str() {
        "admin" => Filecoin::auth_new(client, ADMIN.clone()).await?,
        "sign" => Filecoin::auth_new(client, SIGN.clone()).await?,
        "write" => Filecoin::auth_new(client, WRITE.clone()).await?,
        "read" => Filecoin::auth_new(client, READ.clone()).await?,
        _ => return Err(JsonRpcError::INVALID_PARAMS),
    };
    Ok(ret)
}
