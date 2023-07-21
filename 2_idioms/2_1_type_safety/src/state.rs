use super::post;
use super::user;

#[derive(Clone)]
pub struct Post {
    id: post::Id,
    user_id: user::Id,
    title: post::Title,
    body: post::Body,
}

// Not using enums here, because you can't define methods on variants...
pub struct New(Post);

impl New {
    fn publish(self) -> Unmoderated {
        Unmoderated(self.0)
    }
}

pub struct Unmoderated(Post);

impl Unmoderated {
    fn allow(self) -> Published {
        Published(self.0)
    }

    fn deny(self) -> Deleted {
        Deleted(self.0)
    }
}

pub struct Published(Post);

impl Published {
    fn delete(self) -> Deleted {
        Deleted(self.0)
    }
}

pub struct Deleted(Post);

impl Deleted {
    fn allow(self) -> Published {
        Published(self.0)
    }
}
