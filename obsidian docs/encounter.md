---
tags:
  - game-state/regular
---

Action will replace the current Command and every match between them will be reduce the enemy's hp.
The Encounter's victor will be the only one left with hp. [[reasoning#incentive|why?]]
Players sees the current Command and the next one at the same time. [[reasoning#foresight|Why?]]
# initialization
Possible Actions, enemy hp and attack are defined.
then the game wait for the player to make the first move before attacking.
The amount of buttons and presses of each Action is taken from the received level.
# Command Making
An array of indices, long as the Instruction count, will reference relevant buttons.
The referenced buttons will be displayed in order.
# Command Checking 
The amount of fitting instructions will be reduced from the enemy's hp.

