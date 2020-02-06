extern crate app_dirs;

use app_dirs::*;
use serde::{Deserialize, Serialize};

use simplelog::{Config as LogConfig, LevelFilter, WriteLogger};
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::Once;

const APP_INFO: AppInfo = AppInfo {
    name: "rust-testapp",
    author: "alex",
};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub _config_dir: PathBuf,
    pub _data_dir: PathBuf,
    pub _log_file_name: PathBuf,
    pub _is_intialiazed: bool,
    pub _lastrun: String,
}

impl Default for Config {
    fn default() -> Self {
        let _data_dir = get_app_dir(AppDataType::UserData, &APP_INFO, "").unwrap();
        Self {
            _lastrun: chrono::Local::now().to_rfc3339(),
            _is_intialiazed: false,
            _config_dir: get_app_dir(AppDataType::UserConfig, &APP_INFO, "").unwrap(),
            _data_dir: _data_dir.clone(),
            _log_file_name: [_data_dir.clone(), PathBuf::from("log.log")]
                .iter()
                .collect(),
        }
    }
}
pub struct Context {
    pub _use_count: i32,
    pub _cfg: Config,
    pub _exe_path: PathBuf,
}

impl Context {
    pub fn init(&mut self) -> std::io::Result<()> {
        create_dir_all(&self._cfg._config_dir)?;
        let config_file_name = self.get_config_filename();
        let log_file_name = self.get_log_filename();
        let _logresult = WriteLogger::init(
            LevelFilter::Info,
            LogConfig::default(),
            File::create(log_file_name.clone())?,
        );

        log::info!("Context::init(): Log file created at {}", log_file_name);
        debug_assert_ne!(config_file_name.len(), 0);

        if Path::new(config_file_name.as_str()).exists() {
            log::info!(
                "Context::init(): Found a config file at {} and using it",
                config_file_name
            );
            let mut _file = File::open(config_file_name)?;
            let mut _raw_json: String = String::new();

            _file.read_to_string(&mut _raw_json)?;
            self._cfg = serde_json::from_str(_raw_json.as_str())?;
        } else {
            log::info!(
                "Context::init(): The config file did not exist. Creating at {}",
                config_file_name
            );
            let _l = serde_json::to_string_pretty(&self._cfg).unwrap();
            let mut _file = File::create(config_file_name)?;
            _file.write_all(_l.as_bytes())?;
        }
        Ok(())
    }

    pub fn cleanup(&mut self) {
        let _r = self.write_config();
    }

    pub fn write_config(&self) -> std::io::Result<()> {
        let _config_file_name = self.get_config_filename();
        let mut _file = File::create(_config_file_name)?;
        let _l = serde_json::to_string_pretty(&self._cfg).unwrap();
        _file.write_all(_l.as_bytes())?;
        _file.flush()?;
        Ok(())
    }

    pub fn get_config_filename(&self) -> String {
        let mut config_file_name = self._cfg._config_dir.clone();
        config_file_name.push("conf.json");
        String::from(config_file_name.to_str().unwrap().clone())
    }

    pub fn get_log_filename(&self) -> String {
        let mut log_file_name = self._cfg._data_dir.clone();
        log_file_name.push("darksky-r.log");
        String::from(log_file_name.to_str().unwrap().clone())
    }
}


pub fn get_instance() -> &'static mut Context {
    static mut CTX: *mut Context = 0 as *mut Context;
    static ONCE: Once = Once::new();
    unsafe {
        ONCE.call_once(|| {
            let context = Context {
                _use_count: 0,
                _cfg: Default::default(),
                _exe_path: std::env::current_exe().unwrap(),
            };
            CTX = std::mem::transmute(Box::new(context));

            let ctx = &mut *CTX;
            let _result = ctx.init(); // (*CTX).init(); does the same
            ctx._cfg._is_intialiazed = true;
        });
        (*CTX)._use_count = (*CTX)._use_count + 1;
        &mut *CTX // return mutable reference to the object
    }
}
