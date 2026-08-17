use crate::template_structs::{EventAdd, EventDelete, EventDescription, EventIndex};

pub async fn event_index() -> EventIndex {
    EventIndex {}
}

pub async fn event_add() -> EventAdd {
    EventAdd {}
}

pub async fn event_description() -> EventDescription {
    EventDescription {}
}

pub async fn event_delete() -> EventDelete {
    EventDelete {}
}
