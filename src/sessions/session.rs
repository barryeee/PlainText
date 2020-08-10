use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use crate::game_object::{GameObjectId, GameObjectMapReader};
use crate::transaction_writer::TransactionWriter;

enum SessionState {
    SessionClosed,
    SigningIn,
    SignedIn,
}

pub struct Session {
    pub id: u64,
    game_object_reader: GameObjectMapReader,
    player_id: Option<GameObjectId>,
    socket: TcpStream,
    state: SessionState,
    transaction_writer: TransactionWriter,
}

impl Session {
    pub fn new(
        id: u64,
        socket: TcpStream,
        game_object_reader: GameObjectMapReader,
        transaction_writer: TransactionWriter,
    ) -> Self {
        Self {
            game_object_reader,
            id,
            player_id: None,
            socket,
            state: SessionState::SigningIn,
            transaction_writer,
        }
    }

    pub async fn send(&mut self, data: String) {
        if let Err(error) = self.socket.write(data.as_bytes()).await {
            println!("Could not send data over socket: {:?}", error);
        }
    }
}
