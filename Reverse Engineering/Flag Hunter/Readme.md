## Flag Hunters 

Description

Lyrics jump from verses to the refrain kind of like a subroutine call. There's a hidden refrain this program doesn't print by default. Can you get it to print it? There might be something in it for you.
The program's source code can be downloaded here.

Connect to the program with netcat:

$ nc verbal-sleep.picoctf.net 59025


### Hint
- This program can easily get into undefined states. Don't be shy about Ctrl-C.
- Unsanitized user input is always good, right?
- Is there any syntax that is ripe for subversion?
