//! Bundled agents embedded at compile time.
//!
//! Agents are extracted from the aiandi repo's assets/agents/ directory and embedded
//! in the binary using include_str!(). This makes the binary self-contained.

/// Explore agent - Fast codebase search (Haiku)
pub const EXPLORE_AGENT: &str = include_str!("../../../assets/agents/explore.md");

/// Researcher agent - Web research and synthesis (Sonnet)
pub const RESEARCHER_AGENT: &str = include_str!("../../../assets/agents/researcher.md");

/// Story agent - Requirements to BDD specifications (Sonnet)
pub const STORY_AGENT: &str = include_str!("../../../assets/agents/story.md");

/// Test-writer agent - BDD specs to failing tests (Sonnet)
pub const TEST_WRITER_AGENT: &str = include_str!("../../../assets/agents/test-writer.md");

/// Validator agent - Test execution and phase validation (Sonnet)
pub const VALIDATOR_AGENT: &str = include_str!("../../../assets/agents/validator.md");

/// Builder agent - Code implementation (Opus)
pub const BUILDER_AGENT: &str = include_str!("../../../assets/agents/builder.md");

/// Reviewer agent - Code review and analysis (Sonnet)
pub const REVIEWER_AGENT: &str = include_str!("../../../assets/agents/reviewer.md");

/// Documenter agent - Documentation writing (Sonnet)
pub const DOCUMENTER_AGENT: &str = include_str!("../../../assets/agents/documenter.md");

/// A bundled agent that can be extracted to a project
#[derive(Debug, Clone)]
pub struct BundledAgent {
    /// Agent name (used as filename)
    pub name: &'static str,
    /// Agent content (markdown with YAML frontmatter)
    pub content: &'static str,
}

/// Returns all bundled agents
pub fn bundled_agents() -> Vec<BundledAgent> {
    vec![
        BundledAgent {
            name: "explore",
            content: EXPLORE_AGENT,
        },
        BundledAgent {
            name: "researcher",
            content: RESEARCHER_AGENT,
        },
        BundledAgent {
            name: "story",
            content: STORY_AGENT,
        },
        BundledAgent {
            name: "test-writer",
            content: TEST_WRITER_AGENT,
        },
        BundledAgent {
            name: "validator",
            content: VALIDATOR_AGENT,
        },
        BundledAgent {
            name: "builder",
            content: BUILDER_AGENT,
        },
        BundledAgent {
            name: "reviewer",
            content: REVIEWER_AGENT,
        },
        BundledAgent {
            name: "documenter",
            content: DOCUMENTER_AGENT,
        },
    ]
}

/// Returns a bundled agent by name, if it exists
pub fn get_agent(name: &str) -> Option<BundledAgent> {
    bundled_agents().into_iter().find(|a| a.name == name)
}

/// Returns list of all bundled agent names
pub fn agent_names() -> Vec<&'static str> {
    bundled_agents().iter().map(|a| a.name).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bundled_agents_not_empty() {
        let agents = bundled_agents();
        assert!(!agents.is_empty(), "Should have at least one bundled agent");
        assert_eq!(agents.len(), 8, "Should have exactly 8 agents");
    }

    #[test]
    fn test_explore_agent_has_content() {
        assert!(
            !EXPLORE_AGENT.is_empty(),
            "Explore agent should have content"
        );
        assert!(
            EXPLORE_AGENT.contains("mode: subagent"),
            "Explore agent should be a subagent"
        );
        assert!(
            EXPLORE_AGENT.contains("hidden: true"),
            "Explore agent should be hidden"
        );
        assert!(
            EXPLORE_AGENT.contains("haiku"),
            "Explore agent should use Haiku"
        );
    }

    #[test]
    fn test_builder_agent_has_content() {
        assert!(
            !BUILDER_AGENT.is_empty(),
            "Builder agent should have content"
        );
        assert!(
            BUILDER_AGENT.contains("mode: subagent"),
            "Builder agent should be a subagent"
        );
        assert!(
            BUILDER_AGENT.contains("claude-opus-4-5"),
            "Builder agent should use Opus"
        );
    }

    #[test]
    fn test_story_agent_has_content() {
        assert!(!STORY_AGENT.is_empty(), "Story agent should have content");
        assert!(
            STORY_AGENT.contains("mode: subagent"),
            "Story agent should be a subagent"
        );
        assert!(
            STORY_AGENT.contains("BDD"),
            "Story agent should mention BDD"
        );
    }

    #[test]
    fn test_test_writer_agent_has_content() {
        assert!(
            !TEST_WRITER_AGENT.is_empty(),
            "Test-writer agent should have content"
        );
        assert!(
            TEST_WRITER_AGENT.contains("mode: subagent"),
            "Test-writer agent should be a subagent"
        );
        assert!(
            TEST_WRITER_AGENT.contains("RED"),
            "Test-writer agent should mention RED phase"
        );
    }

    #[test]
    fn test_validator_agent_has_content() {
        assert!(
            !VALIDATOR_AGENT.is_empty(),
            "Validator agent should have content"
        );
        assert!(
            VALIDATOR_AGENT.contains("mode: subagent"),
            "Validator agent should be a subagent"
        );
        assert!(
            VALIDATOR_AGENT.contains("GREEN"),
            "Validator agent should mention GREEN phase"
        );
    }

    #[test]
    fn test_all_agents_are_hidden_subagents() {
        for agent in bundled_agents() {
            assert!(
                agent.content.contains("mode: subagent"),
                "{} should be a subagent",
                agent.name
            );
            assert!(
                agent.content.contains("hidden: true"),
                "{} should be hidden",
                agent.name
            );
        }
    }

    #[test]
    fn test_get_agent_found() {
        let agent = get_agent("explore");
        assert!(agent.is_some(), "Should find explore agent");
        assert_eq!(agent.unwrap().name, "explore");
    }

    #[test]
    fn test_get_agent_not_found() {
        let agent = get_agent("nonexistent");
        assert!(agent.is_none(), "Should not find nonexistent agent");
    }

    #[test]
    fn test_agent_names() {
        let names = agent_names();
        assert!(names.contains(&"explore"));
        assert!(names.contains(&"researcher"));
        assert!(names.contains(&"story"));
        assert!(names.contains(&"test-writer"));
        assert!(names.contains(&"validator"));
        assert!(names.contains(&"builder"));
        assert!(names.contains(&"reviewer"));
        assert!(names.contains(&"documenter"));
    }
}
