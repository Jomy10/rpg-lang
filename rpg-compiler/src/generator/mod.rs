use crate::node::{Node, NodeType, SBFunction};
use crate::node;
use crate::user_output::CompileError;
use crate::node::parse_dyn_node;

pub static mut MAX_CHAR: usize = 10;

/// Code that allows the language to function
const STD_CODE: &str = "\
#![allow(unused)]
use std::io::{stdin,stdout,Write};
use std::fmt::{Formatter, Display};
macro_rules! red {
    ( $str: tt ) => ({
        &format!(\"[31m{}[0m\", $str)
    });
    ( $other: expr) => ({
        &format!(\"[31m{}[0m\", $other)
    })
}
macro_rules! blue {
    ( $str: tt ) => ({
        &format!(\"\x1b[34m{}\x1b[0m\", $str)
    });
    ( $other: expr) => ({
        &format!(\"\x1b[34m{}\x1b[0m\", $other)
    })
}
macro_rules! cyan {
    ( $str: tt ) => ({
        &format!(\"[36m{}[0m\", $str)
    });
    ( $other: expr) => ({
        &format!(\"[36m{}[0m\", $other)
    })
}
macro_rules! runtime_error {
    ($( $arg: tt)*) => ({
        let s = format!($($arg)*);
        eprintln!(\"{}
{}\", cyan!(\"Runtime error\"), red!(s));
        std::process::exit(1)
    })
}
#[derive(Clone)]
/// Either a char or a zombie
struct Actor<'a> {
    id: u32,
    /// Can derive actor type using `health`
    health: ActorHealth,
    attack: u32,
    items: Vec<&'a Item>,
    confused: bool
}
impl<'a> Actor<'a> {
    fn new(id: u32, h: ActorHealth, a: u32) -> Actor<'a> { Actor { id, health: h, attack: a, items: Vec::new(), confused: false } }
    fn attacked(&mut self, val: u32, game: &mut Game) { self.health.attacked(val, self.id, game) }
    fn heal(&mut self, val: u32) { self.health.heal(val) }
    /// Deprecated
    fn validate_actor(&self) -> bool { if let ActorHealth::Char(val) = self.health { return val != (0 as u32); } else { return true; } }
    fn health(&self) -> ActorHealth {
        if self.confused {
            if let ActorHealth::Char(v) = self.health {
                return ActorHealth::Char(v-1);
            } else if let ActorHealth::Zombie(v) = self.health {
                return ActorHealth::Zombie(v-1);
            } else {
                runtime_error!(\"This well never happen.\");
            }
        } else {
            return self.health;
        }
    }
}
struct Merchant;
#[derive(Clone, Copy)]
enum ActorHealth {
    Char(u32),
    Zombie(i32)
}
impl ActorHealth {
    fn attacked(&mut self, a: u32, actor_id: u32, game: &mut Game) {
        match self {
            Self::Char(val) => {
                if *val == 0 {
                    println!(\"Stop beating a dead corpse.\");
                } else if *val <= a {
                    *val = 0;
                    let index_of_dead_actor = game.alive.iter().enumerate().find_map(|(index, act)| {
                        if act == &actor_id {
                            Some(index)
                        } else {
                            None
                        }
                    });
                    game.alive.remove(index_of_dead_actor.unwrap_or_else(|| runtime_error!(\"The now deceased actor was never alive in the first place.\")));
                } else {
                    *val -= a;
                }
            }
            Self::Zombie(val) => {
                *val -= a as i32;
            }
        }
    }
    fn heal(&mut self, h: u32) {
        match self {
            Self::Char(val) => {
                if *val == 0 {
                    runtime_error!(\"Cannot heal a dead actor.\");
                } else {
                    *val += h;
                }
            }
            Self::Zombie(val) => {
                *val += h as i32;
            }
        }
    }
}
#[derive(Clone, Copy, PartialEq)]
enum Item {
    /// (id, healing_value)
    Potion(u32,u32),
    SpellBook
}
impl Item {
    fn set_val(&mut self, v: u32) {
        match self {
            Self::Potion(_, val) => {*val=v}
            Self::SpellBook => {runtime_error!(\"Spellbooks don't have values.\")}
        }
    }
}
impl Display for ActorHealth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ActorHealth::Char(val) => write!(f, \"{}\", val),
            ActorHealth::Zombie(val) => write!(f, \"{}\", val)
        }
    }
}
struct Game {
    alive: Vec<u32>,
    max_chars: usize
}
impl Game {
    fn add_actor(&mut self, actor: u32) {
        self.alive.push(actor);
        if self.alive.len() > self.max_chars {
           // Runtime error
           let s = &format!(\"Your actors exceeded the maximum amount of actors allowed ({})\", self.max_chars);
           eprintln!(\"{}
{}\", cyan!(\"Runtime error\"), red!(s));
           println!(\"{} Actors alive: {:?}\", blue!(\"HINT:\"), self.alive);
           std::process::exit(1)
        }
    }
    fn rm_actor(&mut self, id: u32) {
        let i = self.alive.iter().enumerate().find_map(|(i,a)| {if a == &id {Some(i)}else{None}});
        if let Some(i) = i {
            self.alive.remove(i);
        }
    }
}";

/// RPG Code Generator
pub struct Generator<'a> {
    /// The maximum amount of characters allowed in the program
    max_chars: usize,
    /// The code
    nodes: &'a Vec<Box<dyn Node + Send + Sync>>,
}

impl<'a> Generator<'a> {
    pub fn new(nodes: &'a Vec<Box<dyn Node + Send + Sync>>) -> Self {
        Self {
            max_chars: unsafe{MAX_CHAR},
            nodes,
        }
    }
    
    pub fn generate(&self) -> String {
        format!(
            "{}\nfn main() {{
    let mut game = Game {{ alive: Vec::new(), max_chars: {} }};
    {}
}}",
            STD_CODE,
            self.max_chars,
            self.generate_all().join("\n")
        )
    }
    
    fn generate_all(&self) -> Vec<String> {
        self.nodes.iter().map(|node| self.generate_next(node)).collect::<Vec<String>>()
    }
    
    fn generate_next(&self, node: &Box<dyn Node + Send + Sync>) -> String {
        let node = &**node;
        match node.get_type() {
            NodeType::Char => {
                let char: &node::Char = parse_dyn_node(node);
                return format!(
                    "let mut i{} = Actor::new({},ActorHealth::Char({}),{}); game.add_actor(i{}.id);",
                    char.id,
                    char.id,
                    char.health,
                    char.attack,
                    char.id
                );
            }
            NodeType::Zombie => {
                let zombie: &node::Zombie = parse_dyn_node(node);
                return format!(
                    "let mut i{} = Actor::new({},ActorHealth::Zombie({}), {}); game.add_actor(i{}.id);",
                    zombie.id,
                    zombie.id,
                    zombie.health,
                    zombie.attack,
                    zombie.id
                );
            }
            NodeType::Merchant => {
                let m: &node::Merchant = parse_dyn_node(node);
                return format!(
                    "let i{} = Merchant{{}};",
                    m.id
                );
            }
            NodeType::Potion => {
                let p: &node::Potion = parse_dyn_node(node);
                return format!(
                    "let i{} = Item::Potion({},{});",
                    p.id,
                    p.id,
                    p.value
                )
            }
            NodeType::SpellBook => {
                let sb: &node::SpellBook = parse_dyn_node(node);
                return format!(
                    "let i{} = Item::SpellBook;",
                    sb.id
                );
            }
            NodeType::FnBuys => {
                let f: &node::FnBuys = parse_dyn_node(node);
                return format!(
                    // TODO: display name of dead actor
                    "if game.alive.contains(&{}) {{ i{}.items.push(&i{}); }} else {{ runtime_error!(\"Cannot add an item to the inventory of a dead actor.\") }}",
                    f.user,
                    f.user,
                    f.item
                );
            }
            NodeType::FnAttacks => {
                let f: &node::FnAttacks = parse_dyn_node(node);
                return format!(
                    "if game.alive.contains(&{}) {{ i{}.attacked(i{}.attack, &mut game); }} else {{ runtime_error!(\"A dead actor cannot attack.\") }}",
                    f.attacker,
                    f.attacked,
                    f.attacker
                );
            }
            NodeType::FnUses => {
                // Only potions atm
                let f: &node::FnUses = parse_dyn_node(node);
                // TODO: expect to runtime error
                return format!(
                    "if let Item::Potion(_, heal) = i{} {{ \
                    if game.alive.contains(&{}) {{ \
                    i{}.heal(heal);\
                    let item_index = i{}.items.iter().enumerate().find_map(|(i, p)| {{
                        let mut _val = None;
                        if let Item::Potion(id,val) = p {{if &{} == id {{_val = Some(i);}} else {{_val = None;}} }}
                        _val
                    }});
                    i{}.items.remove(item_index.expect(\"The actor does not own the potion it is trying to use.\"));
                    }}\
                    }}",
                    f.item,
                    f.user,
                    f.user,
                    f.user,
                    f.item,
                    f.user
                );
            }
            NodeType::FnShouts => {
                let expr: &node::FnShouts = parse_dyn_node(node);
                let user = expr.user;
                return format!(
                    "if !game.alive.contains(&{user}) {{ runtime_error!(\"Dead actors can't shout.\") }} \
                    else {{ println!(\"{{}}\", i{user}.clone().health()); }}",
                );
            }
            NodeType::FnShoutsSpeak => {
                let expr: &node::FnShoutsSpeak = parse_dyn_node(node);
                let item = expr.spell_book;
                let usr = expr.user;
                return format!(
                    "if !i{usr}.items.contains(&&i{item}) {{ runtime_error!(\"The spell cannot be called, because the caster doesn't own a spellbook.\") }};\
                    if !game.alive.contains(&{usr}) {{ runtime_error!(\"Dead actors can't shout.\") }} else if let ActorHealth::Char(val) = i{usr}.health() {{ println!(\"{{}}\", (val as u8) as char); }} else {{ runtime_error!(\"Wrong type, only characters can shout speak.\") }}",
                );
            }
            NodeType::FnWhispers => {
                let expr: &node::FnWhispers = parse_dyn_node(node);
                return format!(
                    "if !i{}.validate_actor() {{ runtime_error!(\"Dead actors can't shout.\") }} print!(\"{{}}\", i{}.health());",
                    expr.user,
                    expr.user
                );
            }
            NodeType::FnWhispersSpeak => {
                let expr: &node::FnWhispersSpeak = parse_dyn_node(node);
                let item = expr.spell_book;
                let usr = expr.user;
                return format!(
                    "if !i{usr}.items.contains(&&i{item}) {{ runtime_error!(\"The spell cannot be called, because the caster doesn't own a spellbook.\") }}; \
                    if !i{usr}.validate_actor() {{ runtime_error!(\"Dead actors can't shout.\") }} \
                    else if let ActorHealth::Char(val) = i{usr}.health() {{ print!(\"{{}}\", (val as u8) as char); }} \
                    else {{ runtime_error!(\"Wrong type, only characters can whisper speak.\") }}",
                );
            }
            NodeType::FnUsesCasting => {
                let f: &node::FnUsesCasting = parse_dyn_node(node);
                let usr = f.user;
                let item = f.spell_book;
                let mut return_s = format!("if !i{usr}.items.contains(&&i{item}) {{ runtime_error!(\"The spell cannot be called, because the caster doesn't own a spellbook.\") }};");
                match f.function {
                    SBFunction::UnZombify => {
                        let id = f.parameter.expect_compile_error("Un_zombify called without zombie parameter.");
                        return_s.push_str(&format!(
                            "let i{id} = if let ActorHealth::Zombie(h) = i{id}.health {{\
                            if h <= 0 {{ game.rm_actor({id}); i{id} }} else {{ Actor::new({id}, ActorHealth::Char(h as u32), i{id}.attack) }}\
                            }} else {{runtime_error!(\"Tried to call `un_zombify` on a non-zombie.\")}};"
                        ));
                    }
                    SBFunction::Confuse => {
                        let id = f.parameter.expect_compile_error("Confuse called without parameter.");
                        return_s.push_str(&format!(
                            "i{id}.confused = true;"
                        ));
                    }
                    SBFunction::GodSpeech => {
                        //  TODO: expect to runtime error
                        let user = f.user;
                        return_s.push_str(&format!(
                            "{{\
                            let mut s = String::new();
                            let _ = stdout().flush();
                            stdin().read_line(&mut s).expect(\"Input invalid.\");
                            if let Some('\\n')=s.chars().next_back() {{
                                s.pop();
                            }}
                            if let Some('\\r')=s.chars().next_back() {{
                                s.pop();
                            }}
                            match i{user}.health {{
                                ActorHealth::Char(_) => {{i{user}.health = ActorHealth::Char(s.parse::<u32>().expect(\"Invalid input\"))}}
                                ActorHealth::Zombie(_) => {{i{user}.health = ActorHealth::Zombie(s.parse::<i32>().expect(\"Invalid input\"))}}
                            }}
                            }}"
                        ));
                    }
                    SBFunction::TimeWarp => {
                        let body: Vec<String> = if f.body.is_some() {
                            f.body.as_ref().expect_compile_error("Unkown error: expected body, but was empty.")
                                .body.iter()
                                .map(|node| {
                                    self.generate_next(node)
                                }
                                ).collect::<Vec<String>>()
                        } else {
                            Vec::new()
                        };
                        let consumed = f.parameter.expect_compile_error("Expected a parameter for spell `time_warp`.");
                        return_s.push_str(&format!(
                            /**/
                            "{{
                                let mut loop_times = match &mut i{consumed}.health {{
                                    ActorHealth::Char(val) => {{
                                        *val
                                    }}
                                    ActorHealth::Zombie(val) => {{runtime_error!(\"Zombies don't like loops.\")}}
                                }};
                                while loop_times != 0 {{
                                    {}
                                    i{consumed}.attacked(1);
                                    
                                    loop_times =  match &mut i{consumed}.health {{
                                        ActorHealth::Char(val) => {{
                                            *val
                                        }}
                                        ActorHealth::Zombie(val) => {{runtime_error!(\"Zombies don't like loops.\")}}
                                    }};
                                }}
                            }}",
                            body.join("\n")
                            // NOTE: actors are consumed at the end of an iteration
                        ));
                    }
                    SBFunction::Shift => {
                        let user = f.user;
                        return_s.push_str(&format!("\
                        {{ let health = i{user}.attack;
                        if let ActorHealth::Char(attack) = i{user}.health {{
                            i{user}.attack = attack;
                            i{user}.health = ActorHealth::Char(health);
                        }}
                        }}"));
                    }
                    SBFunction::CreatePot => {
                        let user = f.user;
                        let potion = f.parameter.unwrap();
                        // return_s.push_str(&format!(
                        //     "let i{potion} = Item::Potion({potion}, if let Some(h) = i{user}.health {{\
                        //     h}} else {{runtime_error!(\"Actor does not exist.\"}});"
                        // ))
                        return_s.push_str(&format!(
                            "let potion_index = i{user}.items.iter().enumerate().find_map(|(i,item)| if item == &&i{potion} {{Some(i)}} else {{None}}); \
                            i{user}.items.remove(potion_index.unwrap());\
                            let health: u32 = if let ActorHealth::Char(h) = i{user}.health {{
                                h
                            }} else {{ runtime_error!(\"Only actors can make potions.\") }};
                            let i{potion} = Item::Potion({potion}, health);
                            i{user}.items.push(&i{potion});"
                            //if let ActorHealth::Char(h) = i{user}.health {{\
                            // i{potion}.set_val(h);\
                            // }}"
                        ))
                    }
                }
                return return_s;
            }
            NodeType::FnBody => {}
        }
        unimplemented!("That function has not been implemented.")
    }
}