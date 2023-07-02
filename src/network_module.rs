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


use network_module_util::key::{NodeConnectionKey , ConnectionBuffer , U8KeyUtil , EnumKeyUtil};
use network_module_util::net;

use safe_drive::{error::DynError, logger::Logger, pr_info , pr_error};

use std::os::unix::prelude::OsStringExt;
use std::time::Duration;

// --- UDP Socket ---
use async_std::net::UdpSocket;
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use gethostname::gethostname;

// --- Inter Thread Commnunication
use async_std::channel::{Sender , Receiver};

// --- Get Signal ---
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use signal_hook::flag;

pub async fn main_udp_service(
    socket: UdpSocket,
     lock_search: Sender<bool>,
     ipaddr_send: Sender<SocketAddr>,
    ) -> Result<(), DynError> {
    // --- Logger ---
    let logger = Logger::new("udp_task");

    // --- UDP Socket ---
    let mut connection_buffer = ConnectionBuffer{
        connection_key: NodeConnectionKey::UnknownKey ,
        raw_buffer: [0; 2048] ,
        rcv_size: 0,
        taget_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
    };

    pr_info!(logger, "Listening on {}", socket.local_addr()?);

    // --- Get Signal ---
    let term = Arc::new(AtomicBool::new(false));
    flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term))?;

    loop {
        pr_info!(logger, "loop");
        // --- revive data ---
        let (recv, addr) = socket.recv_from(&mut connection_buffer.raw_buffer).await.unwrap();
        connection_buffer.connection_key = connection_buffer.raw_buffer[0].convert_to_enumkey();
        connection_buffer.rcv_size = recv;
        connection_buffer.taget_address.set_ip(addr.ip());
        connection_buffer.taget_address.set_port(64201);
        
        pr_info!(logger, "Key:{}" , connection_buffer.connection_key);
        match connection_buffer.connection_key {
            NodeConnectionKey::SearchAppResponse => {
                lock_search.send(true).await?;
                ipaddr_send.send(connection_buffer.taget_address).await?;
            },
            NodeConnectionKey::PingResponse => {
                lock_search.send(true).await?;
            },
            NodeConnectionKey::GamepadValue => {
                
            },
            _ => {
                pr_error!(logger, "Unknown ID:{}" , connection_buffer.raw_buffer[0]);
            },
        }

        //--- Check Signal ---
        if term.load(Ordering::Relaxed) {
            break;
        }
    }
    Ok(())
}


pub async fn search_app(socket: UdpSocket , locker: Receiver<bool> ) -> Result<(), DynError>{
    let logger = Logger::new("search_app");
    socket.set_broadcast(true)?;

    // --- create packet ---
    let mut buffer: Vec<u8> = vec![NodeConnectionKey::SearchApp.convert_to_u8key()];
    buffer.push(0);
    buffer.append(&mut net::get_ip());
    let ip_port: u16 = 64201;
    buffer.append(&mut ip_port.to_le_bytes().to_vec());
    buffer.append(&mut gethostname().into_vec());

    buffer.resize(24, 0);

    // --- Get Signal ---
    let term = Arc::new(AtomicBool::new(false));
    flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term))?;

    loop {
        if locker.try_recv() == Ok(true){
            async_std::task::sleep(Duration::from_millis(1000)).await;
            continue;
        }
        pr_info!(logger, "search_app_task");
        socket.send_to(&buffer, &SocketAddr::new(IpAddr::V4(Ipv4Addr::new(255, 255, 255, 255)), 64201)).await?;
        async_std::task::sleep(Duration::from_millis(500)).await; 

        //--- Check Signal ---
        if term.load(Ordering::Relaxed) {
            break;
        }
    }
    Ok(())
}


pub async fn ping_app(socket: UdpSocket , target_info_rcv: Receiver<SocketAddr>) -> Result<(), DynError>{
    let logger = Logger::new("ping_app");

    loop {
        let mut  addr= target_info_rcv.recv().await?;

        loop {
            let mut buffer: Vec<u8> = vec![NodeConnectionKey::PingRequest.convert_to_u8key()];
            buffer.push(0);
            socket.send_to(&buffer, addr).await?;
            pr_info!(logger, "ping send");

            async_std::task::sleep(Duration::from_millis(500)).await;

            match target_info_rcv.try_recv() {
                Ok(socket_addr) => {
                    addr = socket_addr;
                }
                Err(_) => {}
            } 
        }
    }
}