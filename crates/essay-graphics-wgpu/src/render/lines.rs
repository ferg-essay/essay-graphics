use essay_graphics_api::{renderer::Canvas, BezierMesh2d, CapStyle, JoinStyle, Mesh2d, Path, PathCode, Point};


pub fn lines(
    path: &Path<Canvas>, 
    joinstyle: JoinStyle,
    capstyle: CapStyle,
    linewidth: f32,
) -> Option<(Mesh2d, BezierMesh2d)> {
    if linewidth <= 0. {
        return None;
    }
    
    let lw2 = (0.5f32 * linewidth).max(0.5);

    let mut mesh = Mesh2d::new();
    let mut bezier = BezierMesh2d::new();

    let mut p0 = Point::ZERO;
    let mut p_move = p0;
    let mut p_first = p0;
    let mut p_last = p0;
    let mut is_last_bezier = true;

    for code in path.codes() {
        let mut is_next_bezier = false;

        let p_next = match code {
            PathCode::MoveTo(p) => {
                cap_line(&mut mesh, &mut bezier, p_last, p0, lw2, &capstyle);
                
                p0 = *p;
                p_move = p0;
                p_first = p0;
                p_last = p0;
                p0
            }
            PathCode::LineTo(p1) => {
                draw_line(&mut mesh, p0, *p1, lw2);
                // TODO: clip
                *p1
            }
            PathCode::Bezier2(p1, p2) => {
                draw_bezier_line(&mut mesh, &mut bezier, p0, *p1, *p2, lw2);
                is_next_bezier = true;

                *p2
            }
            PathCode::Bezier3(_, _, _) => {
                panic!("Bezier3 should already be split into Bezier2");
            }
            PathCode::ClosePoly(p1) => {
                //self.draw_line(p0.x(), p0.y(), p1.x(), p1.y(), lw_x, lw_y, rgba);
                draw_line(&mut mesh, p0, *p1, lw2);
                draw_line(&mut mesh, *p1, p_move, lw2);

                join_lines(&mut mesh, &mut bezier, p0, *p1, p_move, lw2, &joinstyle);
                join_lines(&mut mesh, &mut bezier, *p1, p_move, p_first, lw2, &joinstyle);

                //*p1
                p_last = p0;
                p_move
            }
        };

        // bezier automatically join lines
        if ! is_last_bezier && ! is_next_bezier {
            join_lines(&mut mesh, &mut bezier, p_last, p0, p_next, lw2, &joinstyle);
        }

        if p_first == p_move {
            p_first = p_next;
            cap_line(&mut mesh, &mut bezier, p_next, p_move, lw2, &capstyle);
        }

        is_last_bezier = is_next_bezier;
        p_last = p0;
        p0 = p_next;
    }
    cap_line(&mut mesh, &mut bezier, p_last, p0, lw2, &capstyle);

    Some((mesh, bezier))
}

fn draw_line(
    mesh: &mut Mesh2d, 
    b0: Point,
    b1: Point,
    lw2: f32,
) {
    let (nx, ny) = line_normal(b0, b1, lw2);

    mesh.triangle(
        [b0.x - nx, b0.y + ny],
        [b0.x + nx, b0.y - ny],
        [b1.x + nx, b1.y - ny],
    );

    mesh.triangle(
        [b1.x + nx, b1.y - ny],
        [b1.x - nx, b1.y + ny],
        [b0.x - nx, b0.y + ny],
    );
}

fn join_lines(
    mesh: &mut Mesh2d,
    bezier: &mut BezierMesh2d,
    b0: Point, 
    b1: Point, 
    b2: Point, 
    lw2: f32, 
    join_style: &JoinStyle
) {
    let min_join = 1.;

    if b0 == b1 || b1 == b2 || lw2 < min_join {
        // small lines can ignore joining.
        return;
    }

    join_lines_sign(mesh, bezier, b0, b1, b2, lw2, join_style, 1.);
    join_lines_sign(mesh, bezier, b0, b1, b2, lw2, join_style, -1.);
}


fn join_lines_sign(
    mesh: &mut Mesh2d,
    bezier: &mut BezierMesh2d,
    b0: Point, 
    b1: Point, 
    b2: Point,
    lw2: f32, 
    join_style: &JoinStyle,
    sign: f32,
) {
    let (nx, ny) = line_normal(b0, b1, lw2);
    let (nx, ny) = (sign * nx, sign * ny);

    // outside edge
    let p0 = Point::new(b0.x + nx, b0.y - ny);
    let p1 = Point::new(b1.x + nx, b1.y - ny);

    let (nx, ny) = line_normal(b1, b2, lw2);
    let (nx, ny) = (sign * nx, sign * ny);

    // outside edge
    let q1 = Point::new(b1.x + nx, b1.y - ny);
    let q2 = Point::new(b2.x + nx, b2.y - ny);

    // add bevel triangle
    mesh.triangle(p1, q1, b1);

    match join_style {
        JoinStyle::Bevel => {},
        JoinStyle::Miter => {
            // TODO: clamp intersections of too-long length
            let mp = line_intersection(p0, p1, q1, q2);

            if mp != p0 { // non-parallel
                let mp = clamp_miter(b1, mp, lw2 * 2.);

                mesh.triangle(p1, mp, q1);
            }
        },
        JoinStyle::Round => {
            let mp = line_intersection(p0, p1, q1, q2);

            if mp != p0 && p0.dist(p1) > 1. && q1.dist(q2) > 1. { // non-parallel
                let mp = clamp_miter(b1, mp, lw2 * 2.);
                
                draw_bezier_fill(bezier, p1, mp, q1);
            }
        }
    }
}

fn draw_bezier_line(
    mesh: &mut Mesh2d,
    bezier: &mut BezierMesh2d, 
    b0: Point,
    b1: Point,
    b2: Point,
    lw2: f32,
) {
    let len = b2.hypot(b0).max(f32::EPSILON);
    let min_bezier = 3.0;

    if len <= min_bezier {
        draw_line(mesh, b0, b2, lw2);
        return;
    }

    let ccw = ccw(b0, b1, b2);

    let min_bezier_area = 1.;
    if ccw.abs() < min_bezier_area {
        draw_line(mesh, b0, b2, lw2);
        return;
    } 

    let lw = 2.0 * lw2;
    // todo: normals for bezier need to be between p0 to p1 and p1 to p2
    // normal to the line
    //let (dx, dy) = ((b2.x() - b0.x()) / len, (b2.y() - b0.y()) / len);
    //let (mut nx, mut ny) = (dy * lw2, dx * lw2);
    //if ccw < 0. {
    //    (nx, ny) = (-nx, -ny);
    //}
    let len0 = b0.hypot(b1);
    let (dx0, dy0) = ((b1.x - b0.x) / len0, (b1.y - b0.y) / len0);

    let len2 = b2.hypot(b1);
    let (dx2, dy2) = ((b2.x - b1.x) / len2, (b2.y - b1.y) / len2);

    let (mut nx0, mut ny0) = (dy0 * lw2, dx0 * lw2);
    let (mut nx2, mut ny2) = (dy2 * lw2, dx2 * lw2);
    if ccw < 0. {
        (nx0, ny0) = (-nx0, -ny0);
        (nx2, ny2) = (-nx2, -ny2);
    }

    let (nx1, ny1) = (0.5 * (nx0 + nx2), 0.5 * (ny0 + ny2));

    // outer bezier's points
    let p0 = Point::new(b0.x + nx0, b0.y - ny0);
    // p1 slightly incorrect
    let p1 = Point::new(b1.x + nx1, b1.y - ny1);
    let p2 = Point::new(b2.x + nx2, b2.y - ny2);
    
    // inner bezier's points
    let q0 = Point::new(b0.x - nx0, b0.y + ny0);
    // let q1 = Point(b1.x() - nx1, b1.y() + ny1);
    let q2 = Point::new(b2.x - nx2, b2.y + ny2);

    // height of p1 from p0 to p2 line 
    let p1_height = 0.5 * vertex_height(p0, p1, p2);

    // linewidth in uv coordinates
    let outer_width = lw.min(p1_height);
    let v_factor = outer_width / p1_height; // * 0.5;

    // outer bezier
    bezier.triangle(p0, p1, p2, 0., v_factor);

    if lw <= outer_width {
        // outer bezier handles the full line thickness

        let p = 0.5 * (1. - (1. - v_factor).sqrt());
        //let len = p0.hypot(p2);
        //let p = lw / len;
        let pa = interpolate(p, p0, p2);
        let pb = interpolate(p, p2, p0);

        mesh.triangle(q0, p0, pa);
        mesh.triangle(p2, q2, pb);

        return;
    }

    let height = lw - outer_width;

    let qa = line_intersection(
        q0, q0 + (b1 - b0),
        p0, p2,
    );
    let qb = line_intersection(
        q2, q2 + (b1 - b2),
        p0, p2,
    );
    let q1 = Point::new(0.5 * (qa.x + qb.x), 0.5 * (qa.y + qb.y));
    
    let q1_height = vertex_height(q0, q1, q2);

    bezier.triangle(q0, q1, q2, 1., (height - q1_height * 0.5) / q1_height);

    mesh.triangle(q0, p0, q1);
    mesh.triangle(p2, q2, q1);
}

fn draw_bezier_fill(
    bezier: &mut BezierMesh2d,
    p0: Point,
    p1: Point,
    p2: Point,
) {
    if ccw(p0, p1, p2) > 0. {
        bezier.triangle(p0, p1, p2, 0., 1.);
    } else {
        bezier.triangle(p0, p1, p2, 1., 0.);
    }
}

fn cap_line(
    mesh: &mut Mesh2d, 
    bezier: &mut BezierMesh2d, 
    b0: Point, 
    b1: Point,
    lw2: f32, 
    cap_style: &CapStyle
) {
    if b0 == b1 || cap_style == &CapStyle::Butt {
        // small lines can ignore joining.
        return;
    }

    let (nx, ny) = line_normal(b0, b1, lw2);
    let (dx, dy) = (ny, nx);

    // outside edge
    let p0 = Point::new(b1.x + nx, b1.y - ny);
    // extended edge
    let p1 = Point::new(b1.x + nx + dx, b1.y - ny + dy);

    // inside edge
    let q0 = Point::new(b1.x - nx, b1.y + ny);
    // extended edge
    let q1 = Point::new(b1.x - nx + dx, b1.y + ny + dy);

    let mp = Point::new(b1.x + dx, b1.y + dy);

    match cap_style {
        CapStyle::Round => {
            mesh.triangle(p0, mp, q0);
            bezier.triangle(p0, p1, mp, 0., 1.);
            bezier.triangle(mp, q1, q0, 0., 1.);
        }
        CapStyle::Projecting => {
            mesh.triangle(p0, p1, q1);
            mesh.triangle(q1, q0, p0);
        },
        CapStyle::Butt => {
            panic!(); // Butt has early exit
        }
    }
}

pub(crate) fn line_normal(
    p0: Point, 
    p1: Point, 
    lw2: f32, 
) -> (f32, f32) {
    let dx = p1.x - p0.x;
    let dy = p1.y - p0.y;

    let len = dx.hypot(dy).max(f32::EPSILON);

    let dx = dx / len;
    let dy = dy / len;

    // normal to the line
    let nx = dy * lw2;
    let ny = dx * lw2;

    (nx, ny)
}

pub(crate) fn intersection(p0: Point, p1: Point, q0: Point, q1: Point) -> Point {
    let det = (p0.x - p1.x) * (q0.y - q1.y)
        - (p0.y - p1.y) * (q0.x - q1.x);

    if det.abs() <= f32::EPSILON {
        return p0; // p0 is marker for coincident or parallel lines
    }

    let p_xy = p0.x * p1.y - p0.y * p1.x;
    let q_xy = q0.x * q1.y - q0.y * q1.x;

    let x = (p_xy * (q0.x - q1.x) - (p0.x - p1.x) * q_xy) / det;
    let y = (p_xy * (q0.y - q1.y) - (p0.y - p1.y) * q_xy) / det;

    Point::new(x, y)
}

pub(crate) fn line_intersection(
    p0: Point, 
    p1: Point, 
    q0: Point, 
    q1: Point
) -> Point {
    let mut det = (p0.x - p1.x) * (q0.y - q1.y)
        - (p0.y - p1.y) * (q0.x - q1.x);

    if det.abs() <= f32::EPSILON {
        return p0; // p0 is marker for coincident or parallel lines
    } else if det.abs() < 0.2 {
        // clamp long extensions for miter
        det = 0.2 * det.signum();
    }


    let p_xy = p0.x * p1.y - p0.y * p1.x;
    let q_xy = q0.x * q1.y - q0.y * q1.x;

    let x = (p_xy * (q0.x - q1.x) - (p0.x - p1.x) * q_xy) / det;
    let y = (p_xy * (q0.y - q1.y) - (p0.y - p1.y) * q_xy) / det;

    Point::new(x, y)
}

pub(super) fn ccw(b0: Point, b1: Point, b2: Point) -> f32 {
    (b1.x - b0.x) * (b2.y - b0.y)
    - (b2.x - b0.x) * (b1.y - b0.y)
}

fn vertex_height(p0: Point, p1: Point, p2: Point) -> f32 {
    let a = p0.hypot(p1);
    let b = p1.hypot(p2);
    let c = p2.hypot(p0);
    // Heron's formula
    let s = 0.5 * (a + b + c);
    let area = (s * (s - a) * (s - b) * (s - c)).sqrt();

    2. * area / c
}

fn interpolate(p: f32, p0: Point, p1: Point) -> Point {
    Point::new(
        (1. - p) * p0.x + p * p1.x,
        (1. - p) * p0.y + p * p1.y,
    )
}

fn clamp_miter(center: Point, miter: Point, lim: f32) -> Point {
    Point::new(
        miter.x.clamp(center.x - lim, center.x + lim),
        miter.y.clamp(center.y - lim, center.y + lim),
    )
}
