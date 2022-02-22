use dxlib::prelude::*;

const RESOLUTION: usize = 40;

fn main() -> anyhow::Result<()> {
    let mut app = Application::builder()
        .screen_size(640, 640)
        .title("テスト")
        .add_plugin(BackgroundPlugin {
            run_always: true,
            color: Color::white(),
        })?
        .build()?;
    app.screen.set_draw_screen(DrawScreen::Back)?;

    let mut builder = Font::builder();
    let font = builder
        .name("游明朝")
        .font_type(FontType::AntiAliasing4x4)
        .size(30)
        .thickness(3)
        .build()?;

    let mut builder = TextStyle::builder();
    let style = builder
        .coord(Vector::from([70, 210]))
        .color(Color::white())
        .font(&font)
        .build();

    let mut image = GraphicModel::load("./resources/images/lena.jpg")?;
    image.position = Vector::from([200, 200]);
    image.pivot = Vector::from([110, 110]);
    let mut t = 0;
    while app.process_message().is_ok() && !KeyBoard::is_hit(Key::ESCAPE) {
        app.screen.clear()?;
        draw_background(t)?;
        image.rotation += Angle::from_radians(0.05);
        image.draw()?;
        style.draw("HELLO, World")?;
        app.screen.flip()?;
        t += 1;
    }

    Ok(())
}

#[allow(clippy::many_single_char_names)]
fn draw_background(t: usize) -> anyhow::Result<()> {
    let r = RESOLUTION;
    let k = 640;
    for i in 0..r {
        for j in 0..r {
            let (p, q) = (((i + t) % k) as f32, ((j + t) % k) as f32);
            let (pp, qq) = (((i + 2 * t) % k) as f32, ((j + 2 * t) % k) as f32);
            let (i, j) = (i as f32, j as f32);
            let r = r as f32;
            let k = k as f32;
            let w = 640.0 / r;
            let h = 640.0 / r;
            let pi = std::f32::consts::PI;
            RectAntiAlias::from_coords(
                j * w,
                i * h,
                (j + 1.0) * w + 1.0,
                (i + 1.0) * h + 1.0,
                Color::new(
                    (255.0 * (2.0 * pi * p * q / k.powf(2.0)).sin().powf(2.0)) as u8,
                    (255.0 * (2.0 * pi * pp * q / k.powf(2.0)).sin().powf(2.0)) as u8,
                    (255.0 * (2.0 * pi * qq * pp / k.powf(2.0)).sin().powf(2.0)) as u8,
                    255,
                ),
                true,
                0.0,
            )
            .draw()?;
        }
    }
    Ok(())
}
