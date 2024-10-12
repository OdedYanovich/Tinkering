c = [[Command]], 
d(x) = x is displayed
a = current [[Action]] 
r(x) = x is replaced and Progress is boosted
$\exists_{1} c(a\in c\implies r(c))$ [[Goals/Dungeon Rush 1/miscellaneous#^commanding|Why Command matter?]], [[Goals/Dungeon Rush 1/miscellaneous#^simple-commands|Why 1 command?]] 

The Encounter end with the first of the Hp/Progress bar. Victory for the Progress, run end for the Hp. [[Goals/Dungeon Rush 1/miscellaneous#^incentive|why?]] 

# Initiation
A Level is received, then the game wait for the player to make the first move before Hp will start to decrease.
The amount of buttons and presses of each Action is taken from the received level.
# Command Making
Possible Commands are determined by the received level.
Random indices, random buttons and signs will be kept together.
The creating is separated into 2 stages.
In the first stage, a random amount of Mix signs will turn into a Here signs and the rest into a Not Here sign.
In the second stage, 2 random arrays of indices are made, 1 for the Command's indices and the other for the array of relevant buttons. There length is equal to the amount of signs. At last these arrays are zipped into an array of Instructions (a Command).
# Command Checking 
For each instruction, the index within it will be used on the Action to see whether the button in the Action matches the instruction. if they are all matching, the player is reworded. 

