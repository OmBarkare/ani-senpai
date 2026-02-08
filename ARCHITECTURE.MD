# ani-senpai — Design & Architecture

ani-senpai is a smart CLI tool that helps users decide *what anime to watch*, then launches the selected anime using `ani-cli`.

It does **not** stream, scrape, or host content.  
It focuses on **intent understanding, discovery, and UX orchestration**.

---

## High-Level Goal

Turn vague human input like:

- “lonely cyberpunk night vibes”
- “something like Steins;Gate but slower”
- “chill slice of life, rural, healing”

into a **curated, paginated list of anime**, and then delegate playback to `ani-cli`.

---

## Core Design Philosophy

- Separation of concerns over cleverness
- Deterministic behavior where possible
- LLMs assist interpretation, not decision-making
- CLI-first, Unix-friendly UX
- Cloud-assisted now, offline-capable later

---

## System Architecture Overview


Each stage has a single responsibility and can be replaced or bypassed.

---

## Component Responsibilities

### 1. CLI Layer

**Responsibility**
- Parse user input
- Handle flags (future: --no-llm, --offline, --sort)
- Control pagination and selection flow

**Non-responsibility**
- No recommendation logic
- No API-specific code

---

### 2. Intent Extraction (Gemini)

**Responsibility**
- Convert unstructured human language into structured intent

**Example Input**
