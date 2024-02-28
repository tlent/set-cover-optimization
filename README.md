Minimum Set Cover
==========

Requirements
------------
* gcc
* make
* A POSIX compliant operating system (because of the function used for
timing the program)

GCC and make are not necessary if building without using the makefile.

Building
---------
A makefile is included. The program can be built using the command `make`. The
compiled program is called msc.

Running
--------
After building the program, it can be run with the command
`./msc test_filename` 

Project Structure
------------------
* The C-output/ directory includes the output of the program on each testcase
that it was able to complete. The included outputs are the same outputs I
submitted in my hard copy submission.
* A much more concise prototype of the final program was written in Python. The
Python prototype is included in the prototype/ directory.
* The src/ directory contains the source code for the final C program.
* The supplied testcases are included in the testcases/ directory. The largest
testcases are in the testcases/long/ directory. Any custom testcases generated 
by the python script are in the testcases/custom/ directory.
* A Python script used for generating custom testcases is included as
custom-testcase-generator.py.
* A makefile is included for building the program. Compiler and linker flags
are set in this file.

Testcase Run Times
--------------------
Testcase s-k-150-250 is the only case this program does not complete in a
reasonable amount of time. s-k-150-250, s-k-100-175 and s-k-200-300 are the only
testcases that do not complete in under 60 seconds. Outputs can be seen in the
C-output/ directory. These outputs are identical to what was included on my
hardcopy submission.

Algorithm
-----------
This program implements a backtracking algorithm for finding a minimum set
cover of given sets. The program recursively chooses different combinations of
sets until the minimum set cover has been found. The following checks are made
in this order to reduce the number of set combinations that need to be
calculated:

* If a set is a subset of another set, remove the subset from consideration.
There is no reason to consider a set that gives the same elements as a larger set.
Selecting a subset of another set covers fewer elements for the same cost in the
set cover. When considering whether or not a set is a subset of another set we
only consider the elements in each set that are still uncovered.
* If an uncovered element is only found in a single set, we select that set as
it is necessary in any valid set cover.
* If neither of the above cases is true, we must consider selecting or not
selecting a set. Because every set is either in the minimum cover or not this
considers every possibility for any sets that we cannot eliminate with one of
the two cases above.
    * We always consider the set with the most uncovered elements remaining
    because it is most likely to be in the minimum set cover and to cause the
    above cases to be true for future sets.
    * For the minimum set cover to be exact, we must also
    consider the set cover if we do not choose this set. We will choose
    whichever set cover is smaller out of the set cover containing this set and
    the set cover not containing this set.
