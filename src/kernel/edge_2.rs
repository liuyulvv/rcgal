use std::{cell::RefCell, rc::Rc};

use super::{
    arc_2::Arc2, face_2::Face2, number_type::NumberType, segment_2::Segment2, vertex_2::Vertex2,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Edge2Type {
    Segment,
    Arc,
}

#[derive(Debug, Clone)]
pub struct Edge2<NT: NumberType> {
    source: Rc<RefCell<Vertex2<NT>>>,
    target: Rc<RefCell<Vertex2<NT>>>,
    twin: Option<Rc<RefCell<Self>>>,
    next: Option<Rc<RefCell<Self>>>,
    prev: Option<Rc<RefCell<Self>>>,
    face: Option<Rc<RefCell<Face2<NT>>>>,
    edge_type: Edge2Type,
}

impl<NT: NumberType> Edge2<NT> {
    pub fn new_segment(source: Rc<RefCell<Vertex2<NT>>>, target: Rc<RefCell<Vertex2<NT>>>) -> Self {
        if source > target {
            Self {
                source,
                target,
                twin: None,
                next: None,
                prev: None,
                face: None,
                edge_type: Edge2Type::Segment,
            }
        } else {
            Self {
                source: target,
                target: source,
                twin: None,
                next: None,
                prev: None,
                face: None,
                edge_type: Edge2Type::Segment,
            }
        }
    }

    pub fn new_arc(source: Rc<RefCell<Vertex2<NT>>>, target: Rc<RefCell<Vertex2<NT>>>) -> Self {
        Self {
            source,
            target,
            twin: None,
            next: None,
            prev: None,
            face: None,
            edge_type: Edge2Type::Arc,
        }
    }

    pub fn source(&self) -> Rc<RefCell<Vertex2<NT>>> {
        self.source.clone()
    }

    pub fn set_source(&mut self, source: Rc<RefCell<Vertex2<NT>>>) {
        self.source = source
    }

    pub fn target(&self) -> Rc<RefCell<Vertex2<NT>>> {
        self.target.clone()
    }

    pub fn set_target(&mut self, target: Rc<RefCell<Vertex2<NT>>>) {
        self.target = target
    }

    pub fn twin(&self) -> Option<Rc<RefCell<Self>>> {
        self.twin.clone()
    }

    pub fn set_twin(&mut self, twin: Rc<RefCell<Self>>) {
        self.twin = Some(twin)
    }

    pub fn next(&self) -> Option<Rc<RefCell<Self>>> {
        self.next.clone()
    }

    pub fn set_next(&mut self, next: Rc<RefCell<Self>>) {
        self.next = Some(next)
    }

    pub fn prev(&self) -> Option<Rc<RefCell<Self>>> {
        self.prev.clone()
    }

    pub fn set_prev(&mut self, prev: Rc<RefCell<Self>>) {
        self.prev = Some(prev)
    }

    pub fn face(&self) -> Option<Rc<RefCell<Face2<NT>>>> {
        self.face.clone()
    }

    pub fn set_face(&mut self, face: Rc<RefCell<Face2<NT>>>) {
        self.face = Some(face.clone())
    }

    pub fn edge_type(&self) -> Edge2Type {
        self.edge_type
    }

    pub fn to_segment(&self) -> Segment2<NT> {
        let source = self.source.borrow();
        let target = self.target.borrow();
        Segment2::new(source.to_point(), target.to_point())
    }

    pub fn to_arc(&self) -> Arc2<NT> {
        todo!()
    }
}

impl<NT: NumberType> PartialEq for Edge2<NT> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}