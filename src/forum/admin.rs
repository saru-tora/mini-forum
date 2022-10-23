use anansi::{init_admin, register, record_admin};
use super::records::Topic;

init_admin! {
    register!(Topic),
}

record_admin! {
    record: Topic,
    search_fields: [title, content, date],
}
