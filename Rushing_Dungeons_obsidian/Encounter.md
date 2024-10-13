c = Command, 
d(x) = x is displayed
a = current Action 
r(x) = x is replaced and Progress is increased
$\exists_{1} c(a\in c\implies r(c))$ [[Goals/Dungeon Rush 1/miscellaneous#^commanding|Why Command matter?]], [[Goals/Dungeon Rush 1/miscellaneous#^simple-commands|Why 1 command?]] 

The Encounter end with the first of the Hp/Progress bar. Victory for the Progress, run end for the Hp. [[Goals/Dungeon Rush 1/miscellaneous#^incentive|why?]] 

Let the player see the current Command and the next one at the same time. [[Goals/Dungeon Rush 1/miscellaneous#^forsite|Why?]] 
# initialization
A Level is received, then the game wait for the player to make the first move before Hp will start to decrease.
The amount of buttons and presses of each Action is taken from the received level.
# Command Making
Possible Commands are determined by the received level.
An array of indices, long as the Instruction count, will reference relevant buttons. 
# Command Checking 
Each instruction has a matching button, if every instruction was matched, the check is passed.

