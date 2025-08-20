Flight Controllers
================================================================================
<!-- what is this library -->

Concept of Operations (CONOPS)
--------------------------------------------------------------------------------
User Operations
* Waypoint Control - set a new waypoint and deadline (time of arrival)
* Manual Control - manually position of the vehicle

Flight controllers provide intent-based navigation of a vehicle. As an
intent-based system, the dynamics of environment and vehicle hardware
performance are internally managed by the flight controller.

Design
--------------------------------------------------------------------------------
### Primitives
* **robust to motor performance** - the flight controller attempts to achieve
    the intent regardless of degraded/failed motors/propellers.

* **self-characterized** - the flight controller continuously characterizes the
   motors/propellers impact on the vehicle

### Safety Primitives
* When **unable** to continue toward intent, the flight controller manages a soft descent


Background
--------------------------------------------------------------------------------
* [PX4 Architecture - Flight Stack](https://docs.px4.io/main/en/concept/architecture.html#flight-stack)


