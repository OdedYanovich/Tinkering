Players are incentivised to press the buttons according to [[Command]]s during an [[Encounter]]. [[Reasoning#^excluding-possibilities|Why?]] 

Button presses are converted to [[Action]]s before passing onwards.

Each [[Command]] is displayed as a list of characters. [[Reasoning#^char|Why?]] 
^command-goal

c = [[Command]], 
e() = during an [[Encounter]]
d(x) = x is displayed
a() = [[Action]] was filled
r(x) = x is replaced
$e()\implies (\exists_{1} c(d(c)\land a()\implies r(c)))$ [[Reasoning#^pipelline|Why?]] 
^response-pipeline

 a = [[Action]], 
k/y = key,
c(x , y) = x contain y
$\forall a \forall k \forall y(c(a,k)\land c(a,y)\implies x\neq y)$ [[Reasoning#^double-press|Why?]]

 a = [[Action]], 
r(x) = x is relevant
f(x) = x is full
$\forall a(f(a) \Longleftrightarrow r(a)))$ [[Reasoning#^short-sequence|Why?]] 

Faster reaction is incentivised during an [[Encounter]]. [[Reasoning#^reaction-time|Why?]] 

The buttons and presses that defines an [[Action]] is determined for each [[Encounter]]. [[Reasoning#^excluding-possibilities|Why?]] 

[[Encounter]]s distinguishing themselves from each other. [[Reasoning#^excluding-possibilities|Why?]] 
^level

A [[Command]] limits the set of wanted [[Action]]s by indexing buttons with the command's order.
^instructions

A [[Command]] is displayed according to it's order. 

Numbers are displayed to express gaps between indexed indices.

Random indexing are associating random buttons to random indices. [[Reasoning#^excluding-possibilities|Why?]] 
^random-command

The game will escalate in difficulty for as long as players will push through the [[status#Boss|Bosses]].
^escalation

Hp/progress is an incentive for:
Faster reaction to commands.
Taking risks. [[Reasoning#^excluding-possibilities|Why?]] 
^incentive

The player's reaction time is always incentivised to decrease and is required to meet the [[Layer]]'s minimum.

Players can simplify Levels with [[Deal]]s.
^custom-difficulty