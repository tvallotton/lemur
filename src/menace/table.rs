#[derive(Clone, Copy, Debug, PartialEq, Hash)]
enum Value {
    X,
    O,
    Nil,
}

#[derive(Clone, Debug, Hash, PartialEq)]
struct Table {
    grid: [[Value; 3]; 3],
}

impl Table {
    fn new() -> Table {
        use Value::Nil as n;
        Table {
            grid: [[n, n, n], [n, n, n], [n, n, n]],
        }
    }

    fn available(&self) -> Vec<(usize, usize)> {
        let mut out = Vec::with_capacity(9);
        for (i, row) in self.grid.iter().enumerate() {
            for (j, el) in row.iter().enumerate() {
                if el == &Value::Nil {
                    out.push((i, j));
                }
            }
        }
        out
    }

    fn winner(&self) -> Option<Value> {
        let g = self.grid;
        use Value::Nil;
        if g[0][0] == g[0][1] && g[0][1] == g[0][2] && Nil != g[0][0] {
            Some(g[0][0])
        } else if g[1][0] == g[1][1] && g[1][1] == g[1][2] && Nil != g[1][0] {
            Some(g[1][0])
        } else if g[2][0] == g[2][1] && g[2][1] == g[2][2] && Nil != g[2][0] {
            Some(g[2][0])
        } else if g[0][0] == g[1][0] && g[1][0] == g[2][0] && Nil != g[0][0] {
            Some(g[0][0])
        } else if g[0][1] == g[1][1] && g[1][1] == g[2][1] && Nil != g[0][1] {
            Some(g[0][1])
        } else if g[0][2] == g[1][2] && g[1][2] == g[2][2] && Nil != g[0][2] {
            Some(g[0][2])
        } else if g[0][0] == g[1][1] && g[1][1] == g[2][2] && Nil != g[1][1] {
            Some(g[1][1])
        } else if g[2][0] == g[1][1] && g[1][1] == g[0][2] && Nil != g[1][1] {
            Some(g[1][1])
        } else {
            None
        }
    }

    fn count_turn(&self) -> i8 {
        let mut turn = 0;
        for row in self.grid.iter() {
            for el in row.iter() {
                turn += 1;
            }
        }
        turn
    }

    pub fn make_move(&mut self, pos: (usize, usize), value: Value) {
        self.grid[pos.0][pos.1] = value;
    }
}

use std::collections::HashMap;
struct Menace {
    pub map: HashMap;
}

impl Menace {

    fn new() -> Menace {
        Menace { pub map: HashMap::new() }
    }

    fn get_move(&self, table: Table) -> (usize, usize) {
        let option = self.map.get(table);
        let choices;
        if let None = option {
            self.set_choices(table);
            choices = self.map[table];
            
        } else {
            choices = option.unwrap();
        }
    }


    
}



        i = np.random.randint(len(choices))
        return choices[i]

//     def reward(self, table, move):
//         try:
//             self.map[table].append(move)
//         except:
//             pass
            

//     def punish(self, table, move):
//         try:
//             if len(self.map[table]) > 1 and (move in self.map[table]):
//                 self.map[table].remove(move)
//         except:
//             pass

        
//     def set_choices(self, table):
//         available = table.available()
//         available *= (12 - table.count_turn()) // 3
//         self.map[table] = available
        


fn main() {
    
    let mut a = [[1, 2, 3], [1, 2, 3]];

    a[0][0] = 3;
    println!("{:?}", a);
}
