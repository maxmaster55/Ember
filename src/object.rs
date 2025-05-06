
#[derive(Clone, Debug)]
pub enum ObjectType {
    Number(i64),
    String(String),
    Boolean(bool),
    Null,
}

pub trait Object {
    fn inspect(&self) -> String;
    fn get_type(&self) -> ObjectType;
    fn to_string(&self) -> String { self.inspect() }

}

impl Object for ObjectType {
    fn inspect(&self) -> String {
        match self {
            ObjectType::Number(int) => int.to_string(),
            ObjectType::String(str) => str.clone(),
            ObjectType::Boolean(bool) => bool.to_string(),
            ObjectType::Null => "Null".to_string(),
        }
    }

    fn get_type(&self) -> ObjectType {
        self.clone()
    }


}

