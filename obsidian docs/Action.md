---
goal: boundary
tags:
  - code
---

 a = Action, 
p/b = button press,
c(x , y) = x contain y
$\forall a \forall p \forall b(c(a,p)\land c(a,b)\implies p\neq b)$ [[Reasoning#^double-press|Why?]]

p = relevant button press, m = max size
i(x) = x is an Action that impacts the game
$\forall p(|p|>m \implies i(p))$ [[Reasoning#^short-sequence|Why one press?]] [[Reasoning#^cenacle-action|Why cancelling?]] 
