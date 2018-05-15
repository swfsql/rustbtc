TODO LIST
MACHINA:
    peer/ version & verack (handshake)
    worker/ version & verack
    admin/ get_headers, 


-> fazer newversion
-> fazer newverack

-> fazer getheaders
-> receber headers

102, 101, 101, 102, 105, 108, 116, 101, 114, 0, 0, 0

----
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
	feefilter
	reject
	mempool

	getblocks
	merkleblock
	blocktxn
	getblocktxn
	sendcmpctblock
	cmpctblock
