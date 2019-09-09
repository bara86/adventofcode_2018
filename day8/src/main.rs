use std::io::{BufRead, BufReader, Read};
use std::fs::File;

struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

fn parse<'a>(mut it: &mut impl Iterator<Item=&'a str>) -> Node {

    let children = it.next().unwrap().parse::<usize>().unwrap();
    let metadata_count = it.next().unwrap().parse::<usize>().unwrap();

    let mut node = Node {children: vec![], metadata: vec![]};

    for i in 0..children {
        let child = parse(it);
        node.children.push(child);
    }

    for i in 0..metadata_count {
        let metadata = it.next().unwrap().parse::<usize>().unwrap();
        node.metadata.push(metadata);
    }

    node
}

fn sum(node: & Node) -> usize {
    node.metadata.iter().sum::<usize>() + node.children.iter().map(| x | sum(x)).sum::<usize>()
}

fn sum1(node: & Node) -> usize {
    if node.children.is_empty() {
        return node.metadata.iter().sum();
    }

    node.metadata.iter().filter(|m|  *m > &0 && *m - 1 < node.children.len())
        .map(|x| sum1(&node.children[x-1])).sum()

}

fn main() -> std::io::Result<()> {

    let f = File::open("input.txt")?;
    let mut reader = BufReader::new(f);

    let mut buf = String::new();
    reader.read_to_string(&mut buf);

    let node = parse(&mut buf.split((" ")));

    println!("{}", sum(&node));
    println!("{}", sum1(&node));

    // 48496

    Ok(())
}
