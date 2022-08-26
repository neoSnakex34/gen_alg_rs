
use gen_alg::{population::Population, member::Member};

fn main() {

    let mutation_rate = 0.05;

    let mut rng = rand::thread_rng();
    let size: usize = 5;
    let half = size/2;
    
    let mut population = Population::new(size, &mut rng);    
    let target = Member::from([1,2,3,4,5,6,7,8,9,10]);

    population.sort_members(&target);

    let mut best = population.members()[0];

    while best.fitness(target) != 0 {
        
        population.step_generation(size, half, mutation_rate, &mut rng, &target);
        best = *population.best().unwrap();
    
        println!("{:?}", best.fitness(target));
    }
    
    println!("{:?}", best.genes());
    
}
