use super::{
    number_type::NumberType, point_2::Point2, segment_2::Segment2, util_enum::Segment2Type,
};

#[derive(Debug, Clone, Copy)]
pub struct LineSegment2<T: NumberType> {
    source: Point2<T>,
    target: Point2<T>,
}

impl<T: NumberType> LineSegment2<T> {
    pub fn new(source: Point2<T>, target: Point2<T>) -> Self {
        Self { source, target }
    }
}

impl<T: NumberType> Segment2<T> for LineSegment2<T> {
    fn source(&self) -> Point2<T> {
        self.source.clone()
    }

    fn source_radian(&self) -> T {
        panic!("LineSegment2 does not have a source radian")
    }

    fn target(&self) -> Point2<T> {
        self.target.clone()
    }

    fn target_radian(&self) -> T {
        panic!("LineSegment2 does not have a target radian")
    }

    fn segment_type(&self) -> Segment2Type {
        return Segment2Type::LineSegment2;
    }

    fn center(&self) -> Point2<T> {
        panic!("LineSegment2 does not have a center point")
    }

    fn radius(&self) -> T {
        panic!("LineSegment2 does not have a radius")
    }
}

impl<T: NumberType> PartialEq for LineSegment2<T> {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source && self.target == other.target
    }
}
