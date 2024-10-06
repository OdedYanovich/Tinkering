A command is a $\mathbb{N}$ long list of letters and numbers that tells the player which [[Action]] will give Progress. [[miscellaneous#^command-goal|Why?]]

d = data

i(x) = x is an instruction,

$\forall d(c\in d\Longrightarrow i(d)$ [[|Why?]]

i = instruction

c(x) = x is a character
$\forall d(i\in d\Longrightarrow c(d)$ [[|Why?]]

a = [[Subject Matter#^Action|action]],
w(x) = x is wanted
c(x) = x is current
$\forall a(c(\exists i) \in \forall a\implies w(a))$

A command is made, kept then checked. [[miscellaneous#^response-pipeline|Why?]]

# Criteria
A set of commands that an [[Encounter]] may require.

Specify the number of tracked buttons, [[Action]]'s length and the potential [[#instruction]]. [[miscellaneous#^level|why?]]

The [[Command]]'s buttons will be distributed randomly across the [[Action]]s with the [[#instruction]]s associated with them randomly. [[miscellaneous#^random-command|Why?]]

# instruction
There are 3 types of instructions:
- Here
- Not Here
- Mix
[[miscellaneous#^instructions|Why?]]

# Command Checking 
The command is iterated upon and each instruction within it will index the responding [[Action]] to check whether the instruction is obeyed. 
