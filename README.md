This is a graduation project being worked by *Felipe Cetrulo* and *Thiago Machado* and oriented/supervisioned by *Paulo Alvarenga*, from *Universidade Federal de Itajubá*. The goal is to use *Rust* (programming language) capabilities for a Bitcoin node implementation. The goal implies studying and learning about both Rust and Bitcoin.

TODO LIST
MACHINA:
    peer/ version & verack (handshake)
    worker/ version & verack
    admin/ get_headers, 



fazer o ator do Router (comutador: ator c/ canais para peers && canal para o bchain)
canal de cadastro do sched pro router (cadastro de peers/adm no ator do router: mata um uso do mutex)
canal do wkr pro comut (mata outro uso do mutex)
fazer o ator do bchain (aí então já teremos todos os atores separados e isolados)
registro inicial do bchain
...
máquinas.. do bchain
...
fazer download de headers,
...
fazer alguma validação de headers,
...
salvar em disco (ou algo do tipo),
...
pedir download de blocos,
...
fazer alguma validação de blocos,   
...

CODEC:
	getdata (prioridade 1)
	notfound (todo)
	getblocks (todo)
	block (prioridade 1)
	inv (prioridade 1)

	reject
	mempool

	merkleblock
	blocktxn
	getblocktxn
	sendcmpctblock
	cmpctblock

# Goal checkpoints
ps. may change anytime, including the enumeration.

- [ ] 1 Codec for the Bitcoin Network Protocol.
    - [x] 1.1 Dummy implementation.
    - [ ] 1.2 Implement for *serde*.
    - [ ] 1.3 Minimum structures (to receive blocks and send transactions).
- [ ] 2 Node for a P2P Network over TCP.
    - [ ] 2.1 Peer over TCP connection.
        - [x] 2.1.1 Dummy TCP connection.
        - [ ] 2.1.2 Peer machine.
            - [x] 2.1.2.1 Dummy implementation.
            - [ ] 2.1.2.2 ...
    - [ ] 2.2 Admin CLI over TCP.
        - [x] 2.2.1 Dummy TCP connection.
        - [x] 2.2.2 Dummy CLI.
        - [ ] 2.2.3 Admin machine.
            - [ ] 2.2.3.1 ...
    - [ ] 2.3 Executions requests and responses
        - [ ] 2.3.1 Basic request receiver and scheduler
        - [ ] 2.3.2 Dummy Execution worker
- [ ] 3 Blockchain Structures
    - [ ] 3.1 Validate transactions
        - [ ] 3.1.1 Execute transactions' scripts.
            - [ ] 3.1.1.1 Codec for the OPCodes.
            - [ ] 3.1.1.2 Functionality for the OPCodes.
            - [ ] 3.1.1.3 ...
    - [ ] 3.2 Validate, save and read blocks.
        - [ ] 3.2.1 Validate the block next to genesis.
        - [ ] 3.2.2 Save blocks on disk.
        - [ ] 3.2.3 ...
    - [ ] 3.3 ...
- [ ] 4 Wallet implementation.
    - [ ] 4.1 ...
- [ ] 5 ...


# Approach

We started by reading about Bitcoin itself, with "Mastering Bitcoin", "en.bitcoin.it", "bitcoin.com" and others. Then we read about Rust, with "The Rust Book" and others; and after that, we studied selected exercises from "ProjectEuler" and others to be solved in Rust. After that, we moved into specific topics such as enum structures, generic parameters, error handling, traits, cargo, futures concept and async networking, and so on. We found out this would be necessary for us to start this node project.

## Server & Peers

The server is run on tokio crate, where tokio's executor manages the threads work stealing for the futures that needs execution, and also the callback propagation among the futures combinations/logistic. Each other connected node is then represented internally as a peer, with it's own isolated structure and workflow; each has it's own instance of a typestate machine, where each state is also isolated from others, and may represent yet inner typestate machines.

Any state from any machine will not do any meaningful operation nor change before depending on the "readness" of a future used in that state, including other futures readness. So if a procedure depends on various consecutives async steps, various states must be defined, where the readness will be tested only once for each state, e.g. readness testing implies in a previous state transition. This subdivision on states may be represented as an inner typestate machine.

Since peers may be run async in various threads, all of their communication is done with messages channels. Exceptions are debug information printing and socket communication (with the other node itself).

## Admin

Admin is similar to a Bitcoin peer itself, where it's communication uses message channels. But the TCP socket codec and structural machine differ. The admin uses telnet connection to the appropriate address, and then may send messages and receive responses similarly to a standard CLI. While a RPC interface is not [learned and] implemented, and for this prototyping purpose, this approach seems sufficient and extensible.















