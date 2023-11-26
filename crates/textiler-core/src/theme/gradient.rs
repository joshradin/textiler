use crate::theme::color::SimpleColor;
use crate::theme::Color;
use crate::utils::bounded_float::BoundedFloat;
use indexmap::IndexMap;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::ops::Index;

/// A gradient defines a gradient, with potential multiple points of color inflection
#[derive(Debug, Serialize, Deserialize)]
pub struct Gradient {
    #[serde(flatten)]
    #[serde(deserialize_with = "de_gradient")]
    points: BTreeMap<BoundedFloat<0, 1>, Color>,
}

fn de_gradient<'de, D: Deserializer<'de>>(
    des: D,
) -> Result<BTreeMap<BoundedFloat<0, 1>, Color>, D::Error> {
    let map = IndexMap::<String, Color>::deserialize(des)?;
    let points = map
        .into_iter()
        .map(|(k, v)| {
            let float: f32 = k.parse().map_err(|e| D::Error::custom(e))?;
            let bounded = BoundedFloat::<0, 1>::new(float)
                .ok_or_else(|| D::Error::custom(format!("{} not in bounds [0,1]", float)))?;
            Ok((bounded, v))
        })
        .collect::<Result<BTreeMap<BoundedFloat<0, 1>, Color>, _>>()?;
    if points.get(&BoundedFloat::<0, 1>::MIN).is_none()
        || points.get(&BoundedFloat::<0, 1>::MAX).is_none()
    {
        return Err(D::Error::custom(
            "must specify 0 value and 1 value in gradient",
        ));
    }
    Ok(points)
}

fn ser_gradient<S: Serializer>(gradient: &Gradient, serde: S) -> Result<S::Ok, S::Error> {
    gradient
        .points
        .iter()
        .map(|(k, v)| (format!("{}", *k), v.clone()))
        .collect::<HashMap<String, Color>>()
        .serialize(serde)
}

impl Gradient {
    pub fn new(low: Color, high: Color) -> Self {
        Self::from_iter([
            (BoundedFloat::<0, 1>::new(0.0).unwrap(), low),
            (BoundedFloat::<0, 1>::new(1.0).unwrap(), high),
        ])
    }

    fn calc_color_at(&self, bounded_float: &BoundedFloat<0, 1>) -> Option<Color> {
        let (&high_pt, high) = self.points.range(*bounded_float..).next()?;
        let (&low_pt, low) = self.points.range(..=*bounded_float).rev().next()?;
        if high_pt == low_pt || high == low {
            return Some(low.clone());
        }

        let color_pt: f32 = (*bounded_float - low_pt) / (high_pt - low_pt);
        match (low.to_simple().ok()?, high.to_simple().ok()?) {
            (SimpleColor::Hsla(l_h, l_s, l_l, l_a), SimpleColor::Hsla(h_h, h_s, h_l, h_a)) => {
                let h = (h_h - l_h) * color_pt + l_h;
                let s = (h_s - l_s) * color_pt + l_s;
                let l = (h_l - l_l) * color_pt + l_l;
                let a = (h_a - l_a) * color_pt + l_a;

                Some(Color::Hsla {
                    h: (h * 360.0).round() as u16,
                    s: (s * 100.0).round() as u8,
                    l: (l * 100.0).round() as u8,
                    a: (a * 100.0).round() as u8,
                })
            }
            (SimpleColor::Rgba(l_r, l_g, l_b, l_a), SimpleColor::Rgba(h_r, h_g, h_b, h_a)) => {
                let r = (h_r as f32 - l_r as f32) * color_pt + l_r as f32;
                let g = (h_g as f32 - l_g as f32) * color_pt + l_g as f32;
                let b = (h_b as f32 - l_b as f32) * color_pt + l_b as f32;
                let a = (h_a as f32 - l_a as f32) * color_pt + l_a as f32;

                Some(Color::Rgba {
                    r: (r).round() as u8,
                    g: (g).round() as u8,
                    b: (b).round() as u8,
                    a: (a).round() as u8,
                })
            }
            _ => None,
        }
    }

    pub fn get(&self, ref bounded_float: BoundedFloat<0, 1>) -> Color {
        self.points.get(bounded_float).cloned().unwrap_or_else(|| {
            self.calc_color_at(bounded_float)
                .expect("could not get a color")
        })
    }

    pub fn get_mut(&mut self, bounded_float: BoundedFloat<0, 1>) -> &mut Color {
        let c = self.calc_color_at(&bounded_float).unwrap();
        self.points.entry(bounded_float).or_insert(c)
    }

    /// Creates an inflection point at the given index, without returning a mutable-reference
    pub fn inflect_at(&mut self, bounded_float: BoundedFloat<0, 1>) {
        let _ = self.get_mut(bounded_float);
    }

    pub fn print_gradient(&self) {
        let mut set = HashSet::new();
        for i in 0..=100 {
            let color = self.get(BoundedFloat::new(i as f32 / 100.0).unwrap());
            let [r, g, b, ..] = color.to_rgba().unwrap();
            print!("\x1b[48;2;{};{};{}m \x1b[0m", r, g, b);
            set.insert(color.to_rgba().unwrap());
        }
        println!("  resolution: {}", set.len());
    }
}

impl FromIterator<(BoundedFloat<0, 1>, Color)> for Gradient {
    fn from_iter<T: IntoIterator<Item = (BoundedFloat<0, 1>, Color)>>(iter: T) -> Self {
        Self {
            points: iter.into_iter().collect(),
        }
    }
}

impl<'a> IntoIterator for &'a Gradient {
    type Item = (&'a BoundedFloat<0, 1>, &'a Color);
    type IntoIter = <&'a BTreeMap<BoundedFloat<0, 1>, Color> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.points.iter()
    }
}

impl IntoIterator for Gradient {
    type Item = (BoundedFloat<0, 1>, Color);
    type IntoIter = <BTreeMap<BoundedFloat<0, 1>, Color> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.points.into_iter()
    }
}
#[cfg(test)]
mod tests {
    use crate::theme::gradient::Gradient;
    use crate::theme::Color;
    use crate::utils::bounded_float::BoundedFloat;

    #[test]
    fn test_gradient() {
        for saturation in 0..=100 {
            let light = 75;
            let mut gradient = Gradient::new(
                Color::hsl(0, saturation, light),
                Color::hsl(360, saturation, light),
            );
            *gradient.get_mut(BoundedFloat::new(1. / 6.).unwrap()) =
                Color::hsl(60, saturation, light);
            *gradient.get_mut(BoundedFloat::new(2. / 6.).unwrap()) =
                Color::hsl(120, saturation, light);
            *gradient.get_mut(BoundedFloat::new(3. / 6.).unwrap()) =
                Color::hsl(180, saturation, light);
            *gradient.get_mut(BoundedFloat::new(4. / 6.).unwrap()) =
                Color::hsl(240, saturation, light);
            *gradient.get_mut(BoundedFloat::new(5. / 6.).unwrap()) =
                Color::hsl(300, saturation, light);

            gradient.print_gradient();
        }
    }
}
