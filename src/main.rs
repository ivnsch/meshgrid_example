use ndarray::{Array2, Array3, Axis};

fn main() {
    let a = 1.;
    let b = 2.;
    let N = 2;

    let x = linspace(0., a, N + 1);
    let y = linspace(0., b, N + 1);
    let z = linspace(0., b, N + 1);

    println!("x: {:?}", x);
    println!("y: {:?}", y);

    // 2d
    let (x_mesh, y_mesh) = meshgrid2(&x, &y);
    println!("x mesh: {:?}", x_mesh);
    println!("y mesh: {:?}", y_mesh);

    let f = x_mesh.sin() * y_mesh.cos();
    println!("f: {:?}", f);

    // 3d
    let (x_mesh, y_mesh, z_mesh) = meshgrid3(&x, &y, &z);
    println!("x mesh: {:?}", x_mesh);
    println!("y mesh: {:?}", y_mesh);
    println!("z mesh: {:?}", z_mesh);
}

pub fn meshgrid2(x: &[f32], y: &[f32]) -> (Array2<f32>, Array2<f32>) {
    let nx = x.len();
    let ny = y.len();

    let mut x_grid = Array2::<f32>::zeros((nx, ny));
    let mut y_grid = Array2::<f32>::zeros((nx, ny));

    for (i, &xi) in x.iter().enumerate() {
        x_grid.index_axis_mut(Axis(1), i).fill(xi);
    }

    for (j, &yj) in y.iter().enumerate() {
        y_grid.index_axis_mut(Axis(0), j).fill(yj);
    }

    (x_grid, y_grid)
}

pub fn meshgrid3(x: &[f32], y: &[f32], z: &[f32]) -> (Array3<f32>, Array3<f32>, Array3<f32>) {
    let nx = x.len();
    let ny = y.len();
    let nz = z.len();

    let mut x_grid = Array3::<f32>::zeros((nx, ny, nz));
    let mut y_grid = Array3::<f32>::zeros((nx, ny, nz));
    let mut z_grid = Array3::<f32>::zeros((nx, ny, nz));

    for (i, &xi) in x.iter().enumerate() {
        x_grid.index_axis_mut(Axis(1), i).fill(xi);
    }

    for (j, &yj) in y.iter().enumerate() {
        y_grid.index_axis_mut(Axis(0), j).fill(yj);
    }

    for (k, &zk) in z.iter().enumerate() {
        z_grid.index_axis_mut(Axis(2), k).fill(zk);
    }

    (x_grid, y_grid, z_grid)
}

fn linspace(start: f32, stop: f32, num: usize) -> Vec<f32> {
    if num < 2 {
        return vec![start];
    }
    let step = (stop - start) / (num as f32 - 1.0);
    (0..num).map(|i| start + step * i as f32).collect()
}
