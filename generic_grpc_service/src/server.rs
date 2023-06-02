/*
 * Copyright (C) 2022.  All rights reserved.
 *
 * Vijaya Manohar Dogiparthi <v.m.dogiparthi@tudelft.nl>
 *
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 * IN NO EVENT SHALL XILINX  BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
 * WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 *
 */

#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
//#![allow(dead_code, unused_imports)]
use adac_remote::adac_remote_server::{AdacRemote, AdacRemoteServer};
use adac_remote::{
    GetAdcVoltageRequest, GetAdcVoltageResponse, SetDacVoltageRequest, SetDacVoltageResponse,
};
use if_addrs::get_if_addrs;
use std::net::{IpAddr, Ipv4Addr};
use tonic::{transport::Server, Request, Response, Status};

//use dac::set_dac_voltage;

pub mod adac_remote {
    tonic::include_proto!("adac_remote");
}

#[derive(Debug, Default)]
pub struct AdacRemoteStruct {}

#[tonic::async_trait]
impl AdacRemote for AdacRemoteStruct {
    async fn set_dac_voltage(
        &self,
        request: Request<SetDacVoltageRequest>,
    ) -> Result<Response<SetDacVoltageResponse>, Status> {
        let voltage = request.get_ref().voltage;
        let channel_num = request.get_ref().dac_channel;
        let verbosity_level = request.get_ref().verbosity_level;

        // Create a Status message

        let rc: Result<i32, String> = Ok(42);

        //let rc = dac::set_dac_voltage(channel_num, voltage, verbosity_level);

        let result: bool = match rc {
            Ok(_) => true,
            Err(_error) => false,
        };

        let dac_response = adac_remote::Status {
            success: if result == true { true } else { false },
            code: if result == true { 200 } else { 409 },
            reason: if result == true {
                format!(
                    "Voltage: {} set on Dac Channel # {} successful!",
                    voltage, channel_num
                )
            } else {
                format!(
                    "Voltage: {} set on Dac Channel # {} is not successful!",
                    voltage, channel_num
                )
            },
        };

        // Process the request and generate a response
        let response = adac_remote::SetDacVoltageResponse {
            status: Some(dac_response),
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let interface_name = "eth0";
    let mut ip: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    // for iface in get_if_addrs().unwrap() {
    //     if !iface.is_loopback() && iface.name == interface_name {
    //         ip = iface.ip();
    //     }
    // }

    //Enable this to test local address
    //let addr = "[::1]:50051";

    // Enable this to test on the RFSoC
    //let ip_addr = ip.to_string() + ":50051";
    let ip_addr = ip.to_string() + ":50051";
    let addr = ip_addr.parse().unwrap();

    let server = AdacRemoteStruct::default();

    Server::builder()
        .add_service(AdacRemoteServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}
