use basecoat_core::{Markup, TabsProps, attrs::escape_attr, classes};

/// Renders a tabs component with tablist nav + optional tab panels.
///
/// Upstream HTML structure:
/// ```html
/// <div
///   class="tabs"
///   id="{id}"
///   data-tabs
///   data-basecoat-hydrate="tabs"
///   data-basecoat-version="0.1"
/// >
///   <nav role="tablist" aria-orientation="horizontal">
///     <button
///       type="button" role="tab"
///       id="{id}-tab-1"
///       aria-controls="{id}-panel-1"
///       aria-selected="true"
///       tabindex="0"
///     >Tab label</button>
///     ...
///   </nav>
///   <div
///     role="tabpanel"
///     id="{id}-panel-1"
///     aria-labelledby="{id}-tab-1"
///     tabindex="-1"
///     aria-selected="true"
///   >Panel content</div>
///   ...
/// </div>
/// ```
pub fn tabs(props: TabsProps) -> Markup {
    let id = props.id.as_deref().unwrap_or("tabs-1").to_owned();
    let class = classes::tabs(&props);
    let orientation = props.orientation.to_string();
    let main_attrs = props.attrs.render();
    let default_idx = props.default_tab_index;

    let mut html = String::new();

    html.push_str(&format!(
        r#"<div class="{class}" id="{eid}" data-tabs data-basecoat-hydrate="tabs" data-basecoat-version="0.1"{main_attrs}>"#,
        class = class,
        eid = escape_attr(&id),
        main_attrs = main_attrs,
    ));

    // Nav / tablist
    html.push_str(&format!(
        r#"<nav role="tablist" aria-orientation="{orientation}">"#
    ));

    for (i, tabset) in props.tabsets.iter().enumerate() {
        let idx = i + 1;
        let selected = if idx == default_idx { "true" } else { "false" };
        html.push_str(&format!(
            r#"<button type="button" role="tab" id="{eid}-tab-{idx}" aria-controls="{eid}-panel-{idx}" aria-selected="{selected}" tabindex="0">{tab}</button>"#,
            eid = escape_attr(&id),
            idx = idx,
            selected = selected,
            tab = tabset.tab,
        ));
    }

    html.push_str("</nav>");

    // Panels
    for (i, tabset) in props.tabsets.iter().enumerate() {
        let idx = i + 1;
        if let Some(panel) = &tabset.panel {
            let selected = if idx == default_idx { "true" } else { "false" };
            let hidden = if idx != default_idx { " hidden" } else { "" };
            html.push_str(&format!(
                r#"<div role="tabpanel" id="{eid}-panel-{idx}" aria-labelledby="{eid}-tab-{idx}" tabindex="-1" aria-selected="{selected}"{hidden}>{panel}</div>"#,
                eid = escape_attr(&id),
                idx = idx,
                selected = selected,
                hidden = hidden,
                panel = panel,
            ));
        }
    }

    html.push_str("</div>");
    Markup::from(html)
}
