#[allow(unused)]
#[warn(dead_code)]
#[warn(unused_variables)]

use std::cell::RefCell;
use std::rc::Rc;

struct User {
    role: String,
}

struct Document {
    state: Box<dyn State>,
    user: Rc<User>,
}

impl Document {
    pub fn new(user: Rc<User>) -> Self {
        Document { state: Box::new(Draft), user: user }
    }

    pub fn set_state(&mut self , state: Box<dyn State>) {
        println!("Changing to: {:?}", state.to_str());
        self.state = state;
    }

    pub fn get_state(&self) -> Box<dyn State> {
        self.state.clone_box()
    }
}



trait State {
    fn clone_box(&self) -> Box<dyn State>;
    fn publish(&self, document: &mut Document) -> Box<dyn State>;
    fn to_str(&self) -> String;
}


#[derive(Clone)]
struct Draft;
impl State for Draft {
    fn clone_box(&self) -> Box<dyn State> {
        Box::new(self.clone())
    }

    fn publish(&self, _document: &mut Document) -> Box<dyn State> {
        println!("Going from Draft to Moderation.");

        return Box::new(Moderation)
    }

    fn to_str(&self) -> String {
        "Draft".to_owned()
    }
}

#[derive(Clone)]
struct Moderation;
impl State for Moderation {
    fn clone_box(&self) -> Box<dyn State> {
        Box::new(self.clone())
    }
    
    fn publish(&self, document: &mut Document) -> Box<dyn State> {
        if document.user.role == "admin" {
            println!("Going from Moderation to Published.");

            return Box::new(Published)
        } else {
            println!("User is not authorized to publish the document.");
            return Box::new(Moderation)
        }
    }

    fn to_str(&self) -> String {
        "Moderation".to_owned()
    }
}

#[derive(Clone)]
struct Published;
impl State for Published {
    fn clone_box(&self) -> Box<dyn State> {
        Box::new(self.clone())
    }

    fn publish(&self, _document: &mut Document) -> Box<dyn State> {
        println!("Document is already published. No further actions.");
        return Box::new(Published)
    }

    fn to_str(&self) -> String {
        "Published".to_owned()
    }
}

fn main() {

    "Teste para caso de user admin"; {

        let user = Rc::new(User { role: String::from("admin") });
        let mut doc = Document::new(user.clone());
    
        let state = doc.get_state(); {
            let state = state.publish(&mut doc);
            doc.set_state(state);
        };
    
        let state = doc.get_state(); {
            let state = state.publish(&mut doc);
            doc.set_state(state);
        };
    
        let state = doc.get_state(); {
            let state = state.publish(&mut doc);
            doc.set_state(state);
        };
    }

    "Teste para caso de user not admin"; {

        let user = Rc::new(User { role: String::from("user") });
        let mut doc = Document::new(user.clone());
    
        let state = doc.get_state(); {
            let state = state.publish(&mut doc);
            doc.set_state(state);
        };
    
        let state = doc.get_state(); {
            let state = state.publish(&mut doc);
            doc.set_state(state);
        };
    
        let state = doc.get_state(); {
            let state = state.publish(&mut doc);
            doc.set_state(state);
        };
    }

}
