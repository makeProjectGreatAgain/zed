use crate::{
    self as gpui, hsla, point, px, relative, rems, AbsoluteLength, AlignItems, CursorStyle,
    DefiniteLength, Display, Fill, FlexDirection, Hsla, JustifyContent, Length, Position,
    SharedString, StyleRefinement, Visibility, WhiteSpace,
};
use crate::{BoxShadow, TextStyleRefinement};
use smallvec::{smallvec, SmallVec};
use taffy::style::Overflow;

pub trait Styled: Sized {
    fn style(&mut self) -> &mut StyleRefinement;

    gpui2_macros::style_helpers!();

    fn z_index(mut self, z_index: u32) -> Self {
        *self.style().z_index_mut() = Some(z_index);
        self
    }

    /// Sets the size of the element to the full width and height.
    fn full(mut self) -> Self {
        let size = self.style().size_mut();
        size.width = Some(relative(1.).into());
        size.height = Some(relative(1.).into());
        self
    }

    /// Sets the position of the element to `relative`.
    /// [Docs](https://tailwindcss.com/docs/position)
    fn relative(mut self) -> Self {
        *self.style().position_mut() = Position::Relative;
        self
    }

    /// Sets the position of the element to `absolute`.
    /// [Docs](https://tailwindcss.com/docs/position)
    fn absolute(mut self) -> Self {
        *self.style().position_mut() = Position::Absolute;
        self
    }

    /// Sets the display type of the element to `block`.
    /// [Docs](https://tailwindcss.com/docs/display)
    fn block(mut self) -> Self {
        *self.style().display_mut() = Display::Block;
        self
    }

    /// Sets the display type of the element to `flex`.
    /// [Docs](https://tailwindcss.com/docs/display)
    fn flex(mut self) -> Self {
        *self.style().display_mut() = Display::Flex;
        self
    }

    /// Sets the visibility of the element to `visible`.
    /// [Docs](https://tailwindcss.com/docs/visibility)
    fn visible(mut self) -> Self {
        *self.style().visibility_mut() = Visibility::Visible;
        self
    }

    /// Sets the visibility of the element to `hidden`.
    /// [Docs](https://tailwindcss.com/docs/visibility)
    fn invisible(mut self) -> Self {
        *self.style().visibility_mut() = Visibility::Hidden;
        self
    }

    fn overflow_hidden(mut self) -> Self {
        let overflow = self.style().overflow_mut();
        overflow.x = Some(Overflow::Hidden);
        overflow.y = Some(Overflow::Hidden);
        self
    }

    fn overflow_hidden_x(mut self) -> Self {
        self.style().overflow_mut().x = Some(Overflow::Hidden);
        self
    }

    fn overflow_hidden_y(mut self) -> Self {
        self.style().overflow_mut().y = Some(Overflow::Hidden);
        self
    }

    fn cursor(mut self, cursor: CursorStyle) -> Self {
        *self.style().mouse_cursor_mut() = Some(cursor);
        self
    }

    /// Sets the cursor style when hovering an element to `default`.
    /// [Docs](https://tailwindcss.com/docs/cursor)
    fn cursor_default(mut self) -> Self {
        *self.style().mouse_cursor_mut() = Some(CursorStyle::Arrow);
        self
    }

    /// Sets the cursor style when hovering an element to `pointer`.
    /// [Docs](https://tailwindcss.com/docs/cursor)
    fn cursor_pointer(mut self) -> Self {
        *self.style().mouse_cursor_mut() = Some(CursorStyle::PointingHand);
        self
    }

    /// Sets cursor style when hovering over an element to `text`.
    /// [Docs](https://tailwindcss.com/docs/cursor)
    fn cursor_text(mut self) -> Self {
        *self.style().mouse_cursor_mut() = Some(CursorStyle::IBeam);
        self
    }

    /// Sets cursor style when hovering over an element to `move`.
    /// [Docs](https://tailwindcss.com/docs/cursor)
    fn cursor_move(mut self) -> Self {
        *self.style().mouse_cursor_mut() = Some(CursorStyle::ClosedHand);
        self
    }

    /// Sets cursor style when hovering over an element to `not-allowed`.
    /// [Docs](https://tailwindcss.com/docs/cursor)
    fn cursor_not_allowed(mut self) -> Self {
        *self.style().mouse_cursor_mut() = Some(CursorStyle::OperationNotAllowed);
        self
    }

    /// Sets cursor style when hovering over an element to `context-menu`.
    /// [Docs](https://tailwindcss.com/docs/cursor)
    fn cursor_context_menu(mut self) -> Self {
        *self.style().mouse_cursor_mut() = Some(CursorStyle::ContextualMenu);
        self
    }

    /// Sets cursor style when hovering over an element to `crosshair`.
    /// [Docs](https://tailwindcss.com/docs/cursor)
    fn cursor_crosshair(mut self) -> Self {
        *self.style().mouse_cursor_mut() = Some(CursorStyle::Crosshair);
        self
    }

    /// Sets cursor style when hovering over an element to `vertical-text`.
    /// [Docs](https://tailwindcss.com/docs/cursor)
    fn cursor_vertical_text(mut self) -> Self {
        *self.style().mouse_cursor_mut() = Some(CursorStyle::IBeamCursorForVerticalLayout);
        self
    }

    /// Sets cursor style when hovering over an element to `alias`.
    /// [Docs](https://tailwindcss.com/docs/cursor)
    fn cursor_alias(mut self) -> Self {
        *self.style().mouse_cursor_mut() = Some(CursorStyle::DragLink);
        self
    }

    /// Sets cursor style when hovering over an element to `copy`.
    /// [Docs](https://tailwindcss.com/docs/cursor)
    fn cursor_copy(mut self) -> Self {
        *self.style().mouse_cursor_mut() = Some(CursorStyle::DragCopy);
        self
    }

    /// Sets cursor style when hovering over an element to `no-drop`.
    /// [Docs](https://tailwindcss.com/docs/cursor)
    fn cursor_no_drop(mut self) -> Self {
        *self.style().mouse_cursor_mut() = Some(CursorStyle::OperationNotAllowed);
        self
    }

    /// Sets cursor style when hovering over an element to `grab`.
    /// [Docs](https://tailwindcss.com/docs/cursor)
    fn cursor_grab(mut self) -> Self {
        *self.style().mouse_cursor_mut() = Some(CursorStyle::OpenHand);
        self
    }

    /// Sets cursor style when hovering over an element to `grabbing`.
    /// [Docs](https://tailwindcss.com/docs/cursor)
    fn cursor_grabbing(mut self) -> Self {
        *self.style().mouse_cursor_mut() = Some(CursorStyle::ClosedHand);
        self
    }

    /// Sets cursor style when hovering over an element to `col-resize`.
    /// [Docs](https://tailwindcss.com/docs/cursor)
    fn cursor_col_resize(mut self) -> Self {
        *self.style().mouse_cursor_mut() = Some(CursorStyle::ResizeLeftRight);
        self
    }

    /// Sets cursor style when hovering over an element to `row-resize`.
    /// [Docs](https://tailwindcss.com/docs/cursor)
    fn cursor_row_resize(mut self) -> Self {
        *self.style().mouse_cursor_mut() = Some(CursorStyle::ResizeUpDown);
        self
    }

    /// Sets cursor style when hovering over an element to `n-resize`.
    /// [Docs](https://tailwindcss.com/docs/cursor)
    fn cursor_n_resize(mut self) -> Self {
        *self.style().mouse_cursor_mut() = Some(CursorStyle::ResizeUp);
        self
    }

    /// Sets cursor style when hovering over an element to `e-resize`.
    /// [Docs](https://tailwindcss.com/docs/cursor)
    fn cursor_e_resize(mut self) -> Self {
        *self.style().mouse_cursor_mut() = Some(CursorStyle::ResizeRight);
        self
    }

    /// Sets cursor style when hovering over an element to `s-resize`.
    /// [Docs](https://tailwindcss.com/docs/cursor)
    fn cursor_s_resize(mut self) -> Self {
        *self.style().mouse_cursor_mut() = Some(CursorStyle::ResizeDown);
        self
    }

    /// Sets cursor style when hovering over an element to `w-resize`.
    /// [Docs](https://tailwindcss.com/docs/cursor)
    fn cursor_w_resize(mut self) -> Self {
        *self.style().mouse_cursor_mut() = Some(CursorStyle::ResizeLeft);
        self
    }

    /// Sets the whitespace of the element to `normal`.
    /// [Docs](https://tailwindcss.com/docs/whitespace#normal)
    fn whitespace_normal(mut self) -> Self {
        self.text_style().white_space = Some(WhiteSpace::Normal);
        self
    }

    /// Sets the whitespace of the element to `nowrap`.
    /// [Docs](https://tailwindcss.com/docs/whitespace#nowrap)
    fn whitespace_nowrap(mut self) -> Self {
        self.text_style().white_space = Some(WhiteSpace::Nowrap);
        self
    }

    /// Sets the flex direction of the element to `column`.
    /// [Docs](https://tailwindcss.com/docs/flex-direction#column)
    fn flex_col(mut self) -> Self {
        *self.style().flex_direction_mut() = FlexDirection::Column;
        self
    }

    /// Sets the flex direction of the element to `column-reverse`.
    /// [Docs](https://tailwindcss.com/docs/flex-direction#column-reverse)
    fn flex_col_reverse(mut self) -> Self {
        *self.style().flex_direction_mut() = FlexDirection::ColumnReverse;
        self
    }

    /// Sets the flex direction of the element to `row`.
    /// [Docs](https://tailwindcss.com/docs/flex-direction#row)
    fn flex_row(mut self) -> Self {
        *self.style().flex_direction_mut() = FlexDirection::Row;
        self
    }

    /// Sets the flex direction of the element to `row-reverse`.
    /// [Docs](https://tailwindcss.com/docs/flex-direction#row-reverse)
    fn flex_row_reverse(mut self) -> Self {
        *self.style().flex_direction_mut() = FlexDirection::RowReverse;
        self
    }

    /// Sets the element to allow a flex item to grow and shrink as needed, ignoring its initial size.
    /// [Docs](https://tailwindcss.com/docs/flex#flex-1)
    fn flex_1(mut self) -> Self {
        *self.style().flex_grow_mut() = 1.;
        *self.style().flex_shrink_mut() = 1.;
        *self.style().flex_basis_mut() = relative(0.).into();
        self
    }

    /// Sets the element to allow a flex item to grow and shrink, taking into account its initial size.
    /// [Docs](https://tailwindcss.com/docs/flex#auto)
    fn flex_auto(mut self) -> Self {
        *self.style().flex_grow_mut() = 1.;
        *self.style().flex_shrink_mut() = 1.;
        *self.style().flex_basis_mut() = Length::Auto;
        self
    }

    /// Sets the element to allow a flex item to shrink but not grow, taking into account its initial size.
    /// [Docs](https://tailwindcss.com/docs/flex#initial)
    fn flex_initial(mut self) -> Self {
        *self.style().flex_grow_mut() = 0.;
        *self.style().flex_shrink_mut() = 1.;
        *self.style().flex_basis_mut() = Length::Auto;
        self
    }

    /// Sets the element to prevent a flex item from growing or shrinking.
    /// [Docs](https://tailwindcss.com/docs/flex#none)
    fn flex_none(mut self) -> Self {
        *self.style().flex_grow_mut() = 0.;
        *self.style().flex_shrink_mut() = 0.;
        self
    }

    /// Sets the element to allow a flex item to grow to fill any available space.
    /// [Docs](https://tailwindcss.com/docs/flex-grow)
    fn flex_grow(mut self) -> Self {
        *self.style().flex_grow_mut() = 1.;
        self
    }

    /// Sets the element to align flex items to the start of the container's cross axis.
    /// [Docs](https://tailwindcss.com/docs/align-items#start)
    fn items_start(mut self) -> Self {
        *self.style().align_items_mut() = Some(AlignItems::FlexStart);
        self
    }

    /// Sets the element to align flex items to the end of the container's cross axis.
    /// [Docs](https://tailwindcss.com/docs/align-items#end)
    fn items_end(mut self) -> Self {
        *self.style().align_items_mut() = Some(AlignItems::FlexEnd);
        self
    }

    /// Sets the element to align flex items along the center of the container's cross axis.
    /// [Docs](https://tailwindcss.com/docs/align-items#center)
    fn items_center(mut self) -> Self {
        *self.style().align_items_mut() = Some(AlignItems::Center);
        self
    }

    /// Sets the element to justify flex items along the container's main axis
    /// such that there is an equal amount of space between each item.
    /// [Docs](https://tailwindcss.com/docs/justify-content#space-between)
    fn justify_between(mut self) -> Self {
        *self.style().justify_content_mut() = Some(JustifyContent::SpaceBetween);
        self
    }

    /// Sets the element to justify flex items along the center of the container's main axis.
    /// [Docs](https://tailwindcss.com/docs/justify-content#center)
    fn justify_center(mut self) -> Self {
        *self.style().justify_content_mut() = Some(JustifyContent::Center);
        self
    }

    /// Sets the element to justify flex items against the start of the container's main axis.
    /// [Docs](https://tailwindcss.com/docs/justify-content#start)
    fn justify_start(mut self) -> Self {
        *self.style().justify_content_mut() = Some(JustifyContent::Start);
        self
    }

    /// Sets the element to justify flex items against the end of the container's main axis.
    /// [Docs](https://tailwindcss.com/docs/justify-content#end)
    fn justify_end(mut self) -> Self {
        *self.style().justify_content_mut() = Some(JustifyContent::End);
        self
    }

    /// Sets the element to justify items along the container's main axis such
    /// that there is an equal amount of space on each side of each item.
    /// [Docs](https://tailwindcss.com/docs/justify-content#space-around)
    fn justify_around(mut self) -> Self {
        *self.style().justify_content_mut() = Some(JustifyContent::SpaceAround);
        self
    }

    /// Sets the background color of the element.
    fn bg<F>(mut self, fill: F) -> Self
    where
        F: Into<Fill>,
        Self: Sized,
    {
        *self.style().background_mut() = Some(fill.into());
        self
    }

    /// Sets the border color of the element.
    fn border_color<C>(mut self, border_color: C) -> Self
    where
        C: Into<Hsla>,
        Self: Sized,
    {
        *self.style().border_color_mut() = Some(border_color.into());
        self
    }

    /// Sets the box shadow of the element.
    /// [Docs](https://tailwindcss.com/docs/box-shadow)
    fn shadow(mut self, shadows: SmallVec<[BoxShadow; 2]>) -> Self {
        *self.style().box_shadow_mut() = shadows;
        self
    }

    /// Clears the box shadow of the element.
    /// [Docs](https://tailwindcss.com/docs/box-shadow)
    fn shadow_none(mut self) -> Self {
        *self.style().box_shadow_mut() = Default::default();
        self
    }

    /// Sets the box shadow of the element.
    /// [Docs](https://tailwindcss.com/docs/box-shadow)
    fn shadow_sm(mut self) -> Self {
        *self.style().box_shadow_mut() = smallvec::smallvec![BoxShadow {
            color: hsla(0., 0., 0., 0.05),
            offset: point(px(0.), px(1.)),
            blur_radius: px(2.),
            spread_radius: px(0.),
        }];
        self
    }

    /// Sets the box shadow of the element.
    /// [Docs](https://tailwindcss.com/docs/box-shadow)
    fn shadow_md(mut self) -> Self {
        *self.style().box_shadow_mut() = smallvec![
            BoxShadow {
                color: hsla(0.5, 0., 0., 0.1),
                offset: point(px(0.), px(4.)),
                blur_radius: px(6.),
                spread_radius: px(-1.),
            },
            BoxShadow {
                color: hsla(0., 0., 0., 0.1),
                offset: point(px(0.), px(2.)),
                blur_radius: px(4.),
                spread_radius: px(-2.),
            }
        ];
        self
    }

    /// Sets the box shadow of the element.
    /// [Docs](https://tailwindcss.com/docs/box-shadow)
    fn shadow_lg(mut self) -> Self {
        *self.style().box_shadow_mut() = smallvec![
            BoxShadow {
                color: hsla(0., 0., 0., 0.1),
                offset: point(px(0.), px(10.)),
                blur_radius: px(15.),
                spread_radius: px(-3.),
            },
            BoxShadow {
                color: hsla(0., 0., 0., 0.1),
                offset: point(px(0.), px(4.)),
                blur_radius: px(6.),
                spread_radius: px(-4.),
            }
        ];
        self
    }

    /// Sets the box shadow of the element.
    /// [Docs](https://tailwindcss.com/docs/box-shadow)
    fn shadow_xl(mut self) -> Self {
        *self.style().box_shadow_mut() = smallvec![
            BoxShadow {
                color: hsla(0., 0., 0., 0.1),
                offset: point(px(0.), px(20.)),
                blur_radius: px(25.),
                spread_radius: px(-5.),
            },
            BoxShadow {
                color: hsla(0., 0., 0., 0.1),
                offset: point(px(0.), px(8.)),
                blur_radius: px(10.),
                spread_radius: px(-6.),
            }
        ];
        self
    }

    /// Sets the box shadow of the element.
    /// [Docs](https://tailwindcss.com/docs/box-shadow)
    fn shadow_2xl(mut self) -> Self {
        *self.style().box_shadow_mut() = smallvec![BoxShadow {
            color: hsla(0., 0., 0., 0.25),
            offset: point(px(0.), px(25.)),
            blur_radius: px(50.),
            spread_radius: px(-12.),
        }];
        self
    }

    fn text_style(&mut self) -> &mut TextStyleRefinement {
        let style: &mut StyleRefinement = self.style();
        style.text_mut()
    }

    fn text_color(mut self, color: impl Into<Hsla>) -> Self {
        self.text_style().color = Some(color.into());
        self
    }

    fn text_bg(mut self, bg: impl Into<Hsla>) -> Self {
        self.text_style().background_color = Some(bg.into());
        self
    }

    fn text_size(mut self, size: impl Into<AbsoluteLength>) -> Self {
        self.text_style().font_size = Some(size.into());
        self
    }

    fn text_xs(mut self) -> Self {
        self.text_style().font_size = Some(rems(0.75).into());
        self
    }

    fn text_sm(mut self) -> Self {
        self.text_style().font_size = Some(rems(0.875).into());
        self
    }

    fn text_base(mut self) -> Self {
        self.text_style().font_size = Some(rems(1.0).into());
        self
    }

    fn text_lg(mut self) -> Self {
        self.text_style().font_size = Some(rems(1.125).into());
        self
    }

    fn text_xl(mut self) -> Self {
        self.text_style().font_size = Some(rems(1.25).into());
        self
    }

    fn text_2xl(mut self) -> Self {
        self.text_style().font_size = Some(rems(1.5).into());
        self
    }

    fn text_3xl(mut self) -> Self {
        self.text_style().font_size = Some(rems(1.875).into());
        self
    }

    fn text_decoration_none(mut self) -> Self {
        self.text_style().underline = None;
        self
    }

    fn text_decoration_color(mut self, color: impl Into<Hsla>) -> Self {
        let underline = self
            .text_style()
            .underline
            .get_or_insert_with(Default::default);
        underline.color = Some(color.into());
        self
    }

    fn text_decoration_solid(mut self) -> Self {
        let underline = self
            .text_style()
            .underline
            .get_or_insert_with(Default::default);
        underline.wavy = false;
        self
    }

    fn text_decoration_wavy(mut self) -> Self {
        let underline = self
            .text_style()
            .underline
            .get_or_insert_with(Default::default);
        underline.wavy = true;
        self
    }

    fn text_decoration_0(mut self) -> Self {
        let underline = self
            .text_style()
            .underline
            .get_or_insert_with(Default::default);
        underline.thickness = px(0.);
        self
    }

    fn text_decoration_1(mut self) -> Self {
        let underline = self
            .text_style()
            .underline
            .get_or_insert_with(Default::default);
        underline.thickness = px(1.);
        self
    }

    fn text_decoration_2(mut self) -> Self {
        let underline = self
            .text_style()
            .underline
            .get_or_insert_with(Default::default);
        underline.thickness = px(2.);
        self
    }

    fn text_decoration_4(mut self) -> Self {
        let underline = self
            .text_style()
            .underline
            .get_or_insert_with(Default::default);
        underline.thickness = px(4.);
        self
    }

    fn text_decoration_8(mut self) -> Self {
        let underline = self
            .text_style()
            .underline
            .get_or_insert_with(Default::default);
        underline.thickness = px(8.);
        self
    }

    fn font(mut self, family_name: impl Into<SharedString>) -> Self {
        self.text_style().font_family = Some(family_name.into());
        self
    }

    fn line_height(mut self, line_height: impl Into<DefiniteLength>) -> Self {
        self.text_style().line_height = Some(line_height.into());
        self
    }
}
