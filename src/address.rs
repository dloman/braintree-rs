use std::fmt::Write;
use xml;

use crate::ToXml;
#[derive(Debug, Default)]
pub struct Address {
    pub company: Option<String>,
    pub country_code_alpha2: Option<String>,
    pub country_code_alpha3: Option<String>,
    pub country_code_numeric: Option<String>,
    pub country_name: Option<String>,
    pub extended_address: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub locality: Option<String>,
    pub postal_code: Option<String>,
    pub region: Option<String>,
    pub street_address: Option<String>,
}

impl ToXml for Address {
    fn to_xml(&self, name: Option<&str>) -> String {
        let name = xml::escape(&name.unwrap_or("address"));
        let mut s = String::new();
        write!(s, "<{}>", name).unwrap();
        write_xml!(s, "company", self.company);
        write_xml!(s, "country-code-alpha2", self.country_code_alpha2);
        write_xml!(s, "country-code-alpha3", self.country_code_alpha3);
        write_xml!(s, "country-code-numeric", self.country_code_numeric);
        write_xml!(s, "country-name", self.country_name);
        write_xml!(s, "extended-address", self.extended_address);
        write_xml!(s, "first-name", self.first_name);
        write_xml!(s, "last-name", self.last_name);
        write_xml!(s, "locality", self.locality);
        write_xml!(s, "postal-code", self.postal_code);
        write_xml!(s, "region", self.region);
        write_xml!(s, "street-address", self.street_address);
        write!(s, "</{}>", name).unwrap();
        s
    }
}

impl From<&elementtree::Element> for Address {
    fn from(root: &elementtree::Element) -> Address {
        Address{
            company: Some(String::from(root.find("company").unwrap().text())),
            country_code_alpha2 : Some(String::from(root.find("country-code-alpha2").unwrap().text())),
            country_code_alpha3 : Some(String::from(root.find("country-code-alpha3").unwrap().text())),
            country_code_numeric: Some(String::from(root.find("country-code-numeric").unwrap().text())),
            country_name: Some(String::from(root.find("country-name").unwrap().text())),
            extended_address: Some(String::from(root.find("extended-address").unwrap().text())),
            first_name: Some(String::from(root.find("first-name").unwrap().text())),
            last_name: Some(String::from(root.find("last-name").unwrap().text())),
            locality: Some(String::from(root.find("locality").unwrap().text())),
            postal_code: Some(String::from(root.find("postal-code").unwrap().text())),
            region: Some(String::from(root.find("region").unwrap().text())),
            street_address: Some(String::from(root.find("street-address").unwrap().text())),
        }
    }
}
