Wireless Mesh
================================================================================
R/C radios follow the tradition of RX on the robot side and TX on the user side.
As the protocols have evolved, RX/TX are often bidirectional
(i.e. the RX sends telemetry).

With the increased availability of RF hardware, it is now common to use a
general purpose RF device for R/C radios.

Where traditionally R/C is point-to-point, **mesh** extends the range
(allowing communications to hop between robots).

WiFi and LoRa
--------------------------------------------------------------------------------
WiFi and LoRa use unlicensed spectrum.

### Range
* **LoRa** - ***15 km***, ***2-5 km*** (urban environment)
* **WifI** - ***150 m***
    * [esp32 wifi LR](https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-guides/wifi.html#long-range-lr) - ***0.5 km***

Additionally, these RF protocols provide enhanced security.

Joining the Mesh
--------------------------------------------------------------------------------
Each node must provide authorization credentials to connect to the mesh.

### Mesh Authorization
* **name** - used to identify the mesh (ssid)
* **password** - credential necessary to connect to the mesh

Communication on the mesh requires authentication.
### Mesh Authentication
* **mission key** - authenticates communication on the mesh

The **mission key** is provisioned by a **root node** upon the start of the
mission via a direct channel between the robot and the **root node**.

NOTE: only one **root node** is required, but multiple **root node**s are
supported (as long as they know the mission key).

Coordination of the Mesh
--------------------------------------------------------------------------------
Every non-root node stores-and-forwards packets (including it's own packets).

Packets on the mesh are wrapped by a channel header - where the channel
identifies the type of data for the packet.

As packet timeliness within the mesh can not be guaranteed, node control systems
must be designed toward self-autonomous control. To minimize the storage
requirements of the mesh, only the most recent packet will be transmitted.

Timeliness of a packet is determined by an integer counter
(incremented upon each send) eliminating the need for synchronizing time
across the system. (see [Vector Clocks](https://en.wikipedia.org/wiki/Vector_clock))

Each node constantly scans to identify peers in the mesh.

1. if there is a direct connection to a **root node**
    * connect to **root node**
    * exchange packets
        * stays connected per the **root node** linger returned parameter
    * disconnect to allow other nodes to interact with the root
2. if there is not a direct connection to a **root node**
    * connect to closest peers
    * exchange packets
    * disconnect to allow other nodes to interact with peer


