use std::fmt::Write;
use xml;

use crate::address::Address as Address;
use crate::ToXml;

/// A record that includes credit card information.
///
/// Generally, it's recommended to use a payment method nonce instead of raw
/// credit card data for compliance reasons. Handling credit card data yourself
/// means that you're subject to [PCI SAQ D
/// compliance](https://www.pcisecuritystandards.org/pci_security/completing_self_assessment).
#[derive(Debug, Default)]
pub struct CreditCard {
    pub cardholder_name: Option<String>,
    pub cvv: Option<String>,
    pub expiration_date: Option<String>,
    pub expiration_month: Option<String>,
    pub expiration_year: Option<String>,
    pub number: Option<String>,
    pub customer_id: Option<String>,
    pub token: Option<String>,
    pub billing_address: Option<Address>,
}

impl ToXml for CreditCard {
    fn to_xml(&self, name: Option<&str>) -> String {
        let name = xml::escape(&name.unwrap_or("credit-card"));
        let mut s = String::new();
        write!(s, "<{}>", name).unwrap();
        write_xml!(s, "cardholder-name", self.cardholder_name);
        write_xml!(s, "cvv", self.cvv);
        write_xml!(s, "expiration-date", self.expiration_date);
        write_xml!(s, "expiration-month", self.expiration_month);
        write_xml!(s, "expiration-year", self.expiration_year);
        write_xml!(s, "number", self.number);
        write_xml!(s, "token", self.token);
        if let Some(ref billing_address) = self.billing_address { write!(s, "{}", billing_address.to_xml(Some("billing-address"))).unwrap(); }
        write!(s, "</{}>", name).unwrap();
        s
    }
}

impl From<&elementtree::Element> for CreditCard {
    fn from(root: &elementtree::Element) -> CreditCard {
        CreditCard{
            cardholder_name: Some(String::from(root.find("cardholder-name").unwrap().text())),
            expiration_month: Some(String::from(root.find("expiration-month").unwrap().text())),
            expiration_year: Some(String::from(root.find("expiration-year").unwrap().text())),
            token: Some(String::from(root.find("token").unwrap().text())),
            customer_id: Some(String::from(root.find("customer-id").unwrap().text())),
            billing_address: Some(Address::from(root.find("billing-address").unwrap())),
            ..Default::default()
        }
    }
}
