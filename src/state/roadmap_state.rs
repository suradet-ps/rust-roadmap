//! Global reactive state for the roadmap.
//! Provide once at the page root; access anywhere via `use_context::<RoadmapState>()`.

use crate::models::roadmap::NodeStatus;
use crate::storage::local_storage;
use leptos::prelude::*;
use std::collections::HashMap;

/// Central state store passed through Leptos context.
/// All fields are `Copy` signal handles, so the struct itself is `Copy`.
#[derive(Clone, Copy)]
pub struct RoadmapState {
  /// Live search term (drives dimming / highlighting in the diagram).
  pub search_term: RwSignal<String>,

  /// Currently selected topic ID; `Some` opens the detail drawer.
  pub selected_topic_id: RwSignal<Option<&'static str>>,

  /// Per-topic progress. Keys are owned Strings (topic IDs).
  pub progress: RwSignal<HashMap<String, NodeStatus>>,

  /// Total number of topics (static, set once at startup).
  pub total_topics: usize,

  /// Reactive count of Done + Skipped topics.
  pub completed_count: Memo<usize>,

  /// Reactive completion percentage 0.0 – 100.0.
  pub completion_pct: Memo<f64>,
}

impl RoadmapState {
  /// Create a new `RoadmapState`.
  /// **Must** be called inside a Leptos reactive root (i.e. within a component).
  pub fn new(total_topics: usize) -> Self {
    // Hydrate initial progress from localStorage.
    let initial = local_storage::load_progress();
    let progress = RwSignal::new(initial);

    let completed_count = Memo::new(move |_| {
      progress
        .get()
        .values()
        .filter(|s| matches!(s, NodeStatus::Done | NodeStatus::Skipped))
        .count()
    });

    let completion_pct = Memo::new(move |_| {
      if total_topics == 0 {
        return 0.0_f64;
      }
      (completed_count.get() as f64 / total_topics as f64) * 100.0
    });

    // Auto-persist to localStorage whenever progress changes.
    Effect::new(move |_| {
      let p = progress.get();
      local_storage::save_progress(&p);
    });

    Self {
      search_term: RwSignal::new(String::new()),
      selected_topic_id: RwSignal::new(None),
      progress,
      total_topics,
      completed_count,
      completion_pct,
    }
  }

  /// Read the current status of a topic (reactive – reads the signal).
  pub fn get_status(&self, topic_id: &str) -> NodeStatus {
    self
      .progress
      .get()
      .get(topic_id)
      .copied()
      .unwrap_or_default()
  }

  /// Set a specific status for a topic.
  /// Removes the entry if `status` is `Untouched` to keep the map lean.
  pub fn set_status(&self, topic_id: &'static str, status: NodeStatus) {
    self.progress.update(|map| {
      if status == NodeStatus::Untouched {
        map.remove(topic_id);
      } else {
        map.insert(topic_id.to_string(), status);
      }
    });
  }

  /// Cycle: Untouched → InProgress → Done → Skipped → Untouched.
  pub fn cycle_status(&self, topic_id: &'static str) {
    self.progress.update(|map| {
      let current = map.get(topic_id).copied().unwrap_or_default();
      let next = current.cycle();
      if next == NodeStatus::Untouched {
        map.remove(topic_id);
      } else {
        map.insert(topic_id.to_string(), next);
      }
    });
  }
}
