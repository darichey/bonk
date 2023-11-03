/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.457.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AssetTransactionCategoryType : Asset Transaction Category Type Enumerated derived by Vendor.

/// Asset Transaction Category Type Enumerated derived by Vendor.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AssetTransactionCategoryType {
    #[serde(rename = "ATMFee")]
    AtmFee,
    #[serde(rename = "Advertising")]
    Advertising,
    #[serde(rename = "AirTravel")]
    AirTravel,
    #[serde(rename = "AlcoholBars")]
    AlcoholBars,
    #[serde(rename = "Allowance")]
    Allowance,
    #[serde(rename = "Amusement")]
    Amusement,
    #[serde(rename = "Arts")]
    Arts,
    #[serde(rename = "AutoTransport")]
    AutoTransport,
    #[serde(rename = "AutoInsurance")]
    AutoInsurance,
    #[serde(rename = "AutoPayment")]
    AutoPayment,
    #[serde(rename = "BabySupplies")]
    BabySupplies,
    #[serde(rename = "BabysitterDaycare")]
    BabysitterDaycare,
    #[serde(rename = "BankFee")]
    BankFee,
    #[serde(rename = "BillsUtilities")]
    BillsUtilities,
    #[serde(rename = "Bonus")]
    Bonus,
    #[serde(rename = "BooksSupplies")]
    BooksSupplies,
    #[serde(rename = "Business Services")]
    BusinessServices,
    #[serde(rename = "Buy")]
    Buy,
    #[serde(rename = "CashATM")]
    CashAtm,
    #[serde(rename = "Charity")]
    Charity,
    #[serde(rename = "Check")]
    Check,
    #[serde(rename = "ChildSupport")]
    ChildSupport,
    #[serde(rename = "Clothing")]
    Clothing,
    #[serde(rename = "CoffeeShops")]
    CoffeeShops,
    #[serde(rename = "CreditCardPayment")]
    CreditCardPayment,
    #[serde(rename = "Dentist")]
    Dentist,
    #[serde(rename = "Doctor")]
    Doctor,
    #[serde(rename = "Education")]
    Education,
    #[serde(rename = "ElectronicsSoftware")]
    ElectronicsSoftware,
    #[serde(rename = "Entertainment")]
    Entertainment,
    #[serde(rename = "Eyecare")]
    Eyecare,
    #[serde(rename = "FastFood")]
    FastFood,
    #[serde(rename = "FederalTax")]
    FederalTax,
    #[serde(rename = "FeesCharges")]
    FeesCharges,
    #[serde(rename = "FinanceCharge")]
    FinanceCharge,
    #[serde(rename = "Financial")]
    Financial,
    #[serde(rename = "FinancialAdvisor")]
    FinancialAdvisor,
    #[serde(rename = "FoodDining")]
    FoodDining,
    #[serde(rename = "Furnishings")]
    Furnishings,
    #[serde(rename = "GasFuel")]
    GasFuel,
    #[serde(rename = "GiftsDonations")]
    GiftsDonations,
    #[serde(rename = "Groceries")]
    Groceries,
    #[serde(rename = "Gym")]
    Gym,
    #[serde(rename = "Hair")]
    Hair,
    #[serde(rename = "HealthFitness")]
    HealthFitness,
    #[serde(rename = "HealthInsurance")]
    HealthInsurance,
    #[serde(rename = "Hobbies")]
    Hobbies,
    #[serde(rename = "Home")]
    Home,
    #[serde(rename = "HomeImprovement")]
    HomeImprovement,
    #[serde(rename = "HomeInsurance")]
    HomeInsurance,
    #[serde(rename = "HomePhone")]
    HomePhone,
    #[serde(rename = "HomeServices")]
    HomeServices,
    #[serde(rename = "HomeSupplies")]
    HomeSupplies,
    #[serde(rename = "Hotel")]
    Hotel,
    #[serde(rename = "Income")]
    Income,
    #[serde(rename = "InterestIncome")]
    InterestIncome,
    #[serde(rename = "Internet")]
    Internet,
    #[serde(rename = "Investments")]
    Investments,
    #[serde(rename = "Kids")]
    Kids,
    #[serde(rename = "KidsActivities")]
    KidsActivities,
    #[serde(rename = "LateFee")]
    LateFee,
    #[serde(rename = "Laundry")]
    Laundry,
    #[serde(rename = "LawnGarden")]
    LawnGarden,
    #[serde(rename = "Legal")]
    Legal,
    #[serde(rename = "LifeInsurance")]
    LifeInsurance,
    #[serde(rename = "LoanInsurance")]
    LoanInsurance,
    #[serde(rename = "LoanPayment")]
    LoanPayment,
    #[serde(rename = "Loans")]
    Loans,
    #[serde(rename = "MobilePhone")]
    MobilePhone,
    #[serde(rename = "MortgageRent")]
    MortgageRent,
    #[serde(rename = "MoviesDVDs")]
    MoviesDvds,
    #[serde(rename = "Music")]
    Music,
    #[serde(rename = "NewspapersMagazines")]
    NewspapersMagazines,
    #[serde(rename = "OfficeSupplies")]
    OfficeSupplies,
    #[serde(rename = "Parking")]
    Parking,
    #[serde(rename = "Paycheck")]
    Paycheck,
    #[serde(rename = "PersonalCare")]
    PersonalCare,
    #[serde(rename = "PetFoodSupplies")]
    PetFoodSupplies,
    #[serde(rename = "PetGrooming")]
    PetGrooming,
    #[serde(rename = "Pets")]
    Pets,
    #[serde(rename = "Pharmacy")]
    Pharmacy,
    #[serde(rename = "Printing")]
    Printing,
    #[serde(rename = "Property Tax")]
    PropertyTax,
    #[serde(rename = "Public Transportation")]
    PublicTransportation,
    #[serde(rename = "Reimbursement")]
    Reimbursement,
    #[serde(rename = "RentalCarTaxi")]
    RentalCarTaxi,
    #[serde(rename = "Restaurants")]
    Restaurants,
    #[serde(rename = "SalesTax")]
    SalesTax,
    #[serde(rename = "ServiceParts")]
    ServiceParts,
    #[serde(rename = "ServiceFee")]
    ServiceFee,
    #[serde(rename = "Shipping")]
    Shipping,
    #[serde(rename = "Shopping")]
    Shopping,
    #[serde(rename = "SpaMassage")]
    SpaMassage,
    #[serde(rename = "SportingGoods")]
    SportingGoods,
    #[serde(rename = "Sports")]
    Sports,
    #[serde(rename = "StateTax")]
    StateTax,
    #[serde(rename = "Student Loan")]
    StudentLoan,
    #[serde(rename = "Taxes")]
    Taxes,
    #[serde(rename = "Television")]
    Television,
    #[serde(rename = "Toys")]
    Toys,
    #[serde(rename = "Transfer")]
    Transfer,
    #[serde(rename = "Travel")]
    Travel,
    #[serde(rename = "Tuition")]
    Tuition,
    #[serde(rename = "Uncategorized")]
    Uncategorized,
    #[serde(rename = "Utilities")]
    Utilities,
    #[serde(rename = "Vacation")]
    Vacation,
    #[serde(rename = "Veterinary")]
    Veterinary,

}

impl ToString for AssetTransactionCategoryType {
    fn to_string(&self) -> String {
        match self {
            Self::AtmFee => String::from("ATMFee"),
            Self::Advertising => String::from("Advertising"),
            Self::AirTravel => String::from("AirTravel"),
            Self::AlcoholBars => String::from("AlcoholBars"),
            Self::Allowance => String::from("Allowance"),
            Self::Amusement => String::from("Amusement"),
            Self::Arts => String::from("Arts"),
            Self::AutoTransport => String::from("AutoTransport"),
            Self::AutoInsurance => String::from("AutoInsurance"),
            Self::AutoPayment => String::from("AutoPayment"),
            Self::BabySupplies => String::from("BabySupplies"),
            Self::BabysitterDaycare => String::from("BabysitterDaycare"),
            Self::BankFee => String::from("BankFee"),
            Self::BillsUtilities => String::from("BillsUtilities"),
            Self::Bonus => String::from("Bonus"),
            Self::BooksSupplies => String::from("BooksSupplies"),
            Self::BusinessServices => String::from("Business Services"),
            Self::Buy => String::from("Buy"),
            Self::CashAtm => String::from("CashATM"),
            Self::Charity => String::from("Charity"),
            Self::Check => String::from("Check"),
            Self::ChildSupport => String::from("ChildSupport"),
            Self::Clothing => String::from("Clothing"),
            Self::CoffeeShops => String::from("CoffeeShops"),
            Self::CreditCardPayment => String::from("CreditCardPayment"),
            Self::Dentist => String::from("Dentist"),
            Self::Doctor => String::from("Doctor"),
            Self::Education => String::from("Education"),
            Self::ElectronicsSoftware => String::from("ElectronicsSoftware"),
            Self::Entertainment => String::from("Entertainment"),
            Self::Eyecare => String::from("Eyecare"),
            Self::FastFood => String::from("FastFood"),
            Self::FederalTax => String::from("FederalTax"),
            Self::FeesCharges => String::from("FeesCharges"),
            Self::FinanceCharge => String::from("FinanceCharge"),
            Self::Financial => String::from("Financial"),
            Self::FinancialAdvisor => String::from("FinancialAdvisor"),
            Self::FoodDining => String::from("FoodDining"),
            Self::Furnishings => String::from("Furnishings"),
            Self::GasFuel => String::from("GasFuel"),
            Self::GiftsDonations => String::from("GiftsDonations"),
            Self::Groceries => String::from("Groceries"),
            Self::Gym => String::from("Gym"),
            Self::Hair => String::from("Hair"),
            Self::HealthFitness => String::from("HealthFitness"),
            Self::HealthInsurance => String::from("HealthInsurance"),
            Self::Hobbies => String::from("Hobbies"),
            Self::Home => String::from("Home"),
            Self::HomeImprovement => String::from("HomeImprovement"),
            Self::HomeInsurance => String::from("HomeInsurance"),
            Self::HomePhone => String::from("HomePhone"),
            Self::HomeServices => String::from("HomeServices"),
            Self::HomeSupplies => String::from("HomeSupplies"),
            Self::Hotel => String::from("Hotel"),
            Self::Income => String::from("Income"),
            Self::InterestIncome => String::from("InterestIncome"),
            Self::Internet => String::from("Internet"),
            Self::Investments => String::from("Investments"),
            Self::Kids => String::from("Kids"),
            Self::KidsActivities => String::from("KidsActivities"),
            Self::LateFee => String::from("LateFee"),
            Self::Laundry => String::from("Laundry"),
            Self::LawnGarden => String::from("LawnGarden"),
            Self::Legal => String::from("Legal"),
            Self::LifeInsurance => String::from("LifeInsurance"),
            Self::LoanInsurance => String::from("LoanInsurance"),
            Self::LoanPayment => String::from("LoanPayment"),
            Self::Loans => String::from("Loans"),
            Self::MobilePhone => String::from("MobilePhone"),
            Self::MortgageRent => String::from("MortgageRent"),
            Self::MoviesDvds => String::from("MoviesDVDs"),
            Self::Music => String::from("Music"),
            Self::NewspapersMagazines => String::from("NewspapersMagazines"),
            Self::OfficeSupplies => String::from("OfficeSupplies"),
            Self::Parking => String::from("Parking"),
            Self::Paycheck => String::from("Paycheck"),
            Self::PersonalCare => String::from("PersonalCare"),
            Self::PetFoodSupplies => String::from("PetFoodSupplies"),
            Self::PetGrooming => String::from("PetGrooming"),
            Self::Pets => String::from("Pets"),
            Self::Pharmacy => String::from("Pharmacy"),
            Self::Printing => String::from("Printing"),
            Self::PropertyTax => String::from("Property Tax"),
            Self::PublicTransportation => String::from("Public Transportation"),
            Self::Reimbursement => String::from("Reimbursement"),
            Self::RentalCarTaxi => String::from("RentalCarTaxi"),
            Self::Restaurants => String::from("Restaurants"),
            Self::SalesTax => String::from("SalesTax"),
            Self::ServiceParts => String::from("ServiceParts"),
            Self::ServiceFee => String::from("ServiceFee"),
            Self::Shipping => String::from("Shipping"),
            Self::Shopping => String::from("Shopping"),
            Self::SpaMassage => String::from("SpaMassage"),
            Self::SportingGoods => String::from("SportingGoods"),
            Self::Sports => String::from("Sports"),
            Self::StateTax => String::from("StateTax"),
            Self::StudentLoan => String::from("Student Loan"),
            Self::Taxes => String::from("Taxes"),
            Self::Television => String::from("Television"),
            Self::Toys => String::from("Toys"),
            Self::Transfer => String::from("Transfer"),
            Self::Travel => String::from("Travel"),
            Self::Tuition => String::from("Tuition"),
            Self::Uncategorized => String::from("Uncategorized"),
            Self::Utilities => String::from("Utilities"),
            Self::Vacation => String::from("Vacation"),
            Self::Veterinary => String::from("Veterinary"),
        }
    }
}

impl Default for AssetTransactionCategoryType {
    fn default() -> AssetTransactionCategoryType {
        Self::AtmFee
    }
}




