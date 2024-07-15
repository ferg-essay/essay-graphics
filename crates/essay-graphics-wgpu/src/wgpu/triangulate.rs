use std::ops::{Index, IndexMut};

use essay_graphics_api::{Point, Canvas, Path, PathCode};

use super::bezier::intersection;

// Seidel's algorithm

pub fn triangulate2(path: &Path<Canvas>) -> Vec<Triangle> {
    let tri = Tri::new(path);

    tri.triangles()
}

struct Tri {
    edges: Vec<Edge>,

    traps: Vec<Trap>,
}

impl Tri {
    fn new(path: &Path<Canvas>) -> Self {
        let mut tri = Self {
            edges: Vec::new(),
            traps: Vec::new(),
        };

        tri.traps.push(Trap::new(TrapId(0), f32::MIN, f32::MAX));

        let codes = path.codes();
        assert!(matches!(codes[0], PathCode::MoveTo(_)));

        let mut prev = Point(f32::MAX, f32::MAX);
        let mut first = prev;

        for code in codes {
            match code {
                PathCode::MoveTo(p) => {
                    prev = *p;
                    first = *p;
                },
                PathCode::LineTo(p) => {
                    tri.add_edge(prev, *p);

                    prev = *p;
                },
                PathCode::Bezier2(p1, p2) => {
                    if Triangle::ccw(prev, *p1, *p2) {
                        tri.add_edge(prev, *p2);
                    } else {
                        tri.add_edge(prev, *p1);
                        tri.add_edge(*p1, *p2);
                    }

                    prev = *p2;
                },
                PathCode::Bezier3(_, _, p) => {
                    tri.add_edge(prev, *p);

                    prev = *p;
                }
                PathCode::ClosePoly(p) => {
                    tri.add_edge(prev, *p);
                    tri.add_edge(*p, first);
                },
            }
        }

        tri
    }

    #[inline]
    fn add_edge(&mut self, p0: Point, p1: Point) {
        let edge = Edge(p0, p1);

        let edge_id = EdgeId(self.edges.len());

        self.edges.push(edge);

        let mut trap_id = if p0.x() < p1.x() {
            self.add_point(p0.x());
            self.add_point(p1.x())
        } else {
            self.add_point(p1.x());
            self.add_point(p0.x())
        };

        while let Some(next_id) = self.add_edge_to_trap(trap_id, edge_id, p0, p1) {
            trap_id = next_id;
        }   
    }

    fn add_point(&mut self, x: f32) -> TrapId {
        let id = self.find_trap(x);
        let trap_x = self[id].x;

        if x == trap_x[1] {
            return id;
        } else {
            let _rid = self.add_trap(id, x, trap_x[1]);
            self[id].x[1] = x;

            id
        }
    }

    fn add_trap(&mut self, id: TrapId, x_min: f32, x_max: f32) -> TrapId {
        let Trap { 
            r0,
            top, bot,
            u0,
            .. 
        } = self[id];

        let id_right = TrapId(self.traps.len());

        let mut trap = Trap::new(id_right, x_min, x_max);
        trap.r0 = r0;
        trap.l0 = id;
        trap.top = top;
        trap.bot = bot;

        assert!(u0.is_none());

        self[id].r0 = id_right;

        if ! r0.is_none() {
            self.traps[r0.i()].l0 = id_right;
        }

        self.traps.push(trap);

        id_right
    }

    fn trap_up(&mut self, id: TrapId) -> TrapId {
        let id_up = self[id].u0;

        if ! id_up.is_none() {
            return id_up;
        }

        let Trap { x, .. } = self[id];

        let id_up = TrapId(self.traps.len());

        let mut trap = Trap::new(TrapId(usize::MAX), x[0], x[1]);
        trap.is_up = true;

        self[id].u0 = id_up;

        self.traps.push(trap);

        id_up
    }

    fn add_edge_to_trap(
        &mut self, 
        id: TrapId, 
        edge_id: EdgeId, 
        p0: Point, 
        p1: Point
    ) -> Option<TrapId> {
        if p0.x() == p1.x() {
            return None;
        }

        let Trap { 
            x, l0, top, bot, .. 
        } = self[id];

        let x_min = p0.x().min(p1.x());

        let result = if x[0] <= x_min {
            None
        } else if l0.is_none() {
            assert!(self[id].is_up);
            // TODO: shouldn't be possible?
            //panic!("Unexpected null l0 for {:?}", x);
            None
        } else {
            Some(l0)
        };

        if ! top.is_none() {
            let Edge(q0, q1) = self[top];
            let Edge(r0, r1) = self[bot];

            let py_0 = interpolate(x[0], p0, p1);
            let py_1 = interpolate(x[1], p0, p1);

            let qy_0 = interpolate(x[0], q0, q1);
            let qy_1 = interpolate(x[1], q0, q1);

            let new_top = if qy_0 == py_0 && qy_1 == py_1 {
                // duplicate edge
                return result;
            } else if qy_0 <= py_0 && qy_1 <= py_1 {
                // need to add above
                edge_id
            } else if py_0 <= qy_0 && qy_1 <= qy_1 {
                // new edge below top
                let ry_0 = interpolate(x[0], r0, r1);
                let ry_1 = interpolate(x[1], r0, r1);

                if ry_0 == py_0 && ry_1 == py_1 {
                    // duplicate edge
                    return result;
                } else if ry_0 <= py_0 && ry_1 <= py_1 {
                    // between top and bottom
                    self[id].top = edge_id;
                    top
                } else if py_0 <= ry_0 && py_1 <= ry_1 {
                    // below bottom
                    self[id].top = bot;
                    self[id].bot = edge_id;
                    top
                } else {
                    // crossing
                    todo!("crossing is not yet implemented");
                }
            } else {
                // TODO: cross
                todo!("crossing is not yet implemented");
            };

            let up = self.trap_up(id);

            let Edge(t0, t1) = self[new_top];

            self.add_edge_to_trap(up, new_top, t0, t1);
        } else if ! bot.is_none() {
            let Edge(q0, q1) = self[bot];

            let py_0 = interpolate(x[0], p0, p1);
            let py_1 = interpolate(x[1], p0, p1);

            let qy_0 = interpolate(x[0], q0, q1);
            let qy_1 = interpolate(x[1], q0, q1);

            if qy_0 == py_0 && qy_1 == py_1 {
            } else if qy_0 <= py_0 && qy_1 <= py_1 {
                self[id].top = edge_id;
            } else if py_0 <= qy_0 && py_1 <= qy_1 {
                self[id].top = bot;
                self[id].bot = edge_id;
            } else {
                let mp = intersection(p0, p1, q0, q1);

                assert!(x[0] < mp.x() && mp.x() < x[1]);

                self[id].bot = EdgeId::none();
                
                self.add_edge(p0, mp);
                self.add_edge(p1, mp);
                self.add_edge(q0, mp);
                self.add_edge(q1, mp);

                /*
                return if x[0] <= x_min {
                    None
                } else if l0.is_none() {
                    // TODO: shouldn't be possible?
                    panic!("Unexpected null l0 for {:?}", x);
                } else {
                    Some(self[id].l0)
                };
                */
            }
        } else {
            self[id].bot = edge_id;
        }

        result
    }

    fn find_trap(&self, x: f32) -> TrapId {
        for trap in self.traps.iter().rev() {
            if trap.x[0] < x && x <= trap.x[1] && ! trap.is_up {
                return trap.id;
            }
        }

        panic!("Can't find trap {}", x);
    }

    fn triangles(&self) -> Vec<Triangle> {
        let mut tri = Vec::<Triangle>::new();

        for index in 0..self.traps.len() {
            let id = TrapId(index);

            self.add_trap_triangles(&mut tri, id);
        }

        tri
    }

    fn add_trap_triangles(&self, tri: &mut Vec<Triangle>, id: TrapId) -> TrapId {
        let trap = &self[id];

        let (top, bot) = (trap.top, trap.bot);

        if top.is_none() {
            return TrapId::none();
        }

        let x = trap.x;

        let Edge(p0, p1) = self[top];
        let py0 = interpolate(x[0], p0, p1);
        let py1 = interpolate(x[1], p0, p1);

        let Edge(q0, q1) = self[bot];
        let qy0 = interpolate(x[0], q0, q1);
        let qy1 = interpolate(x[1], q0, q1);

        if py0 == qy0 {
            tri.push(Triangle(Point(x[0], py0), Point(x[1], qy1), Point(x[1], py1)));
        } else if py1 == qy1 {
            tri.push(Triangle(Point(x[0], py0), Point(x[0], qy0), Point(x[1], qy1)));
        } else {
            tri.push(Triangle(Point(x[0], py0), Point(x[0], qy0), Point(x[1], qy1)));
            tri.push(Triangle(Point(x[1], qy1), Point(x[1], py1), Point(x[0], py0)));
        }

        trap.u0
    }
}

impl Index<TrapId> for Tri {
    type Output = Trap;

    #[inline]
    fn index(&self, index: TrapId) -> &Self::Output {
        &self.traps[index.i()]
    }
}

impl IndexMut<TrapId> for Tri {
    #[inline]
    fn index_mut(&mut self, index: TrapId) -> &mut Self::Output {
        &mut self.traps[index.i()]
    }
}

impl Index<EdgeId> for Tri {
    type Output = Edge;

    #[inline]
    fn index(&self, index: EdgeId) -> &Self::Output {
        &self.edges[index.i()]
    }
}

#[inline]
fn interpolate(x: f32, p0: Point, p1: Point) -> f32 {
    if p0.x() == p1.x() {
        p0.y()
    } else {
        let width = p1.x() - p0.x();
        let t = (x - p0.x()) / width;

        (1. - t) * p0.y() + t * p1.y()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EdgeId(usize);

impl EdgeId {
    #[inline]
    fn i(&self) -> usize {
        self.0
    }

    #[inline]
    fn none() -> Self {
        Self(usize::MAX)
    }

    #[inline]
    fn is_none(&self) -> bool {
        self.0 == usize::MAX
    }
}

struct Edge(Point, Point);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TrapId(usize);

impl TrapId {
    #[inline]
    fn i(&self) -> usize {
        self.0
    }

    #[inline]
    fn none() -> Self {
        Self(usize::MAX)
    }

    #[inline]
    fn is_none(&self) -> bool {
        self.0 == usize::MAX
    }
}

pub struct Trap {
    id: TrapId,

    x: [f32; 2],

    r0: TrapId,

    l0: TrapId,

    top: EdgeId,
    bot: EdgeId,

    u0: TrapId,
    is_up: bool,
}

impl Trap {
    fn new(id: TrapId, x_min: f32, x_max: f32) -> Self {
        Self {
            id,
            x: [x_min, x_max],

            r0: TrapId::none(),

            l0: TrapId::none(),

            u0: TrapId::none(),

            top: EdgeId::none(),
            bot: EdgeId::none(),

            is_up: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Triangle(Point, Point, Point);

impl Triangle {
    pub fn ccw(
        a: impl Into<Point>,
        b: impl Into<Point>,
        c: impl Into<Point>
    ) -> bool {
        let a = a.into();
        let b = b.into();
        let c = c.into();

        (b.x() - a.x()) * (c.y() - a.y()) - (c.x() - a.x()) * (b.y() - a.y()) > 0.
    }
}

impl Index<usize> for Triangle {
    type Output = Point;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Invalid index")
        }
    }
}

#[cfg(test)]
mod test {
    use essay_graphics_api::{Path, PathCode, Point, Canvas};

    use crate::wgpu::triangulate::{Triangle, triangulate2};

    #[test]
    fn test_tri() {
        let path = Path::<Canvas>::new(vec![
            PathCode::MoveTo(Point(0., 0.)),
            PathCode::LineTo(Point(1., 0.)),
            PathCode::ClosePoly(Point(1., 1.)),
        ]);
 
        assert_eq!(
            triangulate2(&path), vec![
                Triangle(Point(0., 0.), Point(1., 0.), Point(1., 1.))
        ]);

        let path = Path::<Canvas>::new(vec![
            PathCode::MoveTo(Point(0., 0.)),
            PathCode::LineTo(Point(0., 1.)),
            PathCode::ClosePoly(Point(1., 1.)),
        ]);
    
        assert_eq!(
            triangulate2(&path), vec![
                Triangle(Point(0., 1.), Point(0., 0.), Point(1., 1.))
        ]);
    }

    #[test]
    fn test_4_square() {
        let path = Path::<Canvas>::new(vec![
            PathCode::MoveTo(Point(0., 0.)),
            PathCode::LineTo(Point(1., 0.)),
            PathCode::LineTo(Point(1., 1.)),
            PathCode::ClosePoly(Point(0., 1.)),
        ]);

        assert_eq!(
            triangulate2(&path), vec![
                Triangle(Point(0., 1.), Point(0., 0.), Point(1., 0.)),
                Triangle(Point(1., 0.), Point(1., 1.), Point(0., 1.)),
        ]);
    }

    #[test]
    fn test_6_square() {
        let path = Path::<Canvas>::new(vec![
            PathCode::MoveTo(Point(0., 0.)),
            PathCode::LineTo(Point(1., 0.)),
            PathCode::LineTo(Point(2., 0.)),
            PathCode::LineTo(Point(2., 1.)),
            PathCode::LineTo(Point(1., 1.)),
            PathCode::ClosePoly(Point(0., 1.)),
        ]);

        assert_eq!(
            triangulate2(&path), vec![
                Triangle(Point(0., 1.), Point(0., 0.), Point(1., 0.)),
                Triangle(Point(1., 0.), Point(1., 1.), Point(0., 1.)),
                Triangle(Point(1., 1.), Point(1., 0.), Point(2., 0.)),
                Triangle(Point(2., 0.), Point(2., 1.), Point(1., 1.)),
        ]);
    }

    #[test]
    fn test_4_cross() {
        let path = Path::<Canvas>::new(vec![
            PathCode::MoveTo(Point(0., 0.)),
            PathCode::LineTo(Point(1., 1.)),
            PathCode::LineTo(Point(1., 0.)),
            PathCode::ClosePoly(Point(0., 1.)),
        ]);

        assert_eq!(
            triangulate2(&path), vec![
                Triangle(Point(0., 1.), Point(0.0, 0.0), Point(0.5, 0.5)),
                Triangle(Point(0.5, 0.5), Point(1., 0.), Point(1., 1.))
        ]);
    }

    #[test]
    fn test_wedge_right() {
        let path = Path::<Canvas>::new(vec![
            PathCode::MoveTo(Point(0., 0.)),
            PathCode::LineTo(Point(10., 10.)),
            PathCode::LineTo(Point(1., 20.)),
            PathCode::ClosePoly(Point(2., 5.)),
        ]);

        assert_eq!(
            triangulate2(&path), vec![
                Triangle(Point(1., 2.5), Point(1., 1.), Point(2., 2.)),
                Triangle(Point(2., 2.), Point(2., 5.), Point(1., 2.5)),
                Triangle(Point(1., 20.), Point(2., 5.), Point(2., 18.88889)),
                Triangle(Point(2., 18.88889), Point(2., 2.), Point(10., 10.)),
                Triangle(Point(1., 20.), Point(2., 5.), Point(2., 18.88889)),
        ]);
    }

    #[test]
    fn test_wedge_left() {
        let path = Path::<Canvas>::new(vec![
            PathCode::MoveTo(Point(10., 0.)),
            PathCode::LineTo(Point(0., 10.)),
            PathCode::LineTo(Point(9., 20.)),
            PathCode::ClosePoly(Point(8., 5.)),
        ]);

        assert_eq!(
            triangulate2(&path), vec![
                Triangle(Point(1., 2.5), Point(1., 1.), Point(2., 2.)),
                Triangle(Point(2., 2.), Point(2., 5.), Point(1., 1.)),
                Triangle(Point(1., 20.), Point(2., 5.), Point(2., 18.88889)),
                Triangle(Point(2., 18.88889), Point(2., 2.), Point(10., 10.)),
                Triangle(Point(1., 20.), Point(2., 5.), Point(2., 18.88889)),
        ]);
    }

    #[test]
    fn test_inner_tri() {
        let path = Path::<Canvas>::new(vec![
            PathCode::MoveTo(Point(0., 0.)),
            PathCode::LineTo(Point(10., 0.)),
            PathCode::ClosePoly(Point(10., 10.)),
            PathCode::MoveTo(Point(5., 1.)),
            PathCode::LineTo(Point(6., 1.)),
            PathCode::ClosePoly(Point(6., 2.)),
        ]);

        assert_eq!(
            triangulate2(&path), vec![
                Triangle(Point(0.0, 0.0), Point(5.0, 0.0), Point(5.0, 5.0)),
                Triangle(Point(5.0, 1.0), Point(5.0, 0.0), Point(6.0, 0.0)),
                Triangle(Point(6.0, 0.0), Point(6.0, 1.0), Point(5.0, 1.0)),
                Triangle(Point(6.0, 6.0), Point(6.0, 0.0), Point(10.0, 0.0)),
                Triangle(Point(10.0, 0.0), Point(10.0, 10.0), Point(6.0, 6.0)),
                Triangle(Point(5.0, 5.0), Point(5.0, 1.0), Point(6.0, 2.0)),
                Triangle(Point(6.0, 2.0), Point(6.0, 6.0), Point(5.0, 5.0)),
        ]);
    }

    #[test]
    fn test_arrow() {
        let path = Path::<Canvas>::new(vec![
            PathCode::MoveTo(Point(1., 1.)),
            PathCode::LineTo(Point(11., 11.)),
            PathCode::LineTo(Point(6., 11.)),
            PathCode::LineTo(Point(7.7, 10.)),
            PathCode::ClosePoly(Point(0., 2.)),
        ]);

        assert_eq!(
            triangulate2(&path), vec![
                Triangle(Point(1.0, 3.038961), Point(1.0, 1.0), Point(6.0, 6.0)), 
                Triangle(Point(6.0, 6.0), Point(6.0, 8.233767), Point(1.0, 3.038961)), 
                Triangle(Point(6.0, 8.233767), Point(6.0, 6.0), Point(7.7, 7.6999993)), 
                Triangle(Point(7.7, 7.6999993), Point(7.7, 10.0), Point(6.0, 8.233767)), 
                Triangle(Point(7.7, 11.0), Point(7.7, 7.6999993), Point(11.0, 11.0)), 
                Triangle(Point(6.0, 11.0), Point(7.7, 10.0), Point(7.7, 11.0)), 
                Triangle(Point(0.0, 2.0), Point(1.0, 1.0), Point(1.0, 3.038961))
        ]);
    }
}