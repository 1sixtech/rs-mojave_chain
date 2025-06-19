//use mojave_chain_types::rpc::FilterId;
//use std::{collections::HashMap, sync::Arc};
//use tokio::sync::Mutex;

//pub struct EthFilter {
//    inner: Arc<Mutex<HashMap<FilterId, Filter>>>,
//}

//impl Clone for EthFilter {
//    fn clone(&self) -> Self {
//        Self {
//            inner: self.inner.clone(),
//        }
//    }
//}

//impl Default for EthFilter {
//    fn default() -> Self {
//        Self {
//            inner: Arc::new(Mutex::new(HashMap::default())),
//        }
//    }
//}

//impl EthFilter {}

//pub struct Filter {}
