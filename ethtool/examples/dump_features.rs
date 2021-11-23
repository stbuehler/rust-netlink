// SPDX-License-Identifier: MIT

use ethtool;
use futures::stream::TryStreamExt;
use tokio;

// Once we find a way to load netsimdev kernel module in CI, we can convert this
// to a test
fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()
        .unwrap();
    rt.block_on(get_feature(None));
}

async fn get_feature(iface_name: Option<&str>) {
    let (connection, mut handle, _) = ethtool::new_connection().unwrap();
    tokio::spawn(connection);

    let mut feature_handle = handle.feature().get(iface_name).execute().await;

    let mut msgs = Vec::new();
    while let Some(msg) = feature_handle.try_next().await.unwrap() {
        msgs.push(msg);
    }
    assert!(msgs.len() > 0);
    for msg in msgs {
        println!("{:?}", msg);
    }
}