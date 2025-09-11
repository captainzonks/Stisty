# STISTY

## Test Commands for Stisty CLI

Single Sample t Test

```ps
.\stisty.exe -Cc "\path\to\example.csv" -n "X: Single Sample t Test"\
 -d "A Single Sample t Test for X. Population average = 8."\
 -S -c X_column -m population_mean
```

Paired Sample t Test

```ps
.\Stisty.exe -Cc "\path\to\example.csv" -n "X/Y: Paired Samples t Test"\
 -d "A Paired Samples t Test for X and Y."\
 -P -c X_column Y_column
```

Independent Groups t Test

```ps
.\Stisty.exe -Cc "\path\to\example.csv" -n "X/Y: Independent Groups t Test"\
 -d "An Independent Groups t Test for X and Y."\
 -I -c continuous_column -n nominal_column
```

One Way ANOVA Test

```ps
.\Stisty.exe -Cc "\path\to\example.csv" -n "X/Y: One Way ANOVA Test"\
 -d "One Way ANOVA Test comparing X and Y."\
 -A -c continuous_column -n nominal_column
```
