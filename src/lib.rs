//! Bindings to Braintree's API.
//!
//! For those unfamiliar with Braintree or payments processing, [Braintree's
//! homepage](https://www.braintreepayments.com/) is a good place to start to
//! learn more, along with the [developer
//! documentation](https://developers.braintreepayments.com/) which provides a
//! good overview of the available tools and API's.
//!
//! Note that this is an unofficial library, with no direct support from
//! Braintree themselves. The goal is to provide a set of reasonably-complete
//! bindings to core functionality, but naturally a lot of it will be incomplete.
//! Pull requests are welcome!
//!
//! The first thing you'll need to do is create a [sandbox
//! account](https://www.braintreepayments.com/sandbox), which you can use
//! to test your integration without needing to go through the full application
//! process. Once you've created an account, follow [these
//! instructions](https://articles.braintreepayments.com/control-panel/important-gateway-credentials#api-credentials)
//! to retrieve your Merchant ID, Public Key, and Private Key. Once you have those,
//! you should be able to create your first transaction! Naturally you'll need to
//! substitute those three values in for the placeholders below, and it bears
//! repeating that you should _never_ commit those credentials to source control:
//!
//! ```rust
//! extern crate braintree;
//!
//! use braintree::{Braintree, CreditCard, Environment};
//! use braintree::transaction;
//! use std::error::Error;
//!
//! fn main() {
//!     // Create a handle to the Braintree API.
//!     let bt = Braintree::new(
//!         Environment::Sandbox,
//!         "<merchant_id>",
//!         "<public_key>",
//!         "<private_key>",
//!     );
//!
//!     // Attempt to charge the provided credit card $10.
//!     let result = bt.transaction().create(transaction::Request{
//!         amount: String::from("10.00"),
//!         credit_card: Some(CreditCard{
//!             number: Some(String::from("4111111111111111")),
//!             expiration_date: Some(String::from("10/20")),
//!             ..Default::default()
//!         }),
//!         options: Some(transaction::Options{
//!             submit_for_settlement: Some(true),
//!             ..Default::default()
//!         }),
//!         ..Default::default()
//!     });
//!
//!     // Check to see if it worked.
//!     match result {
//!         Ok(transaction) => println!("Created transaction: {}", transaction.id),
//!         Err(err) => println!("Error: {}", err.description()),
//!     }
//! }
//! ```
//!
//! Once you've decided that your integration is good to go live, you'll need
//! to get a separate set of production credentials by signing up on
//! Braintree's main site. Remember to also change `Environment::Sandbox` to
//! `Environment::Production` when you make the switch.
//!
//! # Stability Note
//!
//! This crate is very much in a pre-alpha state, and as such the design of its
//! API is subject to change. You have been forewarned!

extern crate elementtree;
#[macro_use] extern crate hyper;
extern crate hyper_native_tls;
extern crate libflate;
extern crate xml;

macro_rules! write_xml {
    ($s:expr, $elem:expr, $value:expr) => {
        if let Some(ref value) = $value {
            write!($s, "<{}>{}</{}>", $elem, &xml::escape(&value.to_string()), $elem).unwrap();
        }
    }
}

macro_rules! write_xml_type {
    ($s:expr, $elem:expr, $typ:expr, $value:expr) => {
        if let Some(ref value) = $value {
            write!($s, "<{} type=\"{}\">{}</{}>", $elem, $typ, &xml::escape(&value.to_string()), $elem).unwrap();
        }
    }
}


header! { (XApiVersion, "X-ApiVersion") => [u8] }

use std::io::Read;
pub mod address;
pub mod client_token;
pub mod credit_card;
pub mod descriptor;
pub mod customer;
pub mod error;
pub mod transaction;

pub use address::Address as Address;
pub use credit_card::CreditCard as CreditCard;
pub use descriptor::Descriptor as Descriptor;
pub use customer::Customer as Customer;
pub use error::Error as Error;

pub struct Braintree {
    creds: Box<dyn Credentials>,
    client: hyper::Client,
    merchant_url: hyper::Url,
    user_agent: String,
}

impl Braintree {
    pub fn new<S>(env: Environment, merchant_id: S, public_key: S, private_key: S) -> Braintree
        where S: Into<String>
    {
        let ssl = hyper_native_tls::NativeTlsClient::new().unwrap();
        let connector = hyper::net::HttpsConnector::new(ssl);

        let merchant_id = merchant_id.into();
        let public_key = public_key.into();
        let private_key = private_key.into();
        // Calculate some things in advance.
        let merchant_url = hyper::Url::parse(&format!("{}/merchants/{}/", env.base_url(), merchant_id)).unwrap();
        Braintree{
            creds: Box::new(ApiKey{
                       env: env,
                       merchant_id: merchant_id,
                       auth_header: hyper::header::Basic{username: public_key.clone(), password: Some(private_key.clone())},
                       public_key: public_key,
                       private_key: private_key,
                   }),
            client: hyper::Client::with_connector(connector),
            merchant_url: merchant_url,
            user_agent: format!("Braintree Rust {}", env!("CARGO_PKG_VERSION")),
        }
    }

    pub fn client_token(&self) -> ClientTokenGateway {
        ClientTokenGateway(self)
    }

    pub fn customer(&self) -> CustomerGateway {
        CustomerGateway(self)
    }

    pub fn transaction(&self) -> TransactionGateway {
        TransactionGateway(self)
    }

    pub fn testing(&self) -> TestingGateway {
        TestingGateway(self)
    }

    fn execute(&self, method: hyper::method::Method, path: &str, body: Option<&[u8]>) -> hyper::error::Result<hyper::client::response::Response> {
        use hyper::header::{self, Quality, QualityItem};
        use hyper::mime::{Mime, TopLevel, SubLevel};

        let url = self.merchant_url.join(&path).unwrap();

        let mut req = self.client.request(method, url)
            .header(header::ContentType(Mime(TopLevel::Application, SubLevel::Xml, vec![])))
            .header(header::Accept(vec![QualityItem::new(Mime(TopLevel::Application, SubLevel::Xml, vec![]), Quality(1000))]))
            .header(header::AcceptEncoding(vec![QualityItem::new(header::Encoding::Gzip, Quality(1000))]))
            .header(header::UserAgent(self.user_agent.clone()))
            .header(header::Authorization(self.creds.authorization_header()))
            .header(XApiVersion(4));

        if let Some(data) = body {
            req = req.body(hyper::client::Body::BufBody(data, data.len()));
        }

        req.send()
    }

    /// Returns a reader that will correctly decode the response body's data based on its Content-Encoding header.
    fn response_reader(&self, response: hyper::client::response::Response) -> hyper::error::Result<Box<dyn Read>> {
        // TODO: This is written this way in order to appease the borrow checker, but there's probably a better way to do this.
        let headers = response.headers.clone();
        let content_encoding = headers.get::<hyper::header::ContentEncoding>();
        let mut r: Box<dyn Read> = Box::new(response);
        // ???: Use Content-Length somehow to provide a hint to the consumer?
        if let Some(content_encoding) = content_encoding {
            match content_encoding[0] {
                hyper::header::Encoding::Gzip => {
                    r = Box::new(libflate::gzip::Decoder::new(r)?);
                },
                _ => panic!("unsupported content encoding: {}", content_encoding[0]),
            }
        }
        Ok(r)
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Environment {
    Sandbox,
    Production,
}

impl Environment {
    fn base_url(&self) -> &str {
        match *self {
            Environment::Sandbox => "https://sandbox.braintreegateway.com",
            Environment::Production => "https://www.braintreegateway.com",
        }
    }
}

trait Credentials {
    fn environment(&self) -> Environment;
    fn merchant_id(&self) -> &str;
    fn authorization_header(&self) -> hyper::header::Basic;
}

#[allow(dead_code)]
struct ApiKey {
    env: Environment,
    merchant_id: String,
    public_key: String,
    private_key: String,
    auth_header: hyper::header::Basic,
}

impl Credentials for ApiKey {
    fn environment(&self) -> Environment { self.env }
    fn merchant_id(&self) -> &str { &self.merchant_id }
    fn authorization_header(&self) -> hyper::header::Basic { self.auth_header.clone() }
}

pub struct ClientTokenGateway<'a>(&'a Braintree);

impl<'a> ClientTokenGateway<'a> {
    /// Generate a client token. The simplest usage is:
    ///
    /// ```rust
    /// let client_token = bt.client_token().generate(Default::default());
    /// ```
    ///
    /// Further customization can be done by manually specifying your own `client_token::Request` value.
    pub fn generate(&self, req: client_token::Request) -> error::Result<client_token::ClientToken> {
        let response = self.0.execute(hyper::method::Method::Post, "client_token", Some(req.to_xml(None).as_bytes()))?;
        match response.status {
            hyper::status::StatusCode::Created => Ok(client_token::ClientToken::from(self.0.response_reader(response)?)),
            _ => Err(Error::from(self.0.response_reader(response)?)),
        }
    }
}

pub struct CustomerGateway<'a>(&'a Braintree);

impl<'a> CustomerGateway<'a> {
    /// Generate a customer. The simplest usage is:
    ///
    /// ```rust
    /// let customer = bt.customer().generate(Default::default());
    /// ```
    ///
    pub fn generate(&self, req: Customer) -> error::Result<customer::Customer> {
        let response = self.0.execute(hyper::method::Method::Post, "customers", Some(req.to_xml(None).as_bytes()))?;
        match response.status {
            hyper::status::StatusCode::Created => Ok(customer::Customer::from(self.0.response_reader(response)?)),
            _ => Err(Error::from(self.0.response_reader(response)?)),
        }
    }
}

pub struct TransactionGateway<'a>(&'a Braintree);

impl<'a> TransactionGateway<'a> {
    /// Create a transaction! This is the meat and potatoes of payments
    /// processing right here. At a minimum you will need to provide an amount
    /// and some form of payment method.
    ///
    /// Note that in order for the transaction to process, you'll need to do
    /// one of two things: create the transaction with the
    /// `submit_for_settlement` option set to `true`, or save the transaction
    /// id and call `submit_for_settlement()` with it later. Failure to do so
    /// will result in a lack of money flowing into your bank account, which is
    /// obviously no good.
    ///
    /// For more information, check out Braintree's documentation on the
    /// [transaction
    /// lifecycle](https://articles.braintreepayments.com/support/get-started/transaction-life-cycle).
    pub fn create(&self, transaction: transaction::Request) -> error::Result<transaction::Transaction> {
        let response = self.0.execute(hyper::method::Method::Post, "transactions", Some(transaction.to_xml(None).as_bytes()))?;
        match response.status {
            hyper::status::StatusCode::Created => Ok(transaction::Transaction::from(self.0.response_reader(response)?)),
            _ => Err(Error::from(self.0.response_reader(response)?)),
        }
    }

    /// Submit an authorized transaction for settlement.
    pub fn submit_for_settlement(&self, transaction_id: String) -> error::Result<transaction::Transaction> {
        let response = self.0.execute(hyper::method::Method::Put, &format!("transactions/{}/submit_for_settlement", transaction_id), None)?;
        match response.status {
            hyper::status::StatusCode::Ok => Ok(transaction::Transaction::from(self.0.response_reader(response)?)),
            _ => Err(Error::from(self.0.response_reader(response)?)),
        }
    }

    /// If a transaction has yet to be captured (i.e. it should be in a state
    /// of `Authorized` or `SubmittedForSettlement`), you can cancel it by
    /// calling void.
    pub fn void(&self, transaction_id: String) -> error::Result<transaction::Transaction> {
        let response = self.0.execute(hyper::method::Method::Put, &format!("transactions/{}/void", transaction_id), None)?;
        match response.status {
            hyper::status::StatusCode::Ok => Ok(transaction::Transaction::from(self.0.response_reader(response)?)),
            _ => Err(Error::from(self.0.response_reader(response)?)),
        }
    }

    /// When a transaction has been settled, you can refund it, which creates a
    /// new credit transaction. You must pass a settled or settling
    /// `transaction_id` in order to execute a valid refund.
    pub fn refund(&self, transaction_id: String) -> error::Result<transaction::Transaction> {
        // TODO: add an optional amount to refund the transaction partially
        let response = self.0.execute(hyper::method::Method::Post, &format!("transactions/{}/refund", transaction_id), None)?;
        match response.status {
            hyper::status::StatusCode::Created|hyper::status::StatusCode::Ok => Ok(transaction::Transaction::from(self.0.response_reader(response)?)),
            _ => Err(Error::from(self.0.response_reader(response)?)),
        }
    }

    /// Retrieve details for a transaction.
    pub fn find(&self, transaction_id: String) -> error::Result<transaction::Transaction> {
        let response = self.0.execute(hyper::method::Method::Get, &format!("transactions/{}", transaction_id), None)?;
        match response.status {
            hyper::status::StatusCode::Ok => Ok(transaction::Transaction::from(self.0.response_reader(response)?)),
            _ => Err(Error::from(self.0.response_reader(response)?)),
        }
    }
}

pub struct TestingGateway<'a>(&'a Braintree);

impl<'a> TestingGateway<'a> {
    fn set_status(&self, transaction_id: String, status: String) -> error::Result<transaction::Transaction> {
        if self.0.creds.environment() == Environment::Production {
            return Err(Error::TestOperationInProduction);
        }
        let response = self.0.execute(hyper::method::Method::Put, &format!("transactions/{}/{}", transaction_id, status), None)?;
        match response.status {
            hyper::status::StatusCode::Ok => Ok(transaction::Transaction::from(self.0.response_reader(response)?)),
            _ => Err(Error::from(self.0.response_reader(response)?)),
        }
    }

    /// Force a transaction into a settled state. Note that this is intended
    /// for testing, and will only work in the Sandbox environment.
    pub fn settle(&self, transaction_id: String) -> error::Result<transaction::Transaction> {
        self.set_status(transaction_id, String::from("settle"))
    }
}

trait ToXml {
    fn to_xml(&self, name: Option<&str>) -> String;
}
