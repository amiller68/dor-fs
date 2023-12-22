// use dor_store::prelude::{IpfsApi, IpfsClient, IpfsError};

// use futures_util::TryStreamExt;
// use serde_json::json;

// #[tokio::test]
// async fn block_put_block_get() -> Result<(), IpfsError> {
//     let client = IpfsClient::default();

//     let json = json!({
//         "hello": "world"
//     });

//     let bytes_in = serde_json::to_vec(&json).unwrap();
//     let byte_in_reader = std::io::Cursor::new(bytes_in.clone());

//     // Write the bytes out to ipfs
//     let bytes_out_hash = client.block_put(byte_in_reader).await?;

//     // Read the bytes back
//     let bytes_out_stream = client.block_get(&bytes_out_hash.key);
//     let bytes_out = bytes_out_stream
//         .map_ok(|chunk| chunk.to_vec())
//         .try_concat()
//         .await?;

//     assert_eq!(bytes_in, bytes_out);
//     let json_out: serde_json::Value = serde_json::from_slice(&bytes_out).unwrap();
//     assert_eq!(json, json_out);

//     Ok(())
// }
