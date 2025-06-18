use mojave_chain_node::MojaveChainNode;

#[tokio::main]
async fn main() {
    match MojaveChainNode::init().await {
        Ok(handle) => {
            handle.await;
        }
        Err(error) => {
            tracing::error!(error = %error, "Error starting DRiP node");
        }
    }
}
