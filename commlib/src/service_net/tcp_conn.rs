use atomic::{Atomic, Ordering};
use parking_lot::RwLock;
use std::sync::Arc;

use message_io::network::Endpoint;
use message_io::node::NodeHandler;

use crate::ServiceRs;

use super::{ConnId, ServiceNetRs};
use super::{NetPacketGuard, PacketResult, PacketType};

/// Tcp connection: all fields are public for easy construct
pub struct TcpConn {
    //
    pub hd: ConnId,

    //
    pub endpoint: Endpoint,
    pub netctrl: NodeHandler<()>,

    //
    pub packet_type: Atomic<PacketType>,
    pub closed: Atomic<bool>,

    //
    pub srv: Arc<dyn ServiceRs>,
    pub srv_net: Arc<ServiceNetRs>,

    //
    pub conn_fn: Arc<dyn Fn(Arc<TcpConn>) + Send + Sync>,
    pub pkt_fn: Arc<dyn Fn(Arc<TcpConn>, NetPacketGuard) + Send + Sync>,
    pub close_fn: RwLock<Arc<dyn Fn(ConnId) + Send + Sync>>,

    //
    pub read_fn: Box<dyn Fn(NetPacketGuard) -> PacketResult + Send + Sync>,
}

impl TcpConn {
    /// buffer 数据处理
    #[inline(always)]
    pub fn handle_read(&self, buffer_pkt: NetPacketGuard) -> PacketResult {
        (self.read_fn)(buffer_pkt)
    }

    /// low level close
    #[inline(always)]
    pub fn close(&self) {
        log::info!("[hd={}] low level close", self.hd);
        self.netctrl.network().remove(self.endpoint.resource_id());
    }

    ///
    #[inline(always)]
    pub fn send(&self, data: &[u8]) {
        log::debug!("[hd={}] send data ...", self.hd);

        self.netctrl.network().send(self.endpoint, data);
    }

    ///
    #[inline(always)]
    pub fn packet_type(&self) -> PacketType {
        self.packet_type.load(Ordering::Relaxed)
    }

    ///
    #[inline(always)]
    pub fn set_packet_type(&self, packet_type: PacketType) {
        self.packet_type.store(packet_type, Ordering::Relaxed);
    }

    ///
    #[inline(always)]
    pub fn is_closed(&self) -> bool {
        self.closed.load(Ordering::Relaxed)
    }

    ///
    #[inline(always)]
    pub fn set_closed(&self, is_closed: bool) {
        self.closed.store(is_closed, Ordering::Relaxed);
    }
}