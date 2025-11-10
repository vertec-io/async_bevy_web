/// Example: Deterministic Robotics Control with Fixed Timestep
/// 
/// This example demonstrates how to use async_bevy_web for robotics applications
/// that require deterministic timing. It shows:
/// 
/// 1. Fixed timestep configuration for deterministic control loops
/// 2. Async I/O with hardware (simulated PLC communication)
/// 3. Synchronization between async tasks and the main ECS loop
/// 4. Proper separation of concerns: async I/O vs deterministic logic
/// 
/// Architecture:
/// - Main Bevy loop runs at 20 Hz (50ms) with FIXED timestep
/// - Control logic in FixedUpdate schedule (deterministic)
/// - Async I/O tasks communicate with hardware via Tokio
/// - Data flows: Hardware -> Tokio -> Bevy ECS -> Control Logic -> Tokio -> Hardware

use async_bevy_web::prelude::*;
use bevy::prelude::*;
use std::time::Duration;

fn main() {
    App::new()
        // Use fixed timestep at 20 Hz for deterministic control
        // This ensures control logic runs at exactly 50ms intervals
        .add_plugins(ABWConfigPlugin::fixed(20.0))
        
        // Initialize resources
        .init_resource::<SensorData>()
        .init_resource::<ControlOutput>()
        
        // Startup: spawn async I/O tasks
        .add_systems(Startup, (
            print_startup_info,
            spawn_sensor_reader,
            spawn_actuator_writer,
        ))
        
        // FixedUpdate: deterministic control logic runs at exactly 20 Hz
        .add_systems(FixedUpdate, (
            process_sensor_data,
            run_control_algorithm,
            prepare_actuator_commands,
        ).chain())
        
        // Update: non-critical monitoring (runs at variable rate)
        .add_systems(Update, monitor_system_health)
        
        .run();
}

// ============================================================================
// Resources - Shared data between async tasks and ECS
// ============================================================================

#[derive(Resource, Default)]
struct SensorData {
    position: f32,
    velocity: f32,
    timestamp: u64,
}

#[derive(Resource, Default)]
struct ControlOutput {
    target_position: f32,
    motor_command: f32,
}

// ============================================================================
// Startup Systems
// ============================================================================

fn print_startup_info() {
    println!("=== Robotics Fixed Timestep Example ===");
    println!("Control loop: 20 Hz (50ms fixed timestep)");
    println!("Time mode: Fixed (deterministic)");
    println!("Starting async I/O tasks...\n");
}

fn spawn_sensor_reader(runtime: Res<TokioTasksRuntime>) {
    runtime.spawn_background_task(|mut ctx| async move {
        // High-frequency sensor reading at 100 Hz (10ms)
        let mut interval = tokio::time::interval(Duration::from_millis(10));
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        
        let mut tick_count = 0u64;
        
        loop {
            interval.tick().await;
            tick_count += 1;
            
            // Simulate reading from hardware (PLC, sensor, etc.)
            let position = simulate_sensor_reading(tick_count);
            let velocity = simulate_velocity_reading(tick_count);
            
            // Send data to Bevy ECS (runs on main thread)
            ctx.run_on_main_thread(move |ctx| {
                let mut sensor_data = ctx.world.resource_mut::<SensorData>();
                sensor_data.position = position;
                sensor_data.velocity = velocity;
                sensor_data.timestamp = tick_count;
            }).await;
            
            // Every 100 ticks (1 second), print status
            if tick_count % 100 == 0 {
                println!("[Sensor Task] Read {} samples", tick_count);
            }
        }
    });
}

fn spawn_actuator_writer(runtime: Res<TokioTasksRuntime>) {
    runtime.spawn_background_task(|mut ctx| async move {
        // Actuator commands at 50 Hz (20ms) - faster than control loop
        let mut interval = tokio::time::interval(Duration::from_millis(20));
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        
        loop {
            interval.tick().await;
            
            // Read control output from Bevy ECS
            let motor_command = ctx.run_on_main_thread(|ctx| {
                ctx.world.resource::<ControlOutput>().motor_command
            }).await;
            
            // Send command to hardware (simulated)
            send_motor_command(motor_command).await;
        }
    });
}

// ============================================================================
// Fixed Update Systems - Deterministic Control Logic (20 Hz)
// ============================================================================

fn process_sensor_data(
    sensor_data: Res<SensorData>,
    time: Res<Time>,
) {
    // This runs at exactly 20 Hz due to FixedUpdate schedule
    // Process and filter sensor data
    if sensor_data.is_changed() {
        println!(
            "[Control Loop] t={:.3}s | Position: {:.2}, Velocity: {:.2}",
            time.elapsed_secs(),
            sensor_data.position,
            sensor_data.velocity
        );
    }
}

fn run_control_algorithm(
    sensor_data: Res<SensorData>,
    mut control_output: ResMut<ControlOutput>,
    time: Res<Time>,
) {
    // Simple PID-like control algorithm
    // In a real system, this would be your actual control logic
    
    let target = 100.0; // Target position
    let error = target - sensor_data.position;
    let kp = 0.5; // Proportional gain
    let kd = 0.1; // Derivative gain
    
    // Calculate control output
    let motor_command = kp * error - kd * sensor_data.velocity;
    
    control_output.target_position = target;
    control_output.motor_command = motor_command.clamp(-100.0, 100.0);
    
    // Log every second
    if (time.elapsed_secs() * 20.0) as u32 % 20 == 0 {
        println!(
            "[Control] Error: {:.2}, Command: {:.2}",
            error,
            control_output.motor_command
        );
    }
}

fn prepare_actuator_commands(
    control_output: Res<ControlOutput>,
) {
    // Prepare commands for actuators
    // This could involve safety checks, rate limiting, etc.
    
    if control_output.motor_command.abs() > 90.0 {
        println!("[Safety] High motor command: {:.2}", control_output.motor_command);
    }
}

// ============================================================================
// Update Systems - Non-Critical Monitoring (Variable Rate)
// ============================================================================

fn monitor_system_health(
    time: Res<Time>,
    sensor_data: Res<SensorData>,
) {
    // This runs at variable rate (whenever the frame loop runs)
    // Good for non-critical monitoring, logging, etc.
    
    // Check if sensor data is stale (no updates for 100ms)
    let current_time = time.elapsed().as_millis() as u64;
    let sensor_age = current_time.saturating_sub(sensor_data.timestamp * 10);
    
    if sensor_age > 100 {
        println!("[Warning] Sensor data is stale: {}ms old", sensor_age);
    }
}

// ============================================================================
// Hardware Simulation Functions
// ============================================================================

fn simulate_sensor_reading(tick: u64) -> f32 {
    // Simulate a sensor reading (e.g., position encoder)
    // In reality, this would read from actual hardware
    let t = tick as f32 * 0.01; // 10ms per tick
    50.0 + 30.0 * (t * 0.5).sin() // Oscillating position
}

fn simulate_velocity_reading(tick: u64) -> f32 {
    // Simulate velocity measurement
    let t = tick as f32 * 0.01;
    15.0 * (t * 0.5).cos() * 0.5 // Derivative of position
}

async fn send_motor_command(command: f32) {
    // Simulate sending command to motor controller
    // In reality, this would use Modbus, EtherCAT, etc.
    
    // Simulate network latency
    tokio::time::sleep(Duration::from_micros(100)).await;
    
    // In a real system, you would:
    // - Send via industrial protocol (Modbus TCP, EtherCAT, etc.)
    // - Handle errors and retries
    // - Log communication issues
}

// ============================================================================
// Key Takeaways
// ============================================================================

/*
ARCHITECTURE SUMMARY:

1. FIXED TIMESTEP (20 Hz):
   - Control logic runs in FixedUpdate schedule
   - Guaranteed to run at exactly 50ms intervals
   - Deterministic behavior for control algorithms
   - If a frame takes too long, multiple updates run to catch up

2. ASYNC I/O (100 Hz sensors, 50 Hz actuators):
   - Tokio tasks handle hardware communication
   - Higher frequency than control loop (oversampling)
   - Non-blocking I/O doesn't affect control timing
   - Data synchronized via run_on_main_thread()

3. SEPARATION OF CONCERNS:
   - FixedUpdate: Deterministic control logic
   - Update: Non-critical monitoring
   - Tokio tasks: Async I/O with hardware

4. TIMING GUARANTEES:
   - Control loop: Deterministic (FixedUpdate)
   - Sensor reading: Best-effort (Tokio interval)
   - Actuator writing: Best-effort (Tokio interval)
   - Monitoring: Variable (Update schedule)

5. REAL-WORLD CONSIDERATIONS:
   - Use real-time Linux kernel for hard real-time requirements
   - Set thread priorities for critical tasks
   - Monitor timing jitter and adjust rates accordingly
   - Implement safety checks and watchdogs
   - Log timing violations for analysis

This architecture is suitable for:
✅ Industrial automation (PLCs, SCADA)
✅ Robotics coordination and planning
✅ Soft real-time control (10-100ms cycles)
✅ Sensor fusion and data processing

NOT suitable for:
❌ Hard real-time systems (<1ms requirements)
❌ Safety-critical applications without RTOS
❌ High-frequency control (>1kHz)
*/

