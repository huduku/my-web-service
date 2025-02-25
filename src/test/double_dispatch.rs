

trait Visitor {
    fn visit_circle(&self, circle: &Circle);
    fn visit_rectangle(&self, rectangle: &Rectangle);
}

trait Visitable {
    fn accept(&self, visitor: &dyn Visitor);
}

struct Circle {
    radius: f64,
}

impl Visitable for Circle {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_circle(self);
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Visitable for Rectangle {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_rectangle(self);
    }
}

struct AreaCalculator;

impl Visitor for AreaCalculator {
    fn visit_circle(&self, circle: &Circle) {
        let area = std::f64::consts::PI * circle.radius * circle.radius;
        println!("Circle Area: {:.2}", area);
    }

    fn visit_rectangle(&self, rectangle: &Rectangle) {
        let area = rectangle.width * rectangle.height;
        println!("Rectangle Area: {:.2}", area);
    }
}



mod test {
    use crate::test::double_dispatch::{AreaCalculator, Circle, Rectangle, Visitable};

    #[test]
    fn it_works() {
        let circle = Circle { radius: 5.0 };
        let rectangle = Rectangle { width: 4.0, height: 6.0 };

        let calculator = AreaCalculator;

        let shapes: Vec<&dyn Visitable> = vec![&circle, &rectangle];

        for shape in shapes {
            shape.accept(&calculator);
        }
    }
}
