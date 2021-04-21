use dxlib::prelude::*;

const RESOLUTION: usize = 100;

fn main() -> anyhow::Result<()> {
    let mut app = Application::builder()
        .screen_size(640, 480)
        .title("テスト")
        .add_plugin(BackgroundPlugin {
            run_always: true,
            color: Color::white(),
        })?
        .build()?;
    app.screen.set_draw_screen(DrawScreen::Back)?;

    // let font = Font::builder()
    //     .font_type(FontType::AntiAliasing4x4)
    //     .size(30)
    //     .thickness(3)
    //     .build()?;

    // let style = TextStyle::builder()
    //     .coord(Vector::from([70, 210]))
    //     .color(Color::white())
    //     .font(font)
    //     .build();

    let mut image = GraphicModel::load("lena.png")?;
    image.position = Vector::from([200, 200]);
    image.pivot = Vector::from([110, 110]);
    while app.update().is_ok() && !KeyBoard::is_hit(Key::ESCAPE) {
        draw_background()?;
        image.rotation += Angle::from_radians(0.05);
        image.draw()?;
        // style.draw("HELLO, World")?;
    }

    Ok(())
}

#[allow(clippy::many_single_char_names)]
fn draw_background() -> anyhow::Result<()> {
    let r = RESOLUTION;
    for i in 0..r {
        for j in 0..r {
            let (i, j) = (i as f32, j as f32);
            let r = r as f32;
            let w = 640.0 / r;
            let h = 480.0 / r;
            RectAntiAlias::from_coords(
                j * w,
                i * h,
                (j + 1.0) * w + 1.0,
                (i + 1.0) * h + 1.0,
                Color::new(
                    (255.0 * (1.0 - j as f32 / r)) as u8,
                    (255.0 * (i as f32 / r)) as u8,
                    (255.0 * (1.0 - 2.0 * (j * i) as f32 / (r * r))).abs() as u8,
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
