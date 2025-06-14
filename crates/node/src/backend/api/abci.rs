use crate::{
    backend::{error::BackendError, Backend},
    service::{AbciRequest, AbciResponse},
};
use drip_chain_abci::{api::AbciApi, types::*};

impl AbciApi for Backend {
    /// TODO: Validate the transaction (Signature, Nonce, Balance, ETC..) after ethrex integration.
    fn check_tx(&self, request: RequestCheckTx) -> ResponseCheckTx {
        let receiver = self.abci_service().send(self.clone(), request);

        // # Safety
        // Very unlikely that the sender drops before sending a response.
        match receiver.blocking_recv() {
            Ok(AbciResponse::CheckTx(response)) => response,
            _others => panic!("Failed to check the transaction"),
        }
    }

    fn finalize_block(&self, request: RequestFinalizeBlock) -> ResponseFinalizeBlock {
        let receiver = self.abci_service().send(self.clone(), request);

        // # Safety
        // Very unlikely that the sender drops before sending a response.
        match receiver.blocking_recv() {
            Ok(AbciResponse::FinalizeBlock(response)) => response,
            _others => panic!("Failed to finalize the block"),
        }
    }

    fn commit(&self) -> ResponseCommit {
        let receiver = self.abci_service().send(self.clone(), AbciRequest::Commit);

        // # Safety
        // Very unlikely that the sender drops before sending a response.
        match receiver.blocking_recv() {
            Ok(AbciResponse::Commit(response)) => response,
            _others => panic!("Failed to commit"),
        }
    }
}

impl Backend {
    pub async fn check_transaction(&self, _request: RequestCheckTx) -> ResponseCheckTx {
        ResponseCheckTx::default()
    }

    pub async fn do_finalize_block(&self, _request: RequestFinalizeBlock) -> ResponseFinalizeBlock {
        // let events: Vec<Event> = request
        //     .txs
        //     .iter()
        //     .map(|_| Event {
        //         r#type: "".to_owned(),
        //         attributes: vec![],
        //     })
        //     .collect();
        // ResponseFinalizeBlock {
        //     events,
        //     ..Default::default()
        // }
        ResponseFinalizeBlock::default()
    }

    pub async fn do_commit(&self) -> Result<ResponseCommit, BackendError> {
        self.evm_client().mine_one().await;
        // # Safety
        // Block is guaranteed to exist as long as mine_one() succeeds.

        let block = self
            .evm_client()
            .block_by_number_full(drip_chain_types::rpc::BlockNumberOrTag::Latest)
            .await?;

        match block {
            Some(block) => {
                let full_block = block.into_inner();
                let new_head = full_block.header;
                self.pubsub_service().publish_new_head(new_head);
                Ok(ResponseCommit::default())
            }
            None => Err(BackendError::BlockNone),
        }
    }
}
