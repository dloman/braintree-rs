use std::collections::HashMap;
use std::fmt::Write;
use std::io::Read;
use xml;

use crate::credit_card::CreditCard as CreditCard;
use crate::ToXml;

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
    /// A Collection of custom field/value pairs. Fields and
    /// values must be less than 255 character. You must set up
    /// each custom field in the Control Panel prior to passing
    /// it with a request. Querying this value returns a collection
    /// of custom field values stored on the customer object.
    pub custom_fields: Option<HashMap<String, String>>,
}

impl ToXml for Customer {
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
        if let Some(ref credit_card) = self.credit_card { write!(s, "{}", credit_card.to_xml(Some("credit_card"))).unwrap(); }
        if let Some(ref custom_fields) = self.custom_fields {
            write!(s, "<custom_fields>").unwrap();
            for (key, value) in custom_fields.into_iter() {
                write_xml!(s, key, Some(value));
            }
            write!(s, "</custom_fields>").unwrap();
        }
        write!(s, "</{}>", name).unwrap();
        s
    }
}

fn make_customer_fields(root: &elementtree::Element) -> HashMap<String, String> {
    let mut custom_fields = HashMap::new();

    for child in root.children() {
        custom_fields.insert(child.tag().to_string(), child.text().to_string());
    }
    custom_fields
}

impl From<Box<dyn Read>> for Customer {
    fn from(xml: Box<dyn Read>) -> Customer {
        let root = elementtree::Element::from_reader(xml).unwrap();

        Customer{
            company: Some(String::from(root.find("company").unwrap().text())),
            first_name: Some(String::from(root.find("first-name").unwrap().text())),
            last_name: Some(String::from(root.find("last-name").unwrap().text())),
            id: Some(String::from(root.find("id").unwrap().text())),
            fax: Some(String::from(root.find("fax").unwrap().text())),
            phone: Some(String::from(root.find("phone").unwrap().text())),
            website: Some(String::from(root.find("website").unwrap().text())),
            credit_card: Some(CreditCard::from(root.find("credit-cards").expect("no credit card found").find("credit-card").unwrap())),
            custom_fields: Some(make_customer_fields(root.find("custom-fields").expect("no custom fields found"))),
            ..Default::default()
        }
    }
}
