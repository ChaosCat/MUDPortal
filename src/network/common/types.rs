pub type Port = u16;
pub type Address<'a> = &'a str;
pub type ConnectionInfo<'a> = (Address<'a>, Port);
