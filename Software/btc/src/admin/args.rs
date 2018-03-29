use codec::msg;
use std::net::SocketAddr;

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
}

#[derive(StructOpt, Debug)]
/// Peer commands
pub enum PeerCmd {
    #[structopt(name = "list")]
    /// Shows a list of peers
    List {
        #[structopt(long = "show-state")]
        /// Shows the peer's state
        show_state: bool,

        #[structopt(long = "show-addr")]
        /// Shows the peer's connection address
        show_addr: bool,

        #[structopt(long = "show-duration")]
        /// Shows the peer's connection duration
        show_duration: bool,
    },

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

        #[structopt(long = "id")]
        /// The peer's ID that is to be disconnected. example: <ID="3">
        id: u64,
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
}
