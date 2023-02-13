
//#[macro_use]
//extern crate log;

use env_logger::Env;

fn main() {
    println!("Hello, world!");

    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    log::trace!("some trace log");
    log::debug!("some debug log");
    log::info!("some information log");
    log::warn!("some warning log");
    log::error!("some error log");

    let mut segment = msg_prost::sample::test::Segment::default();
    segment.name = String::from("ownrisk");
    let msg_id = msg_prost::proto::MsgId::Unknown;
}


//fn main() {
//    println!("Hello, world!");
//
//    let env = Env::default()
//        .filter_or("MY_LOG_LEVEL", "trace")
//        .write_style_or("MY_LOG_STYLE", "always");
//
//    env_logger::init_from_env(env);
//
//    log::trace!("some trace log");
//    log::debug!("some debug log");
//    log::info!("some information log");
//    log::warn!("some warning log");
//    log::error!("some error log");
//
//}
