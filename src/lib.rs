pub mod dna;
pub mod fib;
pub mod gc;
pub mod hamm;
pub mod revc;
pub mod rna;


/// Each of the submodules for problem solvers should have a type called Problem
/// and that type should implement the Solvable trait.
pub trait Solvable {
    type Parameters;

    fn file_parse(&self, input_filename: &str) -> Self::Parameters;

    fn get_solution(&mut self, params: Self::Parameters);

    fn solve(&mut self, input_filename: &str) {
        let params = self.file_parse(input_filename);
        self.get_solution(params);
    }
}