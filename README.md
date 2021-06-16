
<div align="left">
<a href="https://github.com/BlackPhlox/bevy_osc"><img src="https://raw.githubusercontent.com/BlackPhlox/BlackPhlox/master/bevy_osc.svg" alt="bevy_osc"></a>
</div>

[![crates.io](https://img.shields.io/crates/v/bevy_osc)](https://crates.io/crates/bevy_osc)</br>[![docs.rs](https://docs.rs/bevy_osc/badge.svg)](https://docs.rs/bevy_osc)

A plugin that uses [nannou_osc](https://github.com/nannou-org/nannou/tree/master/nannou_osc) that allows you to send (not yet implemented) and receive osc data for all who wants to control bevy from other programs or controllers.

# Setup

To test `simple` and the `custom` example:
1. Install [orca](https://hundredrabbits.itch.io/orca)
2. Copy/paste [this](https://git.sr.ht/~rabbits/orca-examples/tree/master/basics/_osc.orca) snippet into the program 
3. Go to Communication -> Choose OSC Port and enter `34254`
4. Run the example(s) and see the data flowing (as shown below)

https://user-images.githubusercontent.com/25123512/122121093-81b57300-ce2b-11eb-8dcf-1bb3d224e540.mp4

# Support
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)

|bevy|bevy_osc|
|---|---|
|0.5|0.1.X|

# Licensing
The project is under dual license MIT and ISC (functionally equivalent, though ISC removing some language that is no longer necessary), so yoink to your hearts content, just remember the license agreements.

# Contributing
Yes this project is still very much WIP, so PRs are very welcome
