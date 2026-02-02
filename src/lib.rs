//! Quantum Coherence Testing Framework
//!
//! This library provides tools for testing quantum coherence preservation
//! in superposition states across various noise models and conditions.
//!
//! # Features
//!
//! - `quantum`: Enable quantum simulation capabilities
//! - `python`: Enable Python bindings and pipe communication
//! - `plotting`: Enable plotting and visualization
//! - `parallel`: Enable parallel processing
//! - `full`: Enable all features
//!
//! # Examples
//!
//! ```rust
//! use quantum_coherence::{
//! QuantumSystem, test_superposition_preservation, NoiseModel
//! };
//!
//! let mut system = QuantumSystem::new(5, 0.02); // 5 qubits at 20 mK
//! let results = test_superposition_preservation(
//! &mut system,
//! vec![NoiseModel::comprehensive()],
//! 100
//! );
//!
//! if let Ok(results) = results {
//!     println!("Coherence time T2: {} μs", results.overall_stats.average_t2);
//!     println!("Superposition fidelity: {}", results.overall_stats.average_fidelity);
//! }
//! ```

#![warn(missing_docs)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

// Re-exports
pub use crate::coherence::*;
pub use crate::noise::*;
pub use crate::pipes::*;
pub use crate::quantum::*;
pub use crate::testing::*;

pub mod coherence;
pub mod noise;
pub mod pipes;
pub mod quantum;
pub mod testing;
