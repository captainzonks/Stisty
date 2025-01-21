# STISTY

## Test Commands for Stisty CLI

Single Sample t Test

```ps
.\stisty.exe -Cc "..\..\csv-files\test_data.csv" -n "Showers: Single Sample t Test" -d "A Single Sample t Test for 
total showers in a week. Population average = 8." -S -c 4 -m 8
```

Paired Sample t Test

```ps
.\Stisty.exe -Cc "..\..\csv-files\test_data.csv" -n "Stress: Paired Samples t Test" -d "A Paired Samples t Test for 
stress in January and then stress in April." -P -c 6 7
```

Independent Groups t Test

```ps
.\Stisty.exe -Cc "..\..\csv-files\test_data.csv" -n "Cars/Rent: Independent Groups t Test" -d "An Independent 
Groups t Test for rent costs between those who own cars and those who do not." -I -n 3 -c 5
```

One Way ANOVA Test

```ps
.\Stisty.exe -Cc "..\..\csv-files\test_data.csv" -n "Club/Rent: One Way ANOVA Test" -d "One Way ANOVA Test comparing 
club membership 
and rent costs." -A -n 1 -c 5
```
