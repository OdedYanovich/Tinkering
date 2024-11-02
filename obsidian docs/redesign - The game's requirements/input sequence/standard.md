-
# flow chart
```mermaid
flowchart TD 
	program-start --> user-input --> value-response --> sequence-storage --> output --> program-end 
	set-response-pairs --> value-response --> program-end
	capacity --> sequence-storage
	length-response-pairs --> sequence-storage
``` 
# user-input
Abstract any supported input method to a function without parameters.
# value-response
Maps a given value to a response.
# set-response-pairs
Pairs the values to responses.
# sequence-storage
Remember inputs by order their amount can be checked for conditions.
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

