// extern crate multibase;
// extern crate multihash;
// extern crate futures;
// extern crate libp2p_secio;
extern crate libp2p_swarm;
// extern crate libp2p_tcp_transport;
// extern crate ring;
// extern crate tokio_core;
// extern crate tokio_io;
extern crate cid;
use cid::{Cid, Codec, Version};
use std::env;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

// use futures::Future;
// use libp2p_secio::{SecioConfig, SecioKeyPair};
use libp2p_swarm::{multiaddr::Protocol, AddrComponent, Multiaddr, Transport};
// use libp2p_tcp_transport::TcpConfig;
// use tokio_core::reactor::Core;
// use tokio_io::io::write_all;

#[derive(Debug)]
pub struct Environment {
    pub ipfs_path: PathBuf,
}

impl Environment {
    fn new() -> Self {
        Environment {
            ipfs_path: match option_env!("IPFS_PATH") {
                Some(path) => Path::new(path).to_path_buf(),
                _ => env::home_dir()
                    .and_then(|mut path| {
                        path.push(".ipfs");
                        Some(path)
                    })
                    .expect("Home directory did not exist, it might be cause on Windows."),
            },
        }
    }
}

/*
struct IpfsTransport {}

impl Transport for IpfsTransport {
    /// The raw connection to a peer.
    // type RawConn: AsyncRead + AsyncWrite;
    type RawConn = TcpStream;

    // /// The listener produces incoming connections.
    // ///
    // /// An item should be produced whenever a connection is received at the lowest level of the
    // /// transport stack. The item is a `Future` that is signalled once some pre-processing has
    // /// taken place, and that connection has been upgraded to the wanted protocols.
    // type Listener: Stream<Item = Self::ListenerUpgrade, Error = IoError>;
    type Listener = Box<Stream<Item = Self::ListenerUpgrade, Error = IoError>>;

    // /// After a connection has been received, we may need to do some asynchronous pre-processing
    // /// on it (eg. an intermediary protocol negotiation). While this pre-processing takes place, we
    // /// want to be able to continue polling on the listener.
    // type ListenerUpgrade: Future<Item = (Self::RawConn, Multiaddr), Error = IoError>;
    type ListenerUpgrade = FutureResult<(Self::RawConn, Multiaddr), IoError>;

    // /// A future which indicates that we are currently dialing to a peer.
    // type Dial: IntoFuture<Item = (Self::RawConn, Multiaddr), Error = IoError>;
    type Dial = Box<Future<Item = (TcpStream, Multiaddr), Error = IoError>>;

    /// Listen on the given multiaddr. Returns a stream of incoming connections, plus a modified
    /// version of the `Multiaddr`. This new `Multiaddr` is the one that that should be advertised
    /// to other nodes, instead of the one passed as parameter.
    ///
    /// Returns the address back if it isn't supported.
    ///
    /// > **Note**: The reason why we need to change the `Multiaddr` on success is to handle
    /// >             situations such as turning `/ip4/127.0.0.1/tcp/0` into
    /// >             `/ip4/127.0.0.1/tcp/<actual port>`.
    fn listen_on(self, addr: Multiaddr) -> Result<(Self::Listener, Multiaddr), (Self, Multiaddr)>
    where
        Self: Sized,
    {
        unimplemented!();
    }

    /// Dial to the given multi-addr.
    ///
    /// Returns either a future which may resolve to a connection, or gives back the multiaddress.
    fn dial(self, addr: Multiaddr) -> Result<Self::Dial, (Self, Multiaddr)>
    where
        Self: Sized,
    {
        unimplemented!();
    }

    /// Takes a multiaddress we're listening on (`server`), and tries to convert it to an
    /// externally-visible multiaddress. In order to do so, we pass an `observed` address which
    /// a remote node observes for one of our dialers.
    ///
    /// For example, if `server` is `/ip4/0.0.0.0/tcp/3000` and `observed` is
    /// `/ip4/80.81.82.83/tcp/29601`, then we should return `/ip4/80.81.82.83/tcp/3000`. Each
    /// implementation of `Transport` is only responsible for handling the protocols it supports.
    ///
    /// Returns `None` if nothing can be determined. This happens if this trait implementation
    /// doesn't recognize the protocols, or if `server` and `observed` are related.
    fn nat_traversal(&self, server: &Multiaddr, observed: &Multiaddr) -> Option<Multiaddr> {
        unimplemented!();
    }
}
*/
fn main() {
    let environment = Environment::new();
    let addr = "/ipfs/QmWm6xANVhca6YNY97bF7TqVtFgEsT2Kperzgwhmvpi88d"
        .parse::<Multiaddr>()
        .unwrap();
    let protocols = addr.iter()
        .filter_map(|a| match a {
            AddrComponent::IPFS(addr) => Some(addr),
            _ => None,
        })
        .collect::<Vec<_>>();
    let h = protocols.first().unwrap();
    let cid = Cid::new(Codec::DagProtobuf, Version::V1, &h);
    let data = cid.to_bytes();
    println!("{:?}", &data);
    let out = Cid::from(data).unwrap();

    println!("{:?}", &environment);
    println!("{:?}", &out);

    // let mut core = Core::new().unwrap();
    // let transport = TcpConfig::new(core.handle()).with_upgrade(SecioConfig {
    //     key: SecioKeyPair::rsa_from_pkcs8(
    //         include_bytes!("../fixture/test-private-key.pk8"),
    //         include_bytes!("../fixture/test-public-key.der").to_vec(),
    //     ).unwrap(),
    // });

    // let future = match transport.dial(addr) {
    //     Ok(conn) => conn.and_then(|(conn, _addr)| write_all(conn, "Hello world!")),
    //     Err((_upgraded_node, addr)) => {
    //         panic!("Unable to dial node {:?}", addr);
    //     }
    // };

    // match core.run(future) {
    //     Ok((_conn, response)) => println!("Success! [{}]", response),
    //     Err(e) => panic!("Something wrong,\n{:?}", e),
    // };
}
