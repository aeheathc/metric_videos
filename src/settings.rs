use clap::{Arg, App};
use config::{ConfigError, Config, File};
use log::{error/*, warn, info, debug, trace, log, Level*/};
use std::env;
use std::fs;
use std::path::Path;

/**
The portion of the config needed immediately, before we can even do so much as display an error over HTTP.
*/
#[derive(Deserialize)]
pub struct Startup
{
    pub working_dir: String,
    pub listen_addr: String
}

/**
The portion of the config needed for listing available content.
*/
#[derive(Deserialize)]
pub struct Media
{
    pub videos: Vec<String>
}

/**
The main type storing all the configuration data.
*/
#[derive(Deserialize)]
pub struct Settings
{
    pub startup: Startup,
    pub media: Media
}

impl Settings
{
    /**
    Generates a TOML format config file containing the values set in this struct.

    # Examples
    ```
    use metric_videos::settings::*;
    let def_settings: Settings = Settings{
        startup: Startup{
            working_dir: String::from("data"),
            listen_addr: String::from("0.0.0.0:80")
        },
        media: Media{
            videos: vec![
                String::from("http://example.com/vid1.mp4"),
                String::from("http://example.com/vid2.mp4")
            ]
        }
    };

    let default_config_file_contents = def_settings.to_toml();

    assert_eq!(&default_config_file_contents[..30],"[startup]\nworking_dir = \"data\"");
    ```
    */
    pub fn to_toml(&self) -> String
    {
        format!("[startup]\nworking_dir = \"{}\"\nlisten_addr = \"{}\"\n[media]\nvideos = [\"{}\"]",
            self.startup.working_dir,
            self.startup.listen_addr,
            self.media.videos.join("\",\"")
        )
    }

    /**
    Load configuration for app and logger.

    - Load app & logger config, merging values from all sources (cmd, env, file, defaults) with appropriate priority
    - Store app config in a lazy_static ref settings::SETTINGS
    - Set the working directory of the app to what is configured, so relative paths work correctly.
    - If either config file is missing, write a new one with default settings.
    - Start up logger.

    # Panics
    This function makes every attempt to recover from minor issues, but any unrecoverable problem will result in a panic.
    After all, the app can't safely do much of anything without the info it returns, and even the logger isn't available until the very end.
    Possible unrecoverables include CWD change error, filesystem errors, and config parse errors.

    # Undefined behavior
    This should only be called once. Additional calls may result in issues with the underlying config and logger libraries.

    */
    fn new() -> Self
    {
        let path_config = "config/config.toml";
        let path_log4rs_config = "config/log4rs.yml";
        //std::env::set_var("RUST_LOG", "my_errors=debug,actix_web=info");
        //std::env::set_var("RUST_BACKTRACE", "1");
        
        //Load command-line arguments. For those unspecified, load environment variables.
        let cmd_matches = App::new("metric_videos")
            .version("dev")
            .about("actix-web app showing metrics of videos being watched through it")
            .arg(Arg::with_name("working_dir")
                .short("w")
                .long("workingdir")
                .env("metric_videos_WORKING_DIR")
                .help("Working directory. Will look here for the folders config,history,logs,static -- particularly the config file in config/config.toml which will be created if it doesn't exist.")
                .default_value(&DEFAULT_SETTINGS.startup.working_dir)
                .takes_value(true))
            .arg(Arg::with_name("listen_addr")
                .short("l")
                .long("listenaddr")
                .env("metric_videos_LISTEN_ADDR")
                .help("ip:port to listen on. Use 0.0.0.0 for the ip to listen on all interfaces.")
                .default_value(&DEFAULT_SETTINGS.startup.listen_addr)
                .takes_value(true))
            .arg(Arg::with_name("videos")
                .short("v")
                .long("videos")
                .env("metric_videos_video")
                .help("URL to a video to be displayed")
                .default_value("#default") //todo: CLAP 3.0 will allow specifying multivalued default instead of this placeholder
                .takes_value(true)
                .multiple(true))
            .get_matches();
    
        //set cwd
        let working_dir = cmd_matches.value_of("working_dir").expect("Couldn't determine target working dir");
        env::set_current_dir(Path::new(working_dir)).expect("Couldn't set cwd");

        //attempt to load config file
        let mut file_config = Config::new();
        if let Err(ce) = file_config.merge(File::with_name(&path_config))
        {
            match ce //determine reason for failure
            {
                ConfigError::Frozen => panic!("Couldn't load config because it was already frozen/deserialized"),
                ConfigError::NotFound(prop) => panic!("Couldn't load config because the following thing was 'not found': {}",prop),
                ConfigError::PathParse(ek) => panic!("Couldn't load config because the 'path could not be parsed' due to the following: {}", ek.description()),
                ConfigError::FileParse{uri: _, cause: _} => {panic!("Couldn't load config because of a parser failure.")},
                ConfigError::Type{origin:_,unexpected:_,expected:_,key:_} => panic!("Couldn't load config because of a type conversion issue"),
                ConfigError::Message(e_str) => panic!("Couldn't load config because of the following: {}", e_str),
                ConfigError::Foreign(_) =>{
                    //looks like the file is missing, attempt to write new file with defaults then load it. If this also fails then bail
                    if let Err(e) = fs::write(String::from(path_config), DEFAULT_SETTINGS.to_toml()){
                        panic!("Couldn't read main config file or write default main config file: {}", e);
                    }
                    file_config.merge(File::with_name(&path_config)).expect("Couldn't load newly written default main config file.");
                }
            }
        }

        //command line arguments, if given, override what is in the config file
        let set_e = "Couldn't override config setting";
        if cmd_matches.occurrences_of("working_dir"   ) > 0 {file_config.set("startup.working_dir", cmd_matches.value_of("working_dir"   )).expect(set_e);}
        if cmd_matches.occurrences_of("listen_addr"   ) > 0 {file_config.set("startup.listen_addr", cmd_matches.value_of("listen_addr"   )).expect(set_e);}
        if cmd_matches.occurrences_of("videos"        ) > 0 {file_config.set("media.videos",        cmd_matches.value_of("videos"        )).expect(set_e);}

        //attempt to load logging config
        if let Err(le) = log4rs::init_file(path_log4rs_config, Default::default())
        {
            match le //determine reason for failure
            {
                log4rs::Error::Log4rs(_) =>
                {
                    //looks like the file is missing, attempt to write new file with defaults then load it. If this also fails then bail
                    if let Err(e) = fs::write(String::from(path_log4rs_config), DEFAULT_LOG4RS.to_string()){
                        panic!("Couldn't read log config file or write default log config file: {}", e);
                    }
                    log4rs::init_file(path_log4rs_config, Default::default()).expect("Couldn't load newly written default log config file.");
                },
                _ => {panic!("Couldn't parse log config.");}
            }
        }

        //Export config to Settings struct
        match file_config.try_into::<Settings>()
        {
            Err(_) => {let e = "Couldn't export config."; error!("{}",e); panic!(e);},
            Ok(mut s) => {
                //todo: CLAP 3.0 will allow specifying multivalued default instead of this placeholder
                if s.media.videos == vec![String::from("#settings")]
                {
                    s.media.videos = DEFAULT_SETTINGS.media.videos.to_owned();
                }
                s
            }
        }
    }
}

lazy_static!
{
    pub static ref SETTINGS: Settings = Settings::new();

    static ref DEFAULT_SETTINGS: Settings = Settings{
        startup: Startup{
            working_dir: String::from("data"),
            listen_addr: String::from("0.0.0.0:80")
        },
        media: Media{
            videos: vec![String::from("http://reflect-tightytv-vod.cablecast.tv/vod/2-TRMS-Medium-v1.mp4"),
                String::from("http://reflect-tightytv-vod.cablecast.tv/vod/52-CTV-Needs-Interns-Promo-High-v1.mp4"),
                String::from("http://reflect-tightytv-vod.cablecast.tv/vod/3-NAB-2014-Artbeats-30min-High-v4.mp4")
            ]
        }
    };

    static ref DEFAULT_LOG4RS: String = String::from("refresh_rate: 60 seconds
appenders:
  stdout:
    kind: console
    target: stdout
  stderr:
    kind: console
    target: stderr
  main:
    kind: file
    path: \"log/main.log\"
    encoder:
      pattern: \"{d} [{P}:{I}] {l} - {m}{n}\"
  requestlog:
    kind: file
    path: \"log/requests.log\"
    encoder:
      pattern: \"{d} [{P}:{I}] - {m}{n}\"
root:
  level: info
  appenders:
    - main
    - stdout
loggers:
  requests:
    level: info
    appenders:
      - requestlog
    additive: false");
}

/*
Test those functions which weren't able to have good tests as part of their
example usage in the docs, but are still possible to unit-test
*/
#[cfg(test)]
mod tests
{
    use super::*;

	// settings::Settings::new()
	#[test]
	fn config_load()
	{
        //if this function panics, that is what will make the test fail, so no assert is needed.
        let _config = Settings::new();
    }
}