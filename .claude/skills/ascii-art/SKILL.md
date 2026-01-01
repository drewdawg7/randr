---
name: ascii-art
description: Guidelines and procedures for creating, using, and editing ascii art. Use when code changes requires ascii art
---
## Overview
Because this game uses a TUI it uses ascii art throughout in order to provide graphics. This guide covers bestpractices around creating, using, and editing ascii art throughought the project. This guide also aims to provide examples and explanations to make future ascii art generation easier. This directory will also contain examples and documentation to make things easier. Make sure you update this documentation frequently.


## Density
Density refers to how much space a glphy takes up in it's 'glpyh slot' in a terminal. There are, broadly, 3 levels of density:
1. **Low**: These symbols don't take up much space at all. Examples are characters like (comma-separated): ', /, :, -, \, ., *, {, ! .
2. **Medium**: These symbols are the midpoint in density and are made up of a lot of the alphanumeric characters as well as others like (comma-separated): @, %, $, #
3. **High**: These symbols are the most dense. They're any extremely blocky characters that take up most of the glyph spot. 

## Color
ascii art should use color wherever possible. Include multiple shades of colors to attempt to give depth.

## Depth
When possible, use colors and symbols in a way that graphics give a sense of depth.

## Nerdfont
The project uses a nerdfont font so it has access to a lot of additional symbols that can aid in making ascii art. 
Link to list of symbols: https://nerdfonts.ytyng.com

## Borders
Default to using ascii art borders over normal ratatui borders.

## Background Color
Make sure background colors match to the ascii art's colors when possible.

## UI Design
When designing larger UIs, such as screens, use ascii art to fill in deadspace. This doesn't have to be large pieces or full graphics, it can be just small details throught the page.

## Documentation
* When creating ascii art, if the user likes something that has been made add it as an example in .claude/skills/ascii art.
* Name files descriptively and add in actual examples from the code base. Provide explanations on where it was used, colors that were used, etc.
* If a file starts to get too large, split it out and put it into a subdirectory.
* Focus on writing documentation such that it is easy to reference and understand.
* Feel free to create smaller documentation files that just explain smaller concepts but do not have full ascii art. For example, a file that lists a lot of the useful nerdfont symbols and what they were used for.
