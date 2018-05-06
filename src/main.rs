// extern crate multibase;
// extern crate multihash;
extern crate futures;
extern crate libp2p_secio;
extern crate libp2p_swarm;
extern crate libp2p_tcp_transport;
extern crate ring;
extern crate tokio_core;
extern crate tokio_io;

use futures::Future;
use libp2p_secio::{SecioConfig, SecioKeyPair};
use libp2p_swarm::{Multiaddr, Transport};
use libp2p_tcp_transport::TcpConfig;
use tokio_core::reactor::Core;
use tokio_io::io::write_all;

fn main() {
    // let addr = "/ip6/::1/tcp/4001/ipfs/QmTwhzbBFY2gXk3MDCCp6kj26ewNyJxc7GnvHbuxXQf4n4"
    let addr = "/ipfs/QmQ3WBvK3CbjsrfTduFxvrudjDyP59vkp1qa1NpuDugkb8"
        .parse::<Multiaddr>()
        .unwrap();
    // let addr = "/ip4/127.0.0.1/tcp/4001".parse::<Multiaddr>().unwrap();
    println!("Attempt to connect {}", &addr);

    let mut core = Core::new().unwrap();
    let transport = TcpConfig::new(core.handle()).with_upgrade(SecioConfig {
        key: SecioKeyPair::rsa_from_pkcs8(
            include_bytes!("../fixture/test-private-key.pk8"),
            include_bytes!("../fixture/test-public-key.der").to_vec(),
        ).unwrap(),
    });

    let future = match transport.dial(addr) {
        Ok(conn) => conn.and_then(|(conn, _addr)| write_all(conn, "Hello world!")),
        Err((_upgraded_node, addr)) => {
            panic!("Unable to dial node {:?}", addr);
        }
    };

    match core.run(future) {
        Ok((_conn, response)) => println!("Success! [{}]", response),
        Err(e) => panic!("Something wrong,\n{:?}", e),
    };
}
