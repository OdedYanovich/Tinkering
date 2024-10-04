A command is a $\mathbb{N}$ long list of letters and numbers that tells the player which [[Action]] will give Progress. [[miscellaneous#^command-goal|Why?]]

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
