#!/bin/sh

echo "Running simulation for autonomous drone"

# kill all processes at end of this run
trap "trap - TERM && kill -- -$$" INT TERM EXIT

robot="drone"
# start the drone SITL
../../../firmware/robots/gazebo-quadcopter/target/debug/autonomous $robot &

# run gazebo
gazebo.gz sim drone-openworld.sdf
