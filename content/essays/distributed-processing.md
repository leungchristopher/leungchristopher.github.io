---
title: "Distributed processing"
date: 2026-09-29
---

The difference between multiprocessing, multithreading, and asyncio is as follows.


Multiprocessing is where you run multiple CPUs in parallel to run multiple tasks simultaneously. This is useful when you are compute-bound, e.g. in a divide and conquer situation.

Multithreading is useful when you are memory (I/O) bound, e.g. when you're moving a database from one place to another, or downloading/uploading loads of files. A single thread executes, waits for a response, then continues once the response is received. Instead, you can put your CPU to work with multithreading.

We gain lots of control over this with async/await.
