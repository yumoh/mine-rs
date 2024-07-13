#[allow(unused,non_snake_case)]
mod bindings;
mod mine;

#[cfg(feature="py")]
mod pywraper;

pub use mine::{compute_mic,Mine};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn test_mic() {
        // mine_problem prob;
        // mine_parameter param;
        // mine_score *score;
        let x = vec![1.0,2.0,3.0];
        let y = vec![4.0,5.0,6.0];
        let mut mine = Mine::new(0.6,15.0);
        mine.compute_score(x, y);
        println!("{:?}",mine.mic());
    }
}
