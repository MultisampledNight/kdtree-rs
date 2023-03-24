use std::{fmt, ops::Div};

use num_traits::{Float, One, Zero};

use crate::KdTree;

impl<A: Float + Zero + One + fmt::Display, T: std::cmp::PartialEq, U: AsRef<[A]> + std::cmp::PartialEq>
    KdTree<A, T, U>
{
    fn fmt_on_level(&self, f: &mut fmt::Formatter<'_>, level: usize) -> fmt::Result {
        if self.size() == 0 {
            write!(f, "KdTree {{}}")?;
            return Ok(());
        }

        let four_spaces = " ".repeat(4);
        let indent = four_spaces.repeat(level);

        writeln!(f, "KdTree {{")?;
        if let (Some(left), Some(right)) = (&self.left, &self.right) {
            // internal node
            writeln!(
                f,
                "{indent}{four_spaces}split_value: {} on {}",
                self.split_value.unwrap(),
                dimension_label(self.split_dimension.unwrap()),
            )?;

            write!(f, "{indent}{four_spaces}left: ")?;
            left.fmt_on_level(f, level + 1)?;

            write!(f, "{indent}{four_spaces}right: ")?;
            right.fmt_on_level(f, level + 1)?;
        } else {
            // leaf node
            writeln!(f, "{indent}{four_spaces}points: [")?;
            for point in self.points.as_ref().unwrap() {
                write!(f, "{indent}{four_spaces}{four_spaces}(")?;

                for (i, component) in point.as_ref().iter().enumerate() {
                    if i != 0 {
                        write!(f, ",\t")?;
                    }
                    write!(f, "{component:+}")?;
                }
                writeln!(f, ")")?;
            }
            writeln!(f, "{indent}{four_spaces}]")?;
        }
        write!(f, "{indent}}}")?;

        if level != 0 {
            writeln!(f)?;
        }

        Ok(())
    }
}

fn dimension_label(dim: usize) -> String {
    match dim {
        0 => "x".to_string(),
        1 => "y".to_string(),
        2 => "z".to_string(),
        3 => "w".to_string(),
        _ => format!("x{dim}"),
    }
}

impl<A: Float + Zero + One + Div<f64> + fmt::Display, T: std::cmp::PartialEq, U: AsRef<[A]> + std::cmp::PartialEq>
    KdTree<A, T, U>
{
    pub fn display_tikz(&self) -> KdTreeDisplayTikz<'_, A, T, U> {
        if self.dimensions != 2 {
            panic!(
                "can only visualize 2-dimensional kd trees, but this one is at {} dimensions",
                self.dimensions
            );
        }

        KdTreeDisplayTikz(self)
    }
}

impl<A: Float + Zero + One + fmt::Display, T: std::cmp::PartialEq, U: AsRef<[A]> + std::cmp::PartialEq> fmt::Debug
    for KdTree<A, T, U>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_on_level(f, 0)
    }
}

pub struct KdTreeDisplayTikz<
    'a,
    A: Float + Zero + One + Div<f64> + fmt::Display,
    T: std::cmp::PartialEq,
    U: AsRef<[A]> + std::cmp::PartialEq,
>(&'a KdTree<A, T, U>);

impl<A, T, U> fmt::Display for KdTreeDisplayTikz<'_, A, T, U>
where
    A: Float + Zero + One + Div<f64> + fmt::Display,
    T: std::cmp::PartialEq,
    U: AsRef<[A]> + std::cmp::PartialEq,
    <A as Div<f64>>::Output: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [min_x, min_y] = self.0.min_bounds.as_ref() else { panic!() };
        let [max_x, max_y] = self.0.max_bounds.as_ref() else { panic!() };
        let scale = ((*min_x - *max_x).abs() + (*min_y - *max_y).abs()) / 30.0;

        writeln!(
            f,
            r#"\documentclass[border=2cm]{{standalone}}
\usepackage{{mathtools}}
\usepackage{{tikz}}
\usetikzlibrary{{arrows.meta}}

\begin{{document}}
\begin{{tikzpicture}}[circle, very thick, scale={scale}]

\node[anchor=north east] (o) at (0, 0) {{0}};
\draw[->,thin] ({min_x}, 0) -- ({max_x}, 0);
\draw[->,thin] (0, {min_y}) -- (0, {max_y});
"#
        )?;

        writeln!(
            f,
            r#"\end{{tikzpicture}}
\end{{document}}"#
        )?;

        todo!()
    }
}
