//! Bundled skills embedded at compile time.
//!
//! Skills are extracted from the aiandi repo's skills/ directory and embedded
//! in the binary using include_str!(). This makes the binary self-contained.

/// Transmission protocol skill - XML governance communication
pub const TRANSMISSION_SKILL: &str = include_str!("../../../skills/transmission/SKILL.md");

/// GTD skill - Getting Things Done with TaskWarrior
pub const GTD_SKILL: &str = include_str!("../../../skills/gtd/SKILL.md");

/// Governance skill - Identity and routing for Governance sessions
pub const GOVERNANCE_SKILL: &str = include_str!("../../../skills/governance/SKILL.md");

/// A bundled skill that can be extracted to a project
#[derive(Debug, Clone)]
pub struct BundledSkill {
    /// Skill name (used as directory name)
    pub name: &'static str,
    /// Skill content (SKILL.md contents)
    pub content: &'static str,
}

/// Returns all bundled skills
pub fn bundled_skills() -> Vec<BundledSkill> {
    vec![
        BundledSkill {
            name: "transmission",
            content: TRANSMISSION_SKILL,
        },
        BundledSkill {
            name: "gtd",
            content: GTD_SKILL,
        },
        BundledSkill {
            name: "governance",
            content: GOVERNANCE_SKILL,
        },
    ]
}

/// Returns a bundled skill by name, if it exists
pub fn get_skill(name: &str) -> Option<BundledSkill> {
    bundled_skills().into_iter().find(|s| s.name == name)
}

/// Returns list of all bundled skill names
pub fn skill_names() -> Vec<&'static str> {
    bundled_skills().iter().map(|s| s.name).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bundled_skills_not_empty() {
        let skills = bundled_skills();
        assert!(!skills.is_empty(), "Should have at least one bundled skill");
    }

    #[test]
    fn test_transmission_skill_has_content() {
        assert!(
            !TRANSMISSION_SKILL.is_empty(),
            "Transmission skill should have content"
        );
        assert!(
            TRANSMISSION_SKILL.contains("transmission"),
            "Transmission skill should mention transmission"
        );
    }

    #[test]
    fn test_gtd_skill_has_content() {
        assert!(!GTD_SKILL.is_empty(), "GTD skill should have content");
        assert!(
            GTD_SKILL.contains("GTD") || GTD_SKILL.contains("gtd"),
            "GTD skill should mention GTD"
        );
    }

    #[test]
    fn test_get_skill_found() {
        let skill = get_skill("transmission");
        assert!(skill.is_some(), "Should find transmission skill");
        assert_eq!(skill.unwrap().name, "transmission");
    }

    #[test]
    fn test_get_skill_not_found() {
        let skill = get_skill("nonexistent");
        assert!(skill.is_none(), "Should not find nonexistent skill");
    }

    #[test]
    fn test_skill_names() {
        let names = skill_names();
        assert!(names.contains(&"transmission"));
        assert!(names.contains(&"gtd"));
        assert!(names.contains(&"governance"));
    }

    #[test]
    fn test_governance_skill_has_content() {
        assert!(
            !GOVERNANCE_SKILL.is_empty(),
            "Governance skill should have content"
        );
        assert!(
            GOVERNANCE_SKILL.contains("Governance") || GOVERNANCE_SKILL.contains("governance"),
            "Governance skill should mention governance"
        );
    }
}
