use chrono::Utc;
use log::{/*error, warn, info, debug,*/ trace, /*log, Level*/};

use std::net::IpAddr;
use std::thread;
use std::time::Duration;

use crate::metrics::METRICS;

/**
Start the updater loop that will run forever, waiting a few seconds between each attempt to update.
It is up to the caller to run this in a separate thread, or be blocked indefinitely.

# Errors
On most errors it will simply wait the usual interval before trying again.
On serious errors likely to happen again every time, it will terminate.
In either case, it will log what went wrong.

# Examples
```no_run
use metric_videos::updater;
use std::thread;
//Keep the DB updated while the app runs
thread::spawn(|| { updater::updater(); });
```
*/
pub fn updater()
{
    let mut first_iter = true;
    loop{
        /* Wait a few seconds between iterations.
        We have this first_iter guard to start immediately the first time,
        which wouldn't be necessary if we just put the sleep at the end of the loop instead,
        but doing it this way allows using `continue` to abort bad iterations without skipping the sleep.
        */
        if first_iter
        {
            first_iter = false;
        }else{
            thread::sleep(Duration::from_secs(2));
        }

        trace!("Iterating periodic update loop");

        // Prune any watchers that haven't updated in the last 5 seconds
        match METRICS.try_lock()
        {
            Ok(mut g) =>
            {
                let now = Utc::now().timestamp();
                for vid_index in 0 .. g.videos.len()
                {
                    let mut ips_to_remove: Vec<IpAddr> = Vec::new();
                    for (ip,watcher) in &g.videos[vid_index].watchers
                    {
                        if now - watcher.when > 5
                        {
                            ips_to_remove.push(*ip);
                        }
                    }
                    for ip in ips_to_remove
                    {
                        g.videos[vid_index].watchers.remove(&ip);
                    }
                }
            },
            Err(_) => {continue;}
        };
    }
}