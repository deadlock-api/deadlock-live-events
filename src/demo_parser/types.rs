use core::fmt::{Display, Formatter};

use axum::response::sse::Event;
use haste::entities::DeltaHeader;
use serde::Serialize;
use strum::{Display, FromRepr};

use crate::demo_parser::entity_events::{EntityType, EntityUpdateEvents};

#[derive(Serialize, Debug, Clone)]
pub(crate) struct DemoEvent {
    pub(super) tick: i32,
    pub(super) game_time: f32,

    #[serde(flatten)]
    pub(super) event: DemoEventPayload,
}

impl TryInto<Event> for DemoEvent {
    type Error = axum::Error;

    fn try_into(self) -> Result<Event, Self::Error> {
        let event = self.event.to_string();
        Event::default().event(event).json_data(self)
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "event_type")]
#[serde(rename_all = "snake_case")]
pub(super) enum DemoEventPayload {
    EntityUpdate {
        delta: Delta,
        entity_index: i32,
        entity_type: EntityType,
        #[serde(flatten)]
        entity_update: EntityUpdateEvents,
    },
    ChatMessage {
        steam_name: Option<String>,
        steam_id: Option<u32>,
        text: Option<String>,
        all_chat: Option<bool>,
        lane_color: Option<i32>,
    },
    TickEnd,
}

impl Display for DemoEventPayload {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EntityUpdate {
                delta, entity_type, ..
            } => write!(f, "{entity_type}_entity_{delta}"),
            Self::ChatMessage { .. } => write!(f, "chat_message"),
            Self::TickEnd => write!(f, "tick_end"),
        }
    }
}

#[derive(FromRepr, Serialize, Debug, Clone, Copy, PartialEq, Eq, Default, Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub(super) enum Delta {
    #[default]
    #[serde(skip)]
    Invalid,
    Update,
    Leave,
    Create,
    Delete,
}

impl From<DeltaHeader> for Delta {
    fn from(delta_header: DeltaHeader) -> Self {
        match delta_header {
            DeltaHeader::UPDATE => Self::Update,
            DeltaHeader::LEAVE => Self::Leave,
            DeltaHeader::CREATE => Self::Create,
            DeltaHeader::DELETE => Self::Delete,
            _ => Self::default(),
        }
    }
}
