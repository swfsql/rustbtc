
# PROJETO DESENVOLVIDO
\label{n1:proc}

## Considerações Iniciais

Conforme mencionado na~\REF{n3:proto_node} -- Antonopoulos considera que um nodo de *Bitcoin* possui quatro tipos de funcionalidades principais: o de carteira, o da mineração, de armazenar/acessar e processar a *blockchain* completa e de nodo de roteamento. Utilizando esta modularização, o programa foi planejado e o desenvolvimento dele foi iniciado, no qual a ordem de implementação dos módulos escolhida foi: primeiro o módulo de roteamento, então os procedimentos necessários para atender às chamadas de comandos remotos e por fim, e os principais elementos necessários à aquisição dos dados da *Blockchain*.

O funcionamento e a lógica de parte destas funcionalidades para o programa implementado serão mais aprofundados em seções específicas. O código da implementação é *open-source* \cite{rustbtc}.