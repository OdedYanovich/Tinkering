---
game state: example
---
All game state transitions are either from or to the Dungeon. [[Tinkering 1 miscellaneous#^canter-state|Why?]]
The player can restart at any unlocked Layer at any point. [[Tinkering 1 miscellaneous#^escalation|why?]]
Each Layer starts with 2 Encounters to choose each beaten Encounter will unlock another 2. 

# Score
Beaten Encounters increasing the player's score, once he is filled, the Layer is automatically replaced be the next one.

The scoring system tracks the current score, Layer and unlocked Layers.

# Enemy Maker
Enemy define an Action's properties, damage and hp.
The maker is using the current Layer to make an Enemy.

The first Layers are redosing enemies without randomness (2-1, 3-1, 3-2). They will end with a layer that include all of them in a row.
