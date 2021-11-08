use crate::request::requestmodules::{RequestTaskListAll, RequestTaskListByNodeID};
use reqwest::Client;
use reqwest::Response;
use serde_json::{Map, Value};
use std::fmt::Debug;
use url::Url;

//api path const
const API_LOGIN: &str = "/login";
const API_TASK_CREATE: &str = "/api/task/create";
const API_TASK_START: &str = "/api/task/start";
const API_TASK_STOP: &str = "/api/task/stop";
const API_TASK_REMOVE: &str = "/api/task/remove";
const API_TASK_LIST_ALL: &str = "/api/task/listall";
const API_TASK_LIST_BY_IDS: &str = "/api/task/listbyids";
const API_TASK_LIST_BY_NAMES: &str = "/api/task/listbynames";
const API_TASK_LIST_BY_GROUPIDS: &str = "/task/listbygroupids";
const API_TASK_LIST_BY_NODE: &str = "/api/task/listbynode";

const API_IMPORT_FILE_PATH: &str = "/api/v2/file/createtask";
const API_NODE_LIST_ALL: &str = "/api/node/listall";

#[derive(Debug)]
pub enum ResponseError {
    OptionError(String),
}

pub type Result<T, E = ResponseError> = std::result::Result<T, E>;

#[derive(Default, Debug)]
pub struct Request {
    client: reqwest::Client,
    server: String,
}

impl Request {
    pub fn new(server: String) -> Self {
        Self {
            client: Client::default(),
            server,
        }
    }

    pub async fn send(&self, url: Url, body: String) -> Result<Response> {
        let resp = self.client.post(url).body(body).send().await.map_err(|e| {
            return ResponseError::OptionError(e.to_string());
        })?;
        Result::Ok(resp)
    }
}

impl Request {
    pub async fn create_task(&self, body: String) -> Result<Response> {
        let mut server = self.server.clone();
        server.push_str(API_TASK_CREATE);
        let url = Url::parse(server.as_str()).map_err(|e| {
            return ResponseError::OptionError(e.to_string());
        })?;
        let resp = self.send(url, body).await?;
        Result::Ok(resp)
    }

    pub async fn node_list_all(&self) -> Result<Response> {
        let mut server = self.server.clone();
        server.push_str(API_NODE_LIST_ALL);

        let url = Url::parse(server.as_str()).map_err(|e| {
            return ResponseError::OptionError(e.to_string());
        })?;
        let resp = self.send(url, "".to_string()).await?;
        Result::Ok(resp)
    }

    pub async fn task_list_all(&self, module: RequestTaskListAll) -> Result<Response> {
        let body = serde_json::to_string(&module).map_err(|e| {
            return ResponseError::OptionError(e.to_string());
        })?;
        let mut server = self.server.clone();
        server.push_str(API_TASK_LIST_ALL);
        let url = Url::parse(server.as_str()).map_err(|e| {
            return ResponseError::OptionError(e.to_string());
        })?;
        let resp = self.send(url, body).await?;
        Result::Ok(resp)
    }

    pub async fn task_list_by_groupids(&self, groupids: Vec<&str>) -> Result<Response> {
        let mut server = self.server.clone();
        server.push_str(API_TASK_LIST_BY_GROUPIDS);
        let url = Url::parse(server.as_str()).map_err(|e| {
            return ResponseError::OptionError(e.to_string());
        })?;
        let mut map = Map::new();
        map.insert("groupIDs".to_string(), Value::from(groupids));
        let json = Value::Object(map);
        let resp = self.send(url, json.to_string()).await?;
        Result::Ok(resp)
    }

    pub async fn task_list_by_ids(&self, ids: Vec<String>) -> Result<Response> {
        let mut server = self.server.clone();
        server.push_str(API_TASK_LIST_BY_IDS);
        let url = Url::parse(server.as_str()).map_err(|e| {
            return ResponseError::OptionError(e.to_string());
        })?;
        let mut map = Map::new();
        map.insert("taskIDs".to_string(), Value::from(ids));
        let json = Value::Object(map);
        let resp = self.send(url, json.to_string()).await?;
        Result::Ok(resp)
    }

    pub async fn task_list_by_names(&self, names: Vec<&str>) -> Result<Response> {
        let mut server = self.server.clone();
        server.push_str(API_TASK_LIST_BY_NAMES);
        let url = Url::parse(server.as_str()).map_err(|e| {
            return ResponseError::OptionError(e.to_string());
        })?;
        let mut map = Map::new();
        map.insert("taskNames".to_string(), Value::from(names));
        let json = Value::Object(map);
        let resp = self.send(url, json.to_string()).await?;
        Result::Ok(resp)
    }

    pub async fn task_list_by_nodeids(&self, module: RequestTaskListByNodeID) -> Result<Response> {
        let mut url_str = self.server.clone();
        url_str.push_str(API_TASK_LIST_BY_NODE);
        let url = Url::parse(url_str.as_str()).map_err(|e| {
            return ResponseError::OptionError(e.to_string());
        })?;
        let body = serde_json::to_string(&module).map_err(|e| {
            return ResponseError::OptionError(e.to_string());
        })?;
        let resp = self.send(url, body).await?;
        Result::Ok(resp)
    }

    pub async fn task_remove(&self, task_id: String) -> Result<Response> {
        let mut server = self.server.clone();
        server.push_str(API_TASK_REMOVE);
        let url = Url::parse(server.as_str()).map_err(|e| {
            return ResponseError::OptionError(e.to_string());
        })?;
        let mut map = Map::new();
        map.insert("taskID".to_string(), Value::from(task_id));
        let json = Value::Object(map);
        let resp = self.send(url, json.to_string()).await?;
        Result::Ok(resp)
    }

    pub async fn task_start(&self, task_id: String) -> Result<Response> {
        let mut server = self.server.clone();
        server.push_str(API_TASK_START);
        let url = Url::parse(server.as_str()).map_err(|e| {
            return ResponseError::OptionError(e.to_string());
        })?;
        let mut map = Map::new();
        map.insert("taskID".to_string(), Value::from(task_id));
        let json = Value::Object(map);
        let resp = self.send(url, json.to_string()).await?;
        Result::Ok(resp)
    }

    pub async fn task_stop(&self, task_id: String) -> Result<Response> {
        let mut server = self.server.clone();
        server.push_str(API_TASK_STOP);
        let url = Url::parse(server.as_str()).map_err(|e| {
            return ResponseError::OptionError(e.to_string());
        })?;
        let mut map = Map::new();
        map.insert("taskID".to_string(), Value::from(task_id));
        let json = Value::Object(map);
        let resp = self.send(url, json.to_string()).await?;
        Result::Ok(resp)
    }
}

pub async fn get_baidu() -> Result<Response> {
    let client = Client::default();
    let url = Url::parse("https://www.baidu.com").map_err(|e| {
        return ResponseError::OptionError(e.to_string());
    })?;
    let resp = client.get(url).send().await.map_err(|e| {
        return ResponseError::OptionError(e.to_string());
    })?;
    Result::Ok(resp)
}
