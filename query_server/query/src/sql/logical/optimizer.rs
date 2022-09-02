use std::sync::Arc;

use datafusion::{logical_plan::LogicalPlan, optimizer::OptimizerRule};

use spi::query::{session::IsiphoSessionCtx, Result};

use snafu::ResultExt;
use spi::query::LogicalOptimizeSnafu;

pub trait LogicalOptimizer: Send + Sync {
    fn optimize(&self, plan: &LogicalPlan, session: &IsiphoSessionCtx) -> Result<LogicalPlan>;

    fn inject_optimizer_rule(&mut self, optimizer_rule: Arc<dyn OptimizerRule + Send + Sync>);
}

#[derive(Default)]
pub struct DefaultLogicalOptimizer {
    rules: Vec<Arc<dyn OptimizerRule + Send + Sync>>,
}

impl DefaultLogicalOptimizer {
    #[allow(dead_code)]
    fn with_optimizer_rules(mut self, rules: Vec<Arc<dyn OptimizerRule + Send + Sync>>) -> Self {
        self.rules = rules;
        self
    }
}

// impl Default for DefaultLogicalOptimizer {
//     fn default() -> Self {
//         // additional optimizer rule
//         Self {
//             rules: Default::default(),
//         }
//     }
// }

impl LogicalOptimizer for DefaultLogicalOptimizer {
    fn optimize(&self, plan: &LogicalPlan, session: &IsiphoSessionCtx) -> Result<LogicalPlan> {
        session
            .inner()
            .state()
            .with_optimizer_rules(self.rules.clone())
            .optimize(plan)
            .context(LogicalOptimizeSnafu)
    }

    fn inject_optimizer_rule(&mut self, optimizer_rule: Arc<dyn OptimizerRule + Send + Sync>) {
        self.rules.push(optimizer_rule);
    }
}
