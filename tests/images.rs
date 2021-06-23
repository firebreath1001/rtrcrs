use rtrcrs::color::{anti_aliased, Color};
const IMAGE_HEIGHT: i32 = 10;
const IMAGE_WIDTH: i32 = 10;

#[test]
fn first_integration_test() {
    let mut v = vec![];
    for j in (0..IMAGE_HEIGHT).into_iter().rev() {
        for i in 0..IMAGE_HEIGHT {
            let r = (i as f64) / (IMAGE_HEIGHT - 1) as f64;
            let g = (j as f64) / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.25;

            let ir = 255.999 * r;
            let ig = 255.999 * g;
            let ib = 255.999 * b;

            v.push(Color::new(ir, ig, ib));
        }
    }

    let expected = vec![
        Color::new(0.0, 255.999, 63.99975),
        Color::new(28.444333333333333, 255.999, 63.99975),
        Color::new(56.888666666666666, 255.999, 63.99975),
        Color::new(85.333, 255.999, 63.99975),
        Color::new(113.77733333333333, 255.999, 63.99975),
        Color::new(142.22166666666666, 255.999, 63.99975),
        Color::new(170.666, 255.999, 63.99975),
        Color::new(199.11033333333333, 255.999, 63.99975),
        Color::new(227.55466666666666, 255.999, 63.99975),
        Color::new(255.999, 255.999, 63.99975),
        Color::new(0.0, 227.55466666666666, 63.99975),
        Color::new(28.444333333333333, 227.55466666666666, 63.99975),
        Color::new(56.888666666666666, 227.55466666666666, 63.99975),
        Color::new(85.333, 227.55466666666666, 63.99975),
        Color::new(113.77733333333333, 227.55466666666666, 63.99975),
        Color::new(142.22166666666666, 227.55466666666666, 63.99975),
        Color::new(170.666, 227.55466666666666, 63.99975),
        Color::new(199.11033333333333, 227.55466666666666, 63.99975),
        Color::new(227.55466666666666, 227.55466666666666, 63.99975),
        Color::new(255.999, 227.55466666666666, 63.99975),
        Color::new(0.0, 199.11033333333333, 63.99975),
        Color::new(28.444333333333333, 199.11033333333333, 63.99975),
        Color::new(56.888666666666666, 199.11033333333333, 63.99975),
        Color::new(85.333, 199.11033333333333, 63.99975),
        Color::new(113.77733333333333, 199.11033333333333, 63.99975),
        Color::new(142.22166666666666, 199.11033333333333, 63.99975),
        Color::new(170.666, 199.11033333333333, 63.99975),
        Color::new(199.11033333333333, 199.11033333333333, 63.99975),
        Color::new(227.55466666666666, 199.11033333333333, 63.99975),
        Color::new(255.999, 199.11033333333333, 63.99975),
        Color::new(0.0, 170.666, 63.99975),
        Color::new(28.444333333333333, 170.666, 63.99975),
        Color::new(56.888666666666666, 170.666, 63.99975),
        Color::new(85.333, 170.666, 63.99975),
        Color::new(113.77733333333333, 170.666, 63.99975),
        Color::new(142.22166666666666, 170.666, 63.99975),
        Color::new(170.666, 170.666, 63.99975),
        Color::new(199.11033333333333, 170.666, 63.99975),
        Color::new(227.55466666666666, 170.666, 63.99975),
        Color::new(255.999, 170.666, 63.99975),
        Color::new(0.0, 142.22166666666666, 63.99975),
        Color::new(28.444333333333333, 142.22166666666666, 63.99975),
        Color::new(56.888666666666666, 142.22166666666666, 63.99975),
        Color::new(85.333, 142.22166666666666, 63.99975),
        Color::new(113.77733333333333, 142.22166666666666, 63.99975),
        Color::new(142.22166666666666, 142.22166666666666, 63.99975),
        Color::new(170.666, 142.22166666666666, 63.99975),
        Color::new(199.11033333333333, 142.22166666666666, 63.99975),
        Color::new(227.55466666666666, 142.22166666666666, 63.99975),
        Color::new(255.999, 142.22166666666666, 63.99975),
        Color::new(0.0, 113.77733333333333, 63.99975),
        Color::new(28.444333333333333, 113.77733333333333, 63.99975),
        Color::new(56.888666666666666, 113.77733333333333, 63.99975),
        Color::new(85.333, 113.77733333333333, 63.99975),
        Color::new(113.77733333333333, 113.77733333333333, 63.99975),
        Color::new(142.22166666666666, 113.77733333333333, 63.99975),
        Color::new(170.666, 113.77733333333333, 63.99975),
        Color::new(199.11033333333333, 113.77733333333333, 63.99975),
        Color::new(227.55466666666666, 113.77733333333333, 63.99975),
        Color::new(255.999, 113.77733333333333, 63.99975),
        Color::new(0.0, 85.333, 63.99975),
        Color::new(28.444333333333333, 85.333, 63.99975),
        Color::new(56.888666666666666, 85.333, 63.99975),
        Color::new(85.333, 85.333, 63.99975),
        Color::new(113.77733333333333, 85.333, 63.99975),
        Color::new(142.22166666666666, 85.333, 63.99975),
        Color::new(170.666, 85.333, 63.99975),
        Color::new(199.11033333333333, 85.333, 63.99975),
        Color::new(227.55466666666666, 85.333, 63.99975),
        Color::new(255.999, 85.333, 63.99975),
        Color::new(0.0, 56.888666666666666, 63.99975),
        Color::new(28.444333333333333, 56.888666666666666, 63.99975),
        Color::new(56.888666666666666, 56.888666666666666, 63.99975),
        Color::new(85.333, 56.888666666666666, 63.99975),
        Color::new(113.77733333333333, 56.888666666666666, 63.99975),
        Color::new(142.22166666666666, 56.888666666666666, 63.99975),
        Color::new(170.666, 56.888666666666666, 63.99975),
        Color::new(199.11033333333333, 56.888666666666666, 63.99975),
        Color::new(227.55466666666666, 56.888666666666666, 63.99975),
        Color::new(255.999, 56.888666666666666, 63.99975),
        Color::new(0.0, 28.444333333333333, 63.99975),
        Color::new(28.444333333333333, 28.444333333333333, 63.99975),
        Color::new(56.888666666666666, 28.444333333333333, 63.99975),
        Color::new(85.333, 28.444333333333333, 63.99975),
        Color::new(113.77733333333333, 28.444333333333333, 63.99975),
        Color::new(142.22166666666666, 28.444333333333333, 63.99975),
        Color::new(170.666, 28.444333333333333, 63.99975),
        Color::new(199.11033333333333, 28.444333333333333, 63.99975),
        Color::new(227.55466666666666, 28.444333333333333, 63.99975),
        Color::new(255.999, 28.444333333333333, 63.99975),
        Color::new(0.0, 0.0, 63.99975),
        Color::new(28.444333333333333, 0.0, 63.99975),
        Color::new(56.888666666666666, 0.0, 63.99975),
        Color::new(85.333, 0.0, 63.99975),
        Color::new(113.77733333333333, 0.0, 63.99975),
        Color::new(142.22166666666666, 0.0, 63.99975),
        Color::new(170.666, 0.0, 63.99975),
        Color::new(199.11033333333333, 0.0, 63.99975),
        Color::new(227.55466666666666, 0.0, 63.99975),
        Color::new(255.999, 0.0, 63.99975),
    ];
    assert_eq!(v, expected);
}