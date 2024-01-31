use std::collections::HashMap;

use crate::api::v1::*;

use sysinfo::{Components, Disks, Networks, System};
use tonic::{async_trait, Request, Response, Status};
use serde::{Deserialize, Serialize};

use super::v1::analyzer_service_server::AnalyzerService;
#[derive(Debug, Default)]
pub struct AnalyzerImpl {}
#[derive(Serialize, Deserialize, Debug)]
struct ResponseFormat {
    sysInfo: HashMap<String,String>
}
#[async_trait]
impl AnalyzerService for AnalyzerImpl {

    async fn run(&self, request: Request<AnalyzerRunRequest>) -> std::result::Result<Response<AnalyzerRunResponse>, Status> {
        // do the work, in this case, analyzer some OS information
        let mut sys = System::new_all();
        sys.refresh_all();
        println!("total memory: {} bytes", sys.total_memory());
        println!("used memory : {} bytes", sys.used_memory());
        println!("total swap  : {} bytes", sys.total_swap());
        println!("used swap   : {} bytes", sys.used_swap());

        // Display system information:
        println!("System name:             {:?}", System::name());
        println!("System kernel version:   {:?}", System::kernel_version());
        println!("System OS version:       {:?}", System::os_version());
        println!("System host name:        {:?}", System::host_name());

        // Pack the response data with information we care about...
        let mut sys_info_map = HashMap::new();
        sys_info_map.insert("total memory".to_string(), format!(" {} bytes", sys.total_memory()));
        sys_info_map.insert("used memory".to_string(), format!(" {} bytes", sys.used_memory()));
        sys_info_map.insert("total swap".to_string(), format!(" {} bytes", sys.total_swap()));
        sys_info_map.insert("used swap".to_string(), format!(" {} bytes", sys.used_swap()));
        sys_info_map.insert("System OS version".to_string(), format!(" {:?}", System::os_version()));

        let resp = ResponseFormat{
            sysInfo: sys_info_map,
        };
        let serialized = serde_json::to_string(&resp).unwrap();


        Ok(Response::new(AnalyzerRunResponse{ result: Some(Result{
            kind: "HostAnalzyer".to_string(),
            name: "".to_string(),
            error: vec![],
            details: serialized,
            parent_object: "".to_string(),
        }) }))
    }
}
