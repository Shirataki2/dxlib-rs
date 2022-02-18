use dxlib::{dx3d::prelude::*, plugin::FullSceneAntiAliasPlugin, prelude::*};
use dxlib_sys::{dx_DrawPolygon3DToShader, Vertex3dShader as DxVertex3DShader};

fn main() -> anyhow::Result<()> {
    let mut app = Application::builder()
        .screen_size(640, 380)
        .title("プログラマブルシェーダの表示テスト")
        .direct3d(Direct3D::Dx9Ex)
        .add_plugin(BackgroundPlugin {
            run_always: true,
            color: Color::white(),
        })?
        .add_plugin(FullSceneAntiAliasPlugin {
            samples: 4,
            quality: 2,
        })?
        .build()?;
    let vertices = get_vertices();

    let mut texture = GraphicModel::load("./resources/images/lena.jpg")?;
    let mut vshader = VertexShader::load("./resources/shader/VertexShader.vso")?;
    let mut pshader = PixelShader::load("./resources/shader/PixelShader.pso")?;

    app.screen.set_draw_screen(DrawScreen::Back)?;

    let mut x = 0;
    let mut xd = 8;

    let mut color = 0.0;
    let mut colord = 1.0 / 60.0;

    while app.process_message().is_ok() && !KeyBoard::is_hit(Key::ESCAPE) {
        app.screen.clear()?;

        x += xd;
        if x > 200 || x < -200 {
            xd = -xd;
        }
        color += colord;
        if color <= 0.0 || color >= 1.0 {
            colord = -colord;
        }

        vshader.set_float4_const(0, Vector4::from([x as f32, 0.0, 0.0, 0.0]))?;
        pshader.set_float4_const(0, Vector4::from([color, color, color, 1.0]))?;
        texture.assign_shader_texture(0)?;
        let vertices = vertices
            .iter()
            .map(|v| DxVertex3DShader::from(*v))
            .collect::<Vec<_>>();
        unsafe { dx_DrawPolygon3DToShader(vertices.as_ptr(), 2) };

        app.screen.flip()?;
    }

    Ok(())
}

fn get_vertices() -> [Vertex3DShader; 6] {
    [
        Vertex3DShader {
            position: Vector3::from([220.0, 340.0, 0.0]),
            normal: Vector3::back(),
            diffuse: Color::magenta(),
            specular: Color::black(),
            uv: Vector2::from([0.0, 0.0]),
            ..Default::default()
        },
        Vertex3DShader {
            position: Vector3::from([420.0, 340.0, 0.0]),
            normal: Vector3::back(),
            diffuse: Color::blue(),
            specular: Color::black(),
            uv: Vector2::from([1.0, 0.0]),
            ..Default::default()
        },
        Vertex3DShader {
            position: Vector3::from([220.0, 140.0, 0.0]),
            normal: Vector3::back(),
            diffuse: Color::cyan(),
            specular: Color::black(),
            uv: Vector2::from([0.0, 1.0]),
            ..Default::default()
        },
        Vertex3DShader {
            position: Vector3::from([220.0, 140.0, 0.0]),
            normal: Vector3::back(),
            diffuse: Color::green(),
            specular: Color::black(),
            uv: Vector2::from([0.0, 1.0]),
            ..Default::default()
        },
        Vertex3DShader {
            position: Vector3::from([420.0, 340.0, 0.0]),
            normal: Vector3::back(),
            diffuse: Color::yellow(),
            specular: Color::black(),
            uv: Vector2::from([1.0, 0.0]),
            ..Default::default()
        },
        Vertex3DShader {
            position: Vector3::from([420.0, 140.0, 0.0]),
            normal: Vector3::back(),
            diffuse: Color::red(),
            specular: Color::black(),
            uv: Vector2::from([1.0, 1.0]),
            ..Default::default()
        },
    ]
}
