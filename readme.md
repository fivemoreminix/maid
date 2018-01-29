# What is Maid?
**Maid is a project manager for C and C++ that enables anyone to create a project and get started working, without needing to worry about IDEs, or their compiler and its hundreds of options.**

Maid creates and manages projects for you. You'll never need to touch CMake, an IDE, or even custom shell build scripts; ew. I believe you should focus most on actually writing the code than being distracted with other things.

Maid has commands like `build`, which gathers up all your source files, determines the language (C or C++) based on the `main.(c/cpp)` file extension, compiles all the files, and links them. In the project file, you can specify general options like the name of the project, its version, its language of choice, options for each compiler separately, linker options, and much more!

All options are high-level, meaning they are translated into the literal options for the tools Maid decides to use.

Oh, and everyone gets a user config file just like how every project gets one, too. Project preferences, AKA `Maid.toml`, are dominant over user preferences. For example, a user can have a preferred compiler across all projects, but if a project requires a specific compiler, it will use the one it requires, and not the one the user prefers.
# Examples
A basic example of Maid in action.
![Basics](/etc/images/basics.png "Basics")

To be able to automatically display the name and version of your program should be possible without hardcoding it into your program. We accomplish this with preprocessor defines.

*The only defines are `MAID_PACKAGE_NAME` and `MAID_PACKAGE_VERSION`, as of now*
![Preprocessor](/etc/images/preprocessor.png "Preprocessor Example")

You can also shorten your project files by deleting anything that isn't a part of the `package` section.
![Short Project File](/etc/images/short_project_file.png "Short Project File")

You can add a `build.py` in your project, and it will be executed before compilation.
![Python Build Scripts](/etc/images/python_build_scripts.png "Python Build Scripts")

![Folder Structure](/etc/images/folder_structure.png "Folder Structure")
Maid projects have a very straight forward structure, following Cargo and a good structure for C and C++ projects, containing a folder for includes, and a folder for source files.

# FAQ
## Why so many comments in the code?
I don't usually comment my code like that, but *I'd rather someone know way more than needed, than not having a clue.*
## Does using Maid decrease compilation flexability?
Currently, it does. But in the future, when I've had a chance to work on compilation more, you'll never need to build manually. **I aspire to make operating systems, and if I thought that Maid wouldn't be able to do this for me, I'd never have started working on it.**
## Why make something like this for C/C++ if we have Rust?
Rust is a great language, but there are a few times you may find yourself wanting to make a project in C++. Here are just a few reasons you may find Maid useful:
*(Instead of saying "C/C++" a hundred times, I'll just say "C", but I also mean "C++" when I do)*
* C is very stable and does not change often. Rust is still developing, and it's always changing.
* C has tons of libraries and support.
* While we must love Rust and not like C for being unsafe or inconvenient, we must still love them for being where we've evolved.
* C has tons of use and is still the prodominant language in the systems programming industry.
* Most IDEs or build tools for C can be complicated and difficult to prototype projects with because you're writing more build scripts than you are code.
