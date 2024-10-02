Rushing Dungeons is made for the gameplay and everything else is in because constant tension is insufficient for a game.

Gameplay is assumed to compel by instructing the player to press the buttons (or any other from of input but in this docs the input method will always be referred to as buttons) correctly.

The core-mechanics are only relevant during an [[Encounter]] in order to let the game give a breather between encounters and to give them in a satisfying order.

The game response to presses only in a sequence that are called [[Action]]s

[[Command]]s are requirements and they are displayed as a list of characters for ease of implementation and removing ambiguity.
^command-goal

 One [[Command]] is showed at a time and one [[Action]] responses to it in order to make it easier to reason about the ideal requirement for the presses.
^response-pipeline

Every [[Action]] will contain elements that are unique from each other to prevent double presses.

The buttons will be held until the [[Action]] is completed to make the uniqueness of the presses intuitive, to let the player cancel an [[Action]] by releasing a button and to give Rushing Dungeons a feel different then other rhythm games.

The fastest reaction to a [[Command]] is always incentivised in order to offer every player the opportunity to push his limits and to let him finish an easy [[Encounter]] as fast as possible.

[[Action]]s can track more buttons and demand more presses to increase the possibility space and thus make more mistakes possible.
^possibility-space

The "Here" instruction simulate and surpasses the gameplay that rhythm games offers.
"Not Here" is a simple and intuitive subversion of "Here" and they could be combined to emerge new [[Command]]s.
^instructions

The Letters of a [[Command]] will be written left to right in the order of presses. Numbers may will be written before them to indicate that the letter skips spots in the order of presses.

The order of the buttons and the associated [[Command#instruction|Instructions]] are randomized in order to force the player to determine a correct [[Action]] from the entire possibility space for every [[Command]] under time pressure.  
^random-command

In the Dungeon the player will choose his next challenge (for now) in order to give him a light mental exercise in between the exhausting [[Encounter]]s.
^think

Some noteworthy events will happened randomly in order to make memorable moments that will stand out from the rest of the game.
^identity

The game will escalate in difficulty for as long as players will push through the [[Boss]]es.
^escalation

Better play and risk taking will be reworded with [[Hp]].
^incentive

Players that are too good for the [[Encounter]]s there facing could progress faster then others and to make them more interesting with [[Challenge]]s.