# Uncertainty Solver
## Program usage
Download the program from the releases section run it using `.\uncertainty-solver.exe [-r]`. The input values will be remembered if you run the program again.
The optional `-r` will reset the saved values.

Some parameters require screen coordinates. You can use something like ShareX or a similar program to get these values.
Where you should get these values from is shown in the image below.
![](media/screen_values.png)

While in-game, use the displayed keybinding to start the solver. Holding shift will turn all lamps red (unsolve).

## How the Uncertainty Resolver works
For each mode there are predefined groups of squares. The goal is to minimize the color difference for all squares
in a group.
![](media/mode_patterns.png)
