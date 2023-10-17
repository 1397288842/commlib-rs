//! Commlib: event, log, service, ...

///
#[macro_use]
mod macros;

///
mod utils;
pub use utils::*;

///
mod stop_watch;
pub use stop_watch::*;

/// util for promise blocking wait
pub mod pinky_swear;
pub use pinky_swear::{Pinky, PinkySwear};

///
mod commlib_event;
pub use commlib_event::*;

///
mod commlib_log;
pub use commlib_log::*;

///
mod commlib_service;
pub use commlib_service::*;

///
mod clock;
pub use clock::*;

///
pub mod hash_wheel_timer;

///
mod xmlreader;
pub use xmlreader::XmlReader;

///
mod service_signal;
pub use service_signal::ServiceSignalRs;

///
mod service_dns_resolver;
pub use service_dns_resolver::ServiceDnsResolverRs;

///
mod service_net;
pub use service_net::redis_cmds as redis;
pub use service_net::{
    connect_to_redis, connect_to_tcp_server, listen_tcp_addr, start_network, stop_network,
};
pub use service_net::{
    Buffer, ClientStatus, CmdId, ConnId, Connector, NetProxy, ServiceNetRs, TcpClient, TcpConn,
    TcpHandler, TcpListenerId, TcpServer,
};
pub use service_net::{NetPacket, NetPacketGuard, PacketType};
pub use service_net::{RedisClient, RedisCommander, RedisReply, RedisReplyType};
pub use service_net::{ENCRYPT_KEY_LEN, ENCRYPT_MAX_LEN};

///
mod service_http_client;
pub use service_http_client::{HttpRequestType, ServiceHttpClientRs};

/// 全局变量
mod globals;
pub use globals::*;

/// 通用定义
mod commlib_def;
pub use commlib_def::*;
