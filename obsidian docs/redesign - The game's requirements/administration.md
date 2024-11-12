
Any __user input__ will go through the __white list__ to the __sequencer__. Ones the __sequencer__ keeps a desired sequence, said sequence will be passed to the targets throe __target map__. In Combat Mod, __display__ will always be added to targets at the end. Potential targets include __game mods__.
# types
Input
sequence
transition
target
mod 
mod variant
# functionality
__user input__: read: () -> input
__white list__: check(input) -> maybe< input > 
__sequencer__: keep(input) -> maybe< (sequence, &targets) > 
__game mods__: transition(mod variant) -> mod
