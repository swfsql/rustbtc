TODO LIST
MACHINA:
    peer/ version & verack (handshake)
    worker/ version & verack
    admin/ get_headers, 


-> fazer newversion
-> fazer newverack


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

Organizar commons
