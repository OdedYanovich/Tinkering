# stages #goal/reason #future #graph 
To extend [[#brunches goal/reason graph future|brunches]], the graph of the encounter can be divided into subgraphs that are connected by "one-way-streets".
# time window #goal/reason #future 
A major design decision is to not put range of time in which the player is expected to response to a Command, this design limitation is temporary, and ones removed different kind of limitations will become available, like limiting multiple Commands, and requiring brunches to be picked.
# Tinkering 1 miscellaneous #goal/boundary 
The main mechanics are only relevant during Encounters. [[#pipeline|Why?]]
## commanding
The main mechanics incentivising players to play according to requirements that are called Commands. [[#excluding-possibilities|Why?]]
## simple-commands
Commands aren't mixed (yet). 

d = data, c = Command,
i(x) = x is an instruction,
$\forall d(d\in c\Longleftrightarrow i(d))$ 

d = displayed, c = character
i = $Instruction\in Current Command$
$\forall c(c\in i\implies c\in d)$ [[#char|Why?]]

The main mechanics incentivizing faster reaction time. [[#reaction-time|Why?]]

Action is redefined in each game-state transition. [[#excluding-possibilities|Why?]]
## level
Each encounters is unique.

## instructions
Commands making a subset of the level that Action needs to be in.

A Command is displayed according to it's order. 

Numbers are displayed to express gaps between indexed indices.
## random-command
Random indexing is associating random buttons to random indices. [[#excluding-possibilities|Why?]]
## escalation
The game can escalate in difficulty forever.

The player's reaction time is always incentivised to decrease and is required to meet the Layer's minimum.
## canter-state
If all of the states can transition to and from a main state, then all added states will require the same implementations.
# Tinkering future miscellaneous #goal/boundary 
Encounters aren't necessarily random.

Operations on Levels.

A state will be affected by players and Commands independently

Make enemy's attacks effect be determined by the state. 

More options in the Dungeon.
# Tinkering future Not Here implications #goal/reason 
"Not Here" is an alternative instruction that was meant to be mixed with the current "Here".
Not Here restrict the player from pressing the associated button.

Here associated aren't available for Not Here instructions.
Not Here associated and unassociated are available for Not Here instructions but Not Here associated including the exception of the button associated with said instruction.

Each Not Here's associated set can be determined by excluding Here associated or including Not Here members. Those ways require excluding the button associated with the current Not Here. Each button can be replaced with an unassociated button that is known to be safe.

The act of determining a set might require players to examine the entire Command (unlike a Command that only contain Here, in which each instruction can be acted upon immediately).
# unique encounter's glyphs #goal/reason #future 
[[#brunches goal/reason graph future|conditions]] need a way to be express sets of Actions. A unique sigh can be used to express specific set that could be mixed with other subset expressions to make make sets that are fur more complicated then anything that players can be expected to parse for each Command.

Glyphs can even be made to make before the Encounter for it.

