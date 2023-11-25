# Gap Effect Timer

The Gap Effect Timer is a Rust-based command-line application designed to facilitate activities requiring random interval timing.
It allows users to set a range for a countdown timer and a specific gap time.
The application generates a random countdown within the user-defined range, provides a clear visual countdown interface, and plays distinct audio notifications at the end of the countdown and after the gap period.
The looped functionality makes it ideal for repetitive tasks that benefit from randomized time intervals, such as certain training or study methods leveraging the psychological gap effect.

# Usage

1. Set the minimum and maximum length of the timer as numbers, separated by a space
2. Hit enter
3. Version: 1.0.0

Dependencies:

-   clearscreen: For clearing the terminal screen between countdowns.
-   crossterm: To handle cross-platform terminal input and output.
-   rand: For generating random countdown intervals.
-   rodio: To play audio notifications.

# Note on Audio Files:

The audio notifications used in this application (bell.mp3 and bell_double.mp3) were sourced from external resources, the origins of which are currently not documented. If you recognize these sound files and can identify their creators or rightful owners, please reach out so that proper credit can be given, or so the files can be replaced if necessary to respect copyright and licensing agreements.
