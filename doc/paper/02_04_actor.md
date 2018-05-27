## *Actor Model*
\label{n2:actor}

Devido a crescente demanda por computadores com multinúcleos é desejado utilizar suas funcionalidades. Conforme \citeonline{sutter} informa, em apenas seis anos, desde o marco onde cada casa possuía o seu computador, todos os dispositivos possuem processadores com vários núcleos e este fato não irá mudar devido o desempenho que eles oferecem. Um paradigma que que explora o paralelismo para estes novos dispositivos é o *Actor model*.

Neste paradigma só é possível realizar um processamento a partir da análise de uma comunicação. Para \citeonline{agha} este modelo consiste de um ator que executa uma computação quando recebe uma comunicação (contido em uma tarefa), a partir desta noção o sistema pode criar novos atores e outras tarefas e terminar elas quando não tiverem mais uso. Um programa que utiliza este conceito deve possuir: comportamento definido ao se receber uma tarefa; a opção de criar novos atores quando necessário; criar novas tarefas ao executar um comando de envio para outro ator; possuir recepcionista capazes de receber comunicação externa e ter representação de atores externos que não são definidos no programa em si.
