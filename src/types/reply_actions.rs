use crate::handlers::get_text_from_msg;
use crate::InlineKeyBoardHandler;
use teloxide::types::{InlineKeyboardMarkup, Message};
#[derive(Clone, Default, Debug)]
pub struct ImportWallet {
    pub no: u8,
    pub private_key: Option<String>,
}
#[derive(Clone, Default, Debug)]
pub struct SetPasswd {
    pub first_password: Option<String>,
    pub passwd_given_twice: bool,
}
#[derive(Clone, Debug)]
pub struct OrderNo {
    pub wallet_index: usize,
    pub no: String,
}
#[derive(Clone, Default, Debug)]
pub enum CancelOrderStep {
    #[default]
    AskForOrderNo,
    AnswerOrderNo(OrderNo),
}

#[derive(Clone, Default, Debug)]
pub struct MessageToReply {
    pub id: i32,
    pub text: String,
    pub keyboard: InlineKeyboardMarkup,
}

// Assuming the definition of SecretKey and Secp256k1 are available
// pub struct SecretKey<Secp256k1> { /* fields */ }

pub trait ActionImp {
    fn new(msg_id: i32, reply_action: ReplyAction) -> Action;
}
impl ActionImp for Action {
    fn new(msg_id: i32, reply_action: ReplyAction) -> Action {
        Action {
            msg_id,
            reply_action,
        }
    }
}

impl MessageToReply {
    pub fn create_from_msg(msg: &Message) -> anyhow::Result<Self> {
        let text = get_text_from_msg(&msg)?;

        Ok(MessageToReply {
            id: msg.id.0,
            text,
            keyboard: InlineKeyboardMarkup::create_from_msg(msg),
        })
    }
}
#[derive(Clone, Debug)]
pub enum ReplyAction {
    ShowPk,
    ReplaceWallet(u8), // Wallet chosen
    ImportWallet(ImportWallet),
    SetPasswd(SetPasswd), //... potentially other variants
    SetAddress(String, MessageToReply),
    SetTokenName(MessageToReply),
    SetAmountPlain(String, MessageToReply),       //MessageId
    SetAmountPerc(String, MessageToReply),        //MessageId
    SetPositivePerc(MessageToReply),              //MessageId
    SetNegativePerc(MessageToReply),              //MessageId
    SetSlippage(MessageToReply),                  //MessageId
    SetDuration(MessageToReply),                  //MessageId
    Bridge(MessageToReply),                       //MessageId
    Buy(MessageToReply),                          //MessageId
    BuyLimit(MessageToReply),                     //MessageId
    SellLimit(MessageToReply),                    //MessageId
    Sell(MessageToReply),                         //MessageId
    Transfer(MessageToReply),                     //MessageId
    CancelOrder(CancelOrderStep, MessageToReply), //MessageId
}
use crate::handlers::constants_callbacks::*;
use anyhow::{anyhow, Result};

impl ReplyAction {
    //TODO check i i can do it by initiating with message
    pub fn from_str(s: &str, msg: &Message, wallet_no: &str) -> Result<Self> {
        debug!("Replace from str {s} with msg {:?}", msg.text());
        match s {
            SHOW_PK => Ok(Self::ShowPk),
            REPLACE_WALLET => {
                let pk_no: u8 = wallet_no.parse()?;
                Ok(Self::ReplaceWallet(pk_no))
            }
            IMPORT_WALLET => {
                let mut iw_struct = ImportWallet::default();
                iw_struct.no = wallet_no.parse()?;
                Ok(Self::ImportWallet(iw_struct))
            }
            SET_PASSWD => Ok(Self::SetPasswd(SetPasswd::default())),
            /* ADDRESS |*/
            SET_ADDRESS | RECEIVER | TOKEN_TO_SEND | SET_TOKEN_DB => Ok(Self::SetAddress(
                s.to_string(),
                MessageToReply::create_from_msg(msg)?,
            )),
            SET_AMOUNT_PLAIN | AMOUNT_PLAIN => Ok(Self::SetAmountPlain(
                s.to_owned(),
                MessageToReply::create_from_msg(msg)?,
            )),
            SET_TOKEN_NAME => Ok(Self::SetTokenName(MessageToReply::create_from_msg(msg)?)),
            SET_AMOUNT_PERC | AMOUNT_PERC => Ok(Self::SetAmountPerc(
                s.to_string(),
                MessageToReply::create_from_msg(msg)?,
            )),
            SET_NEGATIVE_PERC => Ok(Self::SetNegativePerc(MessageToReply::create_from_msg(msg)?)),
            SET_POSITIVE_PERC => Ok(Self::SetPositivePerc(MessageToReply::create_from_msg(msg)?)),
            SLIPPAGE => Ok(Self::SetSlippage(MessageToReply::create_from_msg(msg)?)),
            SET_DURATION => Ok(Self::SetDuration(MessageToReply::create_from_msg(msg)?)),
            BUY => Ok(Self::Buy(MessageToReply::create_from_msg(msg)?)),
            SELL => Ok(Self::Sell(MessageToReply::create_from_msg(msg)?)),
            TRANSFER => Ok(Self::Transfer(MessageToReply::create_from_msg(msg)?)),
            BRIDGE => Ok(Self::Bridge(MessageToReply::create_from_msg(msg)?)),
            BUY_LIMIT => Ok(Self::BuyLimit(MessageToReply::create_from_msg(msg)?)),
            SELL_LIMIT => Ok(Self::SellLimit(MessageToReply::create_from_msg(msg)?)),
            CANCEL_ORDER => Ok(Self::CancelOrder(
                CancelOrderStep::default(),
                MessageToReply::create_from_msg(msg)?,
            )),
            _ => Err(anyhow!("Wrong reply action: {}", s)),
        }
    }
}

#[derive(Clone)]
pub struct Action {
    pub msg_id: i32,
    pub reply_action: ReplyAction,
}
