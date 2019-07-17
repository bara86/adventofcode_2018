use std::fs;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;

struct Size {
    width: usize,
    height: usize,
}

struct Origin {
    x: usize,
    y: usize,
}

struct Rect {
    id: i32,
    origin: Origin,
    size: Size,
}

fn main() -> std::io::Result<()> {
    let f = fs::File::open("input.txt")?;
    let reader = BufReader::new(f);

    let mut matrix = vec![vec![0; 1000]; 1000];
    // let mut rects = HashSet::new();
    let mut rects = Vec::new();

     for line in reader.lines() {
         let line = line.unwrap();

         let vals = line.split_whitespace().collect::<Vec<_>>();

         let id = vals[0][1..].parse::<i32>().unwrap();

         let origin = vals[2].split(',').collect::<Vec<_>>();
         let x = origin[0].parse::<usize>().unwrap();
         let y = origin[1][0..origin[1].len()-1].parse::<usize>().unwrap();

         let sizes = vals[3].split('x').collect::<Vec<_>>();
         let width = sizes[0].parse::<usize>().unwrap();
         let height = sizes[1].parse::<usize>().unwrap();

         let mut rect = Rect {id, origin: Origin {x, y}, size: Size {width, height} };

         rects.push(rect);

         for i in x..x+width {
             for j in y..y+height {
                 matrix[i][j] += 1;
             }
         }
     }

     let sum: usize = matrix.iter().map(|row| row.iter().filter(|val| **val > 1).count()).sum();
     println!("{}", sum);

    let mut id = -1;
     for rect in rects {
        let mut found = true;

        let origin = rect.origin;
        let size = rect.size;
         for i in origin.x..origin.x+size.width {
             for j in origin.y..origin.y+size.height {
                if matrix[i][j] > 1 {
                    found = false;
                    break;
                }
             }
         }
         if found {
             id = rect.id;
             break;
         }
     }

     println!("{}", id);

    //  for row in matrix {
    //      for col in row {
    //          println!("{:?}", col)
    //      }
    //  }

    Ok(())
}