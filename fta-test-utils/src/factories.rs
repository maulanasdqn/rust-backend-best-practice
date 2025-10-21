use chrono::{DateTime, Utc};
use fake::Fake;
use uuid::Uuid;

pub struct UserFactory;

impl UserFactory {
    pub fn email() -> String {
        use fake::faker::internet::en::SafeEmail;
        SafeEmail().fake()
    }

    pub fn password() -> String {
        use fake::faker::internet::en::Password;
        Password(8..20).fake()
    }

    pub fn first_name() -> String {
        use fake::faker::name::en::FirstName;
        FirstName().fake()
    }

    pub fn last_name() -> String {
        use fake::faker::name::en::LastName;
        LastName().fake()
    }

    pub fn phone_number() -> String {
        use fake::faker::phone_number::en::PhoneNumber;
        PhoneNumber().fake()
    }
}

pub struct AccountFactory;

impl AccountFactory {
    pub fn name() -> String {
        use fake::faker::company::en::CompanyName;
        CompanyName().fake()
    }

    pub fn account_type() -> String {
        use rand::seq::SliceRandom;
        let types = ["checking", "savings", "credit", "investment"];
        types.choose(&mut rand::thread_rng()).unwrap().to_string()
    }

    pub fn balance() -> i64 {
        use rand::Rng;
        rand::thread_rng().gen_range(0..1_000_000)
    }

    pub fn currency() -> String {
        use rand::seq::SliceRandom;
        let currencies = ["USD", "EUR", "GBP", "JPY"];
        currencies
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string()
    }
}

pub struct TransactionFactory;

impl TransactionFactory {
    pub fn description() -> String {
        use fake::faker::lorem::en::Sentence;
        Sentence(3..8).fake()
    }

    pub fn transaction_type() -> String {
        use rand::seq::SliceRandom;
        let types = ["income", "expense", "transfer"];
        types.choose(&mut rand::thread_rng()).unwrap().to_string()
    }

    pub fn amount() -> i64 {
        use rand::Rng;
        rand::thread_rng().gen_range(100..100_000)
    }

    pub fn category() -> String {
        use rand::seq::SliceRandom;
        let categories = [
            "food",
            "transport",
            "entertainment",
            "utilities",
            "shopping",
            "healthcare",
            "education",
            "salary",
            "investment",
        ];
        categories
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string()
    }
}

pub struct BudgetFactory;

impl BudgetFactory {
    pub fn name() -> String {
        use fake::faker::lorem::en::Word;
        format!("{} Budget", Word().fake::<String>())
    }

    pub fn amount() -> i64 {
        use rand::Rng;
        rand::thread_rng().gen_range(10_000..500_000)
    }

    pub fn period() -> String {
        use rand::seq::SliceRandom;
        let periods = ["monthly", "quarterly", "yearly"];
        periods.choose(&mut rand::thread_rng()).unwrap().to_string()
    }

    pub fn category() -> String {
        TransactionFactory::category()
    }
}

pub struct TestData;

impl TestData {
    pub fn uuid() -> Uuid {
        Uuid::new_v4()
    }

    pub fn now() -> DateTime<Utc> {
        Utc::now()
    }

    pub fn future_date() -> DateTime<Utc> {
        use chrono::Duration;
        Utc::now() + Duration::days(30)
    }

    pub fn past_date() -> DateTime<Utc> {
        use chrono::Duration;
        Utc::now() - Duration::days(30)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_factory() {
        let email = UserFactory::email();
        assert!(email.contains('@'));

        let password = UserFactory::password();
        assert!(password.len() >= 8);

        let first_name = UserFactory::first_name();
        assert!(!first_name.is_empty());

        let last_name = UserFactory::last_name();
        assert!(!last_name.is_empty());
    }

    #[test]
    fn test_account_factory() {
        let name = AccountFactory::name();
        assert!(!name.is_empty());

        let account_type = AccountFactory::account_type();
        assert!(["checking", "savings", "credit", "investment"].contains(&account_type.as_str()));

        let balance = AccountFactory::balance();
        assert!(balance >= 0);

        let currency = AccountFactory::currency();
        assert!(["USD", "EUR", "GBP", "JPY"].contains(&currency.as_str()));
    }

    #[test]
    fn test_transaction_factory() {
        let description = TransactionFactory::description();
        assert!(!description.is_empty());

        let transaction_type = TransactionFactory::transaction_type();
        assert!(["income", "expense", "transfer"].contains(&transaction_type.as_str()));

        let amount = TransactionFactory::amount();
        assert!(amount > 0);

        let category = TransactionFactory::category();
        assert!(!category.is_empty());
    }

    #[test]
    fn test_budget_factory() {
        let name = BudgetFactory::name();
        assert!(name.contains("Budget"));

        let amount = BudgetFactory::amount();
        assert!(amount > 0);

        let period = BudgetFactory::period();
        assert!(["monthly", "quarterly", "yearly"].contains(&period.as_str()));
    }
}
