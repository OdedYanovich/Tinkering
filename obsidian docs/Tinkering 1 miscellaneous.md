---
goal: boundary
---

The main mechanics will only be used during in game events that are called [[Encounter]]s. [[Reasoning#^pipelline|Why?]] 

The main mechanics incentivising players to play according to requirements that are called Commands. [[Reasoning#^excluding-possibilities|Why?]] 
^commanding

Commands aren't mixed. [[Reasoning#^small-start|Why?]] 
^simple-commands

d = data, c = Command,
i(x) = x is an instruction,
$\forall d(d\in c\Longleftrightarrow i(d))$ 

d = displayed, c = character
i = $Instruction\in Current Command$
$\forall c(c\in i\implies c\in d)$ [[Reasoning#^char|Why?]] 

Faster reaction is incentivised during an [[Encounter]]. [[Reasoning#^reaction-time|Why?]] 

The buttons and presses that defines an Action is determined for each [[Encounter]]. [[Reasoning#^excluding-possibilities|Why?]] 

[[Encounter]]s distinguishing themselves from each other. [[Reasoning#^excluding-possibilities|Why?]] 
^level

Commands making a subset of the level that [[Action]] needs to be in.
^instructions

A Command is displayed according to it's order. 

An opportunity to decide how to react to information will be given before the option to make the action.
^forsite

Numbers are displayed to express gaps between indexed indices.

Random indexing is associating random buttons to random indices. [[Reasoning#^excluding-possibilities|Why?]] 
^random-command

The game can escalate in difficulty forever.
^escalation

The core mechanics incentivising behaviour with Hp. 
^incentive

The player's reaction time is always incentivised to decrease and is required to meet the [[Layer]]'s minimum.

If all of the states can transition to and from a main state, then all added states will require the same implementations.
^canter-state