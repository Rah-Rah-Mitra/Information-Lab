# System Status

_Updated: 2026-04-24T10:03:01.805402200+00:00_

## Queue

- Pending chunks: **0**

## API Usage — 2026-04-24

| Metric | Value |
| --- | --- |
| Reasoner (extractor) calls | 206 |
| Vision calls | 0 |
| Curator calls | 0 |
| Bridge calls | 0 |
| Harvester calls | 1 |
| Search (Tavily) calls | 0 |
| Theorem calls | 0 |
| Derivation calls | 0 |
| Report calls | 1 |
| Formula extractor calls | 1 |
| Tokens sent (est.) | 2414043 |
| Tokens received (est.) | 245408 |

_Token counts are chars/4 estimates — the autoagents 0.3.7 Google backend does not surface `usageMetadata`._

## Documents

| Source | Path | Progress | Done/Total | Pending | Batched | Errors | Status |
| --- | --- | --- | --- | --- | --- | --- | --- |
| LA | `LA/Numerical_Linear_Algebra.pdf` | `█████████░`  94.3% | 133/141 | 0 | 0 | 0 | complete |
| LA | `LA/Iterative Methods for Sparse Linear.pdf` | `█████████░`  89.4% | 168/188 | 0 | 0 | 0 | complete |
| OR | `OR/Numerical Optimization.pdf` | `█████████░`  93.6% | 278/297 | 0 | 0 | 0 | complete |
| OR | `OR/Model Building in Mathematical Programming.pdf` | `██████████`  96.1% | 173/180 | 0 | 0 | 0 | complete |

## Recent agent activity

| ts | agent | kind | tokens (s/r) | dur (ms) | input → output |
| --- | --- | --- | --- | --- | --- |
| 2026-04-24 09:49:04 | extractor | llm_call | 8229/1072 | 20412 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title": "Numerical Stability and Iterative Methods", "summary": "This note cov… |
| 2026-04-24 09:41:43 | extractor | llm_call | 11838/866 | 17556 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title": "Preconditioning In Iterative Methods", "summary": "Preconditioning is… |
| 2026-04-24 09:37:26 | extractor | llm_call | 11708/1218 | 26941 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title": "Convergence and Preconditioning of Iterative Methods", "summary": "Th… |
| 2026-04-24 09:32:59 | extractor | llm_call | 11318/1161 | 27125 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title":"Conjugate Gradients and Biorthogonalization Methods","summary":"This n… |
| 2026-04-24 09:32:32 | extractor | llm_call | 12041/1086 | 25075 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title": "Lanczos Iteration and Conjugate Gradients", "summary": "The Lanczos i… |
| 2026-04-24 09:28:07 | extractor | llm_call | 12262/785 | 14931 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title": "Preconditioning Strategies and Numerical Analysis Philosophy", "summa… |
| 2026-04-24 09:27:52 | extractor | llm_call | 11708/1308 | 30022 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title": "Convergence and Preconditioning of Krylov Subspace Methods", "summary… |
| 2026-04-24 09:25:22 | extractor | llm_call | 11633/1209 | 28842 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title": "Krylov Subspace Iterative Methods", "summary": "Krylov subspace metho… |
| 2026-04-24 09:24:53 | extractor | llm_call | 11752/1139 | 26418 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title": "The Arnoldi Iteration And Krylov Subspaces", "summary": "The Arnoldi … |
| 2026-04-24 09:24:26 | extractor | llm_call | 12210/1263 | 25897 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title": "Stability and Conditioning of Least Squares", "summary": "This note e… |
| 2026-04-24 09:22:00 | extractor | llm_call | 12200/1129 | 24139 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title": "Eigenvalue Algorithms and Iterative Methods", "summary": "This note c… |
| 2026-04-24 09:21:36 | extractor | llm_call | 11480/1229 | 24532 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title": "Eigenvalue Problems and Matrix Factorizations", "summary": "This note… |
| 2026-04-24 09:21:11 | extractor | llm_call | 11732/1061 | 21322 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title": "Gaussian Elimination and Cholesky Factorization", "summary": "This no… |
| 2026-04-24 09:20:50 | extractor | llm_call | 11206/1283 | 25696 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title": "Stability and Pivoting in Linear Systems", "summary": "This note expl… |
| 2026-04-24 09:20:24 | extractor | llm_call | 12293/1438 | 28609 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title": "Multicoloring and Incomplete Factorization Preconditioners", "summary… |
| 2026-04-24 09:17:56 | extractor | llm_call | 11120/1212 | 24485 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title": "Numerical Stability and Backward Error Analysis", "summary": "Numeric… |
| 2026-04-24 09:17:31 | extractor | llm_call | 11027/998 | 21172 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title": "Least Squares Problems and Numerical Stability", "summary": "Least sq… |
| 2026-04-24 09:17:10 | extractor | llm_call | 11066/1213 | 25333 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title": "QR Factorization and Least Squares", "summary": "QR factorization dec… |
| 2026-04-24 09:16:44 | extractor | llm_call | 11662/1000 | 20872 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title": "QR Factorization and Gram-Schmidt Orthogonalization", "summary": "QR … |
| 2026-04-24 09:16:23 | extractor | llm_call | 10980/1247 | 28710 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title": "Singular Value Decomposition and Matrix Norms", "summary": "The Singu… |
