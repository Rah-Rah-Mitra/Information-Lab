# System Status

_Updated: 2026-04-24T09:18:01.750567700+00:00_

## Queue

- Pending chunks: **92**

## API Usage — 2026-04-24

| Metric | Value |
| --- | --- |
| Reasoner (extractor) calls | 178 |
| Vision calls | 0 |
| Curator calls | 0 |
| Bridge calls | 0 |
| Harvester calls | 1 |
| Search (Tavily) calls | 0 |
| Theorem calls | 0 |
| Derivation calls | 0 |
| Report calls | 1 |
| Formula extractor calls | 1 |
| Tokens sent (est.) | 2089849 |
| Tokens received (est.) | 213383 |

_Token counts are chars/4 estimates — the autoagents 0.3.7 Google backend does not surface `usageMetadata`._

## Documents

| Source | Path | Progress | Done/Total | Pending | Batched | Errors | Status |
| --- | --- | --- | --- | --- | --- | --- | --- |
| LA | `LA/Numerical_Linear_Algebra.pdf` | `███░░░░░░░`  29.8% | 42/141 | 92 | 7 | 0 | processing |
| LA | `LA/Iterative Methods for Sparse Linear.pdf` | `█████████░`  85.1% | 160/188 | 0 | 0 | 8 | complete |
| OR | `OR/Numerical Optimization.pdf` | `█████████░`  93.6% | 278/297 | 0 | 0 | 0 | complete |
| OR | `OR/Model Building in Mathematical Programming.pdf` | `██████████`  96.1% | 173/180 | 0 | 0 | 0 | complete |

## Recent agent activity

| ts | agent | kind | tokens (s/r) | dur (ms) | input → output |
| --- | --- | --- | --- | --- | --- |
| 2026-04-24 09:17:56 | extractor | llm_call | 11120/1212 | 24485 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title": "Numerical Stability and Backward Error Analysis", "summary": "Numeric… |
| 2026-04-24 09:17:31 | extractor | llm_call | 11027/998 | 21172 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title": "Least Squares Problems and Numerical Stability", "summary": "Least sq… |
| 2026-04-24 09:17:10 | extractor | llm_call | 11066/1213 | 25333 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title": "QR Factorization and Least Squares", "summary": "QR factorization dec… |
| 2026-04-24 09:16:44 | extractor | llm_call | 11662/1000 | 20872 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title": "QR Factorization and Gram-Schmidt Orthogonalization", "summary": "QR … |
| 2026-04-24 09:16:23 | extractor | llm_call | 10980/1247 | 28710 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title": "Singular Value Decomposition and Matrix Norms", "summary": "The Singu… |
| 2026-04-24 09:15:55 | extractor | llm_call | 11521/1364 | 26062 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title": "Schwarz Methods and Graph Partitioning", "summary": "Schwarz methods … |
| 2026-04-24 09:13:28 | extractor | llm_call | 11413/1051 | 25198 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title": "Fundamentals of Matrix-Vector Multiplication and Orthogonality", "sum… |
| 2026-04-24 09:13:03 | extractor | llm_call | 11462/1055 | 22438 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title": "Krylov Subspace Methods and Sparse Matrix Storage", "summary": "Krylo… |
| 2026-04-24 09:12:41 | extractor | llm_call | 10749/771 | 17200 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title": "Graph Partitioning and Domain Decomposition", "summary": "Graph parti… |
| 2026-04-24 09:12:23 | extractor | llm_call | 11521/2181 | 43666 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title": "Schwarz Methods and Graph Partitioning", "summary": "Schwarz methods … |
| 2026-04-24 09:11:40 | extractor | llm_call | 11747/1214 | 23983 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title": "Domain Decomposition and Schur Complement", "summary": "Domain decomp… |
| 2026-04-24 09:09:16 | extractor | llm_call | 12055/1239 | 24590 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title": "Approximate Inverse Preconditioning Techniques", "summary": "Approxim… |
| 2026-04-24 09:08:51 | extractor | llm_call | 11702/1000 | 21078 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title": "Incomplete LU Factorization and ILUT", "summary": "Incomplete LU (ILU… |
| 2026-04-24 09:08:30 | extractor | llm_call | 12236/1100 | 21874 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title": "Polynomial Preconditioning Techniques", "summary": "Polynomial precon… |
| 2026-04-24 09:08:08 | extractor | llm_call | 11993/1375 | 25843 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title": "Parallel Sparse Matrix Operations", "summary": "This note explores th… |
| 2026-04-24 09:07:42 | extractor | llm_call | 11455/962 | 18531 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title": "Incomplete LQ Factorization and Parallel Architectures", "summary": "… |
| 2026-04-24 09:07:23 | extractor | llm_call | 11194/2073 | 36042 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title":"Approximate Inverse Preconditioning Techniques","summary":"Approximate… |
| 2026-04-24 09:04:47 | extractor | llm_call | 11606/1126 | 24494 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title": "Incomplete LU Factorization", "summary": "Incomplete LU factorization… |
| 2026-04-24 08:58:23 | extractor | llm_call | 11314/1272 | 29099 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title": "Preconditioning and Incomplete Factorization Techniques", "summary": … |
| 2026-04-24 08:57:54 | extractor | llm_call | 11735/1228 | 27547 | # Skill: Knowledge-Graph Extractor (body content only)  You are a knowledge-grap… → {"title": "Normal Equations and Krylov Subspace Methods", "summary": "This note … |
