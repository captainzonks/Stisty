# STISTY

## Test Commands for Stisty CLI
```ps
.\Stisty.exe -Cc ..\..\csv-files\student-boredom.csv -n "Sample's Average of 'Minutes Wearing Backpack vs 
Population's Average" -d "testing a single sample t test from the command line; column 1 ('Minutes Wearing Backpack')
; mu is 213 minutes" -S -c 1 -m 213
```

```ps
.\Stisty.exe -Cc ..\..\csv-files\final_exam_data.csv -n "Dirty Dishes in the Morning vs at Night" -d "testing a 
paired samples t test from the command line; column 4 ('Dirty Dishes in the Morning') vs column 5 ('Dirty Dishes at 
Night')" -P -c 4 5
```

```ps
.\Stisty.exe -Cc ..\..\csv-files\final_exam_data.csv -n "Indie Groups Test" -d "Independent Groups t Test on masks and ramen" -I -n 3 -c 7
```
