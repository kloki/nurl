use url::Url;
use uuid::Uuid;

#[derive(Debug)]
pub struct Nurl {
    pub id: Uuid,
    pub views: i32,
    pub urls: Vec<Url>,
}

impl Nurl {
    pub fn new(urls: Vec<&str>) -> Result<Nurl, url::ParseError> {
        let mut parsed_urls: Vec<Url> = Vec::with_capacity(urls.len());
        for url in urls {
            parsed_urls.push(url.parse::<Url>()?)
        }
        Ok(Nurl {
            id: Uuid::new_v4(),
            views: 0,
            urls: parsed_urls,
        })
    }
    pub fn default() -> Nurl {
        Nurl {
            id: Uuid::new_v4(),
            views: 0,
            urls: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Nurl;
    #[tokio::test]
    async fn test_success() {
        let nurl = Nurl::new(vec!["http://www.google.nl", "http://facebook.com"]);
        assert!(nurl.is_ok());
        let nurl = nurl.unwrap();
        assert_eq!(nurl.views, 0);
        assert_eq!(nurl.urls.len(), 2);
    }

    #[tokio::test]
    async fn test_fail() {
        let nurl = Nurl::new(vec!["wrong"]);
        assert!(nurl.is_err())
    }
}
