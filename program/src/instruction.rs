use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum ChatInstruction {
    SendMessage { message: String, reciever: String },
    GetMessages,
}
