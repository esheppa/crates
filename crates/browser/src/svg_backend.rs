// code repurposed from: https://github.com/plotters-rs/plotters/blob/master/plotters-svg/src/svg.rs
// MIT License

// Copyright (c) 2019-2021 Hao Hou <haohou302@gmail.com>

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use super::DOCUMENT;

use plotters_backend::{
    text_anchor::{HPos, VPos},
    BackendColor, BackendCoord, BackendStyle, BackendTextStyle, DrawingBackend, DrawingErrorKind,
    FontStyle, FontTransform,
};

use std::{error::Error, fmt::Display};
use uuid::Uuid;
use wasm_bindgen::{JsValue, UnwrapThrowExt};

use web_sys::{Element, HtmlElement};

#[derive(Debug)]
pub struct SvgJsError {
    source: String,
}

impl Display for SvgJsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JsError: {}", self.source)
    }
}

impl Error for SvgJsError {}

impl From<JsValue> for SvgJsError {
    fn from(v: JsValue) -> Self {
        SvgJsError {
            source: format!("{v:?}"),
        }
    }
}

type Result<T> = std::result::Result<T, DrawingErrorKind<SvgJsError>>;

fn make_svg_color(color: BackendColor) -> String {
    let (r, g, b) = color.rgb;
    return format!("#{:02X}{:02X}{:02X}", r, g, b);
}

fn make_svg_opacity(color: BackendColor) -> String {
    return format!("{}", color.alpha);
}

/// The SVG image drawing backend
pub struct SVGBackend {
    target: HtmlElement,
    size: (u32, u32),
    tag_stack: Vec<Element>,
    id: String,
    base_el: Element,
}

impl SVGBackend {
    /// Create a new SVG drawing backend
    pub fn new(size: (u32, u32), target: &HtmlElement) -> Self {
        let id = Uuid::new_v4().as_u128().to_string();
        let el = DOCUMENT
            .with(|d| d.create_element_ns(Some("http://www.w3.org/2000/svg"), "svg"))
            .unwrap_throw();

        // convenient in cases where we want to use the element to size the SVG...
        // still need to handle the height-width ratio though
        el.set_attribute("style", "height: 100%; width: 100%;")
            .unwrap_throw();

        el.set_attribute("width", &size.0.to_string())
            .unwrap_throw();
        el.set_attribute("height", &size.1.to_string())
            .unwrap_throw();
        el.set_attribute("viewBox", &format!("0 0 {} {}", size.0, size.1))
            .unwrap_throw();

        el.set_id(&id);

        Self {
            target: target.clone(),
            size,
            tag_stack: vec![],
            base_el: el,
            id,
        }
    }

    pub fn fill(&mut self, arg: ()) -> Result<()> {
        // TODO
        Ok(())
    }
}

impl DrawingBackend for SVGBackend {
    type ErrorType = SvgJsError;

    fn get_size(&self) -> (u32, u32) {
        self.size
    }

    fn ensure_prepared(&mut self) -> Result<()> {
        Ok(())
    }

    fn present(&mut self) -> Result<()> {
        // add all desired tags
        for t in self.tag_stack.drain(..) {
            // gloo::console::log!("appending child");

            self.base_el
                .append_child(&t)
                .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?;
        }

        match self
            .target
            .query_selector(&format!("#id{}", self.id))
            .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?
        {
            Some(_) => {
                // do nothing
                // gloo::console::log!("exists");
            }
            None => {
                // attach to DOM!
                // gloo::console::log!("new");

                self.target
                    .append_child(&self.base_el)
                    .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?;
            }
        }

        Ok(())
    }

    fn draw_pixel(&mut self, point: BackendCoord, color: BackendColor) -> Result<()> {
        if color.alpha == 0.0 {
            return Ok(());
        }

        let el = DOCUMENT
            .with(|d| d.create_element_ns(Some("http://www.w3.org/2000/svg"), "rect"))
            .unwrap_throw();

        el.set_attribute("x", &format!("{}", point.0))
            .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?;
        el.set_attribute("y", &format!("{}", point.1))
            .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?;
        el.set_attribute("width", "1")
            .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?;
        el.set_attribute("height", "1")
            .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?;
        el.set_attribute("stroke", "none")
            .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?;
        el.set_attribute("opacity", &make_svg_opacity(color))
            .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?;
        el.set_attribute("fill", &make_svg_color(color))
            .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?;

        self.tag_stack.push(el);

        Ok(())
    }

    fn draw_line<S: BackendStyle>(
        &mut self,
        from: BackendCoord,
        to: BackendCoord,
        style: &S,
    ) -> Result<()> {
        if style.color().alpha == 0.0 {
            return Ok(());
        }

        let el = DOCUMENT
            .with(|d| d.create_element_ns(Some("http://www.w3.org/2000/svg"), "line"))
            .unwrap_throw();

        el.set_attribute("opacity", &make_svg_opacity(style.color()))
            .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?;
        el.set_attribute("stroke", &make_svg_color(style.color()))
            .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?;
        el.set_attribute("stroke-width", &format!("{}", style.stroke_width()))
            .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?;
        el.set_attribute("x1", &format!("{}", from.0))
            .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?;
        el.set_attribute("y1", &format!("{}", from.1))
            .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?;
        el.set_attribute("x2", &format!("{}", to.0))
            .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?;
        el.set_attribute("y2", &format!("{}", to.1))
            .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?;

        self.tag_stack.push(el);

        Ok(())
    }

    fn draw_rect<S: BackendStyle>(
        &mut self,
        upper_left: BackendCoord,
        bottom_right: BackendCoord,
        style: &S,
        fill: bool,
    ) -> Result<()> {
        if style.color().alpha == 0.0 {
            return Ok(());
        }

        let el = DOCUMENT
            .with(|d| d.create_element_ns(Some("http://www.w3.org/2000/svg"), "rect"))
            .unwrap_throw();

        let (fill, stroke) = if !fill {
            ("none".to_string(), make_svg_color(style.color()))
        } else {
            (make_svg_color(style.color()), "none".to_string())
        };

        el.set_attribute("x", &format!("{}", upper_left.0))
            .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?;
        el.set_attribute("y", &format!("{}", upper_left.1))
            .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?;
        el.set_attribute("width", &format!("{}", bottom_right.0 - upper_left.0))
            .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?;
        el.set_attribute("height", &format!("{}", bottom_right.1 - upper_left.1))
            .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?;
        el.set_attribute("opacity", &make_svg_opacity(style.color()))
            .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?;
        el.set_attribute("fill", &fill)
            .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?;
        el.set_attribute("stroke", &stroke)
            .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?;

        self.tag_stack.push(el);

        Ok(())
    }

    fn draw_path<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        path: I,
        style: &S,
    ) -> Result<()> {
        if style.color().alpha == 0.0 {
            return Ok(());
        }

        let el = DOCUMENT
            .with(|d| d.create_element_ns(Some("http://www.w3.org/2000/svg"), "polyline"))
            .unwrap_throw();

        el.set_attribute("fill", "none")
            .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?;
        el.set_attribute("opacity", &make_svg_opacity(style.color()))
            .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?;
        el.set_attribute("stroke", &make_svg_color(style.color()))
            .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?;
        el.set_attribute("stroke-width", &format!("{}", style.stroke_width()))
            .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?;
        el.set_attribute(
            "points",
            &path.into_iter().fold(String::new(), |mut s, (x, y)| {
                s.push_str(&format!("{},{} ", x, y));
                s
            }),
        )
        .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?;

        self.tag_stack.push(el);

        Ok(())
    }

    fn fill_polygon<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        path: I,
        style: &S,
    ) -> Result<()> {
        if style.color().alpha == 0.0 {
            return Ok(());
        }
        let el = DOCUMENT
            .with(|d| d.create_element_ns(Some("http://www.w3.org/2000/svg"), "polygon"))
            .unwrap_throw();

        el.set_attribute("opacity", &make_svg_opacity(style.color()))
            .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?;
        el.set_attribute("fill", &make_svg_color(style.color()))
            .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?;
        el.set_attribute(
            "points",
            &path.into_iter().fold(String::new(), |mut s, (x, y)| {
                s.push_str(&format!("{},{} ", x, y));
                s
            }),
        )
        .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?;

        self.tag_stack.push(el);

        Ok(())
    }

    fn draw_circle<S: BackendStyle>(
        &mut self,
        center: BackendCoord,
        radius: u32,
        style: &S,
        fill: bool,
    ) -> Result<()> {
        if style.color().alpha == 0.0 {
            return Ok(());
        }
        let el = DOCUMENT
            .with(|d| d.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle"))
            .unwrap_throw();

        let (stroke, fill) = if !fill {
            (make_svg_color(style.color()), "none".to_string())
        } else {
            ("none".to_string(), make_svg_color(style.color()))
        };

        el.set_attribute("cx", &format!("{}", center.0))
            .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?;
        el.set_attribute("cy", &format!("{}", center.1))
            .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?;
        el.set_attribute("r", &format!("{}", radius))
            .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?;
        el.set_attribute("opacity", &make_svg_opacity(style.color()))
            .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?;
        el.set_attribute("fill", &fill)
            .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?;
        el.set_attribute("stroke", &stroke)
            .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?;
        el.set_attribute("stroke-width", &format!("{}", style.stroke_width()))
            .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?;

        self.tag_stack.push(el);

        Ok(())
    }

    fn draw_text<S: BackendTextStyle>(
        &mut self,
        text: &str,
        style: &S,
        pos: BackendCoord,
    ) -> Result<()> {
        let color = style.color();
        if color.alpha == 0.0 {
            return Ok(());
        }

        let el = DOCUMENT
            .with(|d| d.create_element_ns(Some("http://www.w3.org/2000/svg"), "text"))
            .unwrap_throw();

        let (x0, y0) = pos;
        let text_anchor = match style.anchor().h_pos {
            HPos::Left => "start",
            HPos::Right => "end",
            HPos::Center => "middle",
        };

        let dy = match style.anchor().v_pos {
            VPos::Top => "0.76em",
            VPos::Center => "0.5ex",
            VPos::Bottom => "-0.5ex",
        };

        #[cfg(feature = "debug")]
        {
            let ((fx0, fy0), (fx1, fy1)) =
                font.layout_box(text).map_err(DrawingErrorKind::FontError)?;
            let x0 = match style.anchor().h_pos {
                HPos::Left => x0,
                HPos::Center => x0 - fx1 / 2 + fx0 / 2,
                HPos::Right => x0 - fx1 + fx0,
            };
            let y0 = match style.anchor().v_pos {
                VPos::Top => y0,
                VPos::Center => y0 - fy1 / 2 + fy0 / 2,
                VPos::Bottom => y0 - fy1 + fy0,
            };
            self.draw_rect(
                (x0, y0),
                (x0 + fx1 - fx0, y0 + fy1 - fy0),
                &crate::prelude::RED,
                false,
            )
            .unwrap();
            self.draw_circle((x0, y0), 2, &crate::prelude::RED, false)
                .unwrap();
        }

        let mut attrs = vec![
            ("x", format!("{}", x0)),
            ("y", format!("{}", y0)),
            ("dy", dy.to_owned()),
            ("text-anchor", text_anchor.to_string()),
            ("font-family", style.family().as_str().to_string()),
            ("font-size", format!("{}", style.size() / 1.24)),
            ("opacity", make_svg_opacity(color)),
            ("fill", make_svg_color(color)),
        ];

        match style.style() {
            FontStyle::Normal => {}
            FontStyle::Bold => attrs.push(("font-weight", "bold".to_string())),
            other_style => attrs.push(("font-style", other_style.as_str().to_string())),
        };

        let trans = style.transform();
        match trans {
            FontTransform::Rotate90 => {
                attrs.push(("transform", format!("rotate(90, {}, {})", x0, y0)))
            }
            FontTransform::Rotate180 => {
                attrs.push(("transform", format!("rotate(180, {}, {})", x0, y0)));
            }
            FontTransform::Rotate270 => {
                attrs.push(("transform", format!("rotate(270, {}, {})", x0, y0)));
            }
            _ => {}
        }

        for (name, val) in attrs.iter() {
            el.set_attribute(name, val)
                .map_err(|e| DrawingErrorKind::DrawingError(e.into()))?;
        }

        el.set_text_content(Some(text));

        self.tag_stack.push(el);

        Ok(())
    }
}
