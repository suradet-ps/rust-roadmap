//! SVG node component for rendering roadmap topics.

use crate::models::roadmap::{Level, NodeStatus, TopicType};
use leptos::prelude::*;

#[derive(Clone, Debug)]
pub struct NodeData {
  pub id: &'static str,
  pub title: &'static str,
  pub level: Level,
  pub topic_type: TopicType,
  pub x: f64,
  pub y: f64,
  pub width: f64,
  pub height: f64,
  pub on_click: Callback<&'static str>,
  pub is_highlighted: bool,
  pub is_dimmed: bool,
  pub status: NodeStatus,
}

#[component]
pub fn RoadmapNode(props: NodeData) -> impl IntoView {
  let type_class = match props.topic_type {
    TopicType::Main => "type-main",
    TopicType::Sub => "type-sub",
  };

  let status_class = match props.status {
    NodeStatus::Done => " node-done",
    NodeStatus::InProgress => " node-in-progress",
    NodeStatus::Skipped => " node-skipped",
    NodeStatus::Untouched => "",
  };

  let highlight_class = if props.is_highlighted {
    " node-highlighted"
  } else {
    ""
  };

  let dimmed_class = if props.is_dimmed { " node-dimmed" } else { "" };

  let class_attr = format!(
    "roadmap-node {}{}{}{}",
    type_class, status_class, highlight_class, dimmed_class
  );

  let cursor_style = if props.is_dimmed {
    "cursor: default; pointer-events: none;"
  } else {
    "cursor: pointer;"
  };

  let show_checkmark = props.status == NodeStatus::Done;
  let show_accent = props.status == NodeStatus::InProgress;

  let x = props.x;
  let y = props.y;
  let w = props.width;
  let h = props.height;
  let text_x = x + w / 2.0;
  let text_y = y + h / 2.0;
  let checkmark_x = x + w - 10.0;
  let checkmark_y = y + 10.0;

  view! {
      <g
          class=class_attr
          data-topic-id=props.id
          on:click=move |_| props.on_click.run(props.id)
          style=cursor_style
      >
          <rect
              x=x
              y=y
              width=w
              height=h
              rx="4"
              ry="4"
              class="node-rect"
          />

          // InProgress: left accent bar
          {show_accent.then(|| view! {
              <rect
                  x=x
                  y=y
                  width=4
                  height=h
                  rx="2"
                  ry="2"
                  class="node-status-accent"
              />
          })}

          <text
              x=text_x
              y=text_y
              class="node-text"
              text-anchor="middle"
              dominant-baseline="central"
          >
              {props.title}
          </text>

          // Done: checkmark icon in top-right corner
          {show_checkmark.then(|| view! {
              <text
                  x=checkmark_x
                  y=checkmark_y
                  class="node-status-icon"
                  text-anchor="middle"
                  dominant-baseline="central"
              >
                  "✓"
              </text>
          })}
      </g>
  }
}
