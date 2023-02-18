use gtfs_realtime::transit_realtime;
use prost::Message;
use std::error::Error;

pub struct MTA {
    key: String,
    rt_endpoint: String,
}

pub struct MTAStatic {
    static_endpoint: String,
}

impl MTA {
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_owned(),
            rt_endpoint: "https://api-endpoint.mta.info/Dataservice/mtagtfsfeeds/mnr%2Fgtfs-mnr"
                .to_string(),
        }
    }

    pub fn get_rt(&self) -> Result<(), Box<dyn Error>> {
        let client = reqwest::blocking::Client::new();
        let res = client
            .get(&self.rt_endpoint)
            .header("x-api-key", &self.key)
            .send()?;

        let _msg = transit_realtime::FeedMessage::decode(res.bytes()?);
        //dbg!(msg);

        //assert!(false);

        Ok(())
    }
}

impl MTAStatic {
    pub fn new() -> Self {
        Self {
            static_endpoint: "http://web.mta.info/developers/data/mnr/google_transit.zip"
                .to_string(),
        }
    }
    
    pub fn load(mut self) -> Result<Self, Box<dyn Error>> {
        Ok(self)
    }    
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use std::env;

    #[test]
    fn load_api() {
        dotenv().ok();
        let key = env::var("MTA_API_KEY").unwrap();
        let _mta = MTA::new(&key);
    }

    #[test]
    fn make_api_request() {
        dotenv().ok();
        let key = env::var("MTA_API_KEY").unwrap();
        let mta = MTA::new(&key);
        mta.get_rt().unwrap();
    }

    #[test]
    fn make_static_request() {
        dotenv().ok();
        let _mta_static = MTAStatic::new();
    }
}
