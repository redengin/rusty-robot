Mesh Radio
================================================================================
Inspired by https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-guides/esp-wifi-mesh.html.

The ESP32 can act simultaneously an Access Point (AP) and a Station (STA)
* AP accepts connections (from STA)
* STA creates connections (to AP)
    * one STA connection at a time

Benchmarks
--------------------------------------------------------------------------------
[mesh-benchmark](examples/mesh-benchmark.rs)

### Notes
* Delays about 8 seconds from initialization, for the hardware to detect peers.
* Can take upto 1 second to find a peer. (averaging around 300ms)
* The duration to connect to a peer is insignificant (less than 1ms).

Design
================================================================================
To maximize the range, [ESP32 Long Range](https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-guides/wifi.html#long-range-lr) is used.


Leverages [ESP32 Wifi Security](https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-guides/wifi-security.html) to control access to the mesh.

Implementation Overview
--------------------------------------------------------------------------------
* Announces itself as a mesh node
    * coordinates with peer via (Mesh Protocol)[#Mesh Protocol]
* Scans for other mesh nodes
    * For each peer, exchange data
