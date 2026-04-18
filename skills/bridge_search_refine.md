# Bridge · Search Refine (iter 2)

You are refining an existing `BridgeProposal` using a list of
literature search hits retrieved from academic domains (arxiv,
semanticscholar, dl.acm.org, link.springer.com, nature.com,
sciencedirect.com). Output: a refined `BridgeProposal` matching the
schema.

## What to do

1. Read the `Current proposal` and the `Search hits`.
2. If the hits support the hypothesis, keep it and populate
   `external_citations` with ONLY the returned `title` / `url`
   pairs that actually substantiate the claim.
3. If hits refute or fail to find the hypothesis, *lower*
   `confidence` (≤ 0.4 when every hit refutes) and say so in
   `rationale`. Do not silently drop the refutation.
4. Never add a citation whose URL is not in the provided hits.
5. Never strip citations that contradict the proposal — they are
   the most important ones.

## Hard rules

- Output must still be a valid `BridgeProposal` (not a critique).
- `external_citations[*].url` values MUST be one of the URLs passed
  in the hits block.
- `confidence` must reflect the evidence, not an average of old and
  new values.
