# ⚠️ Disclaimer

This project is intended **solely for educational, research, and software development purposes**. It was created to explore concepts in **Rust programming, information retrieval and semantic search.

The information provided by this application **does not constitute legal advice** and should not be relied upon for making legal decisions. The search results are generated algorithmically and may be incomplete, inaccurate, or fail to account for the specific facts and circumstances of a situation.

Users must **not misuse this project for unlawful activities, legal evasion, or any purpose that could facilitate criminal conduct**. The author does not endorse or encourage illegal behavior of any kind.

If you require legal guidance or assistance, **consult a qualified legal professional or the appropriate legal authority**.

By using this software, you acknowledge that it is provided for educational purposes only and that you assume full responsibility for how you use the information it produces.

---
# LawApp - Semantic Legal Search Engine

A Rust-based semantic legal search engine that allows users to describe an activity in natural language and retrieves the most relevant sections of the Indian Penal Code (IPC) using transformer embeddings and vector similarity search.

This project was built to explore the intersection of backend engineering, information retrieval, data engineering, and AI while learning more about Rust.

---

## Features

* Semantic search using embeddings
* Natural language legal queries
* IPC knowledge base stored as JSON
* Transformer-based text embeddings
* Vector similarity search using cosine similarity
* Structured and modular Rust codebase
* Foundation for future RAG integration

---

## Example

### Input

```text
Describe an activity:

I stole a motorcycle from a parking lot
```

### Output

```text
Top Matches

Similarity: 0.942

Section 379
Punishment for theft

Whoever commits theft shall be punished...

--------------------------------

Similarity: 0.861

Section 380
Theft in dwelling house...
```

---

## How It Works

```text
IPC JSON Dataset
        │
        ▼
Embedding Generation
        │
        ▼
embedded_ipc.json
        │
        ▼
User Query
        │
        ▼
Generate Query Embedding
        │
        ▼
Cosine Similarity Search
        │
        ▼
Top Matching IPC Sections
```

---

## Tech Stack

* Rust
* Serde
* Serde JSON
* FastEmbed
* Transformer Embeddings
* Vector Similarity Search

---

## Project Structure

```text
lawapp/
│
├── src/
│   ├── main.rs
│   ├── models.rs
│   ├── embed.rs
│   └── search.rs
│
├── ipc.json
├── embedded_ipc.json
├── Cargo.toml
├── Cargo.lock
├── README.md
└── LICENSE
```

---

## Installation

### Clone the repository

```bash
git clone https://github.com/yourusername/lawapp.git

cd lawapp
```

### Install Rust

https://www.rust-lang.org/tools/install

Verify installation:

```bash
rustc --version
cargo --version
```

---

## Dependencies

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
fastembed = "5.16.0"
```

---

## Running the Project

### Generate Embeddings

If embeddings have not been generated:

```bash
cargo run
```

This creates:

```text
embedded_ipc.json
```

containing vector representations of every IPC section.

### Search

Run the application:

```bash
cargo run
```

Enter a natural language description of an activity when prompted.

Example:

```text
I hacked into someone's computer

I kidnapped a child

I stole a bicycle

I threatened someone with a weapon
```

The application returns the most semantically relevant IPC sections.

---

## Challenges Faced

One of the biggest challenges during development was data preparation.

Rather than relying on an existing dataset, a custom parser was built to process official IPC documents and convert legal text into machine-readable JSON.

This involved:

* Parsing legal documents
* Cleaning noisy text
* Handling duplicate entries
* Structuring hundreds of IPC sections
* Resolving inconsistent schemas
* Supporting both numeric and alphanumeric section identifiers

---

## Skills Demonstrated

### Backend Engineering

* Rust
* Modular architecture
* File I/O
* Error handling

### Data Engineering

* JSON processing
* Data parsing
* Data cleaning
* Schema normalization

### Information Retrieval

* Semantic search
* Embeddings
* Vector similarity
* Cosine similarity

### AI Foundations

* Transformer embeddings
* Knowledge base construction
* Retrieval pipelines
* RAG architecture fundamentals

---

## Future Roadmap

* Improve search ranking
* Hybrid search (Embeddings + BM25)
* Retrieval-Augmented Generation (RAG)
* LLM integration
* REST API
* Web interface
* Vector database integration
* Support for Bharatiya Nyaya Sanhita (BNS)




