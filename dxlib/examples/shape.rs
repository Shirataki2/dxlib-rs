use dxlib::{application::Application, color::Color, shapes::*, vector::Vector2};

fn main() -> anyhow::Result<()> {
    let _app = Application::builder().screen_size(640, 480).build()?;

    Line {
        start: Vector2::from([0, 60]),
        end: Vector2::from([600, 60]),
        color: Color::red(),
        ..Default::default()
    }
    .draw()?;

    LineAntiAlias {
        start: Vector2::from([500.0, 0.0]),
        end: Vector2::from([0.0, 500.0]),
        color: Color::cyan(),
        ..Default::default()
    }
    .draw()?;

    Rect {
        left_top: Vector2::from([100, 100]),
        right_bottom: Vector2::from([540, 320]),
        color: Color::white(),
        ..Default::default()
    }
    .draw()?;

    unsafe {
        dxlib_sys::dx_WaitKey();
    }

    Ok(())
}
