use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs::File,
    path::PathBuf,
};

use assembly_data::fdb::{
    common::Latin1Str,
    mem::{Database, Row, RowHeaderIter, Table, Tables},
};
use color_eyre::eyre::{eyre, WrapErr};
use mapr::Mmap;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Options {
    /// Path to the CDClient
    file: PathBuf,
}

fn get_table<'a>(tables: Tables<'a>, name: &str) -> color_eyre::Result<Table<'a>> {
    let table = tables
        .by_name(name)
        .ok_or_else(|| eyre!("Missing table '{}'", name))??;
    Ok(table)
}

fn get_column_index(table: Table<'_>, name: &str) -> color_eyre::Result<usize> {
    let index = table
        .column_iter()
        .position(|col| col.name() == name)
        .ok_or_else(|| eyre!("Missing columns '{}'.'{}'", table.name(), name))?;
    Ok(index)
}

fn match_action_key(key: &str) -> bool {
    matches!(
        key,
        "action"
            | "behavior 1"
            | "behavior 2"
            | "miss action"
            | "blocked action"
            | "on_fail_blocked"
            | "action_false"
            | "action_true"
            | "start_action"
            | "behavior 3"
            | "bahavior 2"
            | "behavior 4"
            | "on_success"
            | "behavior 5"
            | "chain_action"
            | "behavior 0"
            | "behavior 6"
            | "behavior 7"
            | "behavior 8"
            | "on_fail_armor"
            | "behavior"
            | "break_action"
            | "double_jump_action"
            | "ground_action"
            | "jump_action"
            | "hit_action"
            | "hit_action_enemy"
            | "timeout_action"
            | "air_action"
            | "falling_action"
            | "jetpack_action"
            | "spawn_fail_action"
            | "action_failed"
            | "action_consumed"
            | "blocked_action"
            | "on_fail_immune"
            | "moving_action"
            | "behavior 10"
            | "behavior 9"
    )
}

struct BehaviorParameter<'a> {
    inner: Row<'a>,
}

impl<'a> BehaviorParameter<'a> {
    fn parameter_id(&self) -> &'a Latin1Str {
        self.inner
            .field_at(1) // bp_col_parameter_id
            .unwrap()
            .into_opt_text()
            .unwrap()
    }

    fn int_value(&self) -> i32 {
        self.inner
            .field_at(2) // bp_col_value
            .unwrap()
            .into_opt_float()
            .unwrap() as i32
    }
}

struct RowFinder<'a> {
    inner: RowHeaderIter<'a>,
    behavior_id: i32,
}

impl<'a> RowFinder<'a> {
    fn new(behavior_parameter: Table<'a>, behavior_id: i32) -> Self {
        let bp_bucket_index = behavior_id as usize % behavior_parameter.bucket_count();
        let bp_bucket = behavior_parameter.bucket_at(bp_bucket_index).unwrap();
        Self {
            inner: bp_bucket.row_iter(),
            behavior_id,
        }
    }
}

impl<'a> Iterator for RowFinder<'a> {
    type Item = BehaviorParameter<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        for row in &mut self.inner {
            let behavior_id = row
                .field_at(0) // bp_col_behavior_id
                .unwrap()
                .into_opt_integer()
                .unwrap();
            if behavior_id == self.behavior_id {
                return Some(BehaviorParameter { inner: row });
            }
        }
        None
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let opts = Options::from_args();

    // Load the database file
    let file = File::open(&opts.file)
        .wrap_err_with(|| format!("Failed to open input file '{}'", opts.file.display()))?;
    let mmap = unsafe { Mmap::map(&file)? };
    let buffer: &[u8] = &mmap;

    // Start using the database
    let db = Database::new(buffer);

    // Find table
    let tables = db.tables()?;

    let skill_behavior = get_table(tables, "SkillBehavior")?;
    //let sb_col_skill_id = get_column_index(skill_behavior, "skillID")?;
    let sb_col_behavior_id = get_column_index(skill_behavior, "behaviorID")?;

    let behavior_parameter = get_table(tables, "BehaviorParameter")?;
    let bp_col_behavior_id = get_column_index(behavior_parameter, "behaviorID")?;
    assert_eq!(bp_col_behavior_id, 0);
    let bp_col_parameter_id = get_column_index(behavior_parameter, "parameterID")?;
    assert_eq!(bp_col_parameter_id, 1);
    let bp_col_value = get_column_index(behavior_parameter, "value")?;
    assert_eq!(bp_col_value, 2);

    //let behavior_template = get_table(tables, "BehaviorTemplate")?;
    //let bt_col_behavior_id = get_column_index(behavior_template, "behaviorID")?;
    //let bt_col_template_id = get_column_index(behavior_template, "templateID")?;

    let mut root_behaviors = HashSet::new();

    for row in skill_behavior.row_iter() {
        let behavior_id = row
            .field_at(sb_col_behavior_id)
            .unwrap()
            .into_opt_integer()
            .unwrap();
        root_behaviors.insert(behavior_id);
    }

    let mut behavior_root: HashMap<i32, i32> = HashMap::new();

    let mut stack = Vec::new();
    let mut soft_roots = HashSet::new();
    let mut conflicts = HashSet::new();
    let mut conflicting_roots = HashSet::new();

    for &root in &root_behaviors {
        stack.push(root);
        while let Some(node) = stack.pop() {
            if let Some(&check_root) = behavior_root.get(&node) {
                // We already know the root of that node, now we need to check whether it's the same
                match (&check_root).cmp(&root) {
                    Ordering::Less => {
                        // OOPS
                        conflicts.insert((check_root, root));
                        conflicting_roots.insert(check_root);
                        conflicting_roots.insert(root);
                        soft_roots.insert(node);
                    }
                    Ordering::Equal => {}
                    Ordering::Greater => {
                        // OOPS
                        conflicts.insert((root, check_root));
                        conflicting_roots.insert(check_root);
                        conflicting_roots.insert(root);
                        soft_roots.insert(node);
                    }
                }
            } else {
                behavior_root.insert(node, root);
                // Now add all possible child nodes to the stack

                let iter = RowFinder::new(behavior_parameter, node);

                for bp in iter {
                    let parameter_id = bp.parameter_id().decode();

                    if match_action_key(parameter_id.as_ref()) {
                        let value = bp.int_value();
                        if value > 0 {
                            stack.push(value);
                        }
                    }
                }
            }
        }
    }

    for &conflict in &conflicts {
        println!("{:?}", conflict);
    }
    println!("Count: {}", conflicts.len());

    for &conflict in &conflicting_roots {
        println!("{:?}", conflict);
    }
    println!("Count: {}", conflicting_roots.len());

    let partition_roots: HashSet<i32> = root_behaviors.union(&soft_roots).copied().collect();
    behavior_root.clear();

    for &root in &root_behaviors {
        stack.push(root);
        while let Some(node) = stack.pop() {
            if let Some(&check_root) = behavior_root.get(&node) {
                // We already know the root of that node, now we need to check whether it's the same
                if check_root != root {
                    panic!("Caught! {} {} {}", node, root, check_root);
                }
            } else if (node != root) && partition_roots.contains(&node) {
                panic!("FOo");
            } else {
                behavior_root.insert(node, root);
                // Now add all possible child nodes to the stack

                let iter = RowFinder::new(behavior_parameter, node);

                for bp in iter {
                    let parameter_id = bp.parameter_id().decode();

                    if match_action_key(parameter_id.as_ref()) {
                        let value = bp.int_value();
                        if value > 0 && !partition_roots.contains(&value) {
                            stack.push(value);
                        }
                    }
                }
            }
        }
    }

    for &part_root in &partition_roots {
        if let Some(&_check_root) = behavior_root.get(&part_root) {
            panic!("Foo");
        }
    }

    Ok(())
}
