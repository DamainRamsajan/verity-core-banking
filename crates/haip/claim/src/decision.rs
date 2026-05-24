use super::types::DefaultOption;

/// Formats options using Hick's law and Miller's law.
pub struct DecisionPresenter;

impl DecisionPresenter {
    pub fn new() -> Self { Self }

    /// Chunk options to ≤7 items (Miller's law), with safe default first.
    pub fn chunk_options(&self, options: &[DefaultOption], max_items: usize) -> Vec<DefaultOption> {
        let mut opts: Vec<DefaultOption> = options.to_vec();
        // Place default first
        if let Some(def_pos) = opts.iter().position(|o| o.is_default) {
            opts.swap(0, def_pos);
        }
        opts.truncate(max_items);
        opts
    }
}
