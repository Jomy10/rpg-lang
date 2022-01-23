use simple_colors::blue;

use crate::node::*;
use crate::{Token, TokenType, uid};
use crate::user_output::CompileError;

pub struct Parser<'a> {
    tokens: &'a mut Vec<Token>,
    /// Contains the names of all the named objects
    ids: Vec<(String, usize)>
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a mut Vec<Token>) -> Self {
        Self { tokens, ids: Vec::new() }
    }
    
    pub fn parse(&mut self) -> Vec<Box<dyn Node + Send + Sync>> {
        let mut nodes: Vec<Box<dyn Node + Send + Sync>> = Vec::new();
        while !self.tokens.is_empty() {
            nodes.push(self.parse_next_statement());
        }
        nodes
    }
    
    fn parse_next_statement(&mut self) -> Box<dyn Node + Send + Sync> {
        // Match all types that can be at the beginning of a statement
        match self.tokens[0].ttype {
            TokenType::Char => Box::new(self.parse_char()),
            TokenType::Zombie => Box::new(self.parse_zombie()),
            TokenType::Merchant => Box::new(self.parse_merchant()),
            TokenType::Potion => Box::new(self.parse_potion()),
            TokenType::SpellBook => Box::new(self.parse_spellbook()),
            TokenType::Identifier => self.parse_identifier(),
            other => crate::compile_error!("Unexpected token type at beginning of statement: {}", other.to_string())
        }
    }
    
    fn parse_char(&mut self) -> node::Char {
        self.consume(TokenType::Char);
        let name = self.consume(TokenType::Identifier).value;
        self.consume(TokenType::Equals);
        self.consume(TokenType::OParen);
        let health = self.consume(TokenType::Integer).value
            .parse::<u32>()
            .expect_compile_error(&format!(
                "Character {} has an invalid value for its health.\n{} Characters can only have \
                non-negative health. Their health must be an unsigned 32-bit integer.",
                name,
                simple_colors::cyan!("HELP:"))
            );
        self.consume(TokenType::Comma);
        let attack = self.consume(TokenType::Integer).value
            .parse::<u32>()
            .expect_compile_error(&format!(
                "Character {} has an invalid value for its attack.\n{} Attack must be non-negative. \
                Attack is an unsigned 32-bit integer.",
                name,
                simple_colors::cyan!("HELP:"))
            );
        self.consume(TokenType::CParen);
        
        let id = uid::generate_uid();
        self.ids.push((name.clone(), id));
        
        node::Char {
            id,
            name,
            health,
            attack
        }
    }
    
    fn parse_zombie(&mut self) -> node::Zombie {
        self.consume(TokenType::Zombie);
        let name = self.consume(TokenType::Identifier).value;
        self.consume(TokenType::Equals);
        self.consume(TokenType::OParen);
        let health = self.consume(TokenType::Integer).value
            .parse::<i32>()
            .expect_compile_error(&format!(
                "Zombie {} has an invalid value for its health.\n{} Make sure you have put in an \
                integer. Zombie's health are signed 32-bit integers.",
                name,
                simple_colors::cyan!("HELP:"))
            );
        self.consume(TokenType::Comma);
        let attack = self.consume(TokenType::Integer).value
            .parse::<u32>()
            .expect_compile_error(&format!(
                "Zombie {} has an invalid value for its attack.\n{} Attack must be non-negative. \
                Attack is an unsigned 32-bit integer.",
                name,
                simple_colors::cyan!("HELP:"))
            );
        self.consume(TokenType::CParen);
        
        let id = uid::generate_uid();
        self.ids.push((name.clone(), id));
        
        node::Zombie {
            id,
            name,
            health,
            attack
        }
    }
    
    /// Parses a merchant creation statement
    fn parse_merchant(&mut self) -> node::Merchant {
        self.consume(TokenType::Merchant);
        let name = self.consume(TokenType::Identifier).value;
        self.consume(TokenType::Equals);
        self.consume(TokenType::OParen);
        self.consume(TokenType::CParen);
        
        let id = uid::generate_uid();
        self.ids.push((name.clone(), id));
        
        node::Merchant {
            id,
            name
        }
    }
    
    /// Parses a potion creation statement
    fn parse_potion(&mut self) -> node::Potion {
        self.consume(TokenType::Potion);
        let name = self.consume(TokenType::Identifier).value;
        self.consume(TokenType::Equals);
        self.consume(TokenType::OParen);
        let value = self.consume(TokenType::Integer).value
            .parse::<u32>()
            .expect_compile_error(&format!(
                "Potion {} has an invalid value for its healing value.\n{} Potions can only have \
                non-negative healing values. Their value must be an unsigned 32-bit integer.",
                name,
                simple_colors::cyan!("HELP:"))
            );
        self.consume(TokenType::CParen);
        
        let id = uid::generate_uid();
        self.ids.push((name.clone(), id));
        
        node::Potion {
            id,
            name,
            value
        }
    }
    
    /// Parses a spellbook creation statement
    fn parse_spellbook(&mut self) -> node::SpellBook {
        self.consume(TokenType::SpellBook);
        let name = self.consume(TokenType::Identifier).value;
        self.consume(TokenType::Equals);
        self.consume(TokenType::OParen);
        self.consume(TokenType::CParen);
        
        let id = uid::generate_uid();
        self.ids.push((name.clone(), id));
        
        node::SpellBook {
            id,
            name
        }
    }
    
    /// Called when an identifier is at the beginning of a statement
    fn parse_identifier(&mut self) -> Box<dyn Node + Send + Sync> {
        let ident = self.consume(TokenType::Identifier);
        return match {
            if let Some(t) = self.peek_type(0) { t }
            else { crate::compile_error!("Expected an action after identifier {}, but got none.", ident.value) }
        } {
            TokenType::FnBuys => Box::new(self.parse_fn_buys(&ident)),
            TokenType::FnAttacks => Box::new(self.parse_fn_attacks(&ident)),
            TokenType::FnShouts => self.parse_fn_shouts(&ident),
            TokenType::FnWhispers => self.parse_fn_whispers(&ident),
            TokenType::FnUses => self.parse_fn_uses(&ident),
            TokenType::FnCasting => crate::compile_error!("Casting can not be used on its own. It has to be used alongside a `uses` action."),
            v => crate::compile_error!("Expected an action after identifier {}, but got {}", ident.value, v.to_string())
        }
    }
    
    /// Parses a buys expression `c1 buys from m1`
    fn parse_fn_buys(&mut self, ident: &Token) -> node::FnBuys {
        let user = &ident.value;
        self.consume(TokenType::FnBuys);
        let item = self.consume(TokenType::Identifier).value;
        self.consume(TokenType::From);
        let merchant = self.consume(TokenType::Identifier).value;
        let user_id = &self.ids.iter().find_map(|obj| {
            if &obj.0 == user {
                Some(obj.1)
            } else {
                None
            }
        }).expect_compile_error(&format!("No character {} exists.\n{} Define the character before you use it.", user, blue!("HINT:")));
        // Note: Parser does not check if the right type is given, only if the ident exists!
        let item_id = &self.ids.iter().find_map(|obj| {
            if &obj.0 == &item {
                Some(obj.1)
            } else {
                None
            }
        }).expect_compile_error(&format!("No item {} exists.\n{} Define the item before you use it.", item, blue!("HINT:")));
        // Same note
        let merchant_id = &self.ids.iter().find_map(|obj| {
            if &obj.0 == &merchant {
                Some(obj.1)
            } else {
                None
            }
        }).expect_compile_error(&format!("No merchant {} exists.\n{} Define the merchant before you use it.", merchant, blue!("HINT:")));
        
        node::FnBuys {
            id: uid::generate_uid(),
            user: *user_id,
            item: *item_id,
            merchant: *merchant_id
        }
    }
    
    /// Parses an attacks expression `c1 attacks c2`
    fn parse_fn_attacks(&mut self, ident: &Token) -> node::FnAttacks {
        let attacker = &ident.value;
        self.consume(TokenType::FnAttacks);
        let attacked = self.consume(TokenType::Identifier).value;
        let attacker_id = self.ids.iter().find_map(|obj| {
            if &obj.0 == attacker {
                Some(obj.1)
            } else {
                None
            }
        }).expect_compile_error(&format!("No character {} exists.\n{} Define the character before you use it.", attacker, blue!("HINT:")));
        let attacked_id = self.ids.iter().find_map(|obj| {
            if &obj.0 == &attacked {
                Some(obj.1)
            } else {
                None
            }
        }).expect_compile_error(&format!("No character {} exists.\n{} Define the character before you use it.", attacked, blue!("HINT:")));
        
        node::FnAttacks {
            id: uid::generate_uid(),
            attacked: attacked_id,
            attacker: attacker_id
        }
    }
    
    fn parse_fn_uses(&mut self, ident: &Token) -> Box<dyn Node + Send + Sync> {
        let user = &ident.value;
        self.consume(TokenType::FnUses);
        let user_id = self.ids.iter().find_map(|obj| {
            if &obj.0 == user {
                Some(obj.1)
            } else {
                None
            }
        })
            .expect_compile_error(&format!("No character {} exists.\n{} Define the character before you use it.", user, blue!("HINT:")));
        let item_or_sb = self.consume(TokenType::Identifier).value;
        let item_or_sb_id = self.ids.iter().find_map(|obj| {
            if &obj.0 == &item_or_sb {
                Some(obj.1)
            } else {
                None
            }
        })
            .expect_compile_error(&format!("No item or spellbook {} exists.\n{} Define the item or spellbook before you use it.", item_or_sb, blue!("HINT:")));
        if let Ok(is_casting) = self.peek(TokenType::FnCasting, 0) {
            if is_casting {
                // Spellbook use
                self.consume(TokenType::FnCasting);
                if let Some(spell) = self.peek_type(0) {
                    return match spell {
                        TokenType::SbFnUnZombify => Box::new(self.parse_spell_un_zombify(user_id, item_or_sb_id)),
                        TokenType::SbFnConfuse => Box::new(self.parse_spell_confuse(user_id, item_or_sb_id)),
                        TokenType::SbFnGodSpeech => Box::new(self.parse_spell_god_speech(user_id, item_or_sb_id)),
                        TokenType::SbFnTimeWarp => Box::new(self.parse_spell_time_warp(user_id, item_or_sb_id)),
                        TokenType::SbFnShift => Box::new(self.parse_spell_shift(user_id, item_or_sb_id)),
                        TokenType::SbFnCreatePotion => Box::new(self.parse_spell_pot(user_id, item_or_sb_id)),
                        val => crate::compile_error!("{}", &format!("Invalid spellbook spell: {}", val.to_string()))
                    }
                }
            }
        }
        // Item (potion) use
        Box::new(
            FnUses {
                id: uid::generate_uid(),
                user: user_id,
                item: item_or_sb_id
            }
        )
    }
    
    fn parse_spell_pot(&mut self, user: usize, sb: usize) -> node::FnUsesCasting {
        self.consume(TokenType::SbFnCreatePotion);
        self.consume(TokenType::OParen);
        let potion = self.consume(TokenType::Identifier).value;
        let potion_id = self.ids.iter().find_map(|obj| {
            if &obj.0 == &potion {
                Some(obj.1)
            } else {
                None
            }
        }).expect_compile_error(&format!("No potion {} exists.\n{} Define the potion before you use it.", potion, blue!("HINT:")));
        self.consume(TokenType::CParen);
        
        node::FnUsesCasting {
            id: uid::generate_uid(),
            user,
            spell_book: sb,
            function: SBFunction::CreatePot,
            parameter: Some(potion_id),
            body: None
        }
    }
    
    fn parse_spell_shift(&mut self, user: usize, sb: usize) -> node::FnUsesCasting {
        self.consume(TokenType::SbFnShift);
        self.consume(TokenType::OParen);
        self.consume(TokenType::CParen);
    
        node::FnUsesCasting {
            id: uid::generate_uid(),
            user,
            spell_book: sb,
            function: SBFunction::Shift,
            parameter: None,
            body: None
        }
    }
    
    fn parse_spell_un_zombify(&mut self, user: usize, sb: usize) -> node::FnUsesCasting {
        self.consume(TokenType::SbFnUnZombify);
        self.consume(TokenType::OParen);
        let zombie = self.consume(TokenType::Identifier).value;
        let zombie_id = self.ids.iter().find_map(|obj| {
            if &obj.0 == &zombie {
                Some(obj.1)
            } else {
                None
            }
        }).expect_compile_error(&format!("No zombie {} exists.\n{} Define the zombie before you use it.", zombie, blue!("HINT:")));
        self.consume(TokenType::CParen);
        
        node::FnUsesCasting {
            id: uid::generate_uid(),
            user,
            spell_book: sb,
            function: SBFunction::UnZombify,
            parameter: Some(zombie_id),
            body: None
        }
    }
    
    fn parse_spell_confuse(&mut self, user: usize, sb: usize) -> node::FnUsesCasting {
        self.consume(TokenType::SbFnConfuse);
        self.consume(TokenType::OParen);
        let confused_char = self.consume(TokenType::Identifier).value;
        self.consume(TokenType::CParen);
        let confused_char_id = self.ids.iter().find_map(|obj| {
            if &obj.0 == &confused_char {
                Some(obj.1)
            } else {
                None
            }
        }).expect_compile_error(&format!("No character or zombie {} exists.\n{} Define the character or zombie before you use it.", confused_char, blue!("HINT:")));
        
        node::FnUsesCasting {
            id: uid::generate_uid(),
            user,
            spell_book: sb,
            function: SBFunction::Confuse,
            parameter: Some(confused_char_id),
            body: None
        }
    }
    
    fn parse_spell_god_speech(&mut self, user: usize, sb: usize) -> node::FnUsesCasting {
        self.consume(TokenType::SbFnGodSpeech);
        self.consume(TokenType::OParen);
        self.consume(TokenType::CParen);
        node::FnUsesCasting {
            id: uid::generate_uid(),
            user,
            spell_book: sb,
            function: SBFunction::GodSpeech,
            parameter: None,
            body: None
        }
    }
    
    fn parse_spell_time_warp(&mut self, user: usize, sb: usize) -> node::FnUsesCasting {
        self.consume(TokenType::SbFnTimeWarp);
        self.consume(TokenType::OParen);
        let consumed = self.consume(TokenType::Identifier).value;
        self.consume(TokenType::CParen);
        let consumed_id = self.ids.iter().find_map(|obj| {
            if &obj.0 == &consumed {
                Some(obj.1)
            } else {
                None
            }
        }).expect_compile_error(&format!("No character {} exists.\n{} Define the character before you use it.", consumed, blue!("HINT:")));
        let mut body: Vec<Box<dyn Node + Send + Sync>> = Vec::new();
        while !self.peek(TokenType::End, 0).expect_compile_error("Expected time warp loop to end with `end`, but got none.") {
            body.push(self.parse_next_statement());
        }
        self.consume(TokenType::End);
        node::FnUsesCasting {
            id: uid::generate_uid(),
            user,
            spell_book: sb,
            function: SBFunction::TimeWarp,
            parameter: Some(consumed_id),
            body: Some(node::FnBody {
                id: uid::generate_uid(),
                body
            })
        }
    }
    
    /// Includes regular shout and spellbook speak version
    fn parse_fn_shouts(&mut self, ident: &Token) -> Box<dyn Node + Send + Sync> {
        self.consume(TokenType::FnShouts);
        let user = &ident.value;
        let user_id = self.ids.iter().find_map(|obj| if &obj.0 == user { Some(obj.1) } else { None })
            .expect_compile_error(&format!("No character {} exists.\n{} Define the character before you use it.", user, blue!("HINT:")));
        if let Ok(is_casting) = self.peek(TokenType::FnCasting, 1) {
            if let Ok(is_speak) = self.peek(TokenType::SbFnSpeak, 2) {
                if is_casting && is_speak {
                    let spellbook = self.consume(TokenType::Identifier).value;
                    let sb_id = self.ids.iter().find_map(|obj| if &obj.0 == &spellbook { Some(obj.1) } else { None })
                        .expect_compile_error(&format!("No spellbook {} exists.\n{} Define the spellbook before you use it.", spellbook, blue!("HINT:")));
                    self.consume(TokenType::FnCasting);
                    self.consume(TokenType::SbFnSpeak);
                    self.consume(TokenType::OParen);
                    self.consume(TokenType::CParen);
                    return Box::new(node::FnShoutsSpeak {
                        id: uid::generate_uid(),
                        user: user_id,
                        spell_book: sb_id
                    });
                }
            }
        }
        Box::new(
            node::FnShouts {
                id: uid::generate_uid(),
                user: user_id
            }
        )
    }
    
    fn parse_fn_whispers(&mut self, ident: &Token) -> Box<dyn Node + Send + Sync> {
        self.consume(TokenType::FnWhispers);
        let user = &ident.value;
        let user_id = self.ids.iter().find_map(|obj| if &obj.0 == user { Some(obj.1) } else { None })
            .expect_compile_error(&format!("No character {} exists.\n{} Define the character before you use it.", user, blue!("HINT:")));
        if let Ok(is_casting) = self.peek(TokenType::FnCasting, 1) {
            if let Ok(is_speak) = self.peek(TokenType::SbFnSpeak, 2) {
                if is_casting && is_speak {
                    let spellbook = self.consume(TokenType::Identifier).value;
                    let sb_id = self.ids.iter().find_map(|obj| if &obj.0 == &spellbook { Some(obj.1) } else { None })
                        .expect_compile_error(&format!("No spellbook {} exists.\n{} Define the spellbook before you use it.", spellbook, blue!("HINT:")));
                    self.consume(TokenType::FnCasting);
                    self.consume(TokenType::SbFnSpeak);
                    self.consume(TokenType::OParen);
                    self.consume(TokenType::CParen);
                    return Box::new(node::FnWhispersSpeak {
                        id: uid::generate_uid(),
                        user: user_id,
                        spell_book: sb_id
                    });
                }
            }
        }
        Box::new(
            node::FnWhispers {
                id: uid::generate_uid(),
                user: user_id
            }
        )
    }
    
    /// Consumes the next token
    ///
    /// Panics if the next token does not match the expected_token_type
    fn consume(&mut self, expected_type: TokenType) -> Token {
        let token = self.tokens.remove(0);
        if token.ttype == expected_type {
            token
        } else {
            crate::compile_error!("Expected token type {} but got {}", expected_type.to_string(), token.ttype.to_string())
        }
    }
    
    /// Returns the token type of the next token. Returns None if there is no token at the given
    /// `offset` position.
    fn peek_type(&self, offset: usize) -> Option<TokenType> {
        if let Some(token) = self.tokens.get(offset) {
            return Some(token.ttype);
        } else {
            None
        }
    }
    
    /// Peeks at the token at the index of `offset` and returns true if the expected
    /// token was found, false otherwise.
    ///
    /// Returns err if there was no token at the offset index
    fn peek(&self, expected_type: TokenType, offset: usize) -> Result<bool, String> {
        if let Some(token) = self.tokens.get(offset) {
            Ok(token.ttype == expected_type)
        } else {
            Err("Incomplete syntax".to_string())
        }
    }
}

pub mod node {
    use std::any::Any;
    use std::fmt;
    use crate::{impl_node, new_node};
    
    new_node!(Char, name: String, health: u32, attack: u32);
    
    new_node!(Zombie, name: String, health: i32, attack: u32);
    
    new_node!(Merchant, name: String);
    
    new_node!(Potion, name: String, value: u32);
    impl Item for Potion {}
    
    new_node!(SpellBook, name: String);
    impl Item for SpellBook {}
    
    // new_node!(FnBuys, user: Char, item: Box<dyn BuyableNode>, merchant: Merchant);
    // Fields: id's of the nodes
    new_node!(FnBuys, user: usize, item: usize, merchant: usize);
    
    // The item here is a potion
    new_node!(FnUses, user: usize, item: usize);
    
    new_node!(FnUsesCasting, user: usize, spell_book: usize, function: SBFunction, parameter: Option<usize>, body: Option<FnBody>);
    
    new_node!(FnBody, body: Vec<Box<dyn Node + Send + Sync>>);
    
    new_node!(FnAttacks, attacked: usize, attacker: usize);
    
    new_node!(FnShouts, user: usize);
    new_node!(FnShoutsSpeak, user: usize, spell_book: usize);
    
    new_node!(FnWhispers, user: usize);
    new_node!(FnWhispersSpeak, user: usize, spell_book: usize);
    
    #[derive(Debug, PartialEq)]
    pub enum NodeType {
        Char,
        Zombie,
        Merchant,
        Potion,
        SpellBook,
        FnBuys,
        FnAttacks,
        FnUses,
        FnShouts,
        /// `c1 shouts sb1 casting speak()`
        FnShoutsSpeak,
        FnWhispers,
        /// `c1 shouts sb1 casting speak()`
        FnWhispersSpeak,
        /// A spell book function cast
        FnUsesCasting,
        /// The body of a `FnUsesCasting` statement of type `TimeWarp`.
        /// "SpellBody"
        FnBody
    }
    
    #[derive(Clone, Copy, Debug)]
    /// Spell book functions
    pub enum SBFunction {
        UnZombify,
        Confuse,
        GodSpeech,
        TimeWarp,
        Shift,
        CreatePot
        // Speak is always in a shouts or whispers
    }
    
    #[macro_export]
    macro_rules! new_node {
        ( $name: ident, $($field: ident: $type: ty),* ) => (
            #[derive(Debug, Clone)]
            pub struct $name {
                pub id: usize,
                $(pub $field: $type),*
            }
            
            unsafe impl Send for $name {}
            unsafe impl Sync for $name {}
            
            impl Node for $name {
                impl_node!{NodeType::$name}
            }
        )
    }
    
    #[macro_export]
    /// Implements node methods
    macro_rules! impl_node {
        ( $type: expr ) => (
            fn get_type(&self) -> NodeType {
                $type
            }
        
            fn as_any(&self) -> &dyn Any {
                self
            }
            
            fn get_id(&self) -> usize {
                self.id
            }
        )
    }
    
    pub trait Node: fmt::Debug + NodeClone + Send + Sync {
        fn get_type(&self) -> NodeType;
        fn as_any(&self) -> &dyn Any;
        fn get_id(&self) -> usize;
    }
    
    pub trait Item: fmt::Debug + ItemClone {}
    
    /// Implements Clone for Box<dyn Node + Send + Sync>
    pub trait NodeClone {
        fn clone_box(&self) -> Box<dyn Node + Send + Sync>;
    }
    
    impl<T> NodeClone for T
        where T: 'static + Node + Clone
    {
        fn clone_box(&self) -> Box<dyn Node + Send + Sync> {
            Box::new(self.clone())
        }
    }
    
    impl Clone for Box<dyn Node + Send + Sync> {
        fn clone(&self) -> Box<dyn Node + Send + Sync> {
            self.clone_box()
        }
    }
    
    pub trait ItemClone {
        fn clone_box(&self) -> Box<dyn Item>;
    }
    
    impl<T> ItemClone for T
        where T: 'static + Item + Clone
    {
        fn clone_box(&self) -> Box<dyn Item> {
            Box::new(self.clone())
        }
    }
    
    impl Clone for Box<dyn Item> {
        fn clone(&self) -> Box<dyn Item> {
            self.clone_box()
        }
    }
    
    pub fn parse_dyn_node<NodeType: 'static>(n: &dyn Node) -> &NodeType {
        let parse_node: &NodeType = match n.as_any().downcast_ref::<NodeType>() {
            Some(n) => n,
            None => crate::compile_error!("Expected wrong type")
        };
        parse_node
    }
}