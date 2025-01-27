
use core::f32;
use std::{ops::RangeInclusive, sync::{Arc, Mutex}};

mod mesh;
mod camera;

use mesh::Mesh;

use camera::Camera;
use eframe::{egui::{self, Rect}, egui_glow};
use egui::{pos2, vec2, InputState, Margin, ViewportBuilder};
use nalgebra::{Rotation3, Vector2, Vector3, Vector4, VectorView3};

mod shader;
use shader::ShaderProgram;
use voxel_manager::VoxelManager;

mod voxel_manager;




fn main() {
    let options = eframe::NativeOptions {
        multisampling: 4,
        renderer: eframe::Renderer::Glow,
        depth_buffer: 16,
        viewport: ViewportBuilder::default().with_min_inner_size(vec2(800.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "MeshView",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    ).expect("Failed to run MeshView")
}


// Main App UI

struct App {
    voxel_manager: VoxelManager,
    mesh: Arc<Mutex<Mesh>>,
    target: Option<(usize, usize)>,
    ghost: Arc<Mutex<Option<Mesh>>>,
    bounding_box: Arc<Mutex<Mesh>>,
    camera: Arc<Mutex<Camera>>,
    shader_program: Arc<Mutex<ShaderProgram>>,
    value: f32,
    angle: (f32, f32, f32),
    speed: f32
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //update mesh
        self.voxel_manager.update();
        self.mesh = Arc::new(Mutex::new(self.voxel_manager.get_mesh(_frame.gl().unwrap())));
        self.mesh.lock().unwrap().load_buffers(_frame.gl().unwrap());

        // raycast

        // let view_proj = self.camera.lock().unwrap().get_proj_view_mat();
        // let mut ray = view_proj * Vector4::new(0.0, 0.0, 0.0, 1.0);
        // ray = ray / ray.w;

        egui::TopBottomPanel::top("Top Panel")
            .frame(egui::Frame { inner_margin: 
                Margin { 
                    left: (10.0), right: (10.0), top: (10.0), bottom: (10.0) 
                }, 
                ..egui::Frame::default()
            })
            .show(ctx, |ui| { });

        egui::TopBottomPanel::bottom("BottomPanel")
            .frame(egui::Frame { inner_margin: 
                Margin { 
                    left: (10.0), right: (10.0), top: (8.0), bottom: (8.0) 
                }, 
                ..egui::Frame::default()
            })
            .show(ctx, |ui| {
            // ui.label(format!("Verts: {}", self.mesh.lock().unwrap().positions.len()));
            // ui.label(format!("Tris: {}", self.mesh.lock().unwrap().indicies.len()/3));

            // ui.collapsing("Visual Properties", |ui| {
                // if ui.toggle_value(&mut self.mesh.lock().unwrap().wireframe, "Wireframe").clicked() {    
                //     self.mesh.lock().unwrap().load_buffers(&_frame.gl().unwrap());
                // }
            // });

            ui.collapsing("Camera Controls", |ui| {
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(&mut self.angle.0).range(RangeInclusive::new(2.0, 20.0)));
                    ui.add(egui::DragValue::new(&mut self.angle.1).range(RangeInclusive::new(0.0, 360.0)));
                    ui.add(egui::DragValue::new(&mut self.angle.2).range(RangeInclusive::new(-80.0, 80.0)));
                });
                ui.label("Speed");
                ui.add(egui::Slider::new(&mut self.speed, RangeInclusive::new(0.0, 20.0)));
            });
        });

        let mut rect: Rect = Rect::from_pos(pos2(0.0, 0.0));
        egui::CentralPanel::default().show(ctx, |ui| {
            let _bounds = egui::Frame::canvas(ui.style()).show(ui, |ui| {
                self.custom_painting(ui);
                rect = ui.max_rect();
            });
        });


        if ctx.input(|i| i.pointer.button_clicked(egui::PointerButton::Primary) && rect.contains(i.pointer.latest_pos().unwrap())) {
            // println!("Space");
            match self.target {
                Some((x, z)) => {
                    self.voxel_manager.voxels[x][19][z] = true;
                },
                None => ()
            }
        }

        // if ctx.input(|i| i.pointer.button_clicked(egui::PointerButton::Primary) && rect.contains(i.pointer.latest_pos().unwrap())) {

        match ctx.pointer_latest_pos() {
            Some(norms) => {
                let norms = (((norms.x - rect.left_top().x) / rect.width()) * 2.0 - 1.0, -(((norms.y - rect.left_top().y) / rect.height()) * 2.0 - 1.0));
                
                // println!("{:?}", norms);

                let view_proj_inv = self.camera.lock().unwrap().get_proj_view_mat_inv();
                let mut ray = view_proj_inv * Vector4::new(norms.0, norms.1, 0.0, 1.0);
                ray = ray / ray.w;
                let dir = (Vector3::new(ray.x, ray.y, ray.z) - self.camera.lock().unwrap().pos).normalize();

                
                if ctx.input(|i| i.key_pressed(egui::Key::A)) {
                    println!("{}", dir);
                } 

                let temp = self.voxel_manager.get_ghost_mesh(_frame.gl().unwrap(), self.camera.lock().unwrap().pos, dir);
                self.target = temp.1.clone();

                self.ghost = Arc::new(Mutex::new(temp.0.clone()))

            },
            None => ()
        }

        // let hit = self.voxel_manager.ray_box_intersection(self.camera.lock().unwrap().pos, ray);
        // }
        

        let (r, mut theta, mut phi) = self.angle;
        phi = -phi;
        phi = phi.to_radians();
        theta = theta.to_radians();

        let look = -Vector3::new(phi.cos()* theta.cos(), phi.sin(), phi.cos()*theta.sin()).normalize();
        let right = (Rotation3::new(90.0_f32.to_radians() * Vector3::new(0.0, 1.0, 0.0)) * Vector3::new(look.x, 0.0, look.z)).normalize();

        // let look = rot * Vector3::new(0.0, 0.0, 1.0);
        // let right = rot * Vector3::new(1.0, 0.0, 0.0);
        // self.camera.lock().unwrap().pos = (-look * r) + Vector3::new((self.voxel_manager.width as f32 * voxel_manager::VOXEL_WIDTH) / 2.0, -(self.voxel_manager.height as f32 * voxel_manager::VOXEL_WIDTH / 2.0), (self.voxel_manager.length as f32 * voxel_manager::VOXEL_WIDTH / 2.0));
        self.camera.lock().unwrap().pos = -look * r;
        self.camera.lock().unwrap().right = right;
        self.camera.lock().unwrap().look = look;
        
        ctx.request_repaint();
    }
}


impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let gl = cc
            .gl
            .as_ref()
            .expect("You need to run eframe with the glow backend");

        let voxel_manager = VoxelManager::new(20, 20, 20);
        let mesh = voxel_manager.get_mesh(gl);
        let bounding_box = voxel_manager.get_bounding_box(gl);

        let shader_program = ShaderProgram::new(gl, "src/main.vert.glsl", "src/main.frag.glsl");
        
        let camera = Camera::default();
        
        Self { 
            voxel_manager, 
            mesh: Arc::new(Mutex::new(mesh)),
            target: None,
            ghost: Arc::new(Mutex::new(None)),
            bounding_box: Arc::new(Mutex::new(bounding_box)),
            shader_program: Arc::new(Mutex::new(shader_program)),
            camera: Arc::new(Mutex::new(camera)),
            value: 0.0,
            angle: (10.0, 0.0, 15.0),
            speed: 3.0
        }
    }

    fn custom_painting(&mut self, ui : &mut egui::Ui) {
        let (w, h) = (ui.available_width(), ui.available_height());

        let (rect, response) =
            ui.allocate_exact_size(egui::vec2(w, h) , egui::Sense::drag());

        self.camera.lock().unwrap().aspect_ratio = w / (h);


        let shader_program = self.shader_program.clone();
        let mesh = self.mesh.clone();
        let ghost = self.ghost.clone();
        let bounding_box = self.bounding_box.clone();
        let camera = self.camera.clone();

        self.angle.2 += response.drag_delta().y * 0.4;
        self.angle.1 += response.drag_delta().x * -0.4;
        self.angle.1 = (self.angle.1 + 360.0) % 360.0;
        
        self.angle.0 = self.angle.0.clamp(1.0, 20.0);
        self.angle.1 = self.angle.1.clamp(0.0, 359.9);
        self.angle.2 = self.angle.2.clamp(-80.0, 80.0);


        let value = self.value;

        let callback = egui::PaintCallback {
            rect,
            callback: std::sync::Arc::new(egui_glow::CallbackFn::new(move |_info: egui::PaintCallbackInfo, painter| {
                shader_program.lock().unwrap().paint(painter.gl(), &mesh.lock().unwrap(), &ghost.lock().unwrap(), &bounding_box.lock().unwrap(),  &camera.lock().unwrap());
            })),
        };
        ui.painter().add(callback);
    }
}