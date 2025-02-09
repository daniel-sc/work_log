# work_log

Log the active window and cursor position to a CSV file.

This can be used to analyze how you spend your time on your computer.


## Usage

```text
Usage: work_log [OPTION]...
Log the active window and cursor position to a CSV file.

Options:
  --frequency=SECONDS  set the frequency of logging (default: 2.0)
  --file=FILE          set the file to write to (default: ~/work_log.csv)
  --stdout             print to stdout instead of a file
  --autostart-install  install the program to run at startup.
    (options are preserved, do not move the executable afterwards)
  --autostart-remove   remove the program from startup
  --help               display this help and exit
```