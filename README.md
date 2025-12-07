Enhance lives across the world
================================================================================
You can create a robot today inexpensively. Here you can find the
best practices and software support to expedite your creation.

Creating your own robot
--------------------------------------------------------------------------------
You are not alone. There are many people willing to help you to create your robot.
The [new robot guidance](docs/new_robot/README.md) provides you with the access
and document templates to rapidly make progress on your robot.

Expediting robots
================================================================================
**Best practices are currently under utilized in industry.** By following 
[best practices](docs/best_practices/README.md), robot creators will learn the state-of-the-art
best practices and be provided tools to easily leverage them.

Outline of the repository
--------------------------------------------------------------------------------
* [docs](docs/) - documentation for robot creators
* [simulator](simulator/) - allows you to test your robot control in simulated
        environments
* [firmware](firmware/) - software ([RUST](https://www.rust-lang.org/)) to support your robot
    * [common](firmware/common/) - reusable data types
    * [drivers](firmware/drivers/) - hardware drivers and interface traits
    * [systems](firmware/systems/) - system traits
        * [flight_controllers](firmware/systems/flight_controllers) - flight controllers
    * [robots](firmware/robots/) - example implementations
        * gazebo simulation support
    * [components](firmware/components) - use inexpensive SoC to create robot system components




