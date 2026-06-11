use crate::models::roadmap::{BadgeKind, NodeStatus, TopicContent};
use crate::state::roadmap_state::RoadmapState;
use leptos::prelude::*;

#[component]
pub fn TopicDetail(
  content: TopicContent,
  on_close: Callback<()>,
  is_open: bool,
  topic_id: &'static str,
  section_label: &'static str,
) -> impl IntoView {
  let state = use_context::<RoadmapState>().expect("RoadmapState context not found");

  // Reactive current status for this topic
  let current_status = Memo::new(move |_| {
    state
      .progress
      .get()
      .get(topic_id)
      .copied()
      .unwrap_or_default()
  });

  // Overall completion reactive signals
  let pct = state.completion_pct;
  let count = state.completed_count;
  let total = state.total_topics;

  let drawer_class = if is_open {
    "drawer drawer--open"
  } else {
    "drawer"
  };

  view! {
      <div
          class=drawer_class
          role="dialog"
          aria-modal="true"
          aria-labelledby="drawer-title"
      >
          // ── Drawer Header ──────────────────────────────────────────────
          <div class="drawer__header">
              <div class="drawer__header-content">
                  <div class="drawer__section-label">
                      {section_label}
                  </div>
                  <h1 class="drawer__title" id="drawer-title">
                      {content.title}
                  </h1>
              </div>
              <button
                  class="drawer__close"
                  on:click=move |_| on_close.run(())
                  aria-label="Close drawer"
              >
                  <svg width="24" height="24" viewBox="0 0 24 24" fill="none"
                      stroke="currentColor" stroke-width="2"
                      stroke-linecap="round" stroke-linejoin="round">
                      <line x1="18" y1="6" x2="6" y2="18"></line>
                      <line x1="6" y1="6" x2="18" y2="18"></line>
                  </svg>
              </button>
          </div>

          // ── Drawer Body ────────────────────────────────────────────────
          <div class="drawer__body">

              // ── Description ───────────────────────────────────────────
              <div class="drawer__section">
                  <p class="drawer__description">
                      {content.description}
                  </p>
              </div>

              // ── Learning Status ────────────────────────────────────────
              <div class="drawer__section">
                  <div class="drawer__status-card">
                      <div class="drawer__status-header">
                          <span class="drawer__status-label">"Learning Status"</span>
                          <span class="drawer__status-percent">
                              {move || {
                                  let s = current_status.get();
                                  match s {
                                      NodeStatus::Done       => "✓ Done".to_string(),
                                      NodeStatus::InProgress => "⟳ In Progress".to_string(),
                                      NodeStatus::Skipped    => "⊘ Skipped".to_string(),
                                      NodeStatus::Untouched  => "— Untouched".to_string(),
                                  }
                              }}
                          </span>
                      </div>

                      // Overall progress bar
                      <div class="drawer__progress-bar">
                          <div
                              class="drawer__progress-fill"
                              style=move || format!("width: {:.1}%", pct.get())
                          />
                      </div>
                      <div style="font-family: 'JetBrains Mono', monospace; font-size: 0.65rem; color: var(--orange-400); margin-bottom: 1rem; opacity: 0.8;">
                          {move || format!("Overall: {}/{} topics complete ({:.0}%)", count.get(), total, pct.get())}
                      </div>

                      // ── Status cycle buttons ───────────────────────────
                      <div class="drawer__status-cycle">
                          // Done button
                          <button
                              class=move || {
                                  let base = "drawer__status-btn drawer__status-btn--done";
                                  if current_status.get() == NodeStatus::Done {
                                      format!("{} active", base)
                                  } else {
                                      base.to_string()
                                  }
                              }
                              on:click=move |_| {
                                  let next = if current_status.get() == NodeStatus::Done {
                                      NodeStatus::Untouched
                                  } else {
                                      NodeStatus::Done
                                  };
                                  state.set_status(topic_id, next);
                              }
                          >
                              "✓ Done"
                          </button>

                          // In Progress button
                          <button
                              class=move || {
                                  let base = "drawer__status-btn drawer__status-btn--in-progress";
                                  if current_status.get() == NodeStatus::InProgress {
                                      format!("{} active", base)
                                  } else {
                                      base.to_string()
                                  }
                              }
                              on:click=move |_| {
                                  let next = if current_status.get() == NodeStatus::InProgress {
                                      NodeStatus::Untouched
                                  } else {
                                      NodeStatus::InProgress
                                  };
                                  state.set_status(topic_id, next);
                              }
                          >
                              "⟳ In Progress"
                          </button>

                          // Skip button
                          <button
                              class=move || {
                                  let base = "drawer__status-btn drawer__status-btn--skip";
                                  if current_status.get() == NodeStatus::Skipped {
                                      format!("{} active", base)
                                  } else {
                                      base.to_string()
                                  }
                              }
                              on:click=move |_| {
                                  let next = if current_status.get() == NodeStatus::Skipped {
                                      NodeStatus::Untouched
                                  } else {
                                      NodeStatus::Skipped
                                  };
                                  state.set_status(topic_id, next);
                              }
                          >
                              "⊘ Skip"
                          </button>
                      </div>
                  </div>
              </div>

              // ── Resources ─────────────────────────────────────────────
              <div class="drawer__section">
                  <h3 class="drawer__section-title">
                      <svg width="14" height="14" viewBox="0 0 24 24" fill="none"
                          stroke="currentColor" stroke-width="2"
                          stroke-linecap="round" stroke-linejoin="round">
                          <polyline points="16 18 22 12 16 6"></polyline>
                          <polyline points="8 6 2 12 8 18"></polyline>
                      </svg>
                      "Resources"
                  </h3>
                  <div class="drawer__resources">
                      {if content.resources.is_empty() {
                          view! {
                              <div class="drawer__empty">
                                  "No resources listed yet."
                              </div>
                          }.into_any()
                      } else {
                          content.resources.iter().map(|res| {
                              let badge_class = match res.badge {
                                  BadgeKind::Official     => "drawer__resource-badge badge--official",
                                  BadgeKind::OpenSource   => "drawer__resource-badge badge--opensource",
                                  BadgeKind::Crate        => "drawer__resource-badge badge--crate",
                                  BadgeKind::Article      => "drawer__resource-badge badge--article",
                                  BadgeKind::Book         => "drawer__resource-badge badge--book",
                                  BadgeKind::Video        => "drawer__resource-badge badge--video",
                                  BadgeKind::Course       => "drawer__resource-badge badge--course",
                                  BadgeKind::Interactive  => "drawer__resource-badge badge--interactive",
                                  BadgeKind::Podcast      => "drawer__resource-badge badge--podcast",
                                  BadgeKind::Newsletter   => "drawer__resource-badge badge--newsletter",
                                  BadgeKind::Community    => "drawer__resource-badge badge--community",
                                  _                       => "drawer__resource-badge badge--default",
                              };

                              let badge_text = match res.badge {
                                  BadgeKind::Official     => "Official",
                                  BadgeKind::OpenSource   => "Open Source",
                                  BadgeKind::Crate        => "Crate",
                                  BadgeKind::Article      => "Article",
                                  BadgeKind::Book         => "Book",
                                  BadgeKind::Video        => "Video",
                                  BadgeKind::Course       => "Course",
                                  BadgeKind::Interactive  => "Interactive",
                                  BadgeKind::Podcast      => "Podcast",
                                  BadgeKind::Newsletter   => "Newsletter",
                                  BadgeKind::Community    => "Community",
                                  BadgeKind::Other(s)     => s,
                              };

                              view! {
                                  <a
                                      href={res.url}
                                      target="_blank"
                                      rel="noopener noreferrer"
                                      class="drawer__resource-card"
                                  >
                                      <div class="drawer__resource-info">
                                          <svg class="drawer__resource-icon" width="16" height="16"
                                              viewBox="0 0 24 24" fill="none" stroke="currentColor"
                                              stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                              <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"></path>
                                              <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"></path>
                                          </svg>
                                          <span class="drawer__resource-label">{res.label}</span>
                                      </div>
                                      <div class="drawer__resource-meta">
                                          <span class=badge_class>{badge_text}</span>
                                          <svg class="drawer__resource-external" width="14" height="14"
                                              viewBox="0 0 24 24" fill="none" stroke="currentColor"
                                              stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                              <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"></path>
                                              <polyline points="15 3 21 3 21 9"></polyline>
                                              <line x1="10" y1="14" x2="21" y2="3"></line>
                                          </svg>
                                      </div>
                                  </a>
                              }
                          }).collect_view().into_any()
                      }}
                  </div>
              </div>

              // ── Syntax Preview ─────────────────────────────────────────
              <div class="drawer__section">
                  <h3 class="drawer__section-title">
                      <svg width="14" height="14" viewBox="0 0 24 24" fill="none"
                          stroke="currentColor" stroke-width="2"
                          stroke-linecap="round" stroke-linejoin="round">
                          <polyline points="16 18 22 12 16 6"></polyline>
                          <polyline points="8 6 2 12 8 18"></polyline>
                      </svg>
                      "Syntax Preview"
                  </h3>
                  <div class="drawer__code-preview">
                      <div class="drawer__code-line">
                          <span class="drawer__code-keyword">"fn"</span>
                          " "
                          <span class="drawer__code-function">"main"</span>
                          "() {"
                      </div>
                      <div class="drawer__code-line" style="padding-left: 1rem;">
                          <span class="drawer__code-comment">"// This is a standard Rust entry point"</span>
                      </div>
                      <div class="drawer__code-line" style="padding-left: 1rem;">
                          <span class="drawer__code-macro">"println!"</span>
                          "("
                          <span class="drawer__code-string">"\"Hello, "{content.title}"!\""</span>
                          ");"
                      </div>
                      <div class="drawer__code-line">
                          "}"
                      </div>
                  </div>
              </div>

          </div>
      </div>
  }
}
