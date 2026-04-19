//! Integration tests for index-splitting in `VaultWriter`.
//!
//! * Writing past `INDEX_ENTRY_CAP` triggers a split into alphabetical
//!   bucket children.
//! * Subsequent writes land in the correct bucket, never re-creating a
//!   monolithic parent.

use std::path::PathBuf;

use edge_kg_agent::agents::extractor::{KgOutput, Relationship};
use edge_kg_agent::vault::VaultWriter;
use tempfile::TempDir;

fn note(title: &str, tag: &str) -> KgOutput {
    KgOutput {
        title: title.to_string(),
        summary: format!("summary for {title}"),
        tags: vec![tag.to_string()],
        entities: vec![],
        relationships: Vec::<Relationship>::new(),
        markdown_snippet: format!("## body\n{title}\n"),
        tokens_sent: 0,
        tokens_received: 0,
    }
}

fn read(p: &PathBuf) -> String {
    std::fs::read_to_string(p).unwrap_or_default()
}

#[tokio::test]
async fn split_triggers_then_routes_to_bucket() {
    let tmp = TempDir::new().unwrap();
    let vault_dir = tmp.path().to_path_buf();
    let cap = 3_usize;
    let writer = VaultWriter::new(vault_dir.clone(), cap);

    // Titles chosen so each lands in a known bucket:
    //  - Alpha  → a-e
    //  - Beta   → a-e
    //  - Crisis → a-e
    //  - Delta  → a-e   (this pushes the topic index over cap → split)
    //  - Kilo   → k-o   (after split, must route to its own bucket)
    for t in ["Alpha", "Beta", "Crisis", "Delta"] {
        writer.write_note("Book", &note(t, "shared")).await.unwrap();
    }

    let topic_index = vault_dir.join("Topics").join("shared.md");
    let split_parent = read(&topic_index);
    assert!(
        split_parent.contains("split: true"),
        "topic index should be a split parent after {} writes with cap={cap}\n---\n{split_parent}",
        4
    );

    let bucket_ae = vault_dir.join("Topics").join("shared").join("a-e.md");
    let body = read(&bucket_ae);
    for t in ["Alpha", "Beta", "Crisis", "Delta"] {
        assert!(
            body.contains(&format!("[[{t}]]")),
            "a-e bucket should contain {t}:\n{body}"
        );
    }

    // Next write with a different starting letter must land in its own bucket,
    // not re-flatten the parent.
    writer.write_note("Book", &note("Kilo", "shared")).await.unwrap();
    let parent_after = read(&topic_index);
    assert!(
        parent_after.contains("split: true"),
        "parent must remain a split pointer after post-split writes"
    );
    let bucket_ko = vault_dir.join("Topics").join("shared").join("k-o.md");
    let ko_body = read(&bucket_ko);
    assert!(
        ko_body.contains("[[Kilo]]"),
        "k-o bucket should now contain Kilo:\n{ko_body}"
    );

    // The a-e bucket must still carry all four prior entries.
    let ae_body_after = read(&bucket_ae);
    for t in ["Alpha", "Beta", "Crisis", "Delta"] {
        assert!(
            ae_body_after.contains(&format!("[[{t}]]")),
            "a-e bucket lost {t} after post-split write:\n{ae_body_after}"
        );
    }
}
