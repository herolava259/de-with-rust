use lib_utils::config::{Logging, LogLevel, LogOutput};



#[test]
fn logging_initial_state() {
    let log = Logging::new(false, lib_utils::config::LogLevel::Debug, LogOutput::Stderr);

    assert_eq!(log.enabled, false);

    assert_eq!(log.destination, LogOutput::Stderr);

    assert!(log.level == LogLevel::Debug);
}


#[test]
#[ignore]
fn crazy_log_should_ignore()
{
    assert_ne!(false, true);


}

#[test]
#[should_panic]
fn demo_should_panic(){
    
    let a = 1 / 0;

}

// cargo test --features experimental
#[cfg(feature = "experimental")]
#[test]
fn demo_test_with_feature_flag()
{
    assert_eq!(1, 1);
    assert!(1 < 10);
}

#[cfg(target_os = "linux")]
#[test]
fn demo_os_linux_test(){
    assert!(true);
}