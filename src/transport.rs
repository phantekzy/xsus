use std::time::Duration;

use crate::{error::XsusError, request::Request};

pub fn execute_network_call(req: &Request, timeout: Duration) -> Result<String, XsusError> {}
