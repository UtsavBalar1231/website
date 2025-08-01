import { DateTime } from "luxon";

export default function(eleventyConfig) {
  // Copy static assets
  eleventyConfig.addPassthroughCopy("src/static");
  eleventyConfig.addPassthroughCopy("src/js");
  eleventyConfig.addPassthroughCopy("src/css");
  
  // Watch for changes in JavaScript files
  eleventyConfig.addWatchTarget("src/js/");
  eleventyConfig.addWatchTarget("src/styles/");

  // Date filters
  eleventyConfig.addFilter("readableDate", dateObj => {
    return DateTime.fromJSDate(dateObj, {zone: 'utc'}).toFormat("dd LLL yyyy");
  });

  eleventyConfig.addFilter("htmlDateString", (dateObj) => {
    return DateTime.fromJSDate(dateObj, {zone: 'utc'}).toFormat('yyyy-LL-dd');
  });

  // Get the first N elements of a collection
  eleventyConfig.addFilter("head", (array, n) => {
    if(!Array.isArray(array) || array.length === 0) {
      return [];
    }
    if( n < 0 ) {
      return array.slice(n);
    }
    return array.slice(0, n);
  });

  // Get current year
  eleventyConfig.addFilter("currentYear", () => {
    return new Date().getFullYear();
  });

  // Markdown processing - use default markdown-it for now
  // eleventyConfig.setLibrary("md", markdownIt({
  //   html: true,
  //   breaks: true,
  //   linkify: true
  // }));

  // Collections
  eleventyConfig.addCollection("tutorials", function(collectionApi) {
    return collectionApi.getFilteredByGlob("content/tutorials/*.md")
      .sort((a, b) => {
        const aNum = parseInt(a.data.part) || 0;
        const bNum = parseInt(b.data.part) || 0;
        return aNum - bNum;
      });
  });

  return {
    templateFormats: [
      "md",
      "njk",
      "html",
      "liquid"
    ],
    
    markdownTemplateEngine: "njk",
    htmlTemplateEngine: "njk",
    
    dir: {
      input: "content",
      includes: "../src/_includes",
      data: "../data",
      output: "_site"
    }
  };
};