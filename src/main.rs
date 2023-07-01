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
    context::Context, error::DynError, logger::Logger, pr_info, topic::publisher::Publisher
};

#[async_std::main]
async fn main() -> Result<(), DynError> {
    // ---- safe drive ----
    //let ctx = Context::new()?;
    //let node = ctx.create_node("my_talker", None, Default::default())?;
    //let publisher = node.create_publisher::<remote_control_msgs::msg::Button>("my_topic", None)?;

    // Create a logger.
    //let logger = Logger::new("my_talker");

    let udp_task = async_std::task::spawn(network_module::udp_task());

    udp_task.await?;
    Ok(())
}

