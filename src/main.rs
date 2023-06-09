use std::collections::HashSet;
use std::borrow::Cow;

fn main() {
    let elements = &[Element{id: 1}, Element{id: 2}, Element{id: 2}];
    let unique_elements = get_unique(elements);
    for item in unique_elements {
        println!("{}", item.id);
    }

    let res = get_unique_cow(elements);
    for item in res.iter() {
        println!("{}", item.id);
    }
}

#[derive(Clone)]
struct Element {
    id: usize,
}

fn get_unique(input:&[Element]) -> Vec<Element> {
    let mut set = HashSet::new();
    let mut ret = Vec::new();
    for element in input{
        if set.contains(&element.id){
            continue;
        }else{
            ret.push(element.clone());
            set.insert(element.id);
        }
    }
    ret
}

fn get_unique_cow<'a>(input: &'a [Element]) -> Cow<'a, [Element]> {
    let mut set = HashSet::new();
    let mut contains_duplicate = false;
    for element in input {
        if set.contains(&element.id) {
            contains_duplicate = true;
        }
        set.insert(element.id);
    }
    if !contains_duplicate {
        return Cow::Borrowed(input);
    }
    let mut ret = Vec::new();
    for element in input{
        if set.contains(&element.id){
            ret.push(element.to_owned());
            set.remove(&element.id);
        }
        // duplicate
    }
    return Cow::Owned(ret);
}

