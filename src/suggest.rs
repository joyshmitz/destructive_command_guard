#![allow(clippy::missing_const_for_fn)]
//! Suggest-allowlist clustering and pattern generation utilities.
//!
//! This module clusters similar denied commands and generates conservative regex
//! patterns for allowlist suggestions. It prioritizes specificity over generality
//! to avoid allowing destructive command variants.
//!
//! # Pattern Generation Strategy
//!
//! Given a cluster of similar commands, generate a regex pattern that:
//! - Matches all commands in the cluster
//! - Stays as specific as possible
//! - Uses token anchoring and explicit alternation over wildcards
//! - Avoids broad `.*` patterns that could allow destructive variants

use crate::normalize::strip_wrapper_prefixes;
use regex::{escape as regex_escape, Regex};
use std::collections::{HashMap, HashSet};

/// Default similarity threshold for clustering (Jaccard over token sets).
const DEFAULT_SIMILARITY_THRESHOLD: f32 = 0.30;

/// Output cluster of similar commands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandCluster {
    /// Original commands in the cluster (deduplicated, stable order).
    pub commands: Vec<String>,
    /// Normalized commands in the cluster (deduplicated, stable order).
    pub normalized: Vec<String>,
    /// Proposed regex pattern covering the cluster.
    pub proposed_pattern: String,
    /// Total frequency across all commands in the cluster.
    pub frequency: usize,
    /// Unique command variants in the cluster.
    pub unique_count: usize,
}

#[derive(Debug, Clone)]
struct CommandRecord {
    original: String,
    normalized: String,
    tokens: Vec<String>,
    program: String,
    count: usize,
}

#[derive(Debug, Clone)]
struct TempCluster {
    records: Vec<CommandRecord>,
    rep_tokens: Vec<String>,
}

impl TempCluster {
    fn new(record: CommandRecord) -> Self {
        Self {
            rep_tokens: record.tokens.clone(),
            records: vec![record],
        }
    }

    fn add(&mut self, record: CommandRecord) {
        self.records.push(record);
    }

    fn into_command_cluster(self) -> CommandCluster {
        let mut commands = Vec::new();
        let mut normalized = Vec::new();
        let mut seen_commands = HashSet::new();
        let mut seen_normalized = HashSet::new();
        let mut frequency = 0_usize;

        for record in &self.records {
            frequency = frequency.saturating_add(record.count);
            if seen_commands.insert(record.original.clone()) {
                commands.push(record.original.clone());
            }
            if seen_normalized.insert(record.normalized.clone()) {
                normalized.push(record.normalized.clone());
            }
        }

        let proposed_pattern = build_proposed_pattern(&normalized);
        let unique_count = normalized.len();

        CommandCluster {
            commands,
            normalized,
            proposed_pattern,
            frequency,
            unique_count,
        }
    }
}

/// Cluster denied commands into similarity groups.
///
/// `commands` is a list of (command, count) pairs.
#[must_use]
pub fn cluster_denied_commands(
    commands: &[(String, usize)],
    min_cluster_size: usize,
) -> Vec<CommandCluster> {
    cluster_denied_commands_with_threshold(commands, min_cluster_size, DEFAULT_SIMILARITY_THRESHOLD)
}

fn cluster_denied_commands_with_threshold(
    commands: &[(String, usize)],
    min_cluster_size: usize,
    similarity_threshold: f32,
) -> Vec<CommandCluster> {
    if commands.is_empty() {
        return Vec::new();
    }

    let mut records = Vec::with_capacity(commands.len());
    for (command, count) in commands {
        let normalized = normalize_for_clustering(command);
        let tokens = tokenize_for_similarity(&normalized);
        let program = tokens.first().cloned().unwrap_or_default();
        records.push(CommandRecord {
            original: command.clone(),
            normalized,
            tokens,
            program,
            count: *count,
        });
    }

    let mut groups: HashMap<String, Vec<CommandRecord>> = HashMap::new();
    for record in records {
        groups
            .entry(record.program.clone())
            .or_default()
            .push(record);
    }

    let mut clusters = Vec::new();
    for (_program, group) in groups {
        let mut temp_clusters: Vec<TempCluster> = Vec::new();
        for record in group {
            let mut record_opt = Some(record);
            let mut placed = false;
            for cluster in &mut temp_clusters {
                let record_ref = record_opt.as_ref().expect("record should be present");
                let similarity = jaccard_similarity(&cluster.rep_tokens, &record_ref.tokens);
                if similarity >= similarity_threshold {
                    let record = record_opt.take().expect("record should be present");
                    cluster.add(record);
                    placed = true;
                    break;
                }
            }
            if !placed {
                let record = record_opt.take().expect("record should be present");
                temp_clusters.push(TempCluster::new(record));
            }
        }

        for cluster in temp_clusters {
            if cluster.records.len() >= min_cluster_size {
                clusters.push(cluster.into_command_cluster());
            }
        }
    }

    clusters.sort_by(|a, b| {
        b.frequency
            .cmp(&a.frequency)
            .then_with(|| a.proposed_pattern.cmp(&b.proposed_pattern))
    });

    clusters
}

fn normalize_for_clustering(command: &str) -> String {
    let stripped = strip_wrapper_prefixes(command);
    collapse_whitespace(stripped.normalized.as_ref())
}

fn collapse_whitespace(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut last_was_space = false;
    for ch in input.chars() {
        if ch.is_whitespace() {
            if !last_was_space {
                out.push(' ');
                last_was_space = true;
            }
        } else {
            out.push(ch);
            last_was_space = false;
        }
    }
    out.trim().to_string()
}

fn tokenize_for_similarity(command: &str) -> Vec<String> {
    command
        .split_whitespace()
        .map(str::to_ascii_lowercase)
        .collect()
}

fn jaccard_similarity(a: &[String], b: &[String]) -> f32 {
    if a.is_empty() && b.is_empty() {
        return 1.0;
    }

    let set_a: HashSet<&str> = a.iter().map(String::as_str).collect();
    let set_b: HashSet<&str> = b.iter().map(String::as_str).collect();

    if set_a.is_empty() && set_b.is_empty() {
        return 1.0;
    }

    let intersection = u32::try_from(set_a.intersection(&set_b).count()).unwrap_or(u32::MAX);
    let union = u32::try_from(set_a.union(&set_b).count()).unwrap_or(u32::MAX);

    if union == 0 {
        0.0
    } else {
        #[allow(clippy::cast_precision_loss)]
        {
            intersection as f32 / union as f32
        }
    }
}

fn build_proposed_pattern(commands: &[String]) -> String {
    if commands.is_empty() {
        return String::new();
    }

    let mut unique = Vec::new();
    let mut seen = HashSet::new();
    for cmd in commands {
        if seen.insert(cmd.clone()) {
            unique.push(cmd.clone());
        }
    }

    if unique.len() == 1 {
        return format!("^{}$", regex_escape(&unique[0]));
    }

    let mut parts = Vec::with_capacity(unique.len());
    for cmd in unique {
        parts.push(regex_escape(&cmd));
    }

    format!("^(?:{})$", parts.join("|"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clusters_similar_commands_by_program() {
        let input = vec![
            ("npm run build --production".to_string(), 10),
            ("npm run test --coverage".to_string(), 5),
            ("git status".to_string(), 2),
        ];

        let clusters = cluster_denied_commands(&input, 2);
        assert_eq!(clusters.len(), 1);
        let cluster = &clusters[0];
        assert_eq!(cluster.unique_count, 2);
        assert!(cluster.proposed_pattern.contains("npm"));
        assert!(cluster.proposed_pattern.contains("run"));
    }

    #[test]
    fn respects_min_cluster_size() {
        let input = vec![("git status".to_string(), 1), ("docker ps".to_string(), 1)];

        let clusters = cluster_denied_commands(&input, 2);
        assert!(clusters.is_empty());
    }

    #[test]
    fn proposed_pattern_is_anchored_and_escaped() {
        let input = vec![("echo foo|bar".to_string(), 3)];
        let clusters = cluster_denied_commands(&input, 1);
        assert_eq!(clusters.len(), 1);
        let pattern = &clusters[0].proposed_pattern;
        assert!(pattern.starts_with('^'));
        assert!(pattern.ends_with('$'));
        assert!(pattern.contains("\\|"));
    }

    #[test]
    fn handles_empty_input() {
        let input: Vec<(String, usize)> = vec![];
        let clusters = cluster_denied_commands(&input, 1);
        assert!(clusters.is_empty());
    }

    #[test]
    fn handles_single_command() {
        let input = vec![("git reset --hard".to_string(), 5)];
        let clusters = cluster_denied_commands(&input, 1);
        assert_eq!(clusters.len(), 1);
        assert_eq!(clusters[0].unique_count, 1);
        assert_eq!(clusters[0].frequency, 5);
        // Single command pattern should be exact match
        assert!(clusters[0].proposed_pattern.starts_with("^"));
        assert!(clusters[0].proposed_pattern.ends_with("$"));
    }

    #[test]
    fn handles_all_different_programs() {
        // Commands with completely different programs don't cluster
        let input = vec![
            ("git status".to_string(), 1),
            ("npm install".to_string(), 1),
            ("docker ps".to_string(), 1),
        ];
        let clusters = cluster_denied_commands(&input, 2);
        assert!(
            clusters.is_empty(),
            "No clusters should form when all programs differ"
        );
    }

    #[test]
    fn strips_wrapper_prefixes_before_clustering() {
        let input = vec![
            ("sudo git reset --hard".to_string(), 3),
            ("git reset --soft".to_string(), 2),
        ];
        let clusters = cluster_denied_commands(&input, 2);
        assert_eq!(clusters.len(), 1);
        // Both commands should cluster together after stripping sudo
        assert!(
            clusters[0]
                .normalized
                .iter()
                .all(|n| !n.starts_with("sudo"))
        );
    }

    #[test]
    fn accumulates_frequency_across_cluster() {
        let input = vec![
            ("git reset --hard".to_string(), 10),
            ("git reset --soft".to_string(), 5),
            ("git reset --mixed".to_string(), 3),
        ];
        let clusters = cluster_denied_commands(&input, 1);
        assert_eq!(clusters.len(), 1);
        assert_eq!(clusters[0].frequency, 18);
    }

    #[test]
    fn deduplicates_identical_commands() {
        let input = vec![("git status".to_string(), 5), ("git status".to_string(), 3)];
        let clusters = cluster_denied_commands(&input, 1);
        assert_eq!(clusters.len(), 1);
        // unique_count should be 1 since same command
        assert_eq!(clusters[0].unique_count, 1);
        // frequency should be sum
        assert_eq!(clusters[0].frequency, 8);
    }

    #[test]
    fn sorts_clusters_by_frequency_descending() {
        let input = vec![
            ("npm run build".to_string(), 1),
            ("npm run test".to_string(), 1),
            ("git status".to_string(), 50),
            ("git log".to_string(), 50),
        ];
        let clusters = cluster_denied_commands(&input, 2);
        assert_eq!(clusters.len(), 2);
        // git cluster has higher frequency (100) so comes first
        assert!(clusters[0].commands[0].starts_with("git"));
        assert!(clusters[1].commands[0].starts_with("npm"));
    }

    #[test]
    fn jaccard_similarity_identical_tokens() {
        let a = vec!["git".to_string(), "reset".to_string(), "--hard".to_string()];
        let b = vec!["git".to_string(), "reset".to_string(), "--hard".to_string()];
        let similarity = jaccard_similarity(&a, &b);
        assert!(
            (similarity - 1.0).abs() < 0.001,
            "Identical tokens should have similarity 1.0"
        );
    }

    #[test]
    fn jaccard_similarity_no_overlap() {
        let a = vec!["git".to_string(), "status".to_string()];
        let b = vec!["npm".to_string(), "install".to_string()];
        let similarity = jaccard_similarity(&a, &b);
        assert!(
            (similarity - 0.0).abs() < 0.001,
            "No overlap should have similarity 0.0"
        );
    }

    #[test]
    fn jaccard_similarity_empty_sets() {
        let a: Vec<String> = vec![];
        let b: Vec<String> = vec![];
        let similarity = jaccard_similarity(&a, &b);
        assert!(
            (similarity - 1.0).abs() < 0.001,
            "Empty sets should have similarity 1.0"
        );
    }

    #[test]
    fn proposed_pattern_alternation_for_multiple_commands() {
        let input = vec![("echo hello".to_string(), 1), ("echo world".to_string(), 1)];
        let clusters = cluster_denied_commands(&input, 2);
        assert_eq!(clusters.len(), 1);
        // Pattern should use alternation for multiple variants
        let pattern = &clusters[0].proposed_pattern;
        assert!(pattern.contains("(?:"));
        assert!(pattern.contains("|"));
    }

    #[test]
    fn handles_commands_with_special_regex_chars() {
        let input = vec![("echo $HOME".to_string(), 1), ("echo $PATH".to_string(), 1)];
        let clusters = cluster_denied_commands(&input, 2);
        assert_eq!(clusters.len(), 1);
        // Pattern should escape the $
        let pattern = &clusters[0].proposed_pattern;
        assert!(pattern.contains("\\$"));
    }

    #[test]
    fn normalize_collapses_whitespace() {
        let input = vec![
            ("git   reset   --hard".to_string(), 1),
            ("git reset --hard".to_string(), 1),
        ];
        let clusters = cluster_denied_commands(&input, 1);
        assert_eq!(clusters.len(), 1);
        // Both should normalize to same and dedupe
        assert_eq!(clusters[0].unique_count, 1);
    }
}
