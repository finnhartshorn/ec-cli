use html2text::from_read;
use regex::Regex;

/// Extract sample/example data from HTML description
///
/// Looks for content within <pre class="note"> tags
pub fn extract_samples(html: &str) -> Vec<String> {
    let re = Regex::new(r#"<pre class="note">(.*?)</pre>"#).unwrap();

    re.captures_iter(html)
        .map(|cap| cap[1].to_string())
        .collect()
}

/// Convert HTML to plain text for terminal display
///
/// Wraps text to specified width and formats for terminal display
pub fn html_to_text(html: &str, width: usize) -> String {
    from_read(html.as_bytes(), width)
}

/// Format submit response for display
pub fn format_submit_response(response: &crate::models::SubmitResponse) -> String {
    let mut output = String::new();

    if response.correct {
        output.push_str("✓ Correct!\n");

        if response.first_correct {
            output.push_str("  🎉 First to solve!\n");
        }

        output.push_str(&format!("  Global place: {}\n", response.global_place));
        output.push_str(&format!("  Global score: {}\n", response.global_score));
        output.push_str(&format!("  Time: {}ms\n", response.time));
    } else {
        output.push_str("✗ Incorrect\n");

        if response.length_correct {
            output.push_str("  (Answer length is correct)\n");
        }
    }

    if !response.message.is_empty() {
        output.push_str(&format!("  Message: {}\n", response.message));
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_samples() {
        let html = r#"<pre class="note">Sample 1</pre><p>text</p><pre class="note">Sample 2</pre>"#;
        let samples = extract_samples(html);
        assert_eq!(samples.len(), 2);
        assert_eq!(samples[0], "Sample 1");
        assert_eq!(samples[1], "Sample 2");
    }

    #[test]
    fn test_extract_samples_empty() {
        let html = r#"<p>No samples here</p>"#;
        let samples = extract_samples(html);
        assert_eq!(samples.len(), 0);
    }
}
