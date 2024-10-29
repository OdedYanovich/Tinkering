pub type Interface(display){
  Action(characters: String, limit: Int)
  Audio(press: fn()->Nil, action:fn(Bool)->Nil,background:fn()->Nil)
  Display(tool:display,massage:String)//make different tool and massage for the identity and the information
}
pub type DisplayInterface


pub type InputProcessing(){
  ReceiveInput:fn()->String
}
fn test(char:String)->Nil{
  case string.byte_size(char){
    1->{}
    _->{panic}
  }
}