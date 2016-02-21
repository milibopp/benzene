[![](https://img.shields.io/crates/v/benzene.svg)](https://crates.io/crates/benzene)

These are abstractions for a reactive application architecture using
[Carboxyl](https://github.com/aepsil0n/carboxyl). See my scribbled notes further
down. Will add better documentation (read: full introduction) soon.


## Architecture

- Human-Computer Interaction as a metaphor
- Hexagonal architecture


FRP operates at the core/driver boundary

Surrounded by drivers for

- window
- user inputs (keyboard, mouse…)
- graphics
- network
- sound
- etc.


Modelling inputs

- streams only when you want to take an action based on an event
- signals when the input represents the state of another system


## Internal factoring

Application logic is a simple reactive function from inputs to outputs

Application has the same shape as its parts

Component trait


## Unsolved problems

- How to deal with integrations/ticks?
  - first of all, avoid them as they are approximations which compose badly


## License

Copyright 2016 Eduard Bopp.

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Lesser General Public License or GNU General Public
License as published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but **without
any warranty**; without even the implied warranty of **merchantability** or
**fitness for a particular purpose**.  See the GNU General Public License for
more details.

You should have received a copy of the GNU General Public License and the GNU
Lesser General Public License along with this program. If not, see
http://www.gnu.org/licenses/.
