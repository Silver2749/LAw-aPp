# LawApp - IPC Legal Search Engine in Rust

## Overview

LawApp is a Rust-based legal search engine that helps users identify potentially relevant sections of the Indian Penal Code (IPC) based on a natural language description of an activity.

The project loads a structured IPC dataset in JSON format and uses fuzzy matching to compare user input against section titles and legal descriptions. It then returns the most relevant legal provisions along with their corresponding section numbers and descriptions.

This project serves as a foundation for a future Retrieval-Augmented Generation (RAG) legal assistant capable of understanding natural language and providing contextual legal information.

## Features

* IPC dataset stored in JSON format
* Fast search using Rust
* Fuzzy matching with typo tolerance
* Search across section titles and descriptions
* Displays top matching legal provisions
* Modular architecture for future AI and RAG integration

## Tech Stack

* Rust
* Serde
* Serde JSON
* Fuzzy Matcher (SkimMatcherV2)

## Project Structure

```text
lawapp/
├── Cargo.toml
├── ipc.json
└── src/
    └── main.rs
```

## Dataset Format

Example IPC entry:

```json
{
  "chapter": 1,
  "chapter_title": "introduction",
  "Section": "1",
  "section_title": "Title and extent of operation of the Code",
  "section_desc": "This Act shall be called the Indian Penal Code..."
}
```

## Installation

### Clone Repository

```bash
git clone <repository-url>
cd lawapp
```

### Install Dependencies

Rust dependencies are managed through Cargo.

```bash
cargo build
```

## Running the Application

Place the IPC dataset file in the project root:

```text
lawapp/
├── ipc.json
├── Cargo.toml
└── src/
```

Run the application:

```bash
cargo run
```

Example:

```text
Describe an activity:
theft
```

Output:

```text
Section 379
Title: Punishment for theft
Chapter: Offences Against Property

Whoever commits theft shall be punished...
```

## Future Roadmap

### Phase 1

* JSON-based legal search
* Fuzzy matching
* CLI interface

### Phase 2

* Improved search ranking
* Keyword extraction
* Penalty extraction

### Phase 3

* Embeddings
* Vector database integration
* Semantic search

### Phase 4

* RAG-powered legal assistant
* Natural language explanations
* Web interface

## Disclaimer

This project is intended for educational and informational purposes only. It is not legal advice and should not be relied upon as a substitute for consultation with a qualified legal professional.

Fuzzy matching usually gets the right results, although big emphasis on "usually", So until I find a way to implement a RAG bot or a better search system into this, Fuzzy will have to do.
