use clap::Parser;

struct SimpleTriangle {
    base: f32,
    height: f32,
}

impl SimpleTriangle {
    fn get_area(&self) -> f32 {
        (&self.base * &self.height) / 2.0
    }
}

struct SidesTriangle {
    side_a: f32,
    side_b: f32,
    side_c: f32,
}

impl SidesTriangle {
    fn get_semi_perimeter(&self) -> f32 {
        (&self.side_a + &self.side_b + &self.side_c) / 2.0
    }

    fn get_area(&self) -> f32 {
        let p: f32 = self.get_semi_perimeter();
        let sides_and_p: f32 = p * (p - &self.side_a) * (p - &self.side_b) * (p - &self.side_c);

        return sides_and_p.sqrt();
    }
}

pub fn triangle_actions(
    base: Option<f32>,
    height: Option<f32>,

    side_a: Option<f32>,
    side_b: Option<f32>,
    side_c: Option<f32>,
) {
    if let (Some(base), Some(height)) = (base, height) {
        let triangle = SimpleTriangle { base, height };

        println!("Area: {}cm", triangle.get_area());
    } else if let (Some(side_a), Some(side_b), Some(side_c)) = (side_a, side_b, side_c) {
        let triangle = SidesTriangle {
            side_a,
            side_b,
            side_c,
        };

        println!("Area: {}cm", triangle.get_area());
        println!("Perimeter: {}cm", triangle.get_semi_perimeter());
    }
}

#[derive(Parser)]
#[command(about="Mathematical operations with triangles", long_about = None)]
pub struct Command {
    #[arg(short, long, help = "Sets the Base of the triangle | e.g. -b 5")]
    pub base: Option<f32>,
    #[arg(
        short = 'e',
        long,
        help = "Sets the hEight of the triangle | e.g. -e 5"
    )]
    pub height: Option<f32>,

    #[arg(short = None, long="side-a", help = "Sets the side A of the triangle | e.g. --side-a 5")]
    pub side_a: Option<f32>,
    #[arg(short = None, long="side-b", help = "Sets the side B of the triangle | e.g. --side-b 5")]
    pub side_b: Option<f32>,
    #[arg(short = None, long="side-c", help = "Sets the side c of the triangle | e.g. --side-c 5")]
    pub side_c: Option<f32>,
}

#[cfg(test)]
mod simple_triangle {
    #[test]
    fn get_area() {
        let triangle = super::SimpleTriangle {
            base: 42.0,
            height: 42.0,
        };

        assert_eq!(triangle.get_area(), 882.0);
    }
}

#[cfg(test)]
mod sides_triangle {
    #[test]
    fn get_semi_perimeter() {
        let triangle = super::SidesTriangle {
            side_a: 42.0,
            side_b: 42.0,
            side_c: 42.0,
        };

        assert_eq!(triangle.get_semi_perimeter(), 63.0);
    }

    #[test]
    fn get_area() {
        let triangle = super::SidesTriangle {
            side_a: 42.0,
            side_b: 42.0,
            side_c: 42.0,
        };

        assert_eq!(triangle.get_area(), 763.8344);
    }
}
