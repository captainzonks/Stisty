# Stisty

**A powerful statistical analysis and genome analyzer toolkit written in Rust**

Stisty provides both a fast CLI for statistical analysis and a privacy-first browser-based genome analyzer. Built for researchers, students, and data analysts who need reliable statistical insights and genome analysis.

## 🌐 Web Application

**Try the browser-based genome analyzer at [your-deployment-url](#)**

- 🔒 **100% Client-Side** - Your genome data never leaves your browser
- 🚀 **WebAssembly Performance** - Fast Rust code compiled to WASM
- 📊 **Interactive Analysis** - Real-time statistics and visualizations
- 🧬 **23andMe Compatible** - Supports 23andMe raw data format

[See deployment guide →](stisty-wasm/README.md)

---

## Features

### Statistical Tests
- **Single Sample t-Test** - Compare sample mean to a known population mean
- **Paired Samples t-Test** - Compare two related samples
- **Independent Groups t-Test** - Compare two independent groups
- **One-Way ANOVA** - Compare means across multiple groups
- **Chi-Squared Goodness of Fit** - Test observed vs. expected frequencies
- **Chi-Squared Test of Independence** - Test association between categorical variables

### Genome Analysis
- **23andMe Data Import** - Parse and analyze 23andMe raw data files
- **VCF Export** - Generate Michigan Imputation Server compatible VCF files
- **Reference Panel Filtering** - 552,550 SNPs with anonymous sample genotypes
- **Quality Controlled Output** - 0 allele switches, passes imputation QC
- **Privacy-First Processing** - All analysis happens in your browser
- **SNP Lookup & Statistics** - Heterozygosity, Ts/Tv ratio, chromosome stats

[See detailed genome usage guide →](GENOME_USAGE.md)

### Additional Features
- Interactive menu mode
- CSV data import and validation
- Comprehensive statistical output
- Support for both continuous and categorical data

---

## Installation

### Build from Source

```bash
git clone https://github.com/captainzonks/Stisty.git
cd Stisty
cargo build --release
```

The compiled binary will be located at `target/release/stisty` (or `stisty.exe` on Windows).

---

## Usage

### Data Format

Stisty reads CSV files with headers. Your data should be organized with:
- **Continuous data**: Numeric columns (e.g., measurements, scores, counts)
- **Categorical data**: Text columns (e.g., groups, categories, phenotypes)

Example CSV structure:
```csv
ID,Group,Score,Treatment,Phenotype
1,Control,45.2,A,Yellow_Round
2,Treatment,52.1,B,Green_Round
...
```

---

## Statistical Test Examples

All examples use column indices (0-based) or the test data file at `tests/test_data.csv`.

### Single Sample t-Test

Test if a sample mean differs from a known population mean.

```bash
stisty -Cc "tests/test_data.csv" \
  -n "Shower Frequency Test" \
  -d "Testing if average showers differ from population mean of 8" \
  -S -c 4 -m 8.0
```

**Arguments:**
- `-Cc` : Path to CSV file
- `-n` : Test name
- `-d` : Description
- `-S` : Single sample t-test flag
- `-c` : Column index for continuous data
- `-m` : Population mean (mu)

---

### Paired Samples t-Test

Compare two related measurements (e.g., before/after, matched pairs).

```bash
stisty -Cc "tests/test_data.csv" \
  -n "Stress Change Test" \
  -d "Comparing stress levels from January to April" \
  -P -c 6 7
```

**Arguments:**
- `-P` : Paired samples t-test flag
- `-c` : Two column indices (first and second measurement)

---

### Independent Groups t-Test

Compare means between two independent groups.

```bash
stisty -Cc "tests/test_data.csv" \
  -n "Rent by Residence" \
  -d "Comparing rent between in-state and out-of-state students" \
  -I -c 5 -n 2
```

**Arguments:**
- `-I` : Independent groups t-test flag
- `-c` : Column index for continuous data
- `-n` : Column index for categorical grouping variable (must have exactly 2 levels)

---

### One-Way ANOVA

Compare means across three or more groups.

```bash
stisty -Cc "tests/test_data.csv" \
  -n "Rent by Club" \
  -d "Comparing rent across different club memberships" \
  -A -c 5 -n 1
```

**Arguments:**
- `-A` : ANOVA test flag
- `-c` : Column index for continuous data
- `-n` : Column index for categorical grouping variable (must have 3+ levels)

---

### Chi-Squared Goodness of Fit

Test if observed frequencies match expected frequencies (useful for testing Mendelian ratios, Hardy-Weinberg equilibrium).

```bash
stisty -Cc "tests/test_data.csv" \
  -n "Pea Phenotype Test" \
  -d "Testing 9:3:3:1 Mendelian ratio for dihybrid cross" \
  -G -c 8 -e 22.5 7.5 7.5 2.5
```

**Arguments:**
- `-G` : Goodness of fit test flag
- `-c` : Column index for categorical data
- `-e` : Expected frequencies (space-separated, must match number of categories)

---

### Chi-Squared Test of Independence

Test if two categorical variables are associated (independent).

```bash
stisty -Cc "tests/test_data.csv" \
  -n "Eye Color and Hair Color Association" \
  -d "Testing independence between eye color and hair color" \
  -X -c 10 11
```

**Arguments:**
- `-X` : Test of independence flag
- `-c` : Two column indices for categorical variables

**Example: Disease association**
```bash
stisty -Cc "tests/test_data.csv" \
  -n "Disease Status by Blood Type" \
  -d "Testing if disease status is independent of blood type" \
  -X -c 9 12
```

---

## Interactive Menu Mode

For a guided experience, use the interactive menu:

```bash
stisty -m
```

This will walk you through:
1. Loading a CSV file
2. Selecting a statistical test
3. Choosing appropriate columns
4. Viewing results

---

## Column Index Reference

When using the test data file (`tests/test_data.csv`), here are the column indices:

| Index | Column Name          | Type        | Description                    |
|-------|---------------------|-------------|--------------------------------|
| 0     | Participant         | Continuous  | Participant ID                 |
| 1     | Club                | Categorical | Club membership                |
| 2     | Residence           | Categorical | In state / out of state        |
| 3     | Car                 | Categorical | Has car (yes/no)              |
| 4     | Showers             | Continuous  | Number of showers per week     |
| 5     | Rent                | Continuous  | Monthly rent ($)               |
| 6     | Stress in January   | Continuous  | Stress level (1-10)            |
| 7     | Stress in April     | Continuous  | Stress level (1-10)            |
| 8     | Pea_Phenotype       | Categorical | Pea plant phenotype            |
| 9     | Blood_Type          | Categorical | Blood type (A/B/AB/O)         |
| 10    | Eye_Color           | Categorical | Eye color                      |
| 11    | Hair_Color          | Categorical | Hair color                     |
| 12    | Disease_Status      | Categorical | Affected/Unaffected           |

---

## Examples with Genetics Data

### Test Mendelian 9:3:3:1 Ratio
```bash
stisty -Cc "tests/test_data.csv" \
  -n "Dihybrid Cross Test" \
  -G -c 8 -e 22.5 7.5 7.5 2.5
```

### Test Hardy-Weinberg Equilibrium for Blood Types
```bash
stisty -Cc "tests/test_data.csv" \
  -n "Blood Type Distribution" \
  -G -c 9 -e 10 10 10 10
```

### Test Eye-Hair Color Independence
```bash
stisty -Cc "tests/test_data.csv" \
  -n "Eye-Hair Association" \
  -X -c 10 11
```

---

## Command-Line Arguments

### Global Options
- `-Cc <path>` : Path to CSV file
- `-n <name>` : Test name (optional)
- `-d <desc>` : Test description (optional)
- `-m` : Interactive menu mode

### Test Selection (choose one)
- `-S` : Single sample t-test
- `-P` : Paired samples t-test
- `-I` : Independent groups t-test
- `-A` : One-way ANOVA
- `-G` : Chi-squared goodness of fit
- `-X` : Chi-squared test of independence

### Data Selection
- `-c <index(es)>` : Column index/indices for continuous or categorical data
- `-n <index>` : Column index for nominal (categorical) grouping variable
- `-m <value>` : Population mean (for single sample t-test)
- `-e <values>` : Expected frequencies (space-separated, for goodness of fit)

---

## Output

Stisty provides detailed output including:
- Test statistics (t, F, χ²)
- Degrees of freedom
- Descriptive statistics (mean, variance, SD)
- Group comparisons
- Contingency tables (for chi-squared)

---

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

---

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

---

## Author

Matthew Barham ([@captainzonks](https://github.com/captainzonks))
