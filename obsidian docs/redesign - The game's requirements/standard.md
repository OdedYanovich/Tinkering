
# flow chart
```mermaid
flowchart TD 
	program-start --> user-input --> response --> sequence-storage --> output --> program-end 
	set-response-pairs --> response --> program-end
	capacity --> sequence-storage
	condition-response-pairs --> sequence-storage
``` 
# user-input
Abstract any supported input method to a function without parameters.
# response
Maps a given value to a response.
# set-response-pairs
Pairs the values to responses.
# sequence-storage
# condition-response-pairs
Defines the effect of inputs.
# output
The user's input or the ordered-set

# value -> response map
## flow chart
```mermaid
flowchart TD
program-start --> value --> map --> program-end 
values-function-pairs --> map
```
## reasoning
The standard's users can ignore unwanted inputs.

# mods
## flow chart
```mermaid
flowchart TD
    combat <--> location
    options <--> location
    credit <--> location
    exit-option <--> location
```
State transitions are caused be output-received.