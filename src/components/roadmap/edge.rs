use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct EdgeData {
  pub from_id: &'static str,
  pub to_id: &'static str,
  pub x1: f64,
  pub y1: f64,
  pub x2: f64,
  pub y2: f64,
  pub is_cross_section: bool,
}

#[component]
pub fn RoadmapEdge(props: EdgeData) -> impl IntoView {
  let class_attr = if props.is_cross_section {
    "roadmap-edge edge-cross-section"
  } else {
    "roadmap-edge"
  };

  // Edge routing logic based on connection type
  let path_d = if (props.y1 - props.y2).abs() < 1.0 {
    // Same row (horizontal branch connection) → Straight horizontal line
    format!("M {} {} L {} {}", props.x1, props.y1, props.x2, props.y2)
  } else if (props.x1 - props.x2).abs() < 1.0 {
    // Same column (vertical spine) → Straight vertical line
    format!("M {} {} L {} {}", props.x1, props.y1, props.x2, props.y2)
  } else {
    // Mixed: horizontal mid-step, then vertical, then horizontal to target
    // This avoids the line "sliding" along the face of the target node.
    let mid_x = (props.x1 + props.x2) / 2.0;
    format!(
      "M {} {} L {} {} L {} {} L {} {}",
      props.x1, props.y1, mid_x, props.y1, mid_x, props.y2, props.x2, props.y2
    )
  };

  view! {
      <path class=class_attr d=path_d fill="none" marker-end="url(#arrowhead)" />
  }
}

#[component]
pub fn ArrowheadMarker() -> impl IntoView {
  view! {
      <defs>
          <marker id="arrowhead" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
              <polygon points="0 0, 10 3.5, 0 7" class="arrowhead-fill" />
          </marker>
      </defs>
  }
}
