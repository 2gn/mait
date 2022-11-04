# mait
zeit alternative written in rust

## current state
It displays window for customizing cron. Pressing "submit" button will write a file named after your username to the root of this project directory.
Technically, it should work on all unix-like systems since it doesn't use any system-native calls but this project's flake.nix is for macos and linux users may install some unnecessary file by this (maybe raise some errors idk cause I didn't test it).

