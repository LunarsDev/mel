use self::{
    nekoslife::client::NekosLifeClient,
    lunardev::client::LunarDevClient
};

pub mod nekoslife;
pub mod lunardev;

lazy_static! {
    static ref NEKOSLIFE_API: NekosLifeClient = NekosLifeClient::default();
    static ref LUNARDEV_API: LunarDevClient = LunarDevClient::default();
}

pub fn get_nekoslife_api() -> &'static NekosLifeClient {
    &NEKOSLIFE_API
}

pub fn get_lunardev_api() -> &'static LunarDevClient {
    &LUNARDEV_API
}