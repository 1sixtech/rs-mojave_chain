use drip_chain_node::DRiPNode;

#[tokio::main]
async fn main() {
    match DRiPNode::init().await {
        Ok(handle) => {
            handle.await;
        }
        Err(error) => {
            tracing::error!(error = %error, "Error starting DRiP node");
        }
    }
}
