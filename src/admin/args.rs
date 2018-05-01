//use codec::msgs::msg;
use std::net::SocketAddr;
use hex::FromHex;
use hex;
use std::str::FromStr;

#[derive(StructOpt, Debug)]
#[structopt(name = "")]
/// Adiministrative commands
pub enum AdminCmd {
    #[structopt(name = "peer")]
    /// Peer-related tasks
    Peer(PeerCmd),

    #[structopt(name = "wallet")]
    /// Wallet-related tasks
    Wallet(WalletCmd),

    #[structopt(name = "bchain")]
    /// Blockchain-related tasks
    Blockchain(BlockchainCmd),

    #[structopt(name = "node")]
    /// Node-related tasks
    Node(NodeCmd),

    #[structopt(name = "util")]
    /// Utilities-related tasks
    Util(UtilCmd),

    #[structopt(name = "debug")]
    /// Utilities-related tasks
    Debug(DebugCmd),

    #[structopt(name = "exit")]
    /// Exits the CLI and disconnects the admin peer
    Exit,
}

#[derive(StructOpt, Debug)]
/// Peer commands
pub enum PeerCmd {
    #[structopt(name = "list")]
    /// Shows a list of peers
    List,

    #[structopt(name = "send")]
    /// Enqueue a message to be sent to some peer
    Send {
        // #[structopt(long = "raw-msg",parse(try_from_str = "parse_todo"))]
        /// Sends a raw message to a choosen peer, when in standby
        // raw_msg: msg::Msg,

        #[structopt(long = "id")]
        /// Specifies the peer's ID
        id: u64,
    },

    #[structopt(name = "add")]
    /// Connects with a new peer
    Add {
        #[structopt(long = "addr")]
        /// The socket address to connect to. example: <addr="127.0.0.1:8080">
        addr: SocketAddr,

        #[structopt(long = "wait")]
        /// Wait for the peer's handhsake initiation.
        wait_handhsake: bool,
    },

    #[structopt(name = "rm")]
    /// Closes the connection with a peer
    Remove {
        #[structopt(long = "addr")]
        /// The peer's socket address that is to be disconnected. example: <addr="127.0.0.1:8080">
        addr: SocketAddr,
    },
}

#[derive(StructOpt, Debug)]
/// Wallet command
pub enum WalletCmd {
    #[structopt(name = "list")]
    /// Shows a list of known addresses
    List {
        #[structopt(long = "hide-spendable")]
        /// Hides the spendable addresses
        hide_spendable: bool,

        #[structopt(long = "hide-unspendable")]
        /// Hides the unspendable addresses
        hide_unspendable: bool,
    },
}

#[derive(StructOpt, Debug)]
/// Blockchain commands
pub enum BlockchainCmd {
    #[structopt(name = "dummy")]
    /// Dummy option
    Dummy,
}

#[derive(StructOpt, Debug)]
/// Node commands
pub enum NodeCmd {
    #[structopt(name = "dummy")]
    /// Shows a list of peers
    Dummy,
}

#[derive(StructOpt, Debug)]
/// Utilities commands
pub enum UtilCmd {
    #[structopt(name = "dummy")]
    /// Shows a list of peers
    Dummy,
}

#[derive(StructOpt, Debug)]
/// Utilities commands
pub enum DebugCmd {
    #[structopt(name = "dummy")]
    /// Shows a list of peers
    Dummy,

    #[structopt(name = "wait")]
    Wait{
        #[structopt(long = "delay")]
        delay: u64,
    },

    #[structopt(name = "print")]
    PeerPrint,

    #[structopt(name = "msg")]
    MsgFromHex {
        #[structopt(short="s", long = "send")]
        /// send messages to connected peers
        send: bool,

        #[structopt(long = "hex", parse(try_from_str))]
        hex: Bytes,
    },
}

#[derive(Debug)]
pub struct Bytes(pub Vec<u8>);
impl FromStr for Bytes {
    type Err = hex::FromHexError;
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let vec = Vec::from_hex(src.trim())?;
        Ok(Bytes(vec))
    }
}

