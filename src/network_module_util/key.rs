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

use std::{fmt::{Display , Formatter}, net::SocketAddr};

pub enum NodeConnectionKey {
    SearchApp,
    PingRequest,
    SearchAppResponse,
    PingResponse,
    DataValue,
    UnknownKey,
}

impl Display for NodeConnectionKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<() , std::fmt::Error> {
        match self {
            NodeConnectionKey::SearchApp => write!(f , "SearchApp"),
            NodeConnectionKey::PingRequest => write!(f , "PingRequest"),
            NodeConnectionKey::SearchAppResponse => write!(f , "SearchAppResponse"),
            NodeConnectionKey::PingResponse => write!(f , "PingResponse"),
            NodeConnectionKey::DataValue => write!(f , "DataValue"),
            _ => write!(f , "Unknown Key"),
        }
    }
}

pub trait EnumKeyUtil {
    fn convert_to_u8key(&self)->u8;
}
impl EnumKeyUtil for NodeConnectionKey {
    fn convert_to_u8key(&self)->u8{
        match self {
            NodeConnectionKey::SearchApp => 0xC9,
            NodeConnectionKey::PingRequest => 0xCA,
            NodeConnectionKey::SearchAppResponse => 0xCB,
            NodeConnectionKey::PingResponse => 0xCC,
            NodeConnectionKey::DataValue => 0xCD,
            _ => 0,
        }
    }
}

pub trait U8KeyUtil {
    fn convert_to_enumkey(&self) -> NodeConnectionKey;
}
impl U8KeyUtil for u8 {
    fn convert_to_enumkey(&self) -> NodeConnectionKey{
        match self {
            0xC9 => NodeConnectionKey::SearchApp,
            0xCA => NodeConnectionKey::PingRequest,
            0xCB => NodeConnectionKey::SearchAppResponse,
            0xCC => NodeConnectionKey::PingResponse,
            0xCD => NodeConnectionKey::DataValue,
            _ => NodeConnectionKey::UnknownKey,
        }
    }
}

pub struct ConnectionBuffer {
    pub connection_key: NodeConnectionKey,
    pub raw_buffer: [u8; 2048],
    pub rcv_size: usize,
    pub taget_address: SocketAddr
}
