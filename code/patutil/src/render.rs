use crate::{csg, Color, Pos, Rect, mesh, points, Line};

use self::text::Text;

pub mod text;

pub trait Render {
    fn draw_rect(&self, rect: Rect, color: Color, round_edge: bool);
    fn draw_line(&self, line: Line, color: Color);
    fn draw_text(&self, pos: Pos, text: &Text);
    fn draw_csg(&self, rect: Rect, csg: csg::SceneRef);
    fn draw_mesh(&self, rect: Rect, mesh: mesh::SceneRef);
    fn draw_points(&self, rect: Rect, points: points::SceneRef);
    fn picking_csg(&self, rect: Rect, csg: csg::SceneRef, picking_pos: Pos) -> Option<u16>;
    fn clear(&self);
}
