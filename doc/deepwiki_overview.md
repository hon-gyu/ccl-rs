Overview
Relevant source files

CCL (Categorical Configuration Language) is a minimalistic yet powerful configuration language based on category theory principles. This document provides a high-level overview of the CCL repository, its architecture, core components, and key features. For more in-depth information about specific aspects, please refer to the corresponding wiki pages referenced throughout this document.
Purpose and Scope

CCL is designed to provide a clean, efficient configuration format that balances simplicity and expressiveness. It achieves this through a key-value mapping system that supports nested structures through indentation, similar to YAML but with a more principled approach based on category theory.

For detailed information on core concepts and theoretical foundations, see Core Concepts. For practical usage instructions, see Usage Guide.
Key Features

CCL supports a wide range of configuration needs through the following features:

    Key-value mappings (fundamental building block)
    Lists
    Strings
    Dates
    Algebraic Data Types
    Comments
    Sections
    Nested records

Sources:
README.md18-30

ccl.opam3-5
System Architecture

CCL consists of three primary components that work together to parse, represent, and manipulate configuration data:

CCL System

CCL Module (Main Interface)

Parser Module

Model Module

String Utilities

Key-Value Store

Client Code

Configuration File

Sources:
lib/ccl.mli1-5

dune-project17-21
Data Flow

The following diagram illustrates how data flows through the CCL system:

decode_file

decode

parse

key-value pairs

structured representation

pretty

merge

access

CCL Configuration File

CCL Module

String Input

Parser Module

Model Module

Configuration Object

Human-readable Output

Application Code

Sources:

lib/ccl.mli7-34
Core Components
CCL Module

The CCL module serves as the main interface to the library, providing functions to decode configuration files and strings.

Key functions:

    decode_file: Reads a file and parses its contents into a configuration model
    decode: Parses a string into a configuration model

Sources:

lib/ccl.mli7-34
Parser Module

The Parser module is responsible for parsing CCL text into key-value pairs, with support for nested structures through indentation.

The parser:

    Processes the input text into key-value pairs
    Handles nested structures through indentation
    Reports parse errors with helpful messages

For more details about the Parser module, see Parser API.

Sources:

lib/ccl.mli4-5
Model Module

The Model module manages the internal representation of configurations as a recursive fixed-point data structure, with operations for:

    Merging configurations
    Creating configurations programmatically
    Pretty-printing configurations

For more details about the Model module, see Model API.

Sources:

lib/ccl.mli1-2
Component Relationships

The following diagram shows the detailed relationships between the core components of CCL:

Uses

Uses

CCL

+decode_file(string) :(Model.t, Parser.error) : result

+decode(string) :(Model.t, Parser.error) : result

Model

+type t

+empty : t

+pretty(t) : : string

+merge(t, t) : : t

Parser

+type error

+parse(string) :(key_val list, error) : result

Sources:

lib/ccl.mli1-34
Usage Patterns

There are several ways to use CCL:

    Configuration files: Write CCL files directly and parse them in your application
    Programmatic usage: Use the embedded Domain-Specific Language (eDSL) to define configurations in code
    Command-line tool: Use the cclq CLI tool for querying CCL files

Example Usage Pattern

Usage Patterns

decoded by

constructs via eDSL

used by

queried by

CCL File

Application

Program Code

CCL Configuration Object

Command Line Tool

For more detailed information on usage patterns, see:

    Configuration File Format
    Programmatic Usage (eDSL)
    Command Line Tool

Sources:
lib/ccl.mli17-28

README.md18-30
Implementation Details

CCL is implemented in OCaml and uses:

    The angstrom library for parsing
    A fixed-point data structure for representing nested configurations
    Category theory principles (reflected in merge operations and composition)

For more implementation details, see Developer Guide.