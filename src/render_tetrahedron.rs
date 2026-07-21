use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, VertexAttributeValues},
    prelude::*,
    render::render_resource::PrimitiveTopology,
};

#[derive(Component)]
pub struct Tetrahedron;

pub fn setup_tetra(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let cube_mesh_handle: Handle<Mesh> = meshes.add(create_tetraheedron());

    commands.spawn((
        Mesh3d(cube_mesh_handle),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            ..default()
        })),
        Tetrahedron,
    ));

    commands.spawn((
        PointLight {
            shadow_maps_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(8.0, 16.0, 8.0),
    ));

    // ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10))),
        MeshMaterial3d(materials.add(Color::from(Srgba::hex("B4E1EB").unwrap())))
    ));

    // commands.spawn((
    //     Camera3d::default(),
    //     Transform::from_xyz(0.0,9., 10.0).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
    // ));
}


fn create_tetraheedron() -> Mesh{
    let s3 = 3.0_f32.sqrt();
    let s6 = 6.0_f32.sqrt();

    // 頂点を変数に入れておく。3頂点ずつが1つの面（=三角形1枚）。
    let positions: Vec<[f32; 3]> = vec![
        // 底面
        [0.0, 0.0, 2.0 * s3 / 3.0],
        [-1.0, 0.0, -s3 / 3.0],
        [1.0, 0.0, -s3 / 3.0],
        // 右面
        [0.0, 0.0, 2.0 * s3 / 3.0],
        [1.0, 0.0, -s3 / 3.0],
        [0.0, 2.0 * s6 / 3.0, 0.0],
        // 左面
        [-1.0, 0.0, -s3 / 3.0],
        [0.0, 0.0, 2.0 * s3 / 3.0],
        [0.0, 2.0 * s6 / 3.0, 0.0],
        // 背面
        [1.0, 0.0, -s3 / 3.0],
        [-1.0, 0.0, -s3 / 3.0],
        [0.0, 2.0 * s6 / 3.0, 0.0],
    ];

    let face_colors = [
        [1.0, 0.0, 0.0, 1.0], // 底面 赤
        [0.0, 1.0, 0.0, 1.0], // 前面 緑
        [0.0, 0.0, 1.0, 1.0], // 左面 青
        [1.0, 1.0, 0.0, 1.0], // 右面 黄
    ];
    let colors: Vec<[f32; 4]> = face_colors.iter().flat_map(|&c| [c, c, c]).collect();

    // 3頂点ごとに face_normal で法線を1つ計算し、その面の3頂点すべてに同じ値を割り当てる。
    let normals: Vec<[f32; 3]> = positions
        .chunks(3)
        .flat_map(|face| {
            let n = face_normal(face[0], face[1], face[2]);
            [n, n, n]
        })
        .collect::<Vec<[f32; 3]>>();

    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
}

fn face_normal(a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> [f32; 3] {
    let ab = [b[0]-a[0], b[1]-a[1], b[2]-a[2]];
    let ac = [c[0]-a[0], c[1]-a[1], c[2]-a[2]];
    let n = [
        ab[1]*ac[2] - ab[2]*ac[1],
        ab[2]*ac[0] - ab[0]*ac[2],
        ab[0]*ac[1] - ab[1]*ac[0],
    ];
    let len = (n[0]*n[0] + n[1]*n[1] + n[2]*n[2]).sqrt();
    [n[0]/len, n[1]/len, n[2]/len]
}

pub fn rotate_tetra(mut query: Query<&mut Transform, With<Tetrahedron>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs() / 2.);
    }
}