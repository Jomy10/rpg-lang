use std::collections::HashMap;
use crate::node::{Node, NodeType, parse_dyn_node};
use crate::node;
use crate::compile_error;

// TODO: type check shift() & create_potion()
pub struct TypeChecker<'a> {
    nodes: &'a Vec<Box<dyn Node + Send + Sync>>,
    var_map: HashMap<usize, NodeType>
}
impl<'a> TypeChecker<'a> {
    pub fn new(nodes: &'a Vec<Box<dyn Node + Send + Sync>>) -> Self {
        Self {
            nodes,
            var_map: HashMap::new()
        }
    }
    pub fn check_types(&mut self) {
        self.check_node_types(self.nodes)
    }
    fn check_node_types(&mut self, nodes: &'a Vec<Box<dyn Node + Send + Sync>>) {
        nodes.iter().for_each(|node| {
            let node = &**node;
            match node.get_type() {
                NodeType::Char => {
                    let node: &node::Char = parse_dyn_node(node);
                    self.var_map.insert(node.id, NodeType::Char);
                }
                NodeType::Zombie => {
                    let node: &node::Zombie = parse_dyn_node(node);
                    self.var_map.insert(node.id, NodeType::Zombie);
                }
                NodeType::Merchant => {
                    let node: &node::Merchant = parse_dyn_node(node);
                    self.var_map.insert(node.id, NodeType::Merchant);
                }
                NodeType::Potion => {
                    let node: &node::Potion = parse_dyn_node(node);
                    self.var_map.insert(node.id, NodeType::Potion);
                }
                NodeType::SpellBook => {
                    let node: &node::SpellBook = parse_dyn_node(node);
                    self.var_map.insert(node.id, NodeType::SpellBook);
                }
                NodeType::FnBuys => {
                    let node: &node::FnBuys = parse_dyn_node(node);
                    if let Some(user) = self.var_map.get(&node.user) {
                        if let Some(item) = self.var_map.get(&node.item) {
                            if let Some(merchant) = self.var_map.get(&node.merchant) {
                                if user == &NodeType::Char || user == &NodeType::Zombie {
                                    if item == &NodeType::Potion || item == &NodeType::SpellBook {
                                        if !(merchant == &NodeType::Merchant) {
                                            compile_error!("Only merchants can sell items.")
                                        }
                                    } else {
                                        compile_error!("Only potions and spellbooks can be bought from a merchant.")
                                    }
                                } else {
                                    compile_error!("The one buying must be an actor.")
                                }
                            } else {
                                compile_error!("No merchant found while buying.")
                            }
                        } else {
                            compile_error!("Item you are trying to buy was not found.")
                        }
                    } else {
                        compile_error!("Actor that is trying to buy not found.")
                    }
                    
                }
                NodeType::FnAttacks => {
                    let node: &node::FnAttacks = parse_dyn_node(node);
                    if let Some(attacked) = self.var_map.get(&node.attacked) {
                        if let Some(attacker) = self.var_map.get(&node.attacker) {
                            if attacked == &NodeType::Char || attacked == &NodeType::Zombie {
                                if !(attacker == &NodeType::Char || attacker == &NodeType::Zombie) {
                                    compile_error!("The one attacking is not an actor.")
                                }
                            } else {
                                compile_error!("The one being attacked is not an actor.")
                            }
                        } else {
                            compile_error!("Attacking actor could not be found.")
                        }
                    } else {
                        compile_error!("Actor being attacked could not be found.")
                    }
                }
                NodeType::FnUses => {
                    let node: &node::FnUses = parse_dyn_node(node);
                    if let Some(user) = self.var_map.get(&node.user) {
                        if let Some(potion) = self.var_map.get(&node.item) {
                            if user == &NodeType::Char || user == &NodeType::Zombie {
                                if !(potion == &NodeType::Potion) {
                                    compile_error!("The item being used is not a potion.")
                                }
                            } else {
                                compile_error!("The user of the potion is not an actor.")
                            }
                        } else {
                            compile_error!("The potion being used was not defined.")
                        }
                    } else {
                        compile_error!("The actor using the potion was not defined.");
                    }
                }
                NodeType::FnShouts => {
                    let node: &node::FnShouts = parse_dyn_node(node);
                    if let Some(shouter) = self.var_map.get(&node.user) {
                        if !(shouter == &NodeType::Char || shouter == &NodeType::Zombie) {
                            compile_error!("The one shouting is not an actor.")
                        }
                    } else {
                        compile_error!("The actor shouting was not defined.")
                    }
                }
                NodeType::FnShoutsSpeak => {
                    let node: &node::FnShoutsSpeak = parse_dyn_node(node);
                    if let Some(shouter) = self.var_map.get(&node.user) {
                        if let Some(spellbook) = self.var_map.get(&node.spell_book) {
                            if shouter == &NodeType::Char || shouter == &NodeType::Zombie {
                                if !(spellbook == &NodeType::SpellBook) {
                                    compile_error!("The actor is not using a spellbook to shout.")
                                }
                            } else {
                                compile_error!("The one shouting is not an actor.")
                            }
                        } else {
                            compile_error!("The spellbook used for speaking was not defined.")
                        }
                    } else {
                        compile_error!("The actor shouting was not defined.")
                    }
                }
                NodeType::FnWhispers => {
                    let node: &node::FnWhispers = parse_dyn_node(node);
                    if let Some(whisperer) = self.var_map.get(&node.user) {
                        if !(whisperer == &NodeType::Char || whisperer == &NodeType::Zombie) {
                            compile_error!("The one shouting is not an actor.")
                        }
                    } else {
                        compile_error!("The actor shouting was not defined.")
                    }
                }
                NodeType::FnWhispersSpeak => {
                    let node: &node::FnWhispersSpeak = parse_dyn_node(node);
                    if let Some(whisperer) = self.var_map.get(&node.user) {
                        if let Some(spellbook) = self.var_map.get(&node.spell_book) {
                            if whisperer == &NodeType::Char || whisperer == &NodeType::Zombie {
                                if !(spellbook == &NodeType::SpellBook) {
                                    compile_error!("The actor is not using a spellbook to shout.")
                                }
                            } else {
                                compile_error!("The one shouting is not an actor.")
                            }
                        } else {
                            compile_error!("The spellbook used for speaking was not defined.")
                        }
                    } else {
                        compile_error!("The actor shouting was not defined.")
                    }
                }
                NodeType::FnUsesCasting => {
                    // TODO
                }
                NodeType::FnBody => {
                    let node: &node::FnBody = parse_dyn_node(node);
                    self.check_node_types(&node.body);
                }
            }
        });
    }
}