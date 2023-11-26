use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
pub struct Breakpoint {
    abbrev: String,
    width: u16,
}

impl Breakpoint {
    /// Creates a new breakpoint
    fn new(abbrev: &str, width: u16) -> Self {
        Self {
            abbrev: abbrev.to_string(),
            width,
        }
    }

    /// Gets the abbreviation used to reference this breakpoint
    pub fn abbrev(&self) -> &str {
        &self.abbrev
    }

    /// Gets the width for this breakpoint
    pub fn width(&self) -> u16 {
        self.width
    }

    /// Gets a mutable reference to the width of this breakpoint
    pub fn width_mut(&mut self) -> &mut u16 {
        &mut self.width
    }
}

impl Default for Breakpoints {
    fn default() -> Self {
        Self::from_iter([
            ("xs", 0),
            ("sm", 600),
            ("md", 768),
            ("lg", 992),
            ("xl", 1200),
        ])
    }
}

impl<'a> FromIterator<(&'a str, u16)> for Breakpoints {
    fn from_iter<T: IntoIterator<Item = (&'a str, u16)>>(iter: T) -> Self {
        let mut bp = Breakpoints::new();
        for (key, width) in iter {
            bp.set(key, width);
        }

        bp
    }
}

impl PartialEq for Breakpoint {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width
    }
}

impl Eq for Breakpoint {}

impl PartialOrd for Breakpoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Breakpoint {
    fn cmp(&self, other: &Self) -> Ordering {
        self.width.cmp(&other.width)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Breakpoints {
    points: BTreeSet<Breakpoint>,
}

impl Breakpoints {
    /// Creates a new set of breakpoints
    pub fn new() -> Self {
        Self {
            points: Default::default(),
        }
    }

    /// Sets a breakpoint with a given name
    pub fn set(&mut self, breakpoint: &str, width: u16) {
        {
            if let Some(mut bp) = self.get_mut(breakpoint) {
                bp.width = width;
                return;
            }
        }
        let _ = self.points.insert(Breakpoint::new(breakpoint, width));
    }

    /// Gets the breakpoint at given value
    pub fn get(&self, breakpoint: &str) -> Option<&Breakpoint> {
        self.points.iter().find(|b| b.abbrev == breakpoint)
    }

    /// Gets a mutable reference to a breakpoint

    pub fn get_mut(&mut self, breakpoint: &str) -> Option<BreakpointMutRef> {
        if let Some(ref found) = self.get(breakpoint).cloned() {
            let took = self.points.take(found).unwrap();
            Some(BreakpointMutRef {
                bp: took,
                points: self,
            })
        } else {
            None
        }
    }

    /// Gets all the breakpoints

    pub fn points(&self) -> impl IntoIterator<Item = &Breakpoint> {
        self.points.iter()
    }
}

#[derive(Debug)]
pub struct BreakpointMutRef<'a> {
    bp: Breakpoint,
    points: &'a mut Breakpoints,
}

impl<'a> Deref for BreakpointMutRef<'a> {
    type Target = Breakpoint;

    fn deref(&self) -> &Self::Target {
        &self.bp
    }
}

impl<'a> DerefMut for BreakpointMutRef<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bp
    }
}

impl Drop for BreakpointMutRef<'_> {
    fn drop(&mut self) {
        let bp = std::mem::replace(&mut self.bp, Breakpoint::new("", 0));
        self.points.points.insert(bp);
    }
}

#[cfg(test)]
mod tests {
    use crate::theme::breakpoint::Breakpoints;

    #[test]
    fn create_default() {
        let bps = Breakpoints::default();
        assert_eq!(bps.get("xs").unwrap().width(), 0);
        assert_eq!(bps.get("sm").unwrap().width(), 600);
        assert_eq!(bps.get("md").unwrap().width(), 768);
        assert_eq!(bps.get("lg").unwrap().width(), 992);
        assert_eq!(bps.get("xl").unwrap().width(), 1200);
    }
}
