use super::{
  ButtplugDeviceResultFuture,
  ButtplugProtocol,
  ButtplugProtocolCommandHandler,
  ButtplugProtocolCreator,
};
use crate::{
  core::messages::{self, ButtplugDeviceCommandMessageUnion, MessageAttributesMap},
  device::{
    protocol::{generic_command_manager::GenericCommandManager, ButtplugProtocolProperties},
    DeviceImpl,
    DeviceWriteCmd,
    Endpoint,
  },
};
use std::sync::Arc;

#[derive(ButtplugProtocol, ButtplugProtocolCreator, ButtplugProtocolProperties)]
pub struct Realov {
  name: String,
  message_attributes: MessageAttributesMap,
  stop_commands: Vec<ButtplugDeviceCommandMessageUnion>,
}

impl Realov {
  pub(super) fn new(name: &str, message_attributes: MessageAttributesMap) -> Self {
    let manager = GenericCommandManager::new(&message_attributes);

    Self {
      name: name.to_owned(),
      message_attributes,
      stop_commands: manager.get_stop_commands(),
    }
  }
}

impl ButtplugProtocolCommandHandler for Realov {
  fn handle_vibrate_cmd(
    &self,
    device: Arc<Box<dyn DeviceImpl>>,
    msg: messages::VibrateCmd,
  ) -> ButtplugDeviceResultFuture {
    // TODO Convert to using generic command manager
    let speed = (msg.speeds[0].speed * 50.0) as u8;
    let msg = DeviceWriteCmd::new(Endpoint::Tx, [0xc5, 0x55, speed, 0xaa].to_vec(), false);
    let fut = device.write_value(msg);
    Box::pin(async {
      fut.await?;
      Ok(messages::Ok::default().into())
    })
  }
}

// TODO Write Tests
