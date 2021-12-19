use std::fmt::Write;
use std::io::Read;
use xml;

pub use credit_card::CreditCard as CreditCard;

#[derive(Debug, Default)]
pub struct Customer {
    pub company: Option<String>,
    pub email: Option<String>,
    pub fax: Option<String>,
    pub first_name: Option<String>,
    pub id: Option<String>,
    pub last_name: Option<String>,
    pub payment_method_nonce: Option<String>,
    pub phone: Option<String>,
    pub website: Option<String>,
    pub credit_card: Option<CreditCard>,
}

impl ::ToXml for Customer {
    fn to_xml(&self, name: Option<&str>) -> String {
        let name = xml::escape(&name.unwrap_or("customer"));
        let mut s = String::new();
        write!(s, "<{}>", name).unwrap();
        write_xml!(s, "company", self.company);
        write_xml!(s, "email", self.email);
        write_xml!(s, "fax", self.fax);
        write_xml!(s, "first-name", self.first_name);
        write_xml!(s, "id", self.id);
        write_xml!(s, "last-name", self.last_name);
        write_xml!(s, "payment_method_nonce", self.payment_method_nonce);
        write_xml!(s, "phone", self.phone);
        write_xml!(s, "website", self.website);
        write!(s, "</{}>", name).unwrap();
        s
    }
}

impl From<Box<dyn Read>> for Customer {
    fn from(xml: Box<dyn Read>) -> Customer {
        let root = elementtree::Element::from_reader(xml).unwrap();
        print!("!!!! \n{:?}\n", root);
        Customer{
            company: Some(String::from(root.find("company").unwrap().text())),
            first_name: Some(String::from(root.find("first-name").unwrap().text())),
            last_name: Some(String::from(root.find("last-name").unwrap().text())),
            id: Some(String::from(root.find("id").unwrap().text())),
            fax: Some(String::from(root.find("fax").unwrap().text())),
            phone: Some(String::from(root.find("phone").unwrap().text())),
            website: Some(String::from(root.find("website").unwrap().text())),
            ..Default::default()
        }
    }
}
