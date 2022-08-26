use rand::{rngs::ThreadRng, Rng};


#[derive(Copy, Clone)]
pub struct Member{
    genes: [u8; 10],

}

impl Member{

    pub fn new(rng: &mut ThreadRng) -> Self{
        let mut genes = [0;10];
        genes    
            .iter_mut()
            .for_each(|t| *t = rng.gen::<u8>());
        
        Self { genes }
    }

    pub fn genes_mut(&mut self) -> &mut[u8; 10]{
        &mut self.genes
    }
    pub fn genes(&self) -> &[u8; 10]{
        &self.genes
    }

    // pub fn target(genes: [u8; 10]) -> Self{
    //     Self { genes }
    // }
    // pub fn from_array(genes: [u8; 10]) -> Self{
    //     Self { genes }
    // }

    pub fn fitness(self, target: Member) -> usize{
        
        self
            .genes
            .iter()
                .zip(
                    target
                        .genes
                        .iter()
                    )
                .map(|(m_el, t_el)| (*m_el as isize - *t_el as isize).abs() as usize)
            .sum()
    }

    pub fn mutate(&mut self, mutation_rate: f32, mut rng: &mut ThreadRng){

        self
            .genes
            .iter_mut()
            .for_each(|el| {
                if rng.gen::<f32>() < mutation_rate{
                    *el = rng.gen()
                }
            });

    }
    
}

impl From<[u8;10]> for Member{
    fn from(genes: [u8;10]) -> Self {
        Self { genes }
    }
}