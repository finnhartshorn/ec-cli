use html2text::from_read;
use regex::Regex;

/// Extract sample/example data from HTML description
///
/// Looks for content within <pre class="note"> tags
pub fn extract_samples(html: &str) -> Vec<String> {
    let re = Regex::new(r#"(?s)<pre class="note">(.*?)</pre>"#).unwrap();

    re.captures_iter(html)
        .map(|cap| cap[1].trim_start().to_string())
        .collect()
}

/// Extract expected answer from HTML description
///
/// Looks for the last occurrence of <pre> <b>ANSWER</b> </pre> pattern
/// which typically appears at the end of example sections
pub fn extract_expected_answer(html: &str) -> Option<String> {
    let re = Regex::new(r#"<pre>\s*<b>([^<]+)</b>\s*</pre>"#).unwrap();

    // Get the last match, as that's typically the final answer
    re.captures_iter(html)
        .last()
        .map(|cap| cap[1].trim().to_string())
}

/// Convert HTML to plain text for terminal display
///
/// Wraps text to specified width and formats for terminal display
pub fn html_to_text(html: &str, width: usize) -> String {
    from_read(html.as_bytes(), width).unwrap_or("Error converting HTML to text".to_string())
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

    #[test]
    fn test_extract_samples_multiline() {
        let html = r#"<pre class="note">
Vyrdax,Drakzyph,Fyrryn,Elarzris

R3,L2,R3,L1
</pre>"#;
        let samples = extract_samples(html);
        assert_eq!(samples.len(), 1);
        assert_eq!(samples[0], "Vyrdax,Drakzyph,Fyrryn,Elarzris\n\nR3,L2,R3,L1\n");
    }

    #[test]
    fn test_extract_expected_answer() {
        let html = r#"<p>The answer is <pre> <b>Drakzyph</b> </pre>.</p>"#;
        let answer = extract_expected_answer(html);
        assert_eq!(answer, Some("Drakzyph".to_string()));
    }

    #[test]
    fn test_extract_expected_answer_with_whitespace() {
        let html = r#"<p>Result: <pre><b> Fyrryn </b></pre></p>"#;
        let answer = extract_expected_answer(html);
        assert_eq!(answer, Some("Fyrryn".to_string()));
    }

    #[test]
    fn test_extract_expected_answer_none() {
        let html = r#"<p>No answer here</p>"#;
        let answer = extract_expected_answer(html);
        assert_eq!(answer, None);
    }

    #[test]
    fn test_extract_expected_answer_last_match() {
        let html = r#"
            <p>First: <pre> <b>Wrong</b> </pre></p>
            <p>Second: <pre> <b>AlsoWrong</b> </pre></p>
            <p>The final answer is <pre> <b>Correct</b> </pre>.</p>
        "#;
        let answer = extract_expected_answer(html);
        assert_eq!(answer, Some("Correct".to_string()));
    }
}
