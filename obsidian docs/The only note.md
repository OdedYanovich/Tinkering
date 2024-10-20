# action state machine
#future #action #goal/code
Actions will be determined by a state machine that will use match less inputs then before. Actions will be determined by a state machine that will use match less inputs then before.

# Action
#goal/code #future #action 
Actions will be determined by a state machine that will use match less inputs then before. Players will be required to learn the state machine of each Encounter. Some Actions will be exclusive to certain state.

 a = Action, 
p/b = button press,
c(x , y) = x contain y
$\forall a \forall p \forall b(c(a,p)\land c(a,b)\implies p\neq b)$ [[#Reasoning#double-press|Why?]]

p = relevant button press, m = max size
i(x) = x is an Action that impacts the game
$\forall p(|p|>m \implies i(p))$ [[#reasoning#short-sequence|Why one press?]] [[#reasoning#cenacle-action|Why cancelling?]]
# Brainstorm
#none 
$Level \ni command \ni instruction = (button,index, relation)$
$Level Handler \xRightarrow[]{\mbox{criterion}}Encounter$

Layers will make the Encounters in the player's desirable range. Players will rush through Layers until a correct one will block them.
What will happened if a Layer's Encounter range will only be partly in the desirable range?
How will the desirable range be effected by the required average reaction time?
What is the desirable range?
# alternative progression
#future #goal/reason

Instead of hp for the player and the enemy, more complicated goals could be introduced, like effecting a [[#brunches|condition]].
# brunches
#goal/reason #graph #future 

As an extension to [[#sequence]], now brunches are introduced. A Command can by connected to many Commands with a condition that will determine the next Command.

The game's subjects is the condition that the responding Action will be in a subset of the potential responses.
## future
There are countless possibilities for types of conditions, from randomness, counters and external effects to any mix of them.

# Challenge
#goal/code #future 
a = Action, c = Challenge,
r(x, y) = x removes y
e(x ,y) = x exposes y
h(x) = x just happened
o(x) = current Command $\owns x$
$\exists c\exists a(h(a)\wedge e(c,a)\wedge o(a)\implies r(c,a))$ [[|Why?]]

 c = challenge,
r() = reword is given
$|c|==0\implies r()$ [[|Why?]]

# Dungeon
#game-state/core
All game state transitions are either from or to the Dungeon. [[#Tinkering 1 miscellaneous#canter-state|Why?]]
The player can restart at any unlocked Layer at any point. [[#Tinkering 1 miscellaneous#escalation|why?]]
Each Layer starts with 2 Encounters to choose each beaten Encounter will unlock another 2. 

## Score
Beaten Encounters increasing the player's score, once he is filled, the Layer is automatically replaced be the next one.

The scoring system tracks the current score, Layer and unlocked Layers.

## Enemy Maker
Enemy define an Action's properties, damage and hp.
The maker is using the current Layer to make an Enemy.

The first Layers are redosing enemies without randomness (2-1, 3-1, 3-2). They will end with a layer that include all of them in a row. 
# Encounter
#game-state/regular 

Action will replace the current Command and every match between them will be reduce the enemy's hp.
The Encounter's victor will be the only one left with hp. [[#Tinkering 1 miscellaneous#incentive|why?]]
Players sees the current Command and the next one at the same time. [[#Tinkering 1 miscellaneous#foresight|Why?]]
## initialization
Possible Actions, enemy hp and attack are defined.
then the game wait for the player to make the first move before attacking.
The amount of buttons and presses of each Action is taken from the received level.
## Command Making
An array of indices, long as the Instruction count, will reference relevant buttons.
The referenced buttons will be displayed in order.
## Command Checking 
The amount of fitting instructions will be reduced from the enemy's hp.

# game state
#goal/code #game-state 
The game is a state machine that is associated with:
data that effects the visuals, audio, Action length and optional function call.
# instruction
#goal/reason #future #action 
instructions could express more then a requirement to press in a relative order, but to avoid said order, or to be in a distance from another button or any possible relationship with any other describable element in the Action or outside of it.

Each of these changes will change the game from a rhythm game to a real action game.
# items
#goal/reason #future  

the rules that will define Encounter will be effected by items that can be gained in the dungeon.
# Multiplayer
#goal/reason #future 
Game mod offers players an encounter that plays for a limited time. A score will be assigned to the player and he will be added to an Instagram of all players. Set of daily Encounters is made for that purpose. Some alternative mod is necessary to make a permanent Instagram.

[[#shard game state|shard game state]] is used by 2 players that are viewing the other's Actions as their Commands.
# shard game state
#goal/reason #future #graph 
To replace [[#brunches]], Actions will no longer be matched to a Command. Instead Commands describe the effect that Actions will have on a state.
# Options
#game-state/regular 
Lets players change: volume, credit, color scheme and font.
Color Schemes and fonts are potential collectables.
# preferred Commands
#goal/reason #graph #future 
To extend [[#brunches]], if players have a say in regards to their future Commands, then choice may bring consequences.
# reasoning
#goal/reason 
## pipeline
The core mechanics aren't expected to be used constantly.
## char
Characters are easy to display.
## small-start
During an Encounter, a single Action is effecting a single Command and both of them are completely independent from other Actions and Commands in order to make it easier for the level designer to reason about the Encounter.
## double-press
Double pressing is annoying.
## cenacle-action
Regret opportunity is desired.
## reaction-time
Improvement is reworded with time investment.
## excluding-possibilities
Determining a subset of known possibilities according to a subset of known restrictions is fun.
## short-sequence
Allowing for sequences of different length will make things complicated.
## no-music
A few sound effects might meet the needs of the entire game
# sequence
#goal/reason #graph #future 
Instead of randomly selected Commands from a set, each Command is linked to another in order to create a circle.

Any link that isn't in the main circle is
# Sound
#goal/boundary
The sound implies that players are tinkering with a machine. [[#reasoning#no-music|Why?]]

They're sounds for each button press.

There is a sound for for cancelled/failed Action.

They're sounds for a successful Action.
# stages
#goal/reason #future #graph 
To extend [[#brunches]], the graph of the encounter can be divided into subgraphs that are connected by "one-way-streets".
# time window
#goal/reason #future 
A major design decision is to not put range of time in which the player is expected to response to a Command, this design limitation is temporary, and ones removed different kind of limitations will become available, like limiting multiple Commands, and requiring brunches to be picked.
# Tinkering 1 miscellaneous
#goal/boundary 
The main mechanics are only relevant during Encounters. [[#reasoning#pipeline|Why?]]
## commanding
The main mechanics incentivising players to play according to requirements that are called Commands. [[#reasoning#excluding-possibilities|Why?]]
## simple-commands
Commands aren't mixed. [[#reasoning#small-start|Why?]]

d = data, c = Command,
i(x) = x is an instruction,
$\forall d(d\in c\Longleftrightarrow i(d))$ 

d = displayed, c = character
i = $Instruction\in Current Command$
$\forall c(c\in i\implies c\in d)$ [[#reasoning#char|Why?]]

The main mechanics incentivizing faster reaction time. [[#reasoning#reaction-time|Why?]]

Action is redefined in each game-state transition. [[#reasoning#excluding-possibilities|Why?]]
## level
Encounters distinguishing themselves from each other. [[#reasoning#excluding-possibilities|Why?]]

## instructions
Commands making a subset of the level that Action needs to be in.

A Command is displayed according to it's order. 
## foresight
An opportunity to decide how to react to information will be given before the option to make the action.


Numbers are displayed to express gaps between indexed indices.
## random-command
Random indexing is associating random buttons to random indices. [[#reasoning#excluding-possibilities|Why?]]
## escalation
The game can escalate in difficulty forever.
## incentive
The core mechanics incentivising behaviour with Hp. 

The player's reaction time is always incentivised to decrease and is required to meet the Layer's minimum.
## canter-state
If all of the states can transition to and from a main state, then all added states will require the same implementations.
# Tinkering future miscellaneous
#goal/boundary 
Encounters aren't necessarily random.

Operations on Levels.

A state will be affected by players and Commands independently

Make enemy's attacks effect be determined by the state. 

More options in the Dungeon.
# Tinkering future Not Here implications
#goal/reason 
"Not Here" is an alternative instruction that was meant to be mixed with the current "Here".
Not Here restrict the player from pressing the associated button.

Here associated aren't available for Not Here instructions.
Not Here associated and unassociated are available for Not Here instructions but Not Here associated including the exception of the button associated with said instruction.

Each Not Here's associated set can be determined by excluding Here associated or including Not Here members. Those ways require excluding the button associated with the current Not Here. Each button can be replaced with an unassociated button that is known to be safe.

The act of determining a set might require players to examine the entire Command (unlike a Command that only contain Here, in which each instruction can be acted upon immediately).
# unique encounter's glyphs
#goal/reason #future 
[[#brunches|conditions]] need a way to be express sets of Actions. A unique sigh can be used to express specific set that could be mixed with other subset expressions to make make sets that are fur more complicated then anything that players can be expected to parse for each Command.

Glyphs can even be made to make before the Encounter for it.