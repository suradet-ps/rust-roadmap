//! Main roadmap diagram component.
//!
//! Renders the full SVG canvas including:
//! - Section group boxes (`SectionGroup`)
//! - Topic nodes (`RoadmapNode`) with status and search-dimming
//! - Connector edges (`RoadmapEdge`) with horizontal cross-section routing

use crate::components::roadmap::edge::{ArrowheadMarker, EdgeData, RoadmapEdge};
use crate::components::roadmap::group::{GroupBoxData, SectionGroup};
use crate::components::roadmap::node::{NodeData, RoadmapNode};
use crate::layout::tree::{LayoutConfig, LayoutResult, TopicPosition};
use crate::models::roadmap::{Dependency, Topic};
use crate::state::roadmap_state::RoadmapState;
use leptos::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

// ---------------------------------------------------------------------------
// Public data type passed from the page
// ---------------------------------------------------------------------------

#[derive(Clone)]
pub struct DiagramData {
  pub topics: &'static [Topic],
  pub dependencies: &'static [Dependency],
  pub layout: LayoutResult,
  pub config: LayoutConfig,
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

fn find_topic<'a>(topics: &'a [Topic], id: &str) -> Option<&'a Topic> {
  topics.iter().find(|t| t.id == id)
}

fn find_position<'a>(positions: &'a [TopicPosition], id: &str) -> Option<&'a TopicPosition> {
  positions.iter().find(|p| p.topic_id == id)
}

/// Returns `true` when `term_lc` (already lowercased) appears in the topic title.
fn topic_matches(topic: &Topic, term_lc: &str) -> bool {
  !term_lc.is_empty() && topic.title.to_lowercase().contains(term_lc)
}

/// Try to scroll the `.roadmap-container` div so the matched node is visible.
fn scroll_to_match(topic_id: &str) {
  let window = match web_sys::window() {
    Some(w) => w,
    None => return,
  };
  let document = match window.document() {
    Some(d) => d,
    None => return,
  };

  let selector = format!("[data-topic-id=\"{}\"]", topic_id);
  let element = match document.query_selector(&selector) {
    Ok(Some(el)) => el,
    _ => return,
  };

  // Prefer scrolling inside the horizontal container
  let container = document.query_selector(".roadmap-container").ok().flatten();

  if let Some(c) = container {
    let c_rect = c.get_bounding_client_rect();
    let el_rect = element.get_bounding_client_rect();

    // How far the element's left edge is from the container's left edge in the viewport
    let offset_in_viewport = el_rect.left() - c_rect.left();
    // Current horizontal scroll of the container
    let current_sl = c.scroll_left() as f64;
    // Target: centre the element horizontally with some left margin
    let target_x = (current_sl + offset_in_viewport - 120.0).max(0.0);

    // ScrollToOptions works on Window; use scrollLeft directly via JS eval is tricky.
    // Fallback: scroll the window to at least bring it into the y-axis view.
    let opts = web_sys::ScrollToOptions::new();
    opts.set_behavior(web_sys::ScrollBehavior::Smooth);
    opts.set_top(window.scroll_y().unwrap_or(0.0) + el_rect.top() - 150.0);
    opts.set_left(target_x);
    window.scroll_to_with_scroll_to_options(&opts);
  } else {
    // Plain vertical scroll fallback
    let rect = element.get_bounding_client_rect();
    let opts = web_sys::ScrollToOptions::new();
    opts.set_behavior(web_sys::ScrollBehavior::Smooth);
    opts.set_top(window.scroll_y().unwrap_or(0.0) + rect.top() - 150.0);
    window.scroll_to_with_scroll_to_options(&opts);
  }
}

// ---------------------------------------------------------------------------
// Component
// ---------------------------------------------------------------------------

#[component]
pub fn RoadmapDiagram(props: DiagramData) -> impl IntoView {
  // Pull global state from context (provided by RoadmapPage).
  let state = use_context::<RoadmapState>().expect("RoadmapState context not found");

  let search_term = state.search_term;
  let progress = state.progress;
  let selected_id = state.selected_topic_id;

  // Callback that wires node clicks to the drawer.
  let on_topic_click = Callback::new(move |id: &'static str| {
    selected_id.set(Some(id));
  });

  // Pre-compute lowercased search term once per change.
  let search_lc = Memo::new(move |_| search_term.get().to_lowercase());

  // Snapshot values for use in closures.
  let config = props.config;
  let topics = props.topics;
  let layout_topics = props.layout.topics.clone();
  let layout_groups = props.layout.groups.clone();
  let topics_for_scroll = props.topics;

  // ── Scroll effect ──────────────────────────────────────────────────────
  let last_scrolled: Rc<RefCell<Option<&'static str>>> = Rc::new(RefCell::new(None));

  {
    let last_scrolled = Rc::clone(&last_scrolled);
    Effect::new(move |_| {
      let term = search_lc.get();
      if term.len() < 2 {
        *last_scrolled.borrow_mut() = None;
        return;
      }
      // Find first topic whose title matches.
      let first = topics_for_scroll
        .iter()
        .find(|t| topic_matches(t, &term))
        .map(|t| t.id);

      if let Some(id) = first {
        let mut last = last_scrolled.borrow_mut();
        if last.map(|prev| prev != id).unwrap_or(true) {
          scroll_to_match(id);
          *last = Some(id);
        }
      }
    });
  }

  // ── Build static edge list ─────────────────────────────────────────────
  // Edges are not reactive (positions never change); only class/style varies
  // with state, which is handled on the node side, not the edge side.
  let edge_props: Vec<EdgeData> = props
    .dependencies
    .iter()
    .filter_map(|dep| {
      let from_pos = find_position(&props.layout.topics, dep.from)?;
      let to_pos = find_position(&props.layout.topics, dep.to)?;

      let is_cross = from_pos.section_id != to_pos.section_id;

      let (x1, y1, x2, y2) = if is_cross {
        // Cross-section: exit via right edge of "from", enter via left edge of "to".
        // Groups flow left → right so from_pos is always to the left of to_pos.
        let x1 = from_pos.x + from_pos.width;
        let y1 = from_pos.y + config.node_height / 2.0;
        let x2 = to_pos.x;
        let y2 = to_pos.y + config.node_height / 2.0;
        (x1, y1, x2, y2)
      } else {
        // Intra-section: bottom-centre of "from" → top-centre of "to".
        let x1 = from_pos.x + from_pos.width / 2.0;
        let y1 = from_pos.y + config.node_height;
        let x2 = to_pos.x + to_pos.width / 2.0;
        let y2 = to_pos.y;
        (x1, y1, x2, y2)
      };

      Some(EdgeData {
        from_id: dep.from,
        to_id: dep.to,
        x1,
        y1,
        x2,
        y2,
        is_cross_section: is_cross,
      })
    })
    .collect();

  // ── Static group box views ─────────────────────────────────────────────
  let group_views: Vec<_> = layout_groups
    .iter()
    .map(|g| {
      let gdata = GroupBoxData {
        section_id: g.section_id,
        x: g.x,
        y: g.y,
        width: g.width,
        height: g.height,
        label: g.label,
        header_height: config.header_height,
      };
      view! { <SectionGroup props=gdata /> }
    })
    .collect();

  // ── viewBox ────────────────────────────────────────────────────────────
  let viewbox = format!(
    "{} 0 {} {}",
    props.layout.min_x, props.layout.total_width, props.layout.total_height
  );

  // ── View ───────────────────────────────────────────────────────────────
  let svg_width = format!("{:.0}", props.layout.total_width);
  let svg_height = format!("{:.0}", props.layout.total_height);

  view! {
      <svg
          class="roadmap-diagram roadmap-diagram--horizontal"
          viewBox=viewbox
          width=svg_width
          height=svg_height
          xmlns="http://www.w3.org/2000/svg"
      >
          <ArrowheadMarker />

          // Groups layer — behind edges and nodes
          <g class="groups-layer">
              {group_views}
          </g>

          // Edges layer — static, rendered once
          <g class="edges-layer">
              {edge_props
                  .into_iter()
                  .map(|ep| view! { <RoadmapEdge props=ep /> })
                  .collect_view()}
          </g>

          // Nodes layer — reactive on search + progress
          <g class="nodes-layer">
              {move || {
                  let term = search_lc.get();
                  let prog = progress.get();
                  let has_search = !term.is_empty();

                  layout_topics
                      .iter()
                      .filter_map(|tp| {
                          let topic = find_topic(topics, tp.topic_id)?;
                          let is_highlighted = topic_matches(topic, &term);
                          let is_dimmed = has_search && !is_highlighted;
                          let status = prog
                              .get(topic.id)
                              .copied()
                              .unwrap_or_default();

                          Some(NodeData {
                              id: topic.id,
                              title: topic.title,
                              level: topic.level,
                              topic_type: topic.topic_type,
                              x: tp.x,
                              y: tp.y,
                              width: tp.width,
                              height: config.node_height,
                              on_click: on_topic_click,
                              is_highlighted,
                              is_dimmed,
                              status,
                          })
                      })
                      .map(|nd| view! { <RoadmapNode props=nd /> })
                      .collect_view()
              }}
          </g>
      </svg>
  }
}
