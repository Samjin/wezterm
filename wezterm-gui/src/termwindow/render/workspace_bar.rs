use crate::quad::TripleLayerQuadAllocator;
use crate::termwindow::render::RenderScreenLineParams;
use crate::termwindow::{UIItem, UIItemType, WorkspaceBarItem};
use config::{TabBarColor, WorkspaceBarColors};
use mux::renderable::RenderableDimensions;
use mux::Mux;
use termwiz::cell::CellAttributes;
use wezterm_term::color::ColorAttribute;
use wezterm_term::Line;
use window::color::LinearRgba;

const WORKSPACE_BAR_WIDTH: usize = 220;
const HEADER_HEIGHT: usize = 48;
const ROW_HEIGHT: usize = 36;
const HORIZONTAL_PADDING: usize = 20;

impl crate::TermWindow {
    pub fn workspace_bar_width_static() -> usize {
        WORKSPACE_BAR_WIDTH
    }

    pub fn workspace_bar_width(&self) -> usize {
        Self::workspace_bar_width_static()
    }

    pub fn paint_workspace_bar(
        &mut self,
        layers: &mut TripleLayerQuadAllocator,
    ) -> anyhow::Result<()> {
        let colors = self
            .config
            .colors
            .as_ref()
            .and_then(|colors| colors.workspace_bar.as_ref())
            .cloned()
            .unwrap_or_else(WorkspaceBarColors::default);
        let border = self.get_os_border();
        let tab_bar_height = if self.show_tab_bar {
            self.tab_bar_pixel_height()? as usize
        } else {
            0
        };
        let top = border.top.get() as usize
            + if self.config.tab_bar_at_bottom {
                0
            } else {
                tab_bar_height
            };
        let bottom = self.dimensions.pixel_height.saturating_sub(
            border.bottom.get() as usize
                + if self.config.tab_bar_at_bottom {
                    tab_bar_height
                } else {
                    0
                },
        );
        let height = bottom.saturating_sub(top);

        self.filled_rectangle(
            layers,
            1,
            euclid::rect(0., top as f32, WORKSPACE_BAR_WIDTH as f32, height as f32),
            colors.background().to_linear(),
        )?;
        self.filled_rectangle(
            layers,
            1,
            euclid::rect(
                (WORKSPACE_BAR_WIDTH - 1) as f32,
                top as f32,
                1.,
                height as f32,
            ),
            colors.inactive_workspace().bg_color.to_linear(),
        )?;

        let header = header_attributes(&colors);
        self.paint_workspace_bar_text(layers, top + 14, HORIZONTAL_PADDING, "Spaces", &header)?;
        self.paint_workspace_bar_text(
            layers,
            top + 14,
            WORKSPACE_BAR_WIDTH
                .saturating_sub(HORIZONTAL_PADDING + self.render_metrics.cell_size.width as usize),
            "+",
            &header,
        )?;
        self.ui_items.push(UIItem {
            x: WORKSPACE_BAR_WIDTH - HEADER_HEIGHT,
            y: top,
            width: HEADER_HEIGHT,
            height: HEADER_HEIGHT,
            item_type: UIItemType::WorkspaceBar(WorkspaceBarItem::NewWorkspace),
        });

        let mux = Mux::get();
        let active_workspace = mux.active_workspace();
        for (idx, workspace) in mux.iter_workspaces().into_iter().enumerate() {
            let row_top = top + HEADER_HEIGHT + idx * ROW_HEIGHT;
            if row_top + ROW_HEIGHT > bottom {
                break;
            }
            let hovered = self.current_mouse_event.is_some()
                && matches!(
                    self.last_ui_item.as_ref().map(|item| &item.item_type),
                    Some(UIItemType::WorkspaceBar(WorkspaceBarItem::Workspace(name))) if name == &workspace
                );
            let style = if workspace == active_workspace {
                colors.active_workspace()
            } else if hovered {
                colors.inactive_workspace_hover()
            } else {
                colors.inactive_workspace()
            };
            self.filled_rectangle(
                layers,
                1,
                euclid::rect(
                    8.,
                    row_top as f32,
                    (WORKSPACE_BAR_WIDTH - 16) as f32,
                    ROW_HEIGHT as f32,
                ),
                style.bg_color.to_linear(),
            )?;
            self.paint_workspace_bar_text(
                layers,
                row_top + 9,
                HORIZONTAL_PADDING,
                &format!("•  {workspace}"),
                &style.as_cell_attributes(),
            )?;
            self.ui_items.push(UIItem {
                x: 8,
                y: row_top,
                width: WORKSPACE_BAR_WIDTH - 16,
                height: ROW_HEIGHT,
                item_type: UIItemType::WorkspaceBar(WorkspaceBarItem::Workspace(workspace)),
            });
        }

        Ok(())
    }

    fn paint_workspace_bar_text(
        &self,
        layers: &mut TripleLayerQuadAllocator,
        top: usize,
        left: usize,
        text: &str,
        attrs: &CellAttributes,
    ) -> anyhow::Result<()> {
        let palette = self.palette().clone();
        let line = Line::from_text(text, attrs, 0, None);
        let gl_state = self.render_state.as_ref().unwrap();
        let window_is_transparent =
            !self.window_background.is_empty() || self.config.window_background_opacity != 1.0;
        self.render_screen_line(
            RenderScreenLineParams {
                top_pixel_y: top as f32,
                left_pixel_x: left as f32,
                pixel_width: WORKSPACE_BAR_WIDTH.saturating_sub(left + HORIZONTAL_PADDING) as f32,
                stable_line_idx: None,
                line: &line,
                selection: 0..0,
                cursor: &Default::default(),
                palette: &palette,
                dims: &RenderableDimensions {
                    cols: (WORKSPACE_BAR_WIDTH / self.render_metrics.cell_size.width as usize)
                        .max(1),
                    physical_top: 0,
                    scrollback_rows: 0,
                    scrollback_top: 0,
                    viewport_rows: 1,
                    dpi: self.terminal_size.dpi,
                    pixel_height: self.render_metrics.cell_size.height as usize,
                    pixel_width: WORKSPACE_BAR_WIDTH,
                    reverse_video: false,
                },
                config: &self.config,
                cursor_border_color: LinearRgba::default(),
                foreground: palette.foreground.to_linear(),
                pane: None,
                is_active: true,
                selection_fg: LinearRgba::default(),
                selection_bg: LinearRgba::default(),
                cursor_fg: LinearRgba::default(),
                cursor_bg: LinearRgba::default(),
                cursor_is_default_color: true,
                white_space: gl_state.util_sprites.white_space.texture_coords(),
                filled_box: gl_state.util_sprites.filled_box.texture_coords(),
                window_is_transparent,
                default_bg: palette
                    .resolve_bg(ColorAttribute::Default)
                    .to_linear()
                    .mul_alpha(if window_is_transparent {
                        0.
                    } else {
                        self.config.text_background_opacity
                    }),
                style: None,
                font: None,
                use_pixel_positioning: self.config.experimental_pixel_positioning,
                render_metrics: self.render_metrics,
                shape_key: None,
                password_input: false,
            },
            layers,
        )?;
        Ok(())
    }
}

fn header_attributes(colors: &WorkspaceBarColors) -> CellAttributes {
    TabBarColor {
        bg_color: colors.background(),
        fg_color: colors.foreground(),
        intensity: wezterm_term::Intensity::Bold,
        ..TabBarColor::default()
    }
    .as_cell_attributes()
}
