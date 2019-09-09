#[derive(Debug)]
struct Marble {
    value: usize,
    next: usize,
    prev: usize,
}

#[derive(Debug)]
struct MarbleList {
    marbles: Vec<Marble>,
    current_pos: usize,
}

impl MarbleList {
    fn new() -> MarbleList {
        MarbleList {marbles: vec![], current_pos: 0}
    }

    fn push(&mut self, marble_value: usize) {
        if self.marbles.is_empty() {
            self.marbles.push(Marble { value: marble_value, prev: 0, next: 0});
            return;
        }

        let mut new_marble = Marble { value: marble_value,
            prev: self.marbles[self.current_pos].next,
            next: self.marbles[self.marbles[self.current_pos].next].next
        };

        self.marbles.push(new_marble);

        let current_next = self.marbles[self.current_pos].next;
        let current_next_next = self.marbles[self.marbles[self.current_pos].next].next;

        self.marbles[current_next].next = self.marbles.len() - 1;
        self.marbles[current_next_next].prev = self.marbles.len() - 1;
//
        self.current_pos = self.marbles.len() - 1;

    }

    fn pop(&mut self) -> usize {
        if self.marbles.is_empty() {
            return 0;
        }

        let mut p = self.current_pos;

        for i in 0..7 {
            p = self.marbles[p].prev;
        }

        let val = self.marbles[p].value;
        let prev = self.marbles[p].prev;
        let next = self.marbles[p].next;
        let p_prev = self.marbles[p].prev;
        let p_next = self.marbles[p].next;

        self.marbles[p_prev].next = next;
        self.marbles[p_next].prev = prev;

        self.current_pos = p_next;
        val
    }
}

fn play(players: usize, last_value: usize) -> usize {
    let mut marble_list = MarbleList::new();

    let mut scores = vec![0; players];

    marble_list.push(0);

    for i in 1..=last_value {
//        if çi == 11 {
//            break;
//        }
//        println!("{:?}", marble_list);

        if i % 23 == 0 && i != 0 {
            let idx = i % players;
            scores[idx] += i + marble_list.pop();
        }

        else {
            marble_list.push(i);
        }
    }

    *scores.iter().max().unwrap()
}

fn main() {
    println!("high score {}", play(476, 71657 * 100));
}
