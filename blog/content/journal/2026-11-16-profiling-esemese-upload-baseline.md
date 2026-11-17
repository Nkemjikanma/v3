+++
title = "Profiling the esemese upload path: baseline"
date = 2026-11-16
description = "First look at where time goes when esemese ingests a batch of photos."
draft = true

[taxonomies]
tags = ["esemese", "rust", "profiling"]
+++

<!--
Journal entry shape. Keep it short; numbers over adjectives.
  1. What I tried
  2. What I measured
  3. What surprised me
  4. What's next
Remove `draft = true` above when the numbers are real.
-->

## What I tried

Uploaded a batch of 50 photos to `esemese-backend-server` running locally, and profiled the request path
(decode → EXIF → resize → blurhash → S3 put → Postgres insert).

Tools: `cargo flamegraph` / `samply` for CPU, `tokio-console` for runtime stalls, k6 for load.

## What I measured

| Metric | Value |
| --- | --- |
| Batch size | 50 photos |
| p50 / p95 upload latency | _ / _ |
| Peak RSS | _ |
| Hottest stack (flamegraph) | _ |
| Tokio worker blocked? | _ |

## What surprised me

_

## What's next

- Move CPU-bound image work off the async runtime (`spawn_blocking` or a rayon pool) and re-measure.
- Same test under k6 with 10 concurrent uploaders.
