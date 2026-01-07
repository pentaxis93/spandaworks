//! Contact tools - wraps khard CLI

use crate::cli::run_command_stdout;

#[derive(Clone, Default)]
pub struct ContactTools;

impl ContactTools {
    pub fn new() -> Self {
        Self
    }

    /// Search contacts by name, email, or phone
    pub async fn search(&self, query: String) -> String {
        // khard list supports search terms
        match run_command_stdout("khard", &["list", &query]).await {
            Ok(output) => {
                if output.trim().is_empty() || output.contains("No contacts found") {
                    format!("No contacts found matching: {}", query)
                } else {
                    format!("Contacts matching '{}':\n\n{}", query, output)
                }
            }
            Err(e) => {
                // khard returns error if no matches, check if it's just empty results
                let err_str = e.to_string();
                if err_str.contains("No contacts found") {
                    format!("No contacts found matching: {}", query)
                } else {
                    format!("Error searching contacts: {}", e)
                }
            }
        }
    }

    /// Get full details of a specific contact
    pub async fn get(&self, name: String) -> String {
        match run_command_stdout("khard", &["show", &name]).await {
            Ok(output) => {
                if output.trim().is_empty() {
                    format!("No contact found with name: {}", name)
                } else {
                    output
                }
            }
            Err(e) => format!("Error getting contact: {}", e),
        }
    }

    /// Create a new contact
    /// Note: khard new is interactive, so we create a vCard directly
    pub async fn create(
        &self,
        name: String,
        email: Option<String>,
        phone: Option<String>,
        organization: Option<String>,
    ) -> String {
        // For now, provide guidance since khard new is interactive
        // A full implementation would write a .vcf file directly

        let mut vcard = format!(
            "BEGIN:VCARD\n\
             VERSION:3.0\n\
             FN:{}\n\
             N:{};;;;\n",
            name, name
        );

        if let Some(ref e) = email {
            vcard.push_str(&format!("EMAIL:{}\n", e));
        }
        if let Some(ref p) = phone {
            vcard.push_str(&format!("TEL:{}\n", p));
        }
        if let Some(ref o) = organization {
            vcard.push_str(&format!("ORG:{}\n", o));
        }
        vcard.push_str("END:VCARD\n");

        // Write to contacts directory
        let contact_dir = format!(
            "{}/.local/share/vdirsyncer/contacts/default",
            std::env::var("HOME").unwrap_or_default()
        );

        // Generate a unique filename
        let uuid = format!(
            "{:x}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        );
        let filename = format!("{}/{}.vcf", contact_dir, uuid);

        match tokio::fs::write(&filename, &vcard).await {
            Ok(_) => {
                format!(
                    "Contact created: {}\n\
                     Email: {}\n\
                     Phone: {}\n\
                     Organization: {}\n\n\
                     Saved to: {}\n\
                     Note: Run 'vdirsyncer sync' to upload to Google Contacts.",
                    name,
                    email.unwrap_or_else(|| "(none)".into()),
                    phone.unwrap_or_else(|| "(none)".into()),
                    organization.unwrap_or_else(|| "(none)".into()),
                    filename
                )
            }
            Err(e) => format!("Error creating contact: {}", e),
        }
    }

    /// Generate a vCard string for a contact
    #[cfg(test)]
    pub fn generate_vcard(
        name: &str,
        email: Option<&str>,
        phone: Option<&str>,
        organization: Option<&str>,
    ) -> String {
        let mut vcard = format!("BEGIN:VCARD\nVERSION:3.0\nFN:{}\nN:{};;;;\n", name, name);

        if let Some(e) = email {
            vcard.push_str(&format!("EMAIL:{}\n", e));
        }
        if let Some(p) = phone {
            vcard.push_str(&format!("TEL:{}\n", p));
        }
        if let Some(o) = organization {
            vcard.push_str(&format!("ORG:{}\n", o));
        }
        vcard.push_str("END:VCARD\n");
        vcard
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_vcard_minimal() {
        let vcard = ContactTools::generate_vcard("John Doe", None, None, None);
        assert!(vcard.contains("BEGIN:VCARD"));
        assert!(vcard.contains("FN:John Doe"));
        assert!(vcard.contains("END:VCARD"));
        assert!(!vcard.contains("EMAIL:"));
    }

    #[test]
    fn test_generate_vcard_full() {
        let vcard = ContactTools::generate_vcard(
            "Jane Smith",
            Some("jane@example.com"),
            Some("+1234567890"),
            Some("Acme Corp"),
        );
        assert!(vcard.contains("FN:Jane Smith"));
        assert!(vcard.contains("EMAIL:jane@example.com"));
        assert!(vcard.contains("TEL:+1234567890"));
        assert!(vcard.contains("ORG:Acme Corp"));
    }
}
