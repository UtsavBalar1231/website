# Website Analysis Script

## Overview

The `analyze-website.sh` script provides comprehensive size and performance analysis for the terminal portfolio website. It generates detailed reports on file sizes, compression ratios, performance metrics, and optimization recommendations.

## Usage

```bash
# Make sure the site is built first
npm run build

# Run the analysis
./analyze-website.sh
```

## Features

### **File Size Breakdown**
- Directory structure analysis with size breakdown
- File type analysis (HTML, CSS, JS, other files)
- Raw vs compressed sizes with compression ratios
- File count statistics

### **Critical File Analysis**
- Top 10 largest files with compression details
- File extension breakdown with totals
- Identification of optimization opportunities

### **Page Analysis**
- Individual HTML page sizes (raw and gzipped)
- Page titles and file paths
- Color-coded size warnings (green < 20KB, yellow < 50KB, red > 50KB)

### **Performance Analysis**
- Total size analysis with compression savings
- Performance budget tracking (200KB limit)
- Critical path analysis (HTML + CSS + JS)
- Estimated load times for different connection speeds
- Transfer size calculations

### **Content Analysis**
- Page count breakdown (main pages vs tutorials)
- Content density metrics
- Word count and line analysis

### **Optimization Recommendations**
- File size optimization suggestions
- Performance improvement recommendations
- Caching and compression strategies
- HTTP/2 server push candidates

### **Executive Summary**
- Key metrics overview
- Performance budget status
- Next steps and recommendations

## Configuration

You can modify the script's behavior by editing these variables at the top:

```bash
SITE_DIR="_site"                    # Build output directory
GZIP_THRESHOLD=1024                 # Show gzip for files > 1KB
CRITICAL_SIZE_LIMIT=204800          # 200KB budget limit
WARNING_SIZE_LIMIT=153600           # 150KB warning threshold
```

## Performance Budget

The script tracks performance against a **200KB transfer size budget**:

- **Green (OK)**: < 150KB (75% of budget)
- **Yellow (WARNING)**: 150KB - 200KB (75-100% of budget)  
- **Red (ERROR)**: > 200KB (over budget)

## Requirements

The script requires these tools (usually pre-installed on most systems):

- `bash` - Shell interpreter
- `bc` - Arbitrary precision calculator
- `gzip` - Compression utility
- `find` - File search utility
- `awk` - Text processing
- `sort` - Sorting utility

## Output Sections

### 1. File Size Breakdown
Shows directory structure and file type analysis with compression ratios.

### 2. Critical File Analysis
Identifies the largest files and potential optimization targets.

### 3. Page Analysis
Analyzes individual HTML pages with size and compression metrics.

### 4. Performance Analysis
Provides comprehensive performance metrics including:
- Total raw vs transfer sizes
- Compression savings
- Performance budget usage
- Critical path size
- Load time estimates

### 5. Content Analysis
Analyzes content density and page distribution.

### 6. Optimization Recommendations
Suggests specific improvements based on the analysis.

### 7. Executive Summary
High-level overview with key metrics and next steps.

## Example Output Interpretation

```bash
Critical Path Total: 7.20 KB
```
This is excellent - the critical resources (HTML + CSS + JS) load very quickly.

```bash
Performance Budget: 38.1% used
Status: WITHIN BUDGET
```
The site uses only 38% of the 200KB budget, leaving plenty of room for growth.

```bash
Compression Savings: 185.14 KB (70.7%)
```
Excellent compression ratio - the site compresses from 261KB to 76KB.

## Continuous Monitoring

Run this script regularly to:
- Monitor size growth as content is added
- Identify performance regressions
- Track optimization improvements
- Ensure budget compliance

## Integration with CI/CD

You can integrate this script into your build process:

```bash
# In package.json scripts
"analyze": "./analyze-website.sh",
"build:analyze": "npm run build && npm run analyze"
```

Or use it as a CI check to fail builds that exceed the performance budget.

## Troubleshooting

### Script fails with "Site directory not found"
Run `npm run build` first to generate the `_site` directory.

### Missing tool errors
Install required tools:
```bash
# Ubuntu/Debian
sudo apt-get install bc gzip findutils gawk coreutils

# macOS (via Homebrew)
brew install bc gzip findutils gawk coreutils
```

### Permission denied
Make the script executable:
```bash
chmod +x analyze-website.sh
```

## Advanced Usage

### Custom Performance Budget
Modify the budget limits for your specific needs:

```bash
CRITICAL_SIZE_LIMIT=153600  # 150KB instead of 200KB
WARNING_SIZE_LIMIT=102400   # 100KB warning threshold
```

### Output to File
Save the analysis report to a file:
```bash
./analyze-website.sh > analysis-report.txt
```

### Compare Builds
Track changes over time by saving timestamped reports:
```bash
./analyze-website.sh > "analysis-$(date +%Y%m%d-%H%M%S).txt"
```

## Best Practices

1. **Run after every build** to catch size increases early
2. **Set up alerts** if the budget usage exceeds 75%
3. **Review recommendations** regularly for optimization opportunities
4. **Track metrics over time** to identify trends
5. **Use in code reviews** to validate performance impact

The script helps maintain the terminal portfolio's excellent performance characteristics while providing detailed insights for continuous optimization.