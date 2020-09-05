use chrono::Utc;
use log::{/*error, */warn, /*info, debug, trace, log, Level*/};

use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Mutex;

use crate::settings::SETTINGS;

#[derive(Serialize)]
pub struct SiteMetrics
{
    pub videos: Vec<VideoMetrics>
}

#[derive(Serialize)]
pub struct VideoMetrics
{
    pub watchers: HashMap<IpAddr,Watcher>
}

#[derive(Serialize)]
pub struct Watcher
{
    pub percent: u8,
    pub when: i64
}

impl SiteMetrics
{
    pub fn new() -> Self
    {
        let mut out = SiteMetrics{videos: Vec::new()};
        for _index in 0 .. SETTINGS.media.videos.len()
        {
            out.videos.push(VideoMetrics{watchers: HashMap::new()});
        }
        out
    }

    pub fn report(&mut self, ip: IpAddr, video_index: usize, percent: u8)
    {
        if video_index >= self.videos.len()
        {
            warn!("Invalid video index: {}", video_index);
            return;
        }

        let now = Utc::now().timestamp();

        match self.videos[video_index].watchers.get_mut(&ip)
        {
            Some(w) => {
                w.percent = percent;
                w.when = now;
            },
            None => {self.videos[video_index].watchers.insert(ip, Watcher{percent: percent, when: now});}
        }
    }
}

lazy_static!
{
    pub static ref METRICS: Mutex<SiteMetrics> = Mutex::new(SiteMetrics::new());
}