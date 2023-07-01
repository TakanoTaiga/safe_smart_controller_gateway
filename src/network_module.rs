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


use network_module_util::key::{NodeConnectionKey , ConnectionBuffer , U8KeyUtil};

use safe_drive::{error::DynError, logger::Logger, pr_info , pr_error};

// --- UDP Socket ---
use async_std::net::UdpSocket;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};


// --- Get Signal ---
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use signal_hook::flag;

pub async fn udp_task() -> Result<(), DynError> {
    // --- Logger ---
    let logger = Logger::new("udp_task");

    // --- UDP Socket ---
    let socket = UdpSocket::bind("0.0.0.0:64201").await?;
    let mut connection_buffer = ConnectionBuffer{
        connection_key: NodeConnectionKey::MissingKey ,
        raw_buffer: [0; 2048] ,
        rcv_size: 0,
        taget_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
    };

    pr_info!(logger, "Listening on {}", socket.local_addr()?);

    // --- Get Signal ---
    let term = Arc::new(AtomicBool::new(false));
    flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term))?;

    loop {
        // --- revive data ---
        let (recv, addr) = socket.recv_from(&mut connection_buffer.raw_buffer).await?;
        connection_buffer.connection_key = connection_buffer.raw_buffer[0].convert_to_enumkey();
        connection_buffer.rcv_size = recv;
        connection_buffer.taget_address.set_ip(addr.ip());
        connection_buffer.taget_address.set_port(64201);
        
        match connection_buffer.connection_key {
            NodeConnectionKey::SearchNode => call_search_node(&logger, &connection_buffer) ,
            NodeConnectionKey::PingRequest => call_ping_request(&logger, &connection_buffer),
            NodeConnectionKey::NodeInfoRequest => call_node_info_request(&logger, &connection_buffer),
            NodeConnectionKey::GamepadValueRequest => call_gamepad_value_request(&logger, &connection_buffer),
            _ => call_unknown_key(&logger, &connection_buffer),
        }

        // --- send data --- 
        socket.send_to(&connection_buffer.raw_buffer, &connection_buffer.taget_address).await?;

        // --- Check Signal ---
        if term.load(Ordering::Relaxed) {
            break;
        }
    }
    Ok(())
}

fn call_search_node(logger: &Logger , connection_buffer: &ConnectionBuffer){
    pr_info!(logger, "Key:{}" , connection_buffer.connection_key);
}

fn call_ping_request(logger: &Logger , connection_buffer: &ConnectionBuffer){
    pr_info!(logger, "Key:{}" , connection_buffer.connection_key);
}

fn call_node_info_request(logger: &Logger , connection_buffer: &ConnectionBuffer){
    pr_info!(logger, "Key:{}" , connection_buffer.connection_key);
}

fn call_gamepad_value_request(logger: &Logger , connection_buffer: &ConnectionBuffer){
    pr_info!(logger, "Key:{}" , connection_buffer.connection_key);
}

fn call_unknown_key(logger: &Logger , connection_buffer: &ConnectionBuffer){
    if let NodeConnectionKey::UnknownKey = connection_buffer.connection_key {
        pr_error!(logger, "Unknown ID:{}" , connection_buffer.raw_buffer[0]);
    }else{
        pr_info!(logger, "Server Key:{}" , connection_buffer.connection_key);
    }
}



// gomi

// let buf = &mut connection_buffer.raw_buffer[..recv];
// let sent = socket.send_to(&buf[..recv], &peer).await?;
// pr_info!(logger, "size: {:?}", recv);
// pr_info!(logger, "message: {:?}", buf[0]);