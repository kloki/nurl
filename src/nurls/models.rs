use url::Url;
use uuid::Uuid;

pub struct Nurl {
    id: Uuid,
    views: isize,
    urls: Vec<Url>,
}
