use dxlib::prelude::*;

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

    RectAntiAlias {
        left_top: Vector2::from([100.0, 100.0]),
        right_bottom: Vector2::from([540.0, 320.0]),
        color: Color::white(),
        ..Default::default()
    }
    .draw()?;

    CircleAntiAlias {
        center: Vector2::from([300.0, 200.0]),
        radius: 100.0,
        color: Color::green(),
        thickness: 10.0,
        ..Default::default()
    }
    .draw()?;

    wait_any_key()?;

    Ok(())
}
