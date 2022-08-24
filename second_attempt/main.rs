use rand::{Rng, seq::{SliceRandom, index}};


fn main() {

    /**
     * passing Rng trait obj to funcs as stated here:
     * https://www.reddit.com/r/rust/comments/tmh6qa/comment/i1yoqrb/?utm_source=share&utm_medium=web2x&context=3
     */
    let mut rng = rand::thread_rng();

    let target: [u8; 10] = [1,2,3,4,5,6,7,8,9,10];
    let size: usize = 100;
    let half = size/2;

    let mut population = generate_population(size, &mut rng);
    population.sort_by_key(|m| fitness(*m, target));
   
    let mut best = population[0];

    while best != target{
        population.drain(half..);
        repopulate(&mut population, size-half, &mut rng);
        population.sort_by_key(|m| fitness(*m, target));
        best = population[0];
        // println!("best: {:?}", best);
        // println!("worst: {:?}", population[size-1]);
        println!("{:?}", fitness(best, target))
    }

    println!("{:?}", best);
}


fn generate_population(size: usize, rng: &mut impl Rng) -> Vec<[u8; 10]>{


    (0..size)
    .map(|_| {
        let mut mem = [0;10];
        mem
        .iter_mut()
        .for_each(|t| {
        *t = rng.gen::<u8>();
        }
        );
        mem
    })
    .collect::<Vec<_>>()

}  

fn fitness(member: [u8; 10], target: [u8; 10]) -> usize{

    member
    .iter()
    .zip(
        target
        .iter())
    .map(|(mel, tel)| (*mel as isize - *tel as isize).abs() as usize )
    .sum()

}

fn repopulate(population: &mut Vec<[u8; 10]>, difference: usize, mut rng: &mut impl Rng) {

    let standard_mutation_rate: f32 = 0.05;

    (0..difference)
    .for_each(|_| {
        let m1 = population.choose(&mut rng).unwrap();
        let m2 = population.choose(&mut rng).unwrap();
        let mut new_member = breed(*m1, *m2, &mut rng);

        mutate(&mut new_member, standard_mutation_rate, &mut rng);
        population.push(new_member);
    })
  
}

fn breed(m1: [u8;10], m2: [u8;10], rng: &mut impl Rng) -> [u8; 10]{


    let mut child = [0;10];

    child
    .iter_mut()
    .enumerate()
    .for_each(|(i, el)|{
        if rng.gen(){
            *el = m1[i];
        }else{
            *el = m2[i];
        }
    });

    child

}

fn mutate(member: &mut[u8; 10], mutation_rate: f32, rng: &mut impl Rng) {


    member.
    iter_mut().
    for_each(|el| {
        if rng.gen::<f32>() < mutation_rate{
            *el = rng.gen()
        }
    });


}