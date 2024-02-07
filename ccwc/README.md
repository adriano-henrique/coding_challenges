# Coding Challenge #1

[Coding Challenge 1 Link](https://codingchallenges.fyi/challenges/challenge-wc/)

## Building your own wc Tool
This Coding Challenge has the main objective of implementing a clone of the wc CLI tool common to Unix systems in Rust. The main purpose of the wc tool is to count word, line and character.

The usage of this tool is the following:

```ccwc <pattern> <path>```
- **pattern**: Argument that defines what you want to count on the file (word, line, character)
    - **-c**: Count characters
    - **-w**: Count words
    - **-l**: Count lines
- **path**: Argument that essentially passses the file that is going to be read

It is worth mentioning that if path is not provided the argument will expect to read from stdin. Meaning that a command like:

```cat <file_path> | ccwc -c``` will count the amount of letters in <file_path>.

If the pattern is not provided, meaning that you pass directly the file the tool will calculate every single count (character, word and line).
