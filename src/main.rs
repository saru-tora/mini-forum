use anansi::*;
use anansi::util::{auth, sessions, admin};

mod urls;
mod project;
mod http_errors;
mod forum;

apps! {
    auth,
    sessions,
    forum,
}

app_statics! {
    admin,
}

app_admins! {
    auth,
    forum,
}

main!();
