use eframe::egui;

use crate::application::use_cases::{ExecuteCommandUseCase, HandleInputUseCase};
use crate::domain::repositories::CommandRepository;
use crate::presentation::texts::WelcomeMessages;
use crate::presentation::input_handler::InputHandler;
use crate::presentation::ui_renderer::UIRenderer;
use crate::presentation::system_monitor::SystemMonitor;
use crate::presentation::matrix_manager::MatrixManager;
use crate::infrastructure::logging::log_application_shutdown;

pub struct MatrixTerminalApp<R>
where
    R: CommandRepository,
{
    input_handler: InputHandler<R>,
    ui_renderer: UIRenderer,
    system_monitor: SystemMonitor,
    matrix_manager: MatrixManager,
    terminal_width: f32,
    terminal_height: f32,
    input_focused: bool,
}

impl<R> MatrixTerminalApp<R>
where
    R: CommandRepository,
{
    pub fn new(
        input_handler: HandleInputUseCase,
        command_executor: ExecuteCommandUseCase<R>,
    ) -> Self {
        let terminal_width = 1200.0;
        let terminal_height = 800.0;
        
        let mut app = Self {
            input_handler: InputHandler::new(input_handler, command_executor),
            ui_renderer: UIRenderer::new(),
            system_monitor: SystemMonitor::new(),
            matrix_manager: MatrixManager::new(terminal_width, terminal_height),
            terminal_width,
            terminal_height,
            input_focused: true,
        };
        
        app.initialize_welcome_messages();
        log::info!("Aplicación MatrixTerminalApp inicializada correctamente");
        app
    }

    fn initialize_welcome_messages(&mut self) {
        for message in WelcomeMessages::all_welcome_messages() {
            self.input_handler.get_input_handler_mut().add_output_line(message);
        }
    }

    fn update_dimensions(&mut self, width: f32, height: f32) {
        self.terminal_width = width;
        self.terminal_height = height;
        self.ui_renderer.update_dimensions(width, height);
        self.matrix_manager.update_dimensions(width, height);
    }

    fn draw_matrix_mode(&mut self, ui: &mut egui::Ui, _input_processed: bool) {
        let (rect, response) = ui.allocate_exact_size(
            ui.available_size(),
            egui::Sense::click_and_drag(),
        );

        let painter = ui.painter();
        painter.rect_filled(rect, 0.0, egui::Color32::BLACK);
        
        let last_input_time = self.input_handler.get_last_input_time();
        self.matrix_manager.draw_matrix_effects(&painter, last_input_time);

        if response.clicked() || !self.input_focused {
            response.request_focus();
            self.input_focused = true;
        }
        
        self.draw_ui_layer(&painter);
    }

    fn draw_ui_layer(&self, painter: &egui::Painter) {
        let input_handler = self.input_handler.get_input_handler();
        let history_commands = self.input_handler.get_history_commands();
        let last_input_time = self.input_handler.get_last_input_time();
        let system_stats = self.system_monitor.get_system_stats();
        
        self.ui_renderer.draw_command_history(painter, input_handler, history_commands);
        self.ui_renderer.draw_prompt(painter, input_handler, last_input_time);
        self.ui_renderer.draw_system_indicators(painter, system_stats);
    }
}

impl<R> Drop for MatrixTerminalApp<R>
where
    R: CommandRepository,
{
    fn drop(&mut self) {
        log_application_shutdown();
    }
}

impl<R> eframe::App for MatrixTerminalApp<R>
where
    R: CommandRepository,
{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let available_size = ctx.available_rect().size();
        self.update_dimensions(available_size.x, available_size.y);

        let input = ctx.input(|i| i.clone());
        let input_processed = self.input_handler.handle_keyboard_input(&input);

        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_matrix_mode(ui, input_processed);
        });

        ctx.request_repaint();
    }
}
