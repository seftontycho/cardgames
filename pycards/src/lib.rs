use cards::game::*;
use pyo3::prelude::*;

// wrapper classes for each game in cards::game::*;
// each class must have only the methods from the Game trait

#[pyclass]
struct HighLow {
    game: highlow::HighLow,
}

#[pymethods]
impl HighLow {
    #[new]
    fn new() -> Self {
        HighLow {
            game: highlow::HighLow::new(),
        }
    }

    fn current_player(&self) -> u32 {
        *self.game.current_player() as u32
    }

    fn legal_actions(&self) -> Vec<u32> {
        self.game
            .legal_actions()
            .iter()
            .map(|a| *a as u32)
            .collect()
    }

    fn observation(&self) -> u32 {
        self.game.observation().into()
    }

    fn step(&mut self, action: u32) -> (u32, f32, bool) {
        let (state, reward, done) = self.game.step(action.into());
        (state.into(), reward.into(), done)
    }

    fn reset(&mut self) {
        self.game.reset();
    }

    fn render(&self) {
        self.game.render();
    }
}

#[pymodule]
fn pycards(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<HighLow>()?;

    Ok(())
}
