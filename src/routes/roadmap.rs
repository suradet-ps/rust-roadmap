use crate::components::roadmap::detail_view::TopicDetail;
use crate::components::roadmap::diagram::{DiagramData, RoadmapDiagram};
use crate::components::ui::footer::Footer;
use crate::components::ui::header::Header;
use crate::components::ui::hero::Hero;
use crate::data::get_topic_content;
use crate::data::{SECTIONS, get_all_dependencies, get_all_topics};
use crate::layout::tree::{LayoutConfig, compute_layout};
use crate::state::roadmap_state::RoadmapState;
use leptos::prelude::*;

#[component]
pub fn RoadmapPage() -> impl IntoView {
  let config = LayoutConfig::default();

  let topics = get_all_topics();
  let total_topics = topics.len();
  let dependencies = get_all_dependencies();

  let static_topics: &'static [_] = Box::leak(topics.into_boxed_slice());
  let static_deps: &'static [_] = Box::leak(dependencies.into_boxed_slice());

  let layout = compute_layout(SECTIONS, static_topics, static_deps, &config);

  // -----------------------------------------------------------------------
  // Global state — provide via context so all child components can access it
  // -----------------------------------------------------------------------
  let state = RoadmapState::new(total_topics);
  provide_context(state);

  // Convenience aliases
  let search_term = state.search_term;
  let selected_topic_id = state.selected_topic_id;

  // -----------------------------------------------------------------------
  // Callbacks
  // -----------------------------------------------------------------------
  let handle_search = Callback::new(move |term: String| {
    search_term.set(term);
  });

  let handle_close_detail = Callback::new(move |_: ()| {
    selected_topic_id.set(None);
  });

  // -----------------------------------------------------------------------
  // Derived signals
  // -----------------------------------------------------------------------
  let is_drawer_open = Memo::new(move |_| selected_topic_id.get().is_some());

  // -----------------------------------------------------------------------
  // Diagram props (on_topic_click now lives inside RoadmapDiagram via context)
  // -----------------------------------------------------------------------
  let diagram_props = DiagramData {
    topics: static_topics,
    dependencies: static_deps,
    layout,
    config,
  };

  view! {
      <div class="roadmap-page">
          // Background decorations
          <div class="bg-decorations">
              <div class="glow-orb"></div>
              <div class="noise-overlay"></div>
          </div>

          // Sticky header (contains search + progress bar)
          <Header search_term=search_term.read_only() on_search=handle_search />

          // Main content
          <main class="main-content">
              <Hero />

              // Horizontally-scrollable roadmap canvas
              <div class="roadmap-container">
                  <RoadmapDiagram props=diagram_props />
              </div>
          </main>

          <Footer />

          // Backdrop (always mounted so the fade-out animation plays)
          <div
              class=move || {
                  if is_drawer_open.get() {
                      "drawer-backdrop drawer-backdrop--visible"
                  } else {
                      "drawer-backdrop"
                  }
              }
              on:click=move |_| handle_close_detail.run(())
          />

          // Detail drawer — re-mounts whenever the selected topic changes
          {move || {
              let topic_id = selected_topic_id.get()?;
              let content = get_topic_content(topic_id)?;

              // Look up the section label from static data
              let section_label = static_topics
                  .iter()
                  .find(|t| t.id == topic_id)
                  .and_then(|t| SECTIONS.iter().find(|s| s.id == t.section_id))
                  .map(|s| s.title)
                  .unwrap_or("");

              Some(view! {
                  <TopicDetail
                      content=content
                      on_close=handle_close_detail
                      is_open=is_drawer_open.get()
                      topic_id=topic_id
                      section_label=section_label
                  />
              })
          }}
      </div>
  }
}
