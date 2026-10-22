# Harness design

A generalisable design is simple: it should have the flexibility to take in complex logic, whilst maintaining a simple interface. What constraints do we then put in?

The workflow should be automated and simple (e.g. plan, execute, observe, improve), instead of being tied to a prompt template set out at the beginning.

The file system acts as a form of persistent memory (and may be versioned with git).

Subagents and backend jobs need to be legible via a process manager, which launches jobs, inspects logs, kills failed runs, and merges the relevant results back to the main thread.

Source: Lilian Weng [(Link)](https://lilianweng.github.io/posts/2026-07-04-harness/)

