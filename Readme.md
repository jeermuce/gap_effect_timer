# Spacing Effect Timer

The Spacing Effect Timer is a Rust-based command-line application designed to facilitate activities requiring random interval timing. It's ideal for repetitive tasks that benefit from randomized time intervals. This project was also a fun way for me to learn Rust.

## Usage

1. Set the minimum and maximum length of the timer as numbers, separated by a space.
2. Hit enter.
3. Do your thing until the audio notification plays.
4. When it does, close your eyes and wait for the second audio notification, but don't think about it, just relax, and let what just happened sink in.
5. When it plays, open your eyes and repeat the process.
6. When you're done, hit ctrl-c to exit the application.

## Dependencies

-   clearscreen: For clearing the terminal screen between countdowns.
-   crossterm: To handle cross-platform terminal input and output.
-   rand: For generating random countdown intervals.
-   rodio: To play audio notifications.
-   winresource: To embed metadata in the Windows executable.

## Note on Audio Files

The audio notifications used in this application (bell.mp3 and bell_double.mp3) were sourced from external resources. If you recognize these sound files and can identify their creators or rightful owners, please reach out so that proper credit can be given, or so the files can be replaced if necessary to respect copyright and licensing agreements.

## Note on the Name

I named this project after the "Spacing Effect", a cognitive phenomenon where learning is more effective when studying is spread out over time, rather than being crammed into a single session. This application leverages this effect by providing random intervals for study sessions, potentially enhancing the retention and understanding of the material.
