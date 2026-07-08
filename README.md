
# Ghost Boss
>A silly game about resource management

This is a silly game, gotten to MVP in a month, now in the process of
being polished and enhanced.

The todo list includes
- Background images
- Foreground images
- Humorous boss quotes
- Small music loop
- Tests
- UI that doesn't scream
- Making a death knell (Make a minor overtone)


## To play Ghost Boss

Currently you have to build from source yourself. Git, Godot 4.7 and Rust 1.29.0
are required. Rust is used as an extension for GDScript in this project, providing type safety
and the swiftness of Rust.

Fedora

`sudo dnf install git`

Debian/Ubuntu

`sudo apt install git`

Download the latest [Godot](https://godotengine.org/) (Currently 4.7)

Download the latest rust build via [rustup](https://rustup.rs/)

Follow the instructions on setting up your path variable once rustup is installed, or the `cargo` line
will fail


### Build steps
clone the repository

`git clone https://github.com/heckmarr/cabbage-truckin.git`

move into the rust source folder

`cd cabbage-truckin/rust`

build the GDExtension

`cargo build`

Then run the Godot binary and open the folder called `godot` in the
root of the repository.

Once everything is all loaded, you should be able to press the "*Play*"
button in the Godot editor, and you'll be off to the races!

### Current state
You can use up your scary boss points to fry the workers, and there is a package
that is randomly assembled and shown, and workers die when pressed too hard. Press left and right
to move the selector, and spacebar to go Bad Boss mode on the worker. When you are out of
points, escape quits.

Thanks for looking!
-bb
