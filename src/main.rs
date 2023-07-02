// Copyright 2023 Hakoroboken
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod network_module;

use safe_drive::{
    //context::Context, 
    error::DynError, 
    //logger::Logger, 
    //pr_info, 
    //topic::publisher::Publisher
};
use async_std::net::UdpSocket;
use async_std::channel::unbounded;

#[async_std::main]
async fn main() -> Result<(), DynError> {
    // ---- safe drive ----
    //let ctx = Context::new()?;
    //let node = ctx.create_node("my_talker", None, Default::default())?;
    //let publisher = node.create_publisher::<remote_control_msgs::msg::Button>("my_topic", None)?;

    // Create a logger.
    //let logger = Logger::new("my_talker"); 

    let (search_locker_send , search_locker_rcv) = unbounded();

    let (target_info_send , target_info_rcv) = unbounded();

    let main_udp_socket = UdpSocket::bind("0.0.0.0:64201").await?;
    let search_app_socket = UdpSocket::bind("0.0.0.0:0").await?;
    let ping_socket = UdpSocket::bind("0.0.0.0:0").await?;

    let main_udp_service_task = async_std::task::spawn( 
        network_module::main_udp_service(main_udp_socket , search_locker_send , target_info_send));
    let search_app_task = async_std::task::spawn(
        network_module::search_app(search_app_socket , search_locker_rcv));
    let ping_task = async_std::task::spawn(
        network_module::ping_app(ping_socket , target_info_rcv));

    main_udp_service_task.await?;
    search_app_task.await?;
    ping_task.await?;
    
    Ok(())
}

