//! Global completion progress bar component.
//!
//! Reads `RoadmapState` from Leptos context and renders a thin animated bar
//! together with a "X/Y · Z%" completion label.

use crate::state::roadmap_state::RoadmapState;
use leptos::prelude::*;

/// Thin progress bar rendered inside the sticky header.
///
/// No props required — all data is pulled from [`RoadmapState`] context.
#[component]
pub fn ProgressBar() -> impl IntoView {
  let state = use_context::<RoadmapState>().expect("RoadmapState context not found");

  let pct = state.completion_pct;
  let count = state.completed_count;
  let total = state.total_topics;

  view! {
      <div class="progress-row">
          <div class="progress-track">
              <div
                  class="progress-fill"
                  style=move || format!("width: {:.1}%", pct.get())
              />
          </div>
          <span class="progress-label">
              {move || format!("{}/{} · {:.0}%", count.get(), total, pct.get())}
          </span>
      </div>
  }
}
