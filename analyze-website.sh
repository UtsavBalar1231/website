#!/bin/bash

# Website Analysis Script
# Comprehensive size and performance analysis for the terminal portfolio

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
WHITE='\033[1;37m'
NC='\033[0m' # No Color

# Configuration
SITE_DIR="_site"
GZIP_THRESHOLD=1024  # Show gzip savings for files > 1KB
CRITICAL_SIZE_LIMIT=204800  # 200KB in bytes
WARNING_SIZE_LIMIT=153600   # 150KB in bytes

# Helper functions
print_header() {
    echo -e "\n${WHITE}════════════════════════════════════════════════════════════════════════════════${NC}"
    echo -e "${WHITE}                        WEBSITE ANALYSIS REPORT                              ${NC}"
    echo -e "${WHITE}════════════════════════════════════════════════════════════════════════════════${NC}"
    echo -e "${CYAN}Site Directory: ${WHITE}$SITE_DIR${NC}"
    echo -e "${CYAN}Analysis Date: ${WHITE}$(date)${NC}"
    echo -e "${WHITE}════════════════════════════════════════════════════════════════════════════════${NC}"
}

bytes_to_human() {
    local bytes=$1
    if [ $bytes -gt 1048576 ]; then
        printf "%.2f MB" $(echo "scale=2; $bytes/1048576" | bc)
    elif [ $bytes -gt 1024 ]; then
        printf "%.2f KB" $(echo "scale=2; $bytes/1024" | bc)
    else
        printf "%d B" $bytes
    fi
}

get_compression_ratio() {
    local original=$1
    local compressed=$2
    if [ $original -gt 0 ]; then
        echo "scale=1; (($original - $compressed) * 100) / $original" | bc
    else
        echo "0"
    fi
}

analyze_file_sizes() {
    echo -e "\n${WHITE}FILE SIZE BREAKDOWN${NC}"
    echo -e "${WHITE}════════════════════════════════════════════════════════════════════════════════${NC}"
    
    # Directory structure analysis
    echo -e "\n${YELLOW}Directory Structure:${NC}"
    du -sh $SITE_DIR/* 2>/dev/null | sort -hr | while read size dir; do
        case "$dir" in
            *css*) echo -e "  ${CYAN}$size${NC} - $dir (Stylesheets)" ;;
            *js*) echo -e "  ${GREEN}$size${NC} - $dir (JavaScript)" ;;
            *tutorials*) echo -e "  ${PURPLE}$size${NC} - $dir (Tutorial Pages)" ;;
            *icons*|*img*|*images*) echo -e "  ${BLUE}$size${NC} - $dir (Images/Icons)" ;;
            *) echo -e "  ${WHITE}$size${NC} - $dir" ;;
        esac
    done
    
    # File type analysis
    echo -e "\n${YELLOW}File Type Analysis:${NC}"
    
    # HTML files
    html_count=$(find $SITE_DIR -name "*.html" | wc -l)
    html_size=$(find $SITE_DIR -name "*.html" -exec ls -la {} \; 2>/dev/null | awk '{sum+=$5} END {print sum+0}')
    html_gzip=$(find $SITE_DIR -name "*.html" -exec gzip -c {} \; 2>/dev/null | wc -c)
    html_ratio=$(get_compression_ratio $html_size $html_gzip)
    
    echo -e "  ${GREEN}HTML Files:${NC}"
    echo -e "    Count: $html_count files"
    echo -e "    Raw Size: $(bytes_to_human $html_size)"
    echo -e "    Gzipped: $(bytes_to_human $html_gzip) (${html_ratio}% compression)"
    
    # CSS files
    css_count=$(find $SITE_DIR -name "*.css" | wc -l)
    css_size=$(find $SITE_DIR -name "*.css" -exec ls -la {} \; 2>/dev/null | awk '{sum+=$5} END {print sum+0}')
    css_gzip=$(find $SITE_DIR -name "*.css" -exec gzip -c {} \; 2>/dev/null | wc -c)
    css_ratio=$(get_compression_ratio $css_size $css_gzip)
    
    echo -e "  ${CYAN}CSS Files:${NC}"
    echo -e "    Count: $css_count files"
    echo -e "    Raw Size: $(bytes_to_human $css_size)"
    echo -e "    Gzipped: $(bytes_to_human $css_gzip) (${css_ratio}% compression)"
    
    # JavaScript files
    js_count=$(find $SITE_DIR -name "*.js" | wc -l)
    js_size=$(find $SITE_DIR -name "*.js" -exec ls -la {} \; 2>/dev/null | awk '{sum+=$5} END {print sum+0}')
    js_gzip=$(find $SITE_DIR -name "*.js" -exec gzip -c {} \; 2>/dev/null | wc -c)
    js_ratio=$(get_compression_ratio $js_size $js_gzip)
    
    echo -e "  ${PURPLE}JavaScript Files:${NC}"
    echo -e "    Count: $js_count files"
    echo -e "    Raw Size: $(bytes_to_human $js_size)"
    echo -e "    Gzipped: $(bytes_to_human $js_gzip) (${js_ratio}% compression)"
    
    # Other files
    other_count=$(find $SITE_DIR -type f ! -name "*.html" ! -name "*.css" ! -name "*.js" | wc -l)
    other_size=$(find $SITE_DIR -type f ! -name "*.html" ! -name "*.css" ! -name "*.js" -exec ls -la {} \; 2>/dev/null | awk '{sum+=$5} END {print sum+0}')
    
    echo -e "  ${YELLOW}Other Files:${NC}"
    echo -e "    Count: $other_count files (manifests, icons, fonts, etc.)"
    echo -e "    Size: $(bytes_to_human $other_size)"
}

analyze_critical_files() {
    echo -e "\n${WHITE}CRITICAL FILE ANALYSIS${NC}"
    echo -e "${WHITE}════════════════════════════════════════════════════════════════════════════════${NC}"
    
    echo -e "\n${YELLOW}Largest Files (Top 10):${NC}"
    find $SITE_DIR -type f -exec ls -la {} \; 2>/dev/null | sort -k5 -nr | head -10 | while read -r line; do
        size=$(echo $line | awk '{print $5}')
        file=$(echo $line | awk '{print $9}')
        filename=$(basename "$file")
        
        # Color code by file type
        case "$filename" in
            *.html) color=$GREEN ;;
            *.css) color=$CYAN ;;
            *.js) color=$PURPLE ;;
            *.png|*.jpg|*.svg|*.ico) color=$BLUE ;;
            *) color=$WHITE ;;
        esac
        
        # Show gzip size for compressible files
        if [[ "$filename" =~ \.(html|css|js|json)$ ]] && [ $size -gt $GZIP_THRESHOLD ]; then
            gzip_size=$(gzip -c "$file" 2>/dev/null | wc -c)
            ratio=$(get_compression_ratio $size $gzip_size)
            echo -e "  ${color}$(bytes_to_human $size)${NC} → ${GREEN}$(bytes_to_human $gzip_size)${NC} (${ratio}%) - $file"
        else
            echo -e "  ${color}$(bytes_to_human $size)${NC} - $file"
        fi
    done
    
    echo -e "\n${YELLOW}File Extension Breakdown:${NC}"
    find $SITE_DIR -type f -name "*.*" | sed 's/.*\.//' | sort | uniq -c | sort -nr | head -10 | while read count ext; do
        total_size=$(find $SITE_DIR -name "*.$ext" -exec ls -la {} \; 2>/dev/null | awk '{sum+=$5} END {print sum+0}')
        echo -e "  ${WHITE}.$ext${NC}: $count files, $(bytes_to_human $total_size)"
    done
}

analyze_pages() {
    echo -e "\n${WHITE}PAGE ANALYSIS${NC}"
    echo -e "${WHITE}════════════════════════════════════════════════════════════════════════════════${NC}"
    
    echo -e "\n${YELLOW}HTML Page Sizes:${NC}"
    find $SITE_DIR -name "*.html" | while read file; do
        size=$(ls -la "$file" | awk '{print $5}')
        gzip_size=$(gzip -c "$file" | wc -c)
        ratio=$(get_compression_ratio $size $gzip_size)
        
        # Extract page title
        title=$(grep -o '<title[^>]*>[^<]*' "$file" 2>/dev/null | sed 's/<title[^>]*>//' | head -1)
        if [ -z "$title" ]; then
            title=$(basename "$file" .html)
        fi
        
        # Color code by size
        if [ $size -gt 50000 ]; then
            color=$RED
        elif [ $size -gt 20000 ]; then
            color=$YELLOW
        else
            color=$GREEN
        fi
        
        echo -e "  ${color}$(bytes_to_human $size)${NC} → ${GREEN}$(bytes_to_human $gzip_size)${NC} (${ratio}%) - ${CYAN}$title${NC}"
        echo -e "    File: $file"
    done
}

analyze_performance() {
    echo -e "\n${WHITE}PERFORMANCE ANALYSIS${NC}"
    echo -e "${WHITE}════════════════════════════════════════════════════════════════════════════════${NC}"
    
    # Total sizes
    total_raw=$(find $SITE_DIR -type f -exec ls -la {} \; 2>/dev/null | awk '{sum+=$5} END {print sum+0}')
    total_gzip=$(find $SITE_DIR -name "*.html" -o -name "*.css" -o -name "*.js" -o -name "*.json" | xargs -I {} gzip -c {} 2>/dev/null | wc -c)
    
    # Add non-compressible files
    non_compressible=$(find $SITE_DIR -type f ! -name "*.html" ! -name "*.css" ! -name "*.js" ! -name "*.json" -exec ls -la {} \; 2>/dev/null | awk '{sum+=$5} END {print sum+0}')
    total_transfer=$((total_gzip + non_compressible))
    
    compression_ratio=$(get_compression_ratio $total_raw $total_transfer)
    
    echo -e "\n${YELLOW}Total Size Analysis:${NC}"
    echo -e "  ${WHITE}Raw Size:${NC} $(bytes_to_human $total_raw)"
    echo -e "  ${WHITE}Transfer Size (gzipped):${NC} $(bytes_to_human $total_transfer)"
    echo -e "  ${WHITE}Compression Ratio:${NC} ${compression_ratio}%"
    echo -e "  ${WHITE}Savings:${NC} $(bytes_to_human $((total_raw - total_transfer)))"
    
    # Performance budget analysis
    echo -e "\n${YELLOW}Performance Budget Analysis:${NC}"
    budget_usage=$(echo "scale=1; ($total_transfer * 100) / $CRITICAL_SIZE_LIMIT" | bc)
    
    if [ $total_transfer -gt $CRITICAL_SIZE_LIMIT ]; then
        status="${RED}OVER BUDGET${NC}"
    elif [ $total_transfer -gt $WARNING_SIZE_LIMIT ]; then
        status="${YELLOW}WARNING${NC}"
    else
        status="${GREEN}WITHIN BUDGET${NC}"
    fi
    
    echo -e "  ${WHITE}Budget Limit:${NC} $(bytes_to_human $CRITICAL_SIZE_LIMIT)"
    echo -e "  ${WHITE}Current Usage:${NC} $(bytes_to_human $total_transfer) (${budget_usage}%)"
    echo -e "  ${WHITE}Status:${NC} $status"
    echo -e "  ${WHITE}Remaining:${NC} $(bytes_to_human $((CRITICAL_SIZE_LIMIT - total_transfer)))"
    
    # Critical path analysis
    echo -e "\n${YELLOW}Critical Path Analysis:${NC}"
    
    # Homepage critical resources
    homepage_html=$(ls -la $SITE_DIR/index.html 2>/dev/null | awk '{print $5}' || echo 0)
    homepage_html_gzip=$(gzip -c $SITE_DIR/index.html 2>/dev/null | wc -c || echo 0)
    main_css=$(ls -la $SITE_DIR/css/main.css 2>/dev/null | awk '{print $5}' || echo 0)
    main_css_gzip=$(gzip -c $SITE_DIR/css/main.css 2>/dev/null | wc -c || echo 0)
    main_js=$(ls -la $SITE_DIR/js/bundle.js 2>/dev/null | awk '{print $5}' || echo 0)
    main_js_gzip=$(gzip -c $SITE_DIR/js/bundle.js 2>/dev/null | wc -c || echo 0)
    
    critical_path_size=$((homepage_html_gzip + main_css_gzip + main_js_gzip))
    
    echo -e "  ${WHITE}Homepage HTML:${NC} $(bytes_to_human $homepage_html) → $(bytes_to_human $homepage_html_gzip)"
    echo -e "  ${WHITE}Main CSS:${NC} $(bytes_to_human $main_css) → $(bytes_to_human $main_css_gzip)"
    echo -e "  ${WHITE}Main JS:${NC} $(bytes_to_human $main_js) → $(bytes_to_human $main_js_gzip)"
    echo -e "  ${WHITE}Critical Path Total:${NC} $(bytes_to_human $critical_path_size)"
    
    # Estimated load times
    echo -e "\n${YELLOW}Estimated Load Times:${NC}"
    echo -e "  ${WHITE}3G (1.6 Mbps):${NC} $(echo "scale=1; $critical_path_size * 8 / 1600000" | bc)s"
    echo -e "  ${WHITE}4G (10 Mbps):${NC} $(echo "scale=1; $critical_path_size * 8 / 10000000" | bc)s"
    echo -e "  ${WHITE}Broadband (50 Mbps):${NC} $(echo "scale=1; $critical_path_size * 8 / 50000000" | bc)s"
}

analyze_content() {
    echo -e "\n${WHITE}CONTENT ANALYSIS${NC}"
    echo -e "${WHITE}════════════════════════════════════════════════════════════════════════════════${NC}"
    
    # Count different content types
    total_pages=$(find $SITE_DIR -name "*.html" | wc -l)
    tutorial_pages=$(find $SITE_DIR -path "*/tutorials/*" -name "*.html" | wc -l)
    main_pages=$((total_pages - tutorial_pages))
    
    echo -e "\n${YELLOW}Page Count:${NC}"
    echo -e "  ${WHITE}Total Pages:${NC} $total_pages"
    echo -e "  ${WHITE}Main Pages:${NC} $main_pages"
    echo -e "  ${WHITE}Tutorial Pages:${NC} $tutorial_pages"
    
    # Analyze content density
    echo -e "\n${YELLOW}Content Density:${NC}"
    
    total_words=0
    total_lines=0
    
    find $SITE_DIR -name "*.html" | while read file; do
        # Extract text content (remove HTML tags)
        content=$(sed 's/<[^>]*>//g' "$file" | tr -d '\n' | tr -s ' ')
        words=$(echo "$content" | wc -w)
        lines=$(wc -l < "$file")
        
        total_words=$((total_words + words))
        total_lines=$((total_lines + lines))
        
        echo -e "  $(basename "$file" .html): ${words} words, ${lines} lines" >> /tmp/content_analysis.tmp
    done
    
    if [ -f /tmp/content_analysis.tmp ]; then
        echo -e "  ${WHITE}Per Page Breakdown:${NC}"
        cat /tmp/content_analysis.tmp | sort -k2 -nr | head -5
        rm /tmp/content_analysis.tmp
    fi
}

generate_optimization_recommendations() {
    echo -e "\n${WHITE}OPTIMIZATION RECOMMENDATIONS${NC}"
    echo -e "${WHITE}════════════════════════════════════════════════════════════════════════════════${NC}"
    
    # File size recommendations
    echo -e "\n${YELLOW}File Size Optimizations:${NC}"
    
    # Check for large HTML files
    large_html=$(find $SITE_DIR -name "*.html" -size +20k)
    if [ -n "$large_html" ]; then
        echo -e "  ${YELLOW}WARNING: Large HTML files detected:${NC}"
        echo "$large_html" | while read file; do
            size=$(ls -la "$file" | awk '{print $5}')
            echo -e "    • $(basename "$file"): $(bytes_to_human $size)"
        done
        echo -e "    ${WHITE}Consider:${NC} Content splitting, lazy loading for large pages"
    fi
    
    # Check CSS efficiency
    css_size=$(find $SITE_DIR -name "*.css" -exec ls -la {} \; | awk '{sum+=$5} END {print sum+0}')
    if [ $css_size -gt 10240 ]; then # > 10KB
        echo -e "  ${YELLOW}WARNING: CSS bundle size: $(bytes_to_human $css_size)${NC}"
        echo -e "    ${WHITE}Consider:${NC} CSS purging, critical CSS extraction"
    fi
    
    # Check JS efficiency
    js_size=$(find $SITE_DIR -name "*.js" -exec ls -la {} \; | awk '{sum+=$5} END {print sum+0}')
    if [ $js_size -gt 15360 ]; then # > 15KB
        echo -e "  ${YELLOW}WARNING: JavaScript bundle size: $(bytes_to_human $js_size)${NC}"
        echo -e "    ${WHITE}Consider:${NC} Code splitting, tree shaking, minification review"
    fi
    
    # Image optimization check
    image_count=$(find $SITE_DIR -name "*.png" -o -name "*.jpg" -o -name "*.jpeg" -o -name "*.gif" | wc -l)
    if [ $image_count -gt 0 ]; then
        image_size=$(find $SITE_DIR -name "*.png" -o -name "*.jpg" -o -name "*.jpeg" -o -name "*.gif" -exec ls -la {} \; | awk '{sum+=$5} END {print sum+0}')
        echo -e "  ${WHITE}Images:${NC} $image_count files, $(bytes_to_human $image_size)"
        echo -e "    ${WHITE}Consider:${NC} WebP format, responsive images, lazy loading"
    fi
    
    # Performance recommendations
    echo -e "\n${YELLOW}Performance Optimizations:${NC}"
    
    # Check for HTTP/2 push opportunities
    echo -e "  ${WHITE}HTTP/2 Server Push candidates:${NC}"
    echo -e "    • CSS: /css/main.css"
    echo -e "    • JS: /js/bundle.js"
    echo -e "    • Fonts: Critical font files (if any)"
    
    # Caching recommendations
    echo -e "  ${WHITE}Caching Strategy:${NC}"
    echo -e "    • Static assets: 1 year cache with versioning"
    echo -e "    • HTML pages: 1 hour cache with ETag validation"
    echo -e "    • Service Worker: Already implemented"
    
    # Compression recommendations
    echo -e "  ${WHITE}Server Configuration:${NC}"
    echo -e "    • Enable Brotli compression for better ratios"
    echo -e "    • Configure proper MIME types for .woff2, .json files"
    echo -e "    • Add security headers (CSP, HSTS, etc.)"
}

generate_summary() {
    echo -e "\n${WHITE}EXECUTIVE SUMMARY${NC}"
    echo -e "${WHITE}════════════════════════════════════════════════════════════════════════════════${NC}"
    
    # Calculate totals
    total_files=$(find $SITE_DIR -type f | wc -l)
    total_raw=$(find $SITE_DIR -type f -exec ls -la {} \; 2>/dev/null | awk '{sum+=$5} END {print sum+0}')
    total_gzip=$(find $SITE_DIR -name "*.html" -o -name "*.css" -o -name "*.js" -o -name "*.json" | xargs -I {} gzip -c {} 2>/dev/null | wc -c)
    non_compressible=$(find $SITE_DIR -type f ! -name "*.html" ! -name "*.css" ! -name "*.js" ! -name "*.json" -exec ls -la {} \; 2>/dev/null | awk '{sum+=$5} END {print sum+0}')
    total_transfer=$((total_gzip + non_compressible))
    
    budget_usage=$(echo "scale=1; ($total_transfer * 100) / $CRITICAL_SIZE_LIMIT" | bc)
    
    echo -e "\n${YELLOW}Key Metrics:${NC}"
    echo -e "  ${WHITE}Total Files:${NC} $total_files"
    echo -e "  ${WHITE}Raw Size:${NC} $(bytes_to_human $total_raw)"
    echo -e "  ${WHITE}Transfer Size:${NC} $(bytes_to_human $total_transfer)"
    echo -e "  ${WHITE}Compression Savings:${NC} $(bytes_to_human $((total_raw - total_transfer))) ($(get_compression_ratio $total_raw $total_transfer)%)"
    echo -e "  ${WHITE}Performance Budget:${NC} ${budget_usage}% used"
    
    if [ $total_transfer -gt $CRITICAL_SIZE_LIMIT ]; then
        echo -e "  ${WHITE}Status:${NC} ${RED}Over Budget - Optimization Required${NC}"
    elif [ $total_transfer -gt $WARNING_SIZE_LIMIT ]; then
        echo -e "  ${WHITE}Status:${NC} ${YELLOW}Approaching Limit - Monitor Closely${NC}"
    else
        echo -e "  ${WHITE}Status:${NC} ${GREEN}Excellent - Well Optimized${NC}"
    fi
    
    echo -e "\n${YELLOW}Next Steps:${NC}"
    if [ $total_transfer -lt $WARNING_SIZE_LIMIT ]; then
        echo -e "  ${GREEN}Site is well optimized and performing excellently${NC}"
        echo -e "  ${WHITE}• Monitor growth as content is added${NC}"
        echo -e "  ${WHITE}• Consider implementing Brotli compression${NC}"
    else
        echo -e "  ${YELLOW}• Review large files and optimize content${NC}"
        echo -e "  ${WHITE}• Implement lazy loading for non-critical resources${NC}"
        echo -e "  ${WHITE}• Consider code splitting for JavaScript${NC}"
    fi
}

# Main execution
main() {
    # Check if site directory exists
    if [ ! -d "$SITE_DIR" ]; then
        echo -e "${RED}Error: Site directory '$SITE_DIR' not found.${NC}"
        echo -e "${YELLOW}Please run 'npm run build' first.${NC}"
        exit 1
    fi
    
    # Check for required tools
    for tool in bc gzip; do
        if ! command -v $tool >/dev/null 2>&1; then
            echo -e "${RED}Error: Required tool '$tool' not found.${NC}"
            exit 1
        fi
    done
    
    print_header
    analyze_file_sizes
    analyze_critical_files
    analyze_pages
    analyze_performance
    analyze_content
    generate_optimization_recommendations
    generate_summary
    
    echo -e "\n${WHITE}════════════════════════════════════════════════════════════════════════════════${NC}"
    echo -e "${GREEN}Analysis complete! Report generated: $(date)${NC}"
    echo -e "${WHITE}════════════════════════════════════════════════════════════════════════════════${NC}\n"
}

# Run the analysis
main "$@"