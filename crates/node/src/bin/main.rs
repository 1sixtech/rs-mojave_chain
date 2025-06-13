use mohave_chain_node::MohaveChainNode;

#[tokio::main]
async fn main() {
    match MohaveChainNode::init().await {
        Ok(handle) => {
            handle.await;
        }
        Err(error) => {
            tracing::error!(error = %error, "Error starting DRiP node");
        }
    }
}
