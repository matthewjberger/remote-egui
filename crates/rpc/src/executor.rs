use crate::{Command, Id, RpcResult};

#[derive(Default)]
pub struct RpcExecutor;

impl RpcExecutor {
    pub fn execute(&mut self, _id: &Id, command: Command) -> RpcResult {
        log::info!("Executing an RPC command: {command:#?}");
        match command {
            Command::Example => RpcResult::default(),
        }
    }
}
