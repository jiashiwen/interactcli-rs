use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RequestTaskListAll {
    batchSize: usize,
    queryID: String,
}

impl RequestTaskListAll {
    pub fn default() -> Self {
        Self {
            batchSize: 10,
            queryID: "".to_string(),
        }
    }
    pub fn new(batch_size: usize, query_id: String) -> Self {
        Self {
            batchSize: batch_size,
            queryID: query_id,
        }
    }

    pub fn set_batch_size(&mut self, batch_size: usize) {
        self.batchSize = batch_size;
    }

    pub fn set_query_id(&mut self, query_id: String) {
        self.queryID = query_id;
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RequestTaskListByNodeID {
    NodeID: String,
    batchSize: usize,
    queryID: String,
}

impl RequestTaskListByNodeID {
    pub fn default() -> Self {
        Self {
            NodeID: "".to_string(),
            batchSize: 10,
            queryID: "".to_string(),
        }
    }
    pub fn new(node_id: String, batch_size: usize, query_id: String) -> Self {
        Self {
            NodeID: node_id,
            batchSize: batch_size,
            queryID: query_id,
        }
    }

    pub fn set_node_id(&mut self, node_id: String) {
        self.NodeID = node_id;
    }

    pub fn set_batch_size(&mut self, batch_size: usize) {
        self.batchSize = batch_size;
    }

    pub fn set_query_id(&mut self, query_id: String) {
        self.queryID = query_id;
    }
}
