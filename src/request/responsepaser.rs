use prettytable::{cell, row, Cell, Row, Table};
use reqwest::Response;
use serde_json::{to_string_pretty, Value};

use super::req;

pub struct ReqResult {
    result: req::Result<Response>,
}

impl ReqResult {
    pub fn new(result: req::Result<Response>) -> Self {
        Self { result }
    }
}

impl ReqResult {
    //处理一般的response，只解析json 并打印错误
    pub async fn normal_parsor(self) {
        match self.result {
            Ok(resp) => match resp.text().await {
                Ok(body) => match serde_json::from_str::<Value>(body.clone().as_str()) {
                    Ok(v) => match serde_json::to_string_pretty(&v) {
                        Ok(str) => {
                            println!("{}", str);
                        }
                        Err(e) => println!("{}", e),
                    },
                    Err(e) => {
                        println!("{}", e)
                    }
                },
                Err(e) => {
                    println!("{}", e);
                }
            },
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
    pub async fn task_list_all_parsor(self) {
        match self.result {
            Ok(resp) => {
                if let Ok(body) = resp.text().await {
                    match serde_json::from_str::<Value>(body.clone().as_str()) {
                        Ok(body) => {
                            // assert!(body["errors"].is_null());
                            if body["errors"].is_null() {
                                let mut table = Table::new();
                                table.add_row(row!["taskID", "source", "target", "status"]);
                                // println!("taskStatus is array: {:?}", body["taskStatus"].as_array());
                                if let Some(valuse) = body["taskStatus"].as_array() {
                                    for iterm in valuse {
                                        let taskid = iterm["taskId"].as_str().unwrap();
                                        let source = iterm["taskStatus"]["sourceRedisAddress"]
                                            .as_str()
                                            .unwrap();
                                        let target = iterm["taskStatus"]["targetRedisAddress"]
                                            .as_str()
                                            .unwrap();
                                        let status =
                                            iterm["taskStatus"]["status"].as_i64().unwrap();
                                        table.add_row(Row::new(vec![
                                            Cell::new(taskid),
                                            Cell::new(source),
                                            Cell::new(target),
                                            Cell::new(status.to_string().as_str()),
                                        ]));
                                    }
                                };

                                // Print the table to stdout
                                table.printstd();
                                println!("query ID: {}", body["queryID"]);
                                println!("current Page: {}", body["currentPage"]);
                                println!("is last page: {}", body["lastPage"]);
                            } else {
                                match serde_json::to_string_pretty(&body) {
                                    Ok(str) => {
                                        println!("{}", str);
                                    }
                                    Err(e) => println!("{}", e),
                                }
                            }
                        }
                        Err(e) => {
                            println!("{}", e.to_string());
                        }
                    }
                };
            }
            Err(e) => {
                // error!("{:?}", e);
                println!("{:?}", e)
            }
        }
    }

    pub async fn task_list_byid_parsor(self) {
        match self.result {
            Ok(resp) => {
                // println!("{:?}", resp);
                if let Ok(body) = resp.text().await {
                    match serde_json::from_str::<Value>(body.clone().as_str()) {
                        Err(e) => {
                            println!("{}", e.to_string());
                        }
                        Ok(body) => {
                            if let Some(tasks) = body["result"].as_array() {
                                println!("{}", to_string_pretty(&tasks.get(0)).unwrap());
                            }
                        }
                    }
                }
            }
            Err(e) => {
                println!("{:?}", e)
            }
        }
    }

    pub async fn task_list_bynames_parsor(self) {
        match self.result {
            Ok(resp) => {
                if let Ok(body) = resp.text().await {
                    match serde_json::from_str::<Value>(body.clone().as_str()) {
                        Err(e) => {
                            println!("{}", e.to_string());
                        }
                        Ok(body) => {
                            if let Some(tasks) = body["result"].as_array() {
                                let mut table = Table::new();
                                table.add_row(row![
                                    "taskName", "taskID", "source", "target", "status"
                                ]);
                                for task in tasks {
                                    if task["errors"].is_null() {
                                        let taskname = task["taskName"].as_str().unwrap();
                                        let taskid = task["taskStatus"]["taskId"].as_str().unwrap();
                                        let source = task["taskStatus"]["sourceRedisAddress"]
                                            .as_str()
                                            .unwrap();
                                        let target = task["taskStatus"]["targetRedisAddress"]
                                            .as_str()
                                            .unwrap();
                                        let status = task["taskStatus"]["status"].as_i64().unwrap();
                                        table.add_row(Row::new(vec![
                                            Cell::new(taskname),
                                            Cell::new(taskid),
                                            Cell::new(source),
                                            Cell::new(target),
                                            Cell::new(status.to_string().as_str()),
                                        ]));
                                    }
                                }
                                table.printstd();
                            }
                        }
                    }
                }
            }
            Err(e) => {
                println!("{:?}", e)
            }
        }
    }
}
