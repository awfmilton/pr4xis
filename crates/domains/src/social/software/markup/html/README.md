# HTML5 Ontology

This directory contains the formal HTML5 ontology for the `pr4xis` markup stack.

## Overview

The HTML5 ontology models HTML documents as formally verified categorical structures, grounded in the WHATWG HTML Living Standard.

## Status

- **Structural Ontology**: Implemented (Document, Doctype, Element, Attribute, Text, Comment).
- **Validation**: Axiomatic well-formedness (Single Root Element, Valid Nesting).
- **Emission**: Generic `MarkupNode` conversion for downstream source generation.

## Future Work

- Full DOM parsing and reader implementation.
- ARIA and accessibility ontology depth.
- Browser and execution semantics.
