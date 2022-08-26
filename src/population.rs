
use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};
use crate::member::{Member};

#[derive(Clone)]
pub struct Population{
    members: Vec<Member>,
    fitness_sorted: bool,
}

impl Population {
    
    pub fn new(size: usize, rng: &mut ThreadRng) -> Self{
        Self {
            members: 
                (0..size)
                    .map(|_| { 
                        Member::new(rng)
                    })
                    .collect::<Vec<_>>(),

            fitness_sorted: false
            }

    }

    pub fn members(&mut self) -> &mut Vec<Member>{
        &mut self.members
    }
    
    // fn drain_pop(&mut self, range: RangeFrom<usize>) -> Drain<Member> {
    //     self.members.drain(range)
    // }

    pub fn sort_members(&mut self, target: &Member){
        self.members.sort_by_key(|m| m.fitness(*target));   
        self.fitness_sorted = true;
    }
   
   

    fn breed(p1: Member,p2: Member, rng: &mut ThreadRng) -> Member{
        

        let mut child = Member::from([0;10]);

        
        child
            .genes_mut()
            .iter_mut()
            .zip(p1.genes().iter()
                    .zip(p2.genes().iter()))
            .for_each(|(c, (gm1, gm2))|{
                *c = if rng.gen() { *gm1 } else { *gm2 } 
            });
        
        child
    }

    fn repopulate(&mut self, difference: usize, mutation_rate: f32, mut rng: &mut ThreadRng) {
        self.fitness_sorted = false;

        (0..difference)
            .for_each(|_| {
                let m1 = self.members.choose(&mut rng).unwrap();
                let m2 = self.members.choose(&mut rng).unwrap();

                let mut new_member = Self::breed(*m1, *m2, rng);
                
                new_member.mutate(mutation_rate, rng);
                self.members.push(new_member);
            })

    }

    pub fn step_generation(&mut self, size: usize, half: usize, mutation_rate: f32, rng: &mut ThreadRng, target: &Member) {
        self.members.drain(half..);
        self.repopulate(size-half, mutation_rate, rng);
        self.sort_members(target);

    }

    
    pub fn best(&self) -> Result<&Member, &str>{
        if self.fitness_sorted {
            Ok(&self.members[0])
        }else{
            Err("Unsorted population, cannot retrieve best")
        }
    }
}