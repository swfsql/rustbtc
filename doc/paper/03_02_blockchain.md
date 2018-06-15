
## *Blockchain* completo
\label{n3:bchain}

Este módulo consiste em um *actor* que se comporta de acordo com uma máquina de estado e realiza funções essenciais que envolvem os dados de uma *blockchain* de *Bitcoin*. A definição do comportamento e da máquina estão em fase de prototipação. Inicialmente, as funções que necessárias de se implementar estão representadas por máquinas internas (estados da máquina externa) e são listadas na~\REF{fig:syncing} -- processo de sincronização dos dados referentes à *blockchain* do *Bitcoin*.

\begin{figure}[ht]
	\centering
	\caption{Máquina de estados *Syncing*}
	\includegraphics[width=0.64\textwidth]{sr_bchain.png}
	\begin{minipage}{\textwidth}
		\centering
		{\small Fonte: Os Autores (2018).}\par
	\end{minipage}
	\label{fig:syncing}
\end{figure}

O processo de sincronização é parte essencial de uma criptomoeda, para que o nodo possa verificar quaisquer recebimentos e realizar envios com base nestes recebimentos. Tal processo envolve o *download*, armazenamento, verificação, validação e gerenciamento dos blocos e demais informações. Com a existência dos requisitos de assincronia e concorrência, o comportamento esperado deste ator se mostrou mais complexo do que o inicialmente esperado.

Devido à necessidade dos nodos de escolher apenas a *blockchain* que tiver a maior somatória de *Proof-of-Work*, o nodo deve lidar com a competição entre várias *blockchains*, que são estruturas que podem ter pontos de intersecção uma com as outras, tornando-as complexas. Portanto o nodo desenvolvido irá sempre desconfiar da *blockchain* que segue, mesmo que de fato for a seguida pelos demais nodos, de um modo geral. Assim o nodo deverá sempre buscar por encadeamentos alternativos e verificar se algum é potencialmente superior à cadeia preferida. Neste aspecto, tal função é executada indefinidamente[^60], pois mesmo que todos os *peers* da rede já tivessem sido consultados, não há garantias de que algum *peer* terá uma *blockchain* com maior *Proof-of-Work* no futuro. Portanto, foi esboçado as principais funções relacionadas ao processo de sincronização e também foi destacado que tais funções podem, dependendo da situação em que o nodo se encontra, serem executadas em paralelo.

[^60]: Mesmo sendo uma função complexa a sua execução em vários momentos distintos não seria prejudicial ao desempenho do nodo, visto que o processo de sincronização para nodos que possuem a mesma *blockchain* consiste apenas no pedido de uma mensagem de "*Header*" e sua conferência.


A~\REF{fig:syncing} -- mostra as três funções principais do processo de sincronização: (a) *Look for, download and validate the best header chain*; (b) *Download the best block chain* e (c) *Validate the best block chain*. A separação de dois tipos de *downloads* e um tipo de verificação e validação é possível graças à própria estrutura da *blockchain* do *Bitcoin*, no qual os *headers* são até 4 ordens de magnitude menor do que os blocos, e podem indicar informações valiosas para tomadas de decisão.

\begin{figure}[!ht]
	\centering
	\caption{Máquina de estados *Syncing Headers* }
	\includegraphics[width=0.64\textwidth]{sr_bchain_header.png}
	\begin{minipage}{\textwidth}
		\centering
		{\small Fonte: Os Autores (2018).}\par
	\end{minipage}
	\label{fig:syncing_header}
\end{figure}

Um esboço da função (a) é demonstrado na~\REF{fig:syncing_header} -- no qual o nodo lida com as possibilidades de estar desconectado, de ser vítima de nodos que enviam dados inválidos ou indesejáveis[^61]. A finalidade deste procedimento é obter um conjunto de *header chains* com uma potencial *Proof-of-Work* maior do que a melhor *header chain* já verificada (neste caso, com os blocos também verificados e validados).


[^61]: Portanto, o nodo deve buscar economizar recursos de processamento e disco e banda de rede e então deve penalizar a conexão com tais *peers*.

Obtendo-se a *header chain* com o maior potencial, é desejável iniciar o download dos blocos referentes a esta cadeia dos *peers* específicos e que corroboram com esta cadeia -- processo (b). Porém, também pode ser desejável que o processo (a) recomece o seu próprio ciclo.

O processo (b), com um comportamento funcional pode ser visto na~\REF{fig:sr_bchain_blocks} -- possui alguns requisitos funcionais, como: muitos *downloads* em paralelo dos blocos com um ou mais *peers* -- sempre da *header chain* com maior potencial; penalização de *peers* com comportamentos indesejados; a escrita em disco dos blocos salvos; e a possibilidade da liberação de recursos do disco em caso de detecção de invalidade. Tais funcionalidades visam realizar o *download* e armazenamento de todas as informações indicadas pela melhor *header chain* em conhecimento do nodo -- *download* dos blocos completos.

\begin{figure}[!ht]
	\centering
	\caption{Máquina de estados *Syncing Blocks* (*download*) }
	\includegraphics[width=0.64\textwidth]{sr_bchain_blocks.png}
	\begin{minipage}{\textwidth}
		\centering
		{\small Fonte: Os Autores (2018).}\par
	\end{minipage}
	\label{fig:sr_bchain_blocks}
\end{figure}

\begin{figure}[!ht]
	\centering
	\caption{Máquina de estados *Syncing Blocks* (validação) }
	\includegraphics[width=0.64\textwidth]{sr_bchain_validate.png}
	\begin{minipage}{\textwidth}
		\centering
		{\small Fonte: Os Autores (2018).}\par
	\end{minipage}
	\label{fig:sr_bchain_validate}
\end{figure}

O processo (c), pode ser visualizado na~\REF{fig:sr_bchain_validate} -- e também tem alguns requisitos inicias, como: realizar verificações em paralelo, quando possível, na modalidade *master-slave* das transações de um bloco individualmente;  lidar com re-verificações caso sejam necessárias, quando um mesmo bloco possuir duas ou mais transações digitalmente encadeadas; fazer uso da biblioteca padrão de consenso[^63] para o corpo e *script* das transações; realizar validações parciais de blocos posteriores ao último bloco validado de uma cadeia totalmente validada, para potencialmente aproveitar recursos disponíveis; atualizar o conjunto *UTXO* a cada bloco validado; dentre outros.


[^63]: Escrita em C++ e de difícil e improdutivo reprodução em outras linguagens, do *Bitcoin Core*, é um componente destacável deste programa e suporta as verificações do histórico do *Bitcoin* e abrange muitos casos de validade ou invalidade desta criptomoeda.

Existem outras funções que não estão relacionadas com a sincronização do nodo (em primeira pessoa), mas sim como uma interface de serviço para exploradores e administradores de *blockchain*, carteiras e outros *peers* da rede. Tais procedimentos não foram elaborados nem implementados por este trabalho.




