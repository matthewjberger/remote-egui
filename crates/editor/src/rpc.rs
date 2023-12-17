use broker::Client;
use rpc::{Response, RpcClient};
use ui::contract::{Broker, ClientHandle, Message};

#[cfg(not(target_arch = "wasm32"))]
use rpc::RpcExecutor;

#[cfg(not(target_arch = "wasm32"))]
use crate::app::BackendConnectionStrategy;

pub struct Rpc {
    #[cfg(not(target_arch = "wasm32"))]
    rpc_executor: RpcExecutor,

    #[cfg(not(target_arch = "wasm32"))]
    pub connection_strategy: BackendConnectionStrategy,

    rpc_client: Option<RpcClient>,
    frontend_client: ClientHandle,
    subscribed: bool,
    has_connected: bool,
}

impl Default for Rpc {
    fn default() -> Self {
        Self {
            #[cfg(not(target_arch = "wasm32"))]
            rpc_executor: RpcExecutor,

            #[cfg(not(target_arch = "wasm32"))]
            connection_strategy: BackendConnectionStrategy::Internal,

            frontend_client: Client::with_ring_buffer_size(100),
            rpc_client: None,
            subscribed: false,
            has_connected: false,
        }
    }
}

impl Rpc {
    pub fn new() -> Self {
        Self::default()
    }

    // In wasm, the only valid backend connection strategy is remote
    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_connection_strategy(&mut self, strategy: &BackendConnectionStrategy) {
        self.connection_strategy = *strategy;
    }

    pub fn has_connected(&self) -> bool {
        self.has_connected
    }

    pub fn connect(&mut self, url: &str, wake_up: impl Fn() + Send + Sync + 'static) {
        match ewebsock::connect_with_wakeup(url, wake_up) {
            Ok((ws_sender, ws_receiver)) => {
                self.rpc_client = Some(RpcClient::new(ws_sender, ws_receiver));
                self.has_connected = true;
            }
            Err(error) => {
                log::error!("Failed to connect to {url:?}: {error}");
            }
        }
    }

    pub fn client_available(&mut self) -> bool {
        self.rpc_client.is_some()
    }

    fn subscribe(&mut self, broker: &mut Broker) {
        broker.subscribe(&Message::rpc_command_topic(), &self.frontend_client);
        self.subscribed = true;
    }

    pub fn update(&mut self, broker: &mut Broker) {
        if !self.subscribed {
            self.subscribe(broker);
        }

        let mut messages = Vec::new();
        while let Some(message) = self.frontend_client.borrow_mut().next_message() {
            log::debug!("RPC message: {message:#?}");
            messages.push(message);
        }

        #[cfg(not(target_arch = "wasm32"))]
        messages.into_iter().for_each(|message| {
            if let Message::RpcCommand { id, command } = message {
                match self.connection_strategy {
                    BackendConnectionStrategy::Internal => {
                        log::debug!("Executing internal rpc command: {command:#?}");
                        let result = self.rpc_executor.execute(&id, command);
                        publish_result(broker, &id, result);
                    }
                    BackendConnectionStrategy::Remote => {
                        if let Some(client) = self.rpc_client.as_mut() {
                            client.send(id, command)
                        }
                    }
                }
            }
        });

        if let Some(response) = self.rpc_client.as_mut().and_then(|client| client.receive()) {
            let Response { id, result } = response;
            publish_result(broker, &id, result);
        }
    }
}

fn publish_result(broker: &mut broker::Broker<Message>, id: &str, result: rpc::RpcResult) {
    broker.publish(
        &Message::rpc_result_topic(id),
        Message::RpcResult { result },
    );
}
