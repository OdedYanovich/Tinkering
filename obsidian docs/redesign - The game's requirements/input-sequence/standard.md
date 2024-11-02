
# flow chart
```mermaid
flowchart TD 
	invocation --> user-input --> value-response --> sequence-storage --> output --> program-end 
	value-response --> return
``` 
# user-input
Abstract any supported input method to a function without parameters. 
# value-response
Maps a given value to a response.
# sequence-storage
Remember inputs by order.
# output
The sequence.
