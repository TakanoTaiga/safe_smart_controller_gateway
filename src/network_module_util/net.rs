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

use pnet::datalink;
use std::net::IpAddr;

pub fn get_ip() -> Vec<u8> {
    for interface in datalink::interfaces() {
        // not empy and is active
        if !interface.ips.is_empty() && interface.is_up() {
            for ip_net in interface.ips {
                // not loopback is ipv4
                if ip_net.is_ipv4() && !ip_net.ip().is_loopback() {
                    match ip_net.ip() {
                        IpAddr::V4(ipv4) => return ipv4.octets().to_vec(),
                        IpAddr::V6(ipv6) => return ipv6.octets().to_vec(),
                    }
                }
            }
        }
    };

    let vec = Vec::new();
    return vec;
}
