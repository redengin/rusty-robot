Flight Controller Design
================================================================================

Concept of Operations (CONOPS)
--------------------------------------------------------------------------------
### User Stories
* User wants the drone to manuever between geographic points
    1. User supplies a waypoint (and optionally an arrival deadline)
    2. Flight controller manages the manuevers to move the drone toward the
        desired waypoint
* User wants to manuever the drone manually
    1. User specifies the trajectory (3D direction and rate) - aka headless mode
    2. Flight controller manages the manuevers to move the drone along that
        trajectory
* User wants the drone to drone to stay in their vicinity - aka drone bodyguard
    1. User provides their location (for example bluetooth leash)
    2. Flight controller manages the manuevers to maintain the drone at
        a safe distance within the proximity of the user
        * possible performing more complex maneuvers such as circling the
            user

Design Guidance
================================================================================
### Minimize barriers to usage
Users want to make immediate use of the system. Designs **shouldn't** require
the user to expend effort into understanding and tuning the system.

### Primitives
* **self-characterized** - flight controller continuously characterizes the
   actuation (motors/propellers) impact on the vehicle
    * system can determine when it can no longer meet intents
    <!-- * robust implementations can rotate the body frame toward meeting intents -->

### Safety Primitives
* When **unable** to continue toward intent, the flight controller manages a soft descent



Design
================================================================================
[Multicopter Design](../src/multicopters/docs/design.md)


