#[derive(Debug)]
struct Dimensions {
    l: u32,
    w: u32,
    h: u32,
}

impl Dimensions {
    fn volume(&self) -> u32 {
        self.l * self.w * self.h
    }

    fn smallest_perimeter(&self) -> u32 {
        (self.l * 2 + self.w * 2)
            .min(self.l * 2 + self.h * 2)
            .min(self.w * 2 + self.h * 2)
    }
}

#[derive(Debug)]
struct Sides {
    lw: u32,
    lh: u32,
    wh: u32,
}

impl Sides {
    fn smallest(&self) -> u32 {
        self.lw.min(self.lh).min(self.wh)
    }
}

fn main() {
    let input = include_str!("input.txt");
    let dimensions: Vec<Dimensions> = input
        .lines()
        .map(|line| {
            let parts: Vec<u32> = line
                .split('x')
                .map(|s| s.parse().expect("Invalid input"))
                .collect();
            assert_eq!(parts.len(), 3);
            Dimensions {
                l: parts[0],
                w: parts[1],
                h: parts[2],
            }
        })
        .collect();

    let wrapping_paper: u32 = (&dimensions)
        .iter()
        .map(|Dimensions { l, w, h }| Sides {
            lw: l * w,
            lh: l * h,
            wh: w * h,
        })
        .map(|sides| 2 * sides.lw + 2 * sides.lh + 2 * sides.wh + sides.smallest())
        .sum();

    let ribbon: u32 = (&dimensions)
        .iter()
        .map(|dimension| dimension.smallest_perimeter() + dimension.volume())
        .sum();

    println!("Wrapping Paper {:?}, Ribbon: {:?}", wrapping_paper, ribbon);
}
