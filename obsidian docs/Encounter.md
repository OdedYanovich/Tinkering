c = Command, 
d(x) = x is displayed
a = current Action 
r(x) = x is replaced and enemy is damaged
$\exists_{1} c(a\in c\implies r(c))$ [[Goals/Dungeon Rush 1/miscellaneous#^commanding|Why Command matter?]], [[Goals/Dungeon Rush 1/miscellaneous#^simple-commands|Why 1 command?]] 

The Encounter's victor will be the only one left with hp. [[Goals/Dungeon Rush 1/miscellaneous#^incentive|why?]] 

Let the player see the current Command and the next one at the same time. [[Goals/Dungeon Rush 1/miscellaneous#^forsite|Why?]] 
# initialization
Possible Actions, enemy hp and attack are defined.
then the game wait for the player to make the first move before attacking.
The amount of buttons and presses of each Action is taken from the received level.
# Command Making
An array of indices, long as the Instruction count, will reference relevant buttons.
The referenced buttons will be displayed in order.
# Command Checking 
The amount of fitting instructions will be reduced from the enemy's hp.

