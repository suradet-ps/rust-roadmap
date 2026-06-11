use leptos::prelude::*;

#[component]
pub fn Hero() -> impl IntoView {
  view! {
      <div class="hero">
          <div class="hero__badge">
              <span class="hero__badge-pulse">
                  <span class="hero__badge-ping"></span>
                  <span class="hero__badge-dot"></span>
              </span>
              <span class="hero__badge-text">"v1.0.0 Stable"</span>
          </div>

          <h2 class="hero__title">
              "Master the "
              <span class="text-gradient">"System"</span>
          </h2>

          <p class="hero__subtitle">
              "An interactive, type-safe path to Rust mastery."
              <br class="hero__subtitle-break"/>
              "Designed for clarity, built for speed."
          </p>
      </div>
  }
}
