use lazy_static::lazy_static;
use regex::Regex;
use url::Url;
use uuid::Uuid; // 1.3.0

lazy_static! {
    static ref BANNER: Regex = Regex::new(r"^[^/\\:]*$").unwrap();
}

#[derive(Debug, PartialEq)]
pub enum Nurlet {
    Url(String),
    Banner(String),
}

impl TryFrom<String> for Nurlet {
    type Error = String;
    fn try_from(input: String) -> Result<Self, Self::Error> {
        if input.parse::<Url>().is_ok() {
            return Ok(Self::Url(input));
        };
        if BANNER.is_match(&input) {
            return Ok(Self::Banner(input));
        }
        Err("Not a nurlet".to_string())
    }
}

#[derive(Debug)]
pub struct Nurl {
    pub id: Uuid,
    pub title: String,
    pub views: i32,
    pub urls: Vec<Url>,
}

impl Nurl {
    pub fn new(title: &str, urls: Vec<&str>) -> Result<Nurl, url::ParseError> {
        let mut parsed_urls: Vec<Url> = Vec::with_capacity(urls.len());
        for url in urls {
            parsed_urls.push(url.parse::<Url>()?)
        }
        Ok(Nurl {
            title: title.to_string(),
            id: Uuid::new_v4(),
            views: 0,
            urls: parsed_urls,
        })
    }
    pub fn default() -> Nurl {
        Nurl {
            title: "Title".to_string(),
            id: Uuid::new_v4(),
            views: 0,
            urls: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Nurl, Nurlet};
    #[tokio::test]
    async fn test_success() {
        let nurl = Nurl::new("test", vec!["http://www.google.nl", "http://facebook.com"]);
        assert!(nurl.is_ok());
        let nurl = nurl.unwrap();
        assert_eq!(nurl.views, 0);
        assert_eq!(nurl.urls.len(), 2);
    }

    #[tokio::test]
    async fn test_fail() {
        let nurl = Nurl::new("test", vec!["wrong"]);
        assert!(nurl.is_err())
    }

    #[tokio::test]
    async fn test_nurlet_parse_banner() {
        let inputs = vec![
            "hello".to_string(),
            "hello again.".to_string(),
            "hello again 😄".to_string(),
            "www.google.nl".to_string(),
        ];
        for input in inputs {
            let nurlet: Nurlet = input.clone().try_into().unwrap();
            assert_eq!(nurlet, Nurlet::Banner(input));
        }
    }
    #[tokio::test]
    async fn test_nurlet_parse_url() {
        let inputs = vec!["https://google.nl".to_string()];
        for input in inputs {
            let nurlet: Nurlet = input.clone().try_into().unwrap();
            assert_eq!(nurlet, Nurlet::Url(input));
        }
    }

    #[tokio::test]
    async fn test_nurlet_parse_error() {
        let inputs = vec!["//".to_string()];
        for input in inputs {
            let result: Result<Nurlet, _> = input.clone().try_into();
            assert!(result.is_err());
        }
    }
}
