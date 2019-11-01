# Arma Lint - Config

Parser, Preprocessor, and Renderer for Arma Config Files (config.cpp / Description.ext)

## Parser

The ArmaLint Config parser creates a full Abstract Syntax Tree of the config file, its includes, and its PreProcessor Commands. Macros are also parsed into their AST version to ensure accurate syntax.

## Preprocessor

The ArmaLint Config preprocessor tries to act as closely as possible to one found in Arma 3. It processes the entire file while preserving where each part of a line was originally written.

### CURRENTLY UNSUPPORTED
__EXEC  
__EVAL

## Renderer

The ArmaLint Config renderer can be used to create a processed version of a config file. All preprocessor commands like includes, macros, and defines can be seen after they have been executed.

## Rapifier

WIP
