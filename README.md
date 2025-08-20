Enhance lives across the world
================================================================================
You can create a robot today inexpensively. This repository is here to provide
the knowledge and best practices to expedite your creation.

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
* [docs](docs/) - documentation for users
* [simulator](simulator/) - allows you to test your robot control in simulated
        environments
* [firmware](firmware/) - software ([RUST](https://www.rust-lang.org/)) to support your robot
    * [drivers](firmware/drivers/) - hardware drivers and interface traits
    * [robots](firmware/robots/) - interface traits for common robot systems
        * example implementations (which support simulation)
    * [flight_controllers](firmware/flight_controllers/) - control systems for flying robots







