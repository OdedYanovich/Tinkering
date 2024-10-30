
# flow chart
```mermaid
flowchart TD 
	program-start --> input --> sequence-keeper --> output --> program-end 
	input --> program-end 
``` 
# reasoning
The standard's uses an input function, sequence descriptions, and a value white list
to return values that

The standard's implementors can produce values that can guarantee:
A sequence of given values 
Only 
# comments
## ordered-set
Keep inputs then sand them.
When a limit is reached, sand itself instead, then clear itself.
Certain input will make an early clear.
## output
The user's input or the ordered-set
# iterations
## 1
```mermaid
flowchart TD
    input <--> output
```
## 2
```mermaid
flowchart TD
    input-function <--> input-function/set --> input-function-usage/ordered-set --> input-function/output --> input-function
    input-function-usage/ordered-set --> ordered-set/limit/output --> input-function
```
## 3
```mermaid
flowchart TD
    input-receiver --> input-function-usage/ordered-set
    input-function-usage/ordered-set --> ordered-set/limit/output --> input-receiver
    input-function-usage/ordered-set --> input-function/output --> input-receiver  
```
## 4
```mermaid
flowchart TD
    input-receiver --> ordered-set--> output --> input-receiver
```
## 5
```mermaid
flowchart TD
   program-start --> input --> ordered-set--> output --> input
    output --> program-end
```
## 6
```mermaid
flowchart TD 
	program-start --> input --> ordered-set --> output --> program-end 
	input --> program-end 
``` 
## 7
```mermaid
flowchart TD 
	program-start --> input --> ordered-set --> output --> program-end 
	input --> program-end 
	clearing-set --> ordered-set
	size-limit --> ordered-set
	last-ordered-set --> ordered-set
``` 
## 8
```mermaid
flowchart TD 
	program-start --> input --> sequence-keeper --> output --> program-end 
	input --> program-end 
``` 
