fn main() {
    
    let mut lab = 15;
    let mut class = 50;
    let min = 4;
    let max = 7;

    while lab < class {
    	lab+=min;
    	class-=max;
    	println!("The value of class = {}",class);
    }
}
