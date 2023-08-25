# Uncertainty Solver
Reads, solves, and inputs (using your mouse) TecTech uncertainty resolver puzzles. A simple bruteforce algorithm is
employed, either searching for a very good solution or a solution which requires minimal permutations. 

Finding a very good solution is beneficial, because this will maximize the amount of time the puzzle stays solved. 

## Program usage
Download the program from the [releases](https://github.com/tth05/uncertainty-solver/releases) section run it using 
`.\uncertainty-solver.exe [-r]`. The input values will be remembered if you run the program again. The optional `-r` 
will reset the saved values.

Some parameters require screen coordinates. You can use something like ShareX or a similar program to get these values.
Where you should get these values from is shown in the image below.
![](media/screen_values.png)
_(Example values for a 4k monitor with GUI scale "auto": x=1340, y=510, offset=145)_

While in-game, use the displayed keybinding to start the solver. Holding shift will turn all lamps red (unsolve).

## How the Uncertainty Resolver works
For each mode there are predefined groups of squares. The goal is to minimize the color difference for all squares in
a group. Note that the normal resolver uses blinking speed instead of a color to display the value of a cell.
Initially, all squares are assigned a random value from the range [0, 999].
![](media/mode_patterns.png)
_(Groups are marked with the same number)_
