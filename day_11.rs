#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    op: fn(u64) -> u64,
    test: fn(u64) -> bool,
    decision: [usize; 2],
    processed: usize,
}

fn main() {
    const MOD: u64 = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19;

    let mut monkeys = vec![
        Monkey {
            items: vec![52, 60, 85, 69, 75, 75],
            op: |x| (x * 17) % MOD,
            test: |x| x % 13 == 0,
            decision: [7, 6],
            processed: 0,
        },
        Monkey {
            items: vec![96, 82, 61, 99, 82, 84, 85],
            op: |x| (x + 8) % MOD,
            test: |x| x % 7 == 0,
            decision: [7, 0],
            processed: 0,
        },
        Monkey {
            items: vec![95, 79],
            op: |x| (x + 6) % MOD,
            test: |x| x % 19 == 0,
            decision: [3, 5],
            processed: 0,
        },
        Monkey {
            items: vec![88, 50, 82, 65, 77],
            op: |x| (x * 19) % MOD,
            test: |x| x % 2 == 0,
            decision: [1, 4],
            processed: 0,
        },
        Monkey {
            items: vec![66, 90, 59, 90, 87, 63, 53, 88],
            op: |x| (x + 7) % MOD,
            test: |x| x % 5 == 0,
            decision: [0, 1],
            processed: 0,
        },
        Monkey {
            items: vec![92, 75, 62],
            op: |x| (x * x) % MOD,
            test: |x| x % 3 == 0,
            decision: [4, 3],
            processed: 0,
        },
        Monkey {
            items: vec![94, 86, 76, 67],
            op: |x| (x + 1) % MOD,
            test: |x| x % 11 == 0,
            decision: [2, 5],
            processed: 0,
        },
        Monkey {
            items: vec![57],
            op: |x| (x + 2) % MOD,
            test: |x| x % 17 == 0,
            decision: [2, 6],
            processed: 0,
        },
    ];

    let n = monkeys.len();
    for _round in 1..=10000 /* 20 */ {

        for i in 0..n {
            let mut monkey = monkeys[i].clone();

            for &item in &monkey.items {
                // eprintln!("Monkey #{i} inspects an item with a worry level of {item}.");
                let item = (monkey.op)(item) /* / 3 */;
                let dest = monkey.decision[(monkey.test)(item) as usize];
                // eprintln!("Monkey #{i} throws {item} to #{dest}.");

                monkeys[dest].items.push(item);
            }
            monkey.processed += monkey.items.len();
            monkey.items = vec![];

            monkeys[i] = monkey;
        }

        // eprintln!("After round {round}:");
        // for monkey in &monkeys {
        //     eprintln!(" - {monkey:?}");
        // }
    }

    let mut processed: Vec<_> = monkeys.iter().map(|m| m.processed).collect();
    processed.sort();
    processed.reverse();

    let business = processed[0] * processed[1];
    println!("Monkey business level is {business}.");
}
