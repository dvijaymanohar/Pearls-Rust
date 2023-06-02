#![allow(dead_code, unused_imports)]

use adac_remote::adac_remote_client::AdacRemoteClient;

use adac_remote::{
    GetAdcVoltageRequest, GetAdcVoltageResponse, SetDacVoltageRequest, SetDacVoltageResponse,
};

pub mod adac_remote {
    tonic::include_proto!("adac_remote");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = AdacRemoteClient::connect("http://[::1]:50051").await?;

    //let mut client = AdacRemoteClient::connect("http://192.168.1.100:50051").await?;

    let request = tonic::Request::new(SetDacVoltageRequest {
        dac_channel: 0,
        voltage: 4.5,
        verbosity_level: true,
    });

    let response = client.set_dac_voltage(request).await?;

    println!("RESPONSE={:?}", response);

    let request = tonic::Request::new(GetAdcVoltageRequest {
        adc_channel: 0,
        verbosity_level: true,
    });

    // let response = client.get_adc_voltage(request).await?;

    // println!("RESPONSE={:?}", response);

    Ok(())
}
