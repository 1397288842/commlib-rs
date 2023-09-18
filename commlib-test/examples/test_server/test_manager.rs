//!
//! TestManager
//!

use std::sync::Arc;

use commlib::{with_tls, with_tls_mut};
use commlib::{Blowfish, CmdId, ConnId, NetProxy, NodeState, PacketType, ServiceRs, TcpConn};
use commlib::{ENCRYPT_KEY_LEN, ENCRYPT_MAX_LEN};
use commlib::{G_SERVICE_NET, G_SERVICE_SIGNAL};

use app_helper::G_CONF;

use crate::proto;
use prost::Message;

use super::test_conf::G_TEST_CONF;
use super::test_service::TestService;
use super::test_service::G_TEST_SERVICE;

thread_local! {
    ///
    pub static G_MAIN: std::cell::RefCell<TestManager> = {
        std::cell::RefCell::new(TestManager::new())
    };
}

///
pub struct TestManager {
    pub c2s_proxy: NetProxy, // client to server
}

impl TestManager {
    ///
    pub fn new() -> TestManager {
        let mut c2s_proxy = NetProxy::new(PacketType::Client);
        c2s_proxy.set_encrypt_token_handler(|proxy, conn: &TcpConn| {
            send_encrypt_token(proxy, conn);
        });

        TestManager {
            c2s_proxy: c2s_proxy,
        }
    }

    ///
    pub fn init(&mut self, srv: &Arc<TestService>) -> bool {
        let handle = srv.get_handle();

        /// 消息处理
        self.c2s_proxy.set_packet_handler(
            proto::EnumMsgType::EncryptToken as CmdId,
            Self::handle_encrypt_token,
        );

        // ctrl-c stop, DEBUG ONLY
        G_SERVICE_SIGNAL.listen_sig_int(G_TEST_SERVICE.as_ref(), || {
            println!("WTF!!!!");
        });
        log::info!("\nTest init ...\n");

        //
        with_tls_mut!(G_TEST_CONF, cfg, { cfg.init(handle.xml_config()) });

        //
        handle.set_state(NodeState::Start);
        true
    }

    ///
    pub fn lazy_init(&mut self, srv: &Arc<TestService>) {
        log::info!("lazy init:");
    }

    /// 消息处理: encrypt token
    pub fn handle_encrypt_token(proxy: &mut NetProxy, conn: &TcpConn, cmd: CmdId, slice: &[u8]) {
        // 消息包加密 key
        let msg = proto::S2cEncryptToken::decode(slice).unwrap();

        let key = msg.token();

        log::info!("handle_encrypt_token: key: {} -- {:?}", key.len(), key);
    }
}

fn send_encrypt_token(proxy: &mut NetProxy, conn: &TcpConn) {
    let hd = conn.hd;

    // 消息包加密 key
    let mut code_buf = vec![0_u8; ENCRYPT_KEY_LEN];
    commlib::gen_random_code(&mut code_buf);

    //
    let g_conf = G_CONF.load();
    // 发送前先加密
    let mut encrypt_buf =
        Blowfish::encrypt(g_conf.encrypt_token.as_slice(), code_buf.as_slice()).unwrap();

    // send
    {
        let msg = proto::S2cEncryptToken {
            token: Some(encrypt_buf.clone()),
        };

        // send encrypt key
        proxy.send_proto(conn, proto::EnumMsgType::EncryptToken as CmdId, &msg);
    }

    // 设置 encrypt key（为方便解码，对 encrypt_buf 进行扩展，避免解码时超出边界）
    encrypt_buf.extend_from_within(..ENCRYPT_MAX_LEN);
    proxy.set_encrypt_key(hd, encrypt_buf.as_slice());
}