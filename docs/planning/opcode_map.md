# TFS / OTClient — Mapa Semântico de Opcodes

> Referências:
> - Server: https://github.com/otland/forgottenserver
> - Client: https://github.com/otland/otclient

---

## Dados Especiais de Tile/Mapa (inline no stream de mapa)

| Hex | Constante | Descrição |
|-----|-----------|-----------|
| 0x60 | `StaticText` | Texto estático embutido no stream de tile |
| 0x61 | `UnknownCreature` | Criatura desconhecida; dados completos seguem |
| 0x62 | `OutdatedCreature` | Criatura conhecida mas com dados desatualizados no cache |
| 0x63 | `Creature` | Referência a criatura já conhecida (só ID) |

---

## 1. Autenticação / Login Server

| Hex | Constante | Direção | Descrição |
|-----|-----------|---------|-----------|
| 0x01 | `ClientEnterAccount` | C→S | Autenticação inicial (conta + senha + versão do client) |
| 0x0A | `LoginServerError` | S→C | Login falhou; string de erro |
| 0x14 | `LoginServerMotd` | S→C | Mensagem do dia (MOTD) |
| 0x1E | `LoginServerUpdateNeeded` | S→C | Versão do client desatualizada |
| 0x64 | `LoginServerCharacterList` | S→C | Lista de personagens da conta após login bem-sucedido |

---

## 2. Session / Handshake (Game Server)

| Hex | Constante | Direção | Descrição |
|-----|-----------|---------|-----------|
| 0x0A | `GameServerLoginOrPendingState` | S→C | Servidor sinaliza estado pendente/loading |
| 0x0F | `GameServerEnterGame` / `ClientEnterGame` | S↔C | Servidor libera entrada; client confirma pronto |
| 0x11 | `GameServerUpdateNeeded` | S→C | Versão do client muito antiga |
| 0x14 | `GameServerLoginError` / `ClientLeaveGame` | S↔C | Falha de login (S→C) / logout voluntário (C→S) |
| 0x15 | `GameServerLoginAdvice` | S→C | Mensagem de aviso de login (ex: segurança da conta) |
| 0x16 | `GameServerLoginWait` | S→C | Client em fila de login com tempo de espera |
| 0x17 | `GameServerLoginSuccess` | S→C | Login aceito; sessão estabelecida |
| 0x18 | `GameServerLoginToken` | S→C | Token de autenticação / sinal de fim de sessão |
| 0x1F | `GameServerChallenge` | S→C | Desafio XOR para encriptação do pacote de login |
| 0x28 | `GameServerDeath` | S→C | Notificação de morte do personagem |

---

## 3. Rede / Keep-alive

| Hex | Constante | Direção | Descrição |
|-----|-----------|---------|-----------|
| 0x1D | `GameServerPingBack` / `ClientPing` | S↔C | Server responde ping / client inicia ping |
| 0x1E | `GameServerPing` / `ClientPingBack` | S↔C | Server inicia ping / client responde |
| 0x32 | `GameServerExtendedOpcode` / `ClientExtendedOpcode` | S↔C | OTClient: opcode customizado para scripts Lua |
| 0x33 | `GameServerChangeMapAwareRange` / `ClientChangeMapAwareRange` | S↔C | Negociar viewport visível do mapa |

---

## 4. Mapa / Mundo

### Server → Client

| Hex | Constante | Descrição |
|-----|-----------|-----------|
| 0x64 | `GameServerFullMap` | Dados completos do viewport (login ou teleporte) |
| 0x65 | `GameServerMapTopRow` | Scroll norte: nova linha superior de tiles |
| 0x66 | `GameServerMapRightRow` | Scroll leste: nova coluna direita |
| 0x67 | `GameServerMapBottomRow` | Scroll sul: nova linha inferior |
| 0x68 | `GameServerMapLeftRow` | Scroll oeste: nova coluna esquerda |
| 0x69 | `GameServerUpdateTile` | Atualizar todos os itens de um tile |
| 0x6A | `GameServerCreateOnMap` | Adicionar item ou spawnar criatura no mapa |
| 0x6B | `GameServerChangeOnMap` | Atualizar item ou criatura existente no tile |
| 0x6C | `GameServerDeleteOnMap` | Remover item ou criatura do tile |
| 0x6D | `GameServerMoveCreature` | Criatura movida de um tile para outro |
| 0x82 | `GameServerAmbient` | Mudança de luz ambiente global |
| 0x83 | `GameServerGraphicalEffect` | Efeito visual mágico/spell numa posição |
| 0x84 | `GameServerTextEffect` | Texto/número flutuante (dano, cura) |
| 0x85 | `GameServerMissleEffect` | Animação de projétil entre duas posições |
| 0xBE | `GameServerFloorChangeUp` | Jogador sobe andar (escada / ladder) |
| 0xBF | `GameServerFloorChangeDown` | Jogador desce andar (buraco / escada) |

### Client → Server (Movimento)

| Hex | Constante | Descrição |
|-----|-----------|-----------|
| 0x64 | `ClientAutoWalk` | Enviar caminho de auto-walk (lista de direções) |
| 0x65 | `ClientWalkNorth` | Um passo norte |
| 0x66 | `ClientWalkEast` | Um passo leste |
| 0x67 | `ClientWalkSouth` | Um passo sul |
| 0x68 | `ClientWalkWest` | Um passo oeste |
| 0x69 | `ClientStop` | Cancelar auto-walk |
| 0x6A | `ClientWalkNorthEast` | Um passo nordeste (diagonal) |
| 0x6B | `ClientWalkSouthEast` | Um passo sudeste (diagonal) |
| 0x6C | `ClientWalkSouthWest` | Um passo sudoeste (diagonal) |
| 0x6D | `ClientWalkNorthWest` | Um passo noroeste (diagonal) |
| 0x6F | `ClientTurnNorth` | Virar personagem para norte (sem mover) |
| 0x70 | `ClientTurnEast` | Virar para leste |
| 0x71 | `ClientTurnSouth` | Virar para sul |
| 0x72 | `ClientTurnWest` | Virar para oeste |
| 0xBE | `ClientCancelAttackAndFollow` | Cancelar ataque e follow simultaneamente |

---

## 5. Container / Inventário

### Server → Client

| Hex | Constante | Descrição |
|-----|-----------|-----------|
| 0x6E | `GameServerOpenContainer` | Abrir janela de container com lista de itens |
| 0x6F | `GameServerCloseContainer` | Fechar janela de container |
| 0x70 | `GameServerCreateContainer` | Adicionar item num container aberto |
| 0x71 | `GameServerChangeInContainer` | Atualizar slot de item no container |
| 0x72 | `GameServerDeleteInContainer` | Remover item de slot do container |
| 0x78 | `GameServerSetInventory` | Definir item num slot de equipamento/inventário |
| 0x79 | `GameServerDeleteInventory` | Limpar slot de equipamento |
| 0xF5 | `GameServerPlayerInventory` | Snapshot completo do inventário/equipamento |

### Client → Server

| Hex | Constante | Descrição |
|-----|-----------|-----------|
| 0x77 | `ClientEquipItem` | Equipar item via hotkey |
| 0x78 | `ClientMove` | Mover item entre posições (inventário ↔ container ↔ chão) |
| 0x87 | `ClientCloseContainer` | Fechar container |
| 0x88 | `ClientUpContainer` | Navegar um nível acima na hierarquia de containers |
| 0xCA | `ClientRefreshContainer` | Solicitar atualização do conteúdo do container |
| 0xCB | `ClientBrowseField` | Visualizar todos os itens empilhados num tile |
| 0xCC | `ClientSeekInContainer` | Navegar/paginar dentro de container grande |

---

## 6. Itens / Interação

### Client → Server

| Hex | Constante | Descrição |
|-----|-----------|-----------|
| 0x82 | `ClientUseItem` | Usar/ativar item (uso próprio, ex: comida, corda) |
| 0x83 | `ClientUseItemWith` | Usar item em outro item (ex: chave em porta) |
| 0x84 | `ClientUseOnCreature` | Usar item em criatura (ex: poção em player) |
| 0x85 | `ClientRotateItem` | Rotacionar item no chão |
| 0x89 | `ClientEditText` | Submeter texto editado em item escrevível |
| 0x8A | `ClientEditList` | Submeter lista editada (ex: lista de acesso de porta) |
| 0x8C | `ClientLook` | Examinar item ou posição |
| 0x8D | `ClientLookCreature` | Examinar criatura na battle list |

### Server → Client

| Hex | Constante | Descrição |
|-----|-----------|-----------|
| 0x96 | `GameServerEditText` | Abrir janela de edição de texto em item escrevível |
| 0x97 | `GameServerEditList` | Abrir janela de edição de lista (ex: lista de porta) |
| 0xF4 | `GameServerItemInfo` | Metadados de item (descrição, stats) |

---

## 7. Comércio NPC / Entre Players / Mercado

### Server → Client

| Hex | Constante | Descrição |
|-----|-----------|-----------|
| 0x7A | `GameServerOpenNpcTrade` | Abrir loja do NPC com lista e preços |
| 0x7B | `GameServerPlayerGoods` | Lista de itens do player disponíveis para venda ao NPC |
| 0x7C | `GameServerCloseNpcTrade` | Fechar janela de comércio NPC |
| 0x7D | `GameServerOwnTrade` | Mostrar oferta própria em troca entre players |
| 0x7E | `GameServerCounterTrade` | Mostrar oferta do outro player |
| 0x7F | `GameServerCloseTrade` | Troca concluída (aceita, rejeitada ou cancelada) |
| 0xF6 | `GameServerMarketEnter` | Inicializar e abrir interface de mercado |
| 0xF7 | `GameServerMarketLeave` | Fechar interface de mercado |
| 0xF8 | `GameServerMarketDetail` | Detalhes de uma listagem específica |
| 0xF9 | `GameServerMarketBrowse` | Resultados de busca/browse do mercado |

### Client → Server

| Hex | Constante | Descrição |
|-----|-----------|-----------|
| 0x79 | `ClientInspectNpcTrade` | Inspecionar item da loja NPC |
| 0x7A | `ClientBuyItem` | Comprar item do NPC |
| 0x7B | `ClientSellItem` | Vender item ao NPC |
| 0x7C | `ClientCloseNpcTrade` | Fechar comércio NPC |
| 0x7D | `ClientRequestTrade` | Iniciar troca entre players |
| 0x7E | `ClientInspectTrade` | Inspecionar item na troca atual |
| 0x7F | `ClientAcceptTrade` | Aceitar troca |
| 0x80 | `ClientRejectTrade` | Rejeitar/cancelar troca |
| 0xF4 | `ClientMarketLeave` | Sair da interface de mercado |
| 0xF5 | `ClientMarketBrowse` | Buscar itens no mercado |
| 0xF6 | `ClientMarketCreate` | Criar oferta de compra ou venda |
| 0xF7 | `ClientMarketCancel` | Cancelar oferta própria |
| 0xF8 | `ClientMarketAccept` | Aceitar oferta de outro player |

---

## 8. Estado de Criatura / Player

### Server → Client

| Hex | Constante | Descrição |
|-----|-----------|-----------|
| 0x8C | `GameServerCreatureHealth` | Atualizar barra de vida da criatura (%) |
| 0x8D | `GameServerCreatureLight` | Atualizar emissão de luz da criatura |
| 0x8E | `GameServerCreatureOutfit` | Atualizar aparência/outfit da criatura |
| 0x8F | `GameServerCreatureSpeed` | Atualizar velocidade de movimento |
| 0x90 | `GameServerCreatureSkull` | Atualizar indicador de skull PvP |
| 0x91 | `GameServerCreatureParty` | Atualizar ícone de party |
| 0x92 | `GameServerCreatureUnpass` | Atualizar flag de passabilidade |
| 0x93 | `GameServerCreatureMarks` | Atualizar marcações coloridas na criatura |
| 0x95 | `GameServerCreatureType` | Identificar tipo (Player/Monster/NPC/Summon) |
| 0x9C | `GameServerBlessings` | Atualizar bênçãos divinas ativas |
| 0x9F | `GameServerPlayerDataBasic` | Stats principais: vocação, status premium |
| 0xA0 | `GameServerPlayerData` | Stats extendidos: HP, mana, capacidade, level, XP, stamina |
| 0xA1 | `GameServerPlayerSkills` | Todos os níveis de skill |
| 0xA2 | `GameServerPlayerState` | Flags de condição (envenenado, queimando, etc.) |
| 0xA3 | `GameServerClearTarget` | Limpar alvo de ataque/follow atual |
| 0xA7 | `GameServerPlayerModes` | Confirmar configurações de modo de combate |

---

## 9. Combate

### Server → Client

| Hex | Constante | Descrição |
|-----|-----------|-----------|
| 0xA4 | `GameServerSpellDelay` | Cooldown individual de spell |
| 0xA5 | `GameServerSpellGroupDelay` | Cooldown de grupo de spell |
| 0xA6 | `GameServerMultiUseDelay` | Cooldown de ação multi-uso de item |
| 0xB5 | `GameServerCancelWalk` | Cancelar caminho de auto-walk do player |
| 0xB6 | `GameServerWalkWait` | Instrução de delay/espera de movimento |
| 0xB7 | `GameServerUnjustifiedStats` | Penalidades de kill injustificado (PvP) |
| 0xB8 | `GameServerPvpSituations` | Flags de contexto PvP ativo |

### Client → Server

| Hex | Constante | Descrição |
|-----|-----------|-----------|
| 0xA0 | `ClientChangeFightModes` | Definir modo de combate (ofensivo/balanceado/defensivo) |
| 0xA1 | `ClientAttack` | Iniciar ataque em criatura |
| 0xA2 | `ClientFollow` | Seguir uma criatura |

---

## 10. Sistema de Party

### Client → Server

| Hex | Constante | Descrição |
|-----|-----------|-----------|
| 0xA3 | `ClientInviteToParty` | Enviar convite de party |
| 0xA4 | `ClientJoinParty` | Aceitar convite de party |
| 0xA5 | `ClientRevokeInvitation` | Cancelar convite enviado |
| 0xA6 | `ClientPassLeadership` | Transferir liderança da party |
| 0xA7 | `ClientLeaveParty` | Sair da party |
| 0xA8 | `ClientShareExperience` | Alternar XP compartilhado na party |
| 0xA9 | `ClientDisbandParty` | Dissolver a party |

---

## 11. Chat / Comunicação

### Server → Client

| Hex | Constante | Descrição |
|-----|-----------|-----------|
| 0xAA | `GameServerTalk` | Fala de criatura/player (balão ou mensagem de canal) |
| 0xAB | `GameServerChannels` | Lista de canais públicos disponíveis |
| 0xAC | `GameServerOpenChannel` | Abrir janela de canal com ID e nome |
| 0xAD | `GameServerOpenPrivateChannel` | Abrir canal privado com player |
| 0xB2 | `GameServerOpenOwnChannel` | Abrir canal pessoal do player |
| 0xB3 | `GameServerCloseChannel` | Fechar canal de chat |
| 0xB4 | `GameServerTextMessage` | Mensagem de sistema/status (info, aviso, loot, etc.) |
| 0xF3 | `GameServerChannelEvent` | Evento de canal (player entrou, saiu, expulso) |

### Client → Server

| Hex | Constante | Descrição |
|-----|-----------|-----------|
| 0x96 | `ClientTalk` | Enviar mensagem de chat (local/grito/sussurro/canal/privado) |
| 0x97 | `ClientRequestChannels` | Solicitar lista de canais disponíveis |
| 0x98 | `ClientJoinChannel` | Entrar num canal por ID |
| 0x99 | `ClientLeaveChannel` | Sair de canal |
| 0x9A | `ClientOpenPrivateChannel` | Abrir sessão de mensagem privada |
| 0x9E | `ClientCloseNpcChannel` | Encerrar conversa com NPC |
| 0xAA | `ClientOpenOwnChannel` | Solicitar criação de canal pessoal |
| 0xAB | `ClientInviteToOwnChannel` | Convidar player ao canal pessoal |
| 0xAC | `ClientExcludeFromOwnChannel` | Remover player do canal pessoal |

---

## 12. Aparência / Outfit

### Server → Client

| Hex | Constante | Descrição |
|-----|-----------|-----------|
| 0xC8 | `GameServerChooseOutfit` | Abrir janela de seleção de outfit/aparência |

### Client → Server

| Hex | Constante | Descrição |
|-----|-----------|-----------|
| 0xD2 | `ClientRequestOutfit` | Solicitar abertura do dialog de outfit |
| 0xD3 | `ClientChangeOutfit` | Submeter mudanças de outfit selecionadas |
| 0xD4 | `ClientMount` | Alternar montaria |

---

## 13. VIP / Lista de Amigos

### Server → Client

| Hex | Constante | Descrição |
|-----|-----------|-----------|
| 0xD2 | `GameServerVipAdd` | Adicionar entrada VIP à lista de amigos |
| 0xD3 | `GameServerVipState` | Atualizar status online/offline de amigo VIP |
| 0xD4 | `GameServerVipLogout` | Notificar desconexão de amigo VIP |

### Client → Server

| Hex | Constante | Descrição |
|-----|-----------|-----------|
| 0xDC | `ClientAddVip` | Adicionar player à lista VIP |
| 0xDD | `ClientRemoveVip` | Remover player da lista VIP |
| 0xDE | `ClientEditVip` | Editar entrada VIP (descrição/ícone) |

---

## 14. Quests

### Server → Client

| Hex | Constante | Descrição |
|-----|-----------|-----------|
| 0xF0 | `GameServerQuestLog` | Enviar diário completo de quests |
| 0xF1 | `GameServerQuestLine` | Enviar detalhes de uma missão específica |

### Client → Server

| Hex | Constante | Descrição |
|-----|-----------|-----------|
| 0xF0 | `ClientRequestQuestLog` | Solicitar diário de quests |
| 0xF1 | `ClientRequestQuestLine` | Solicitar detalhe de quest específica |

---

## 15. UI / Modal

### Server → Client

| Hex | Constante | Descrição |
|-----|-----------|-----------|
| 0xFA | `GameServerModalDialog` | Exibir dialog modal com botões e escolhas |

### Client → Server

| Hex | Constante | Descrição |
|-----|-----------|-----------|
| 0xF9 | `ClientAnswerModalDialog` | Submeter escolha do player no dialog |

---

## Ranges de ID de Criatura

| Range | Início | Fim | Notas |
|-------|--------|-----|-------|
| Players | 0x10000000 | 0x3FFFFFFF | IDs de criaturas player |
| Monsters | 0x40000000 | 0x7FFFFFFF | IDs de criaturas monstro |
| NPCs | 0x80000000 | 0xFFFFFFFF | IDs de NPC |

---

## Colisões de Opcode (mesmo hex, direções opostas)

> O mesmo byte hex é reutilizado em direções diferentes — não há ambiguidade pois cada direção tem seu próprio stream TCP.

| Hex | S→C | C→S |
|-----|-----|-----|
| 0x1D | `GameServerPingBack` | `ClientPing` |
| 0x1E | `GameServerPing` | `ClientPingBack` |
| 0x64 | `GameServerFullMap` | `ClientAutoWalk` |
| 0x7A | `GameServerOpenNpcTrade` | `ClientBuyItem` |
| 0x83 | `GameServerGraphicalEffect` | `ClientUseItemWith` |
| 0x96 | `GameServerEditText` | `ClientTalk` |
| 0xA0 | `GameServerPlayerData` | `ClientChangeFightModes` |
| 0xAA | `GameServerTalk` | `ClientOpenOwnChannel` |
| 0xD2 | `GameServerVipAdd` | `ClientRequestOutfit` |
| 0xDC | `GameServerTutorialHint` | `ClientAddVip` |
