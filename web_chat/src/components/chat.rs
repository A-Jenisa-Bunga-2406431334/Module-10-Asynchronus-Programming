use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;
use crate::{services::websocket::WebsocketService, User};

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    _producer: Box<dyn Bridge<EventBus>>,
    wss: WebsocketService,
    messages: Vec<MessageData>,
    current_user: String,
}
impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        if let Ok(_) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
        {
            log::debug!("message sent successfully");
        }

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            current_user: username,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                            .map(|u| UserProfile {
                                name: u.into(),
                                avatar: format!(
                                    "https://avatars.dicebear.com/api/adventurer-neutral/{}.svg",
                                    u
                                )
                                .into(),
                            })
                            .collect();
                        return true;
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(input.value()),
                        data_array: None,
                    };
                    if let Err(e) = self
                        .wss
                        .tx
                        .clone()
                        .try_send(serde_json::to_string(&message).unwrap())
                    {
                        log::debug!("error sending to channel: {:?}", e);
                    }
                    input.set_value("");
                };
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);
        let current_user = self.current_user.clone();

        html! {
            <div class="flex w-screen" style="background-color: #f0fdf4;">
                // Users Panel - hijau muda
                <div class="flex-none w-56 h-screen" style="background-color: #bbf7d0;">
                    <div class="text-xl p-3 font-bold" style="color: #14532d;">{"🌿 Users"}</div>
                    {
                        self.users.clone().iter().map(|u| {
                            html!{
                                <div class="flex m-3 rounded-lg p-2" style="background-color: #86efac;">
                                    <div>
                                        <img class="w-12 h-12 rounded-full" src={u.avatar.clone()} alt="avatar"/>
                                    </div>
                                    <div class="flex-grow p-3">
                                        <div class="flex text-xs justify-between">
                                            <div style="color: #14532d; font-weight: bold;">{u.name.clone()}</div>
                                        </div>
                                        <div class="text-xs" style="color: #166534;">
                                            {"🟢 Online"}
                                        </div>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
                // Chat Area
                <div class="grow h-screen flex flex-col">
                    // Header
                    <div class="w-full h-14 border-b-2" style="border-color: #86efac; background-color: #dcfce7;">
                        <div class="text-xl p-3 font-bold" style="color: #14532d;">{"🌱 A-Jenisa-Bunga-2406431334's Chat!"}</div>
                    </div>
                    // Messages
                    <div class="w-full grow overflow-auto border-b-2 p-4" style="border-color: #86efac;">
                        {
                            self.messages.iter().map(|m| {
                                let is_mine = m.from == current_user;
                                let user = self.users.iter().find(|u| u.name == m.from).unwrap();
                                if is_mine {
                                    html!{
                                        // Pesan sendiri - hijau tua, di kanan
                                        <div class="flex items-end justify-end m-4">
                                            <div class="p-3 rounded-tl-lg rounded-tr-lg rounded-bl-lg max-w-xs" style="background-color: #16a34a; color: white;">
                                                <div class="text-xs font-bold mb-1">{m.from.clone()}</div>
                                                <div class="text-sm">
                                                    if m.message.ends_with(".gif") {
                                                        <img class="mt-3" src={m.message.clone()}/>
                                                    } else {
                                                        {m.message.clone()}
                                                    }
                                                </div>
                                            </div>
                                            <img class="w-8 h-8 rounded-full ml-3" src={user.avatar.clone()} alt="avatar"/>
                                        </div>
                                    }
                                } else {
                                    html!{
                                        // Pesan orang lain - hijau muda, di kiri
                                        <div class="flex items-end m-4">
                                            <img class="w-8 h-8 rounded-full mr-3" src={user.avatar.clone()} alt="avatar"/>
                                            <div class="p-3 rounded-tl-lg rounded-tr-lg rounded-br-lg max-w-xs" style="background-color: #bbf7d0; color: #14532d;">
                                                <div class="text-xs font-bold mb-1">{m.from.clone()}</div>
                                                <div class="text-sm">
                                                    if m.message.ends_with(".gif") {
                                                        <img class="mt-3" src={m.message.clone()}/>
                                                    } else {
                                                        {m.message.clone()}
                                                    }
                                                </div>
                                            </div>
                                        </div>
                                    }
                                }
                            }).collect::<Html>()
                        }
                    </div>
                    // Input
                    <div class="w-full h-14 flex px-3 items-center" style="background-color: #dcfce7;">
                        <input ref={self.chat_input.clone()} type="text" placeholder="Ketik pesan..." class="block w-full py-2 pl-4 mx-3 rounded-full outline-none" style="background-color: #bbf7d0; color: #14532d;" name="message" required=true />
                        <button onclick={submit} class="p-3 shadow-sm w-10 h-10 rounded-full flex justify-center items-center" style="background-color: #16a34a;">
                            <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" class="fill-white">
                                <path d="M0 0h24v24H0z" fill="none"></path><path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"></path>
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}