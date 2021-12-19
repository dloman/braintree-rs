use elementtree;
use std::convert::From;
use std::io::Read;
use std::fmt::Write;
use xml;

/// A request to retrieve a new client token.
#[derive(Debug)]
pub struct Request {
    /// The identification value for an existing customer. This value only
    /// applies to the Drop-in UI, and is used to display the customer's
    /// saved payment methods.
    pub customer_id: Option<String>,
    /// The merchant account ID that you want to use to create the transactions.
    /// If not specified, your account's default merchant account will be used.
    pub merchant_account_id: Option<String>,
    pub options: Option<Options>,
    /// The version of the client token to generate. The default value is 2,
    /// which is what most of the client SDK's currently use. Verify your
    /// client SDK's supported versions before specifying a different value.
    pub version: u8,
}

impl Default for Request {
    fn default() -> Request {
        Request{
            plan_id: None,
            payment_method_nonce: None,
        }
    }
}

impl ::ToXml for Request {
    fn to_xml(&self, name: Option<&str>) -> String {
        let name = xml::escape(&name.unwrap_or("subscription"));
        let mut s = String::new();
        write!(s, "<{}>", name).unwrap();

        write_xml!(s, "plan-id", self.plan_id);
        write_xml!(s, "payment-method-nonce", self.payment_method_nonce);

        write!(s, "<options>" ).unwrap();
        write_xml_type!(s, "start-immediately", "boolean", true);
        write!(s, "</options>" ).unwrap();
        write!(s, "</{}>", name).unwrap();
        s
    }
}

pub struct Subscription {
    /// The value of the client token.
    pub value: String,
}

impl From<Box<dyn Read>> for Subscription {
    fn from(xml: Box<dyn Read>) -> Subscription {
        let root = elementtree::Element::from_reader(xml).unwrap();
        print!("!!!! \n{:?}\n", root);
        Subscription{
            value: String::from(root.find("value").unwrap().text()),
        }
    }
}
