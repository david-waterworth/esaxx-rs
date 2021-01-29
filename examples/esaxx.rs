use esaxx_rs::suffix_rs;


pub struct ExtendedSuffixArray<T> {
    pub chars: Vec<char>,
    pub sa: Vec<T>,
    pub l: Vec<T>,
    pub r: Vec<T>,
    pub d: Vec<T>,
    pub node_num: usize,
}

fn main() {
    let string = "▁ahu-1-1-znt▁ahu-1-1-sat▁ahu-1-2-znt▁ahu-1-2-sat▁ahu-2-1-sat";
    let esa = suffix_rs(&string).unwrap();

    let suffix_exposed: ExtendedSuffixArray<usize> = unsafe {
        std::mem::transmute(esa)
    };
    
    let S = suffix_exposed.chars;
    let SA = suffix_exposed.sa;
    let R = suffix_exposed.r;
    let L = suffix_exposed.l;
    let D = suffix_exposed.d;
    let n = S.len();

    println!("SA\tsuffix");
    for i in 0..n {
        let suffix: String = S[SA[i]..n].iter().collect();
        println!("{:?}\t{:?}", SA[i], suffix);
    }

    println!("i\tfreq\tlen\tsubstring");
    for i in 0..suffix_exposed.node_num {
        let beg = SA[L[i]]; 
        let len = D[i];
        let freq = R[i] - L[i];

        let substring: String = S[beg..beg+len].iter().collect();
        println!("{:?}\t{:?}\t{:?}\t{:?}", i, freq, len, substring);
    }
}
