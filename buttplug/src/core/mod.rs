// Buttplug Rust Source Code File - See https://buttplug.io for more info.
//
// Copyright 2016-2020 Nonpolynomial Labs LLC. All rights reserved.
//
// Licensed under the BSD 3-Clause license. See LICENSE file in the project root
// for full license information.

//! Protocol message and error definitions.

pub mod errors;
pub mod messages;

use errors::ButtplugError;
use futures::future::{self, BoxFuture};

pub type ButtplugResult = Result<(), ButtplugError>;
pub type ButtplugResultFuture = BoxFuture<'static, ButtplugResult>;

impl<T> From<ButtplugError> for BoxFuture<'static, Result<T, ButtplugError>>
where
  T: Send + 'static,
{
  fn from(error: ButtplugError) -> BoxFuture<'static, Result<T, ButtplugError>> {
    Box::pin(future::ready(Err(error)))
  }
}
