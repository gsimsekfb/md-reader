/// In-document search state and logic.

/// Holds the state for Ctrl+F in-document search.
#[derive(Debug)]
pub struct SearchState {
    /// Whether the search bar is visible.
    pub visible: bool,
    /// Current search query text.
    pub query: String,
    /// Byte offsets of all matches in the raw content.
    pub matches: Vec<usize>,
    /// Index into `matches` — which match is currently active.
    pub current: usize,
    /// When set, the scroll area should jump to this offset (consumed once).
    pub scroll_to: Option<f32>,
    /// Remembered content height from the previous frame's ScrollArea output.
    pub content_height: f32,
}

impl Default for SearchState {
    fn default() -> Self {
        Self {
            visible: false,
            query: String::new(),
            matches: Vec::new(),
            current: 0,
            scroll_to: None,
            content_height: 0.0,
        }
    }
}

impl SearchState {
    /// Toggle the search bar on/off. Clears state when closing.
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
        if !self.visible {
            self.clear();
        }
    }

    /// Close the search bar and clear state.
    pub fn close(&mut self) {
        self.visible = false;
        self.clear();
    }

    fn clear(&mut self) {
        self.query.clear();
        self.matches.clear();
        self.current = 0;
        self.scroll_to = None;
    }

    /// Rebuild the match list by doing a case-insensitive substring search.
    pub fn update_matches(&mut self, content: &str) {
        self.matches.clear();
        self.current = 0;

        if self.query.is_empty() {
            return;
        }

        let query_lower = self.query.to_lowercase();
        let content_lower = content.to_lowercase();
        let mut start = 0;

        while let Some(pos) = content_lower[start..].find(&query_lower) {
            self.matches.push(start + pos);
            start += pos + query_lower.len();
        }
    }

    /// Navigate to the next match.
    pub fn next(&mut self, content_len: usize) {
        if self.matches.is_empty() {
            return;
        }
        self.current = (self.current + 1) % self.matches.len();
        self.compute_scroll(content_len);
    }

    /// Navigate to the previous match.
    pub fn prev(&mut self, content_len: usize) {
        if self.matches.is_empty() {
            return;
        }
        if self.current == 0 {
            self.current = self.matches.len() - 1;
        } else {
            self.current -= 1;
        }
        self.compute_scroll(content_len);
    }

    /// Compute an approximate scroll offset based on the match's byte position.
    pub fn compute_scroll(&mut self, content_len: usize) {
        if content_len == 0 || self.matches.is_empty() {
            return;
        }
        let offset = self.matches[self.current] as f32;
        let fraction = offset / content_len as f32;
        // Scroll to that fraction of the total content height, with a small
        // upward nudge so the match isn't right at the top edge.
        let target = (fraction * self.content_height - 40.0).max(0.0);
        self.scroll_to = Some(target);
    }

    /// Format the match counter for display, e.g. "3/12".
    pub fn match_label(&self) -> String {
        if self.query.is_empty() {
            String::new()
        } else if self.matches.is_empty() {
            "0/0".to_string()
        } else {
            format!("{}/{}", self.current + 1, self.matches.len())
        }
    }
}
