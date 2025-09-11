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

### Background (Optional)
* [Quadcopter Dynamics, Simulation, and Control](https://andrew.gibiansky.com/downloads/pdf/Quadcopter%20Dynamics,%20Simulation,%20and%20Control.pdf)
    - good introduction to quadcopters physics
* [Aircraft Control and Simulation](https://agorism.dev/book/eng/aircraft-control_johnson-lewis-stevens.pdf)
generalizes the control algorithm to a nonlinear curve fitting[^1] and suggests the use of a
neural network[^2] as a robust solution (w/ mathematical analysis and simulation).
    * requires computational power orders of magnitudes beyond the currently used hardware

### Implementation Guidance
What we need is a mathematically sound architecture that can be evaluated on current hardware.
[PX4 Controller Architecture](https://docs.px4.io/main/en/flight_stack/controller_diagrams)
is in use on current hardware and is based upon sound control-theory.

Background
--------------------------------------------------------------------------------
* [PX4 Architecture - Flight Stack](https://docs.px4.io/main/en/concept/architecture.html#flight-stack)

<!-- foot notes -->
[^1]: [CHAPTER 9 ADAPTIVE CONTROL WITH APPLICATION TO MINIATURE AERIAL VEHICLES](https://agorism.dev/book/eng/aircraft-control_johnson-lewis-stevens.pdf#page=678)

[^2]: [9.3 NEURAL NETWORK ADAPTIVE CONTROL]()https://agorism.dev/book/eng/aircraft-control_johnson-lewis-stevens.pdf#page=682

