# flow chart
```mermaid
flowchart TD
   program-start --> input --> ordered-set--> output --> input
    output --> program-end
```

# reasoning
The standard's users can response to sequences of unique inputs.
# comments
## ordered-set
Keep inputs.
Sand them as output.
When a limit is reached, sand itself as output instead, then clear itself.
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
