# bevy_osc

</br>
<div align="left">
<a href="https://crates.io/crates/bevy_osc"><img src="https://img.shields.io/crates/v/bevy_osc" alt="link to crates.io"></a>
<a href="https://docs.rs/bevy_osc"><img src="https://docs.rs/bevy_osc/badge.svg" alt="link to docs.rs"></a>
<a href="https://github.com/BlackPhlox/bevy_osc/blob/master/credits/CREDITS.md"><img src="https://img.shields.io/crates/l/bevy_osc" alt="link to license"></a>
<a href="https://crates.io/crates/bevy_osc"><img src="https://img.shields.io/crates/d/bevy_osc" alt="downloads/link to crates.io"></a>   
<a href="https://github.com/BlackPhlox/bevy_osc"><img src="https://img.shields.io/github/stars/BlackPhlox/bevy_osc" alt="stars/github repo"></a>
<a href="https://github.com/BlackPhlox/bevy_osc/actions/workflows/master.yml"><img src="https://github.com/BlackPhlox/bevy_osc/actions/workflows/master.yml/badge.svg" alt="github actions"></a>
<a href="https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking"><img src="https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue" alt="tracking bevy release branch"></a>
</div>
</br>

A plugin that uses [nannou_osc](https://github.com/nannou-org/nannou/tree/master/nannou_osc) that allows you to send and receive osc data for all who wants to control bevy from other programs or controllers.

# Setup

To test `simple`, `send` and the `custom` example:
1. Install [orca](https://hundredrabbits.itch.io/orca)
2. Copy/paste [this](https://git.sr.ht/~rabbits/orca-examples/tree/master/basics/_osc.orca) snippet into the program 
3. Go to Communication -> Choose OSC Port and enter `34254`
4. Run the [example(s)](/examples/) and see the data flowing (as shown below)

https://user-images.githubusercontent.com/25123512/122121093-81b57300-ce2b-11eb-8dcf-1bb3d224e540.mp4

# Support
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)

|bevy|bevy_osc|
|---|---|
|0.5|0.1.X|
|0.6|0.2.X|
|0.7|0.3.X|
|0.8|0.4.X|
|0.9|0.5.X|

# Licensing
The project is under dual license MIT and Apache 2.0, so joink to your hearts content, just remember the license agreements.

# Contributing
Yes this project is still very much WIP, so PRs are very welcome
