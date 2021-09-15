
pub fn sentences(s: String) -> Vec<String> {
    let mut pos: usize = 0;
    let mut sentence_list: Vec<String> = Vec::new();
    //println!("{}", s);
    let charlist = s.chars().collect::<Vec<char>>();
    let l = charlist.len();
    while pos < l {
        let p: usize;
        let q: usize;
        let e: usize;
        let instring: String = charlist[pos..].iter().collect();
        match instring.chars().position(|c| c == '.') {
            Some(z) => {
                p = z;
            },
            None => {
                p = l;
            },
        }
        match instring.chars().position(|c| c == '?') {
            Some(z) => {
                q = z;
            },
            None => {
                q = l;
            },
        }
        match instring.chars().position(|c| c == '!') {
            Some(z) => {
                e = z;
            },
            None => {
                e = l;
            },
        }
        let minvec = vec![p, q, e];
        //println!("{:?} {:?}", minvec);
        let end: usize = *minvec.iter().min().unwrap() ;
        //println!("{}", end);
        let outstring: String = charlist[pos..end].iter().collect();
        if outstring.trim() != "" {
            sentence_list.push(outstring.trim().to_string());
        }
        
        pos = end + 1;
    }
    //println!("sentencelist {:?}", sentence_list.clone());
    if sentence_list.len() == 0 {
        sentence_list.push(s);
    }
    sentence_list
}