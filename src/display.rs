use std::{fmt, ops::Div};

use num_traits::{Float, One, Zero};

use crate::KdTree;

#[derive(Copy, Clone)]
enum FormatMode<A: Float + Zero + One + fmt::Display> {
    Text { level: usize },
    TikZ { min_x: A, max_x: A, min_y: A, max_y: A },
}

impl<A: Float + Zero + One + fmt::Display, T: std::cmp::PartialEq, U: AsRef<[A]> + std::cmp::PartialEq>
    KdTree<A, T, U>
{
    fn fmt_recursively(&self, f: &mut fmt::Formatter<'_>, mode: FormatMode<A>) -> fmt::Result {
        if self.size() == 0 {
            if let FormatMode::Text { .. } = mode {
                write!(f, "KdTree {{}}")?;
            }
            return Ok(());
        }

        let four_spaces = " ".repeat(4);
        let indent = match mode {
            FormatMode::Text { level } => four_spaces.repeat(level),
            FormatMode::TikZ { .. } => four_spaces.clone(),
        };

        match (&self.left, &self.right, mode) {
            (Some(left), Some(right), FormatMode::Text { level }) => {
                // internal node
                writeln!(f, "KdTree {{")?;
                writeln!(
                    f,
                    "{indent}{four_spaces}split_value: {} on {}",
                    self.split_value.unwrap(),
                    dimension_label(self.split_dimension.unwrap()),
                )?;

                write!(f, "{indent}{four_spaces}left: ")?;
                left.fmt_recursively(f, FormatMode::Text { level: level + 1 })?;

                write!(f, "{indent}{four_spaces}right: ")?;
                right.fmt_recursively(f, FormatMode::Text { level: level + 1 })?;
                write!(f, "{indent}}}")?;
            }
            (_, _, FormatMode::Text { .. }) => {
                // leaf node
                writeln!(f, "KdTree {{")?;
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
                write!(f, "{indent}}}")?;
            }

            (Some(left), Some(right), FormatMode::TikZ { min_x, max_x, min_y, max_y }) => {
                // internal node
                // draw the split line
                todo!()
            }
            (_, _, FormatMode::TikZ { .. }) => {
                // leaf node
                // just draw each point
                write!(f, r"\draw[fill=black]")?;
                for point in self.points.as_ref().unwrap() {
                    write!(f, "\n{indent}({}, {}) circle[radius=0.05]", point.as_ref()[0], point.as_ref()[1])?;
                }
                writeln!(f, ";")?;
            }
        }

        if let FormatMode::Text { level: 1.. } = mode {
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

impl<A: Float + Zero + One + fmt::Display, T: std::cmp::PartialEq, U: AsRef<[A]> + std::cmp::PartialEq> fmt::Debug
    for KdTree<A, T, U>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_recursively(f, FormatMode::Text { level: 0 })
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
        let &[min_x, min_y] = self.0.min_bounds.as_ref() else { panic!() };
        let &[max_x, max_y] = self.0.max_bounds.as_ref() else { panic!() };

        writeln!(
            f,
            r#"\documentclass[border=2cm]{{standalone}}
\usepackage{{mathtools}}
\usepackage{{tikz}}
\usetikzlibrary{{arrows.meta}}

\begin{{document}}
\begin{{tikzpicture}}

\draw[->, black!40] ({min_x}, 0) -- ({max_x}, 0);
\draw[->, black!40] (0, {min_y}) -- (0, {max_y});
"#
        )?;

        self.0.fmt_recursively(f, FormatMode::TikZ { min_x, max_x, min_y, max_y })?;

        writeln!(
            f,
            r#"
\end{{tikzpicture}}
\end{{document}}"#
        )
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
