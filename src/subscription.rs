use elementtree;
use std::convert::From;
use std::io::Read;
use std::fmt::Write;
use xml;

use crate::ToXml;

/// A request to retrieve a new client token.
#[derive(Debug, Default)]
pub struct Request {
    /// The plan identifies. Values can use only letters numbers "-" and "_".
    /// Plans must be created in the Contorl Panel
    pub plan_id: Option<String>,
    /// An alphanumeric value that references a specific payment method
    /// stored in yourt Vault. It is required when creating a subscription
    /// unless you pass a payment_method_nonce instead, which can be done
    /// under certain circumstances
    pub payment_method_token: Option<String>,
}

impl ToXml for Request {
    fn to_xml(&self, name: Option<&str>) -> String {
        let name = xml::escape(&name.unwrap_or("subscription"));
        let mut s = String::new();
        write!(s, "<{}>", name).unwrap();

        write_xml!(s, "plan-id", self.plan_id);
        write_xml!(s, "payment-method-token", self.payment_method_token);

        write!(s, "<options>" ).unwrap();
        write_xml_type!(s, "start-immediately", "boolean", Some(true));
        write!(s, "</options>" ).unwrap();
        write!(s, "</{}>", name).unwrap();
        s
    }
}

#[derive(Debug)]
pub struct Subscription {
    /// The value of the client token.
    pub id: String,
    pub plan_id: String,
    pub payment_method_token: String,
    pub status: String,
    pub billing_period_start_date: String,
    pub billing_period_end_date: String,
    pub created_at: String,
}

impl From<Box<dyn Read>> for Subscription {
    fn from(xml: Box<dyn Read>) -> Subscription {
        let root = elementtree::Element::from_reader(xml).unwrap();
        Subscription{
            id: String::from(root.find("id").unwrap().text()),
            plan_id: String::from(root.find("plan-id").unwrap().text()),
            payment_method_token: String::from(root.find("payment-method-token").unwrap().text()),
            status: String::from(root.find("status").unwrap().text()),
            billing_period_start_date: String::from(root.find("billing-period-start-date").unwrap().text()),
            billing_period_end_date: String::from(root.find("billing-period-end-date").unwrap().text()),
            created_at: String::from(root.find("created-at").unwrap().text()),
        }
    }
}
