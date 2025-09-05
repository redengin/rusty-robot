Flight Controllers
================================================================================
<!-- what is this library -->

Concept of Operations (CONOPS)
--------------------------------------------------------------------------------
### User Stories
* User wants the drone to manuever across a region
    * uses Autonomous Control to provide a sequential set of waypoints and
        deadlines.
* User wants to navigate the drone directly
    * uses Direct Control to manually change the drone position
* User wants the drone to drone to stay in their vicinity
    * leverages Autonomous Control to maintain the drone position by updating
        the waypoints per the leash algorithm

### Intent-Driven Design Pattern
Users want the functionality. This design creates a self-characterizing
system - eliminating the need for any tuning parameters.

Self-characterizing systems autonomously adapt toward user intent
    * adapt to environmental conditions
    * overcome hardware degradation - i.e. if a propellor is damaged,
        the system will compensate

#### Intents
* Autonomous Control
    * User provides waypoint and deadline (desired time of arrival)
    * flight controller manages the manuever to move from its current
        position to the waypoint

* Direct Control
    * User provides trajectory (3D direction, rate)
    * flight controller manages motor actuation to produce the desired
        trajectory

Design
--------------------------------------------------------------------------------


## Primitives
* **self-characterized** - flight controller continuously characterizes the
   actuation (motors/propellers) impact on the vehicle
   * to maximize performance toward intent, the flight controller should
        manage yaw - i.e. leverage the best motors toward the trajectory/rate.
        * NOTE: to minimize the disruption to the characterization, it's
                simpler to translate the input (i.e. rotate the world around
                the drone)

## Safety Primitives
* When **unable** to continue toward intent, the flight controller manages a soft descent

### Design Guidance


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

