//! Section group box SVG component.

use leptos::prelude::*;

/// Data for rendering a single section group box.
#[derive(Clone, Debug, PartialEq)]
pub struct GroupBoxData {
  pub section_id: &'static str,
  pub x: f64,
  pub y: f64,
  pub width: f64,
  pub height: f64,
  pub label: &'static str,
  pub header_height: f64,
}

/// Renders an SVG group box with a coloured header bar and a title label.
///
/// Structure:
/// ```text
/// ┌─────────────────────┐  ← header bar (orange tint)
/// │  SECTION TITLE      │
/// ├─────────────────────┤
/// │  [Topic Node]       │
/// │  [Topic Node]       │
/// │  ...                │
/// └─────────────────────┘
/// ```
#[component]
pub fn SectionGroup(props: GroupBoxData) -> impl IntoView {
  let x = props.x;
  let y = props.y;
  let w = props.width;
  let h = props.height;
  let hh = props.header_height;

  view! {
      <g class="section-group" data-section-id=props.section_id>
          // Full background rect
          <rect
              x=x
              y=y
              width=w
              height=h
              rx="6"
              ry="6"
              class="section-group__bg"
          />
          // Header bar — rounded top corners only.
          // Step 1: draw a rect with all corners rounded (matches the bg).
          <rect
              x=x
              y=y
              width=w
              height=hh
              rx="6"
              ry="6"
              class="section-group__header"
          />
          // Step 2: square off the bottom half of the header's rounded corners
          // by overlaying a plain rect over the lower portion.
          <rect
              x=x
              y=y + hh * 0.5
              width=w
              height=hh * 0.5
              class="section-group__header"
          />
          // Title label centred in the header bar
          <text
              x=x + w * 0.5
              y=y + hh * 0.5
              class="section-group__title"
              text-anchor="middle"
              dominant-baseline="central"
          >
              {props.label}
          </text>
      </g>
  }
}
