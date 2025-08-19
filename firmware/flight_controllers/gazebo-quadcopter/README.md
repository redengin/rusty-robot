Gazebo Sim for a quadcopter
================================================================================

Autonomous Flight Controller
--------------------------------------------------------------------------------
### Building
from **firmware/** (a RUST workspace)
```sh
cargo build --release --package rusty-robot-gazebo-quadcopter --bin autonomous
```

### Running
from **firmware/** (a RUST workspace)
```sh
cargo run --release --package rusty-robot-gazebo-quadcopter --bin autonomous drone
```

"drone" is the name used in the [simulator](../../../simulator/README.md).
