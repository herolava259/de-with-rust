//! This module contains the configuration options for the application.
//! # Examples:
//! ```
//! use lib_utils::config::Logging;
//! let config = Logging::default();
//! ```
//! 
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

pub enum LogOutput {
    Stdout,
    Stderr,
    File(String),
}

/// This struct contains configuration options for the application.
/// # Examples:
/// ```
/// use lib_utils::config::{Logging, LogLevel, LogOutput};
/// let config = Logging::new(false, LogLevel::Warn, LogOutput::Stderr);
/// ```
/// 
/// Creating a new instance of the Logging struct:
/// ```
/// use lib_utils::config::{Logging, LogLevel, LogOutput};
/// let config = Logging{ enabled: true, level: LogLevel::Info, destination: LogOutput::Stdout };
/// let debugConfig = Logging{ enabled: true, level: LogLevel::Debug, destination: LogOutput::File(String::from("debug.log")) };
/// ```
pub struct Logging {
    pub enabled: bool,
    pub level: LogLevel,
    pub destination: LogOutput,   
}

impl Logging {
    pub fn new(enabled: bool, level: LogLevel, output: LogOutput) -> Self {
        Self {
            enabled: enabled,
            level: level,
            destination: output,
        }
    }

    pub fn level(&self) -> &str {
        match self.level {
            LogLevel::Debug => "Debug",
            LogLevel::Info => "Info",
            LogLevel::Warn => "Warn",
            LogLevel::Error => "Error",
        }
    }

    pub fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }

    //get the enabled
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    //set the enabled
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn default() -> Self {
        Self {
            level: LogLevel::Info,
            enabled: true,
            destination: LogOutput::Stdout,
        }
    }

    pub const DEFAULT_LEVEL: LogLevel = LogLevel::Info;
    pub const DEFAULT_ENABLED: bool = true;
    pub const DEFAULT_DESTINATION: LogOutput = LogOutput::Stdout;
    
}

pub struct AppConfig {
    pub name: String,
    version: u32
}

impl AppConfig {
    pub fn new(name: &str, version: u32) -> Self {
        Self {
            name: name.to_string(),
            version,
        }
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    
}

pub struct HostConfig {
    pub host: String,
    pub port: u16,
}


/// Configuration for the connecting external service/host/server.
/// 
/// # Examples:
/// ## Basic usage:
/// ```
/// use lib_utils::config::HostConfig;
/// let config = HostConfig::new("localhost", "8080");
/// println!("Host: {}, Port: {}", config.host, config.port);
/// assert_eq!(config.address(), "localhost:8080");
/// ```
/// 
/// ## Edge cases: port 0 (system assigned)
/// ```
/// use lib_utils::config::HostConfig;
/// 
/// let cfg = HostConfig::new("127.0.0.1", "0");
/// assert_eq!(cfg.address(), "127.0.0.1:0]")
/// ```
/// Returns a new Config if port is valid.
///
/// ```
/// use lib_utils::config::HostConfig;
///
/// let cfg = HostConfig::new_with_port_num("localhost", 65535);
/// assert_eq!(cfg.port, 65535);
/// ```
/// Compilation failure if port is out of range (0-65535).
/// ```compile_fail
/// use lib_utils::config::Config;
///
/// let cfg = Config::new(123, 8080); // host must be &str
/// ```
/// ```should_panic
/// panic!("expected panic");
/// ```
/// 
/// ## Ignore while doc-testing:
/// ```ignore
/// expensive_network_call();
/// ```
/// 
/// ## No-run example:
/// ```no_run
/// use lib_utils::config::HostConfig;
///
/// let cfg = HostConfig::new_with_port_num("example.com", 80);
/// println!("{}", cfg.address());
/// ```
impl HostConfig{
    pub fn new(host: &str, port: &str) -> Self {
        Self {
            host: host.to_string(),
            port: port.parse().unwrap_or(8080),
        }
    }

    pub fn new_with_port_num(host: &str, port: u16) -> Self {
        Self {
            host: host.to_string(),
            port: port,
        }
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 8080,
        }
    }

    pub fn localhost(port: &str) -> Self {
        Self {
            host: "localhost".to_string(),
            port: port.parse().unwrap_or(8080),
        }
    }

    pub fn localhost_ipv4(port: &str) -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: port.parse().unwrap_or(8080),
        }
    }

    pub fn from_env() -> Self {
        let host = std::env::var("HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse().unwrap_or(8080);
        Self { host, port }
    }
}