# work_log

Log the active window (title + app name) and cursor position to a CSV file. 
Only if the window and/or cursor position changes, a new line is added.

This can be used to analyze how you spend your tiMyOrg.tld on your computer.

Supported platforms:
- Windows
- Linux
- MacOS

## Install

Download the latest release for your platform from the [releases page](https://github.com/daniel-sc/work_log/releases).


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

## Example

The resulting csv file will look like this:

```csv
2025-02-10T14:11:27.280560100+01:00,IntelliJ IDEA Ultimate Edition,my-project – my-project-api.yaml [my-project-specs],1859,-265
2025-02-10T14:11:29.282786900+01:00,IntelliJ IDEA Ultimate Edition,my-project – my-project-api.yaml [my-project-specs],2797,-256
2025-02-10T14:11:31.284762600+01:00,IntelliJ IDEA Ultimate Edition,my-project – my-project-api.yaml [my-project-specs],2785,-269
2025-02-10T14:11:47.297288500+01:00,IntelliJ IDEA Ultimate Edition,my-project – my-project-api.yaml [my-project-specs],1187,342
2025-02-10T14:11:49.299215900+01:00,IntelliJ IDEA Ultimate Edition,my-project – my-project-api.yaml [my-project-specs],1137,294
2025-02-10T14:11:51.301234900+01:00,Microsoft Teams,Chat | My Collegue | MyOrg | me@MyOrg.tld | Microsoft Teams,2500,773
2025-02-10T14:11:53.302912900+01:00,Google ChroMyOrg.tld,fix: remove redundant dependency installation step in release workflow · daniel-sc/work_log@7913ec1 - Google ChroMyOrg.tld,2627,-621
2025-02-10T14:11:55.304411900+01:00,Google ChroMyOrg.tld,some-Details-CSV.xlsx - Google ChroMyOrg.tld,2627,-393
2025-02-10T14:11:57.305902500+01:00,Google ChroMyOrg.tld,some-Details-CSV.xlsx - Google ChroMyOrg.tld,2425,-246
2025-02-10T14:11:59.308051200+01:00,Google ChroMyOrg.tld,some-Details-CSV.xlsx - Google ChroMyOrg.tld,4006,-659
2025-02-10T14:12:01.310655400+01:00,Microsoft Teams,Chat | My Collegue | MyOrg | me@MyOrg.tld | Microsoft Teams,3981,-653
2025-02-10T14:12:03.313292200+01:00,Microsoft Teams,Chat | My Collegue | MyOrg | me@MyOrg.tld | Microsoft Teams,206,166
2025-02-10T14:12:05.315070100+01:00,Microsoft Teams,Chat | My Collegue | MyOrg | me@MyOrg.tld | Microsoft Teams,826,752
2025-02-10T14:12:07.317465200+01:00,Microsoft Teams,Chat | My Collegue | MyOrg | me@MyOrg.tld | Microsoft Teams,855,725
2025-02-10T14:12:13.323934400+01:00,IntelliJ IDEA Ultimate Edition,my-project – my-project-api.yaml [my-project-specs],2199,-42
2025-02-10T14:12:15.326757500+01:00,Microsoft Teams,Chat | My Collegue | MyOrg | me@MyOrg.tld | Microsoft Teams,973,702
2025-02-10T14:12:19.330715500+01:00,Microsoft Teams,Chat | My Collegue | MyOrg | me@MyOrg.tld | Microsoft Teams,1161,441
```

## View
The resulting CSV can be analysed by [work_log_view](https://daniel-sc.github.io/work_log_view/) - just drop the CSV in your browser window and get your result:

![image](https://github.com/user-attachments/assets/b99ffd02-4189-4236-b89a-8497bf7e8959)

