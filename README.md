# Switch Game Merger Utility

The purpose of this project is to merge NSP & XCI files into a single file.

Occassionally if you have a NSP or XCI file that is split into multiple parts, you may want to merge or combine them.

This is a graphical program (written in Rust) designed to handle that process for you.

![image](images/image.png)

---

## Compatability

Current supported operating system(s):

* MacOS (Silicon)
* Linux

**NOTE: There will be a Windows build pushed this week. Working out the kinks**

---

## Installation:

Visit the releases page, and get the current version for your system.

---

**ADVANCED**

You will need rust installed.

* Clone the Repo
* Navigate inside the repo
* Run `cargo run` to have the program launch.

---

## Usage

To use the program, simply launch the downloaded program.

* Select if your game files you are going to merge are NSP or XCI.
* Click `Select Game Files` and choose all of the NSP/XCI files to be merged.
* Enter a output name - WITHOUT the .nsp or .xci extension (EX: My Merged Files)
* Click `Start`

The program will say "Finished!" when completed, and the file will be in the same directory as the program.

There is no need to add .nsp or .xci when entering the name, as the program will do this for you.
