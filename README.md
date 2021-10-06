# Erratic

**!! Work in progress !!**

Erratic is an [ECS](https://en.wikipedia.org/wiki/Entity_component_system) crate designed to be fully runtime editable (no dependency on compile time types). Erratic api is designed to be usable and completely inspectable in scripting environments.

### Goals:
* Runtime creation of systems
* Changing systems execution order at runtime
* ECS metadata inspection

### Non goals:
* Maximum perfomance. Runtime may add noticeable overhead, but it is expected to have decent perfomance.
