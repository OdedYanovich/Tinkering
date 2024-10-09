d = data
i(x) = x is an instruction,
$\forall d(d\in c\Longleftrightarrow i(d)$ [[|Why?]]

i = instruction
d = data
c(x) = x is a character
$\forall d(i\in d\Longrightarrow c(d)$ [[Goals/Dungeon Rush 1/miscellaneous#^command-goal|Why?]]

c(x) = x is current
i = instruction
a = [[Subject Matter#^Action|action]],
w(x) = x is wanted
$\forall a(c(\exists i) \in a\implies w(a))$ [[|Why?]]

c(x) = x is current
m = [[Subject Matter#^^Command|command]],
$c(\exists_{1}m)$ [[Goals/Dungeon Rush 1/miscellaneous#^response-pipeline|Why?]]
# Criteria
A set of commands that an [[Encounter]] may require.

The command buttons will be distributed randomly across the [[Action]]s with the [[#Relations]]s associated with them randomly. [[Goals/Dungeon Rush 1/miscellaneous#^random-command|Why?]]

# Relations
There are 3 types of Relations:
- Here
- Not Here
- Mix
[[Goals/Dungeon Rush 1/miscellaneous#^instructions|Why?]]
# Command Making
## declaration

## definition
# Command Checking 
The command is iterated upon and each instruction within it will index the responding [[Action]] to check whether the instruction is obeyed. 

