use chrono::prelude::*;

use clap::Parser;
use firefly_report::firefly::*;
use tracing::Instrument;
use tracing_subscriber::layer::SubscriberExt;

#[derive(Debug, serde::Serialize)]
struct BudgetSummary {
    id: String,
    name: String,
    amount: f64,
    spent: f64,
    currency_symbol: String,
}

#[derive(Debug, serde::Serialize)]
struct TemplateContext {
    bills: Vec<api::Bill>,
    budgets: Vec<BudgetSummary>,
}


#[derive(Debug, clap::Parser)]
struct CliArgs {
    #[clap(long)]
    firefly_addr: String,
    #[clap(long)]
    access_token: String,
    #[clap(long, default_value = "output.html")]
    output_file: std::path::PathBuf,
}

static TEMPLATE: &'static str = include_str!("../templates/html-report.html");

async fn load_bills(session: std::sync::Arc<Session>, next_month: NaiveDate) -> Vec<api::Bill> {
    tracing::info!("Loading Bills...");

    let bills = session.load_bills().await.unwrap();

    let bills_to_consider: Vec<api::Bill> = bills
        .iter()
        .filter(|bill| {
            if !bill.attributes.active {
                return false;
            }

            if !bill
                .attributes
                .pay_dates
                .iter()
                .any(|paydate| paydate.month() == next_month.month())
            {
                return false;
            }

            true
        })
        .cloned()
        .collect();

    tracing::info!("Bills for Month {}", next_month.month());
    for bill in bills_to_consider.iter() {
        tracing::info!("[{}] {}", bill.id, bill.attributes.name);
    }

    bills_to_consider
}

async fn load_budgets(
    session: std::sync::Arc<Session>,
    start: NaiveDate,
    end: NaiveDate,
) -> Vec<BudgetSummary> {
    tracing::info!("Loading Budgets...");

    let budgets = session.load_budgets(Some((start, end))).await.unwrap();

    for budget in budgets.iter() {
        let budget_amount: f64 = budget.attributes.auto_budget_amount.parse().unwrap();
        let spent_budget: f64 = budget
            .attributes
            .spent
            .iter()
            .fold(0.0, |acc, v| acc + v.sum.parse::<f64>().unwrap());

        let free_budget = budget_amount + spent_budget;

        tracing::info!(
            "[{}] {}: Budgeted {} - Spent {} -> {}",
            budget.id,
            budget.attributes.name,
            budget_amount,
            spent_budget,
            free_budget
        );
    }

    budgets
        .into_iter()
        .map(|budget| {
            let budget_amount: f64 = budget.attributes.auto_budget_amount.parse().unwrap();
            let spent_budget: f64 = budget
                .attributes
                .spent
                .iter()
                .fold(0.0, |acc, v| acc + v.sum.parse::<f64>().unwrap());

            BudgetSummary {
                id: budget.id,
                name: budget.attributes.name,
                amount: budget_amount,
                spent: spent_budget,
                currency_symbol: budget
                    .attributes
                    .spent
                    .iter()
                    .map(|s| s.currency.currency_symbol.clone())
                    .last()
                    .unwrap_or("€".to_string()),
            }
        })
        .collect()
}

fn main() {
    let args = CliArgs::parse();

    let registry = tracing_subscriber::registry().with(tracing_subscriber::fmt::layer());
    tracing::subscriber::set_global_default(registry).unwrap();

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    tracing::info!("Starting");

    let session = std::sync::Arc::new(Session::new(
        args.firefly_addr,
        args.access_token,
    ));

    let current_date = Utc::now();
    tracing::info!("Current-Date: {:?}", current_date);

    let next_month = current_date
        .checked_add_months(chrono::Months::new(1))
        .unwrap();
    tracing::info!("Next-Month: {:?}", next_month);

    let month_start =
        chrono::NaiveDate::from_ymd_opt(current_date.year(), current_date.month(), 1).unwrap();
    let month_end = current_date
        .date_naive()
        .iter_days()
        .take_while(|d| d.month0() == current_date.month0())
        .last()
        .unwrap();

    // Bills stuff
    let bills_handle = runtime.spawn({
        let session = session.clone();
        load_bills(session, next_month.date_naive()).instrument(tracing::info_span!("Bills"))
    });

    // Budgets
    let budgets_handle = runtime.spawn({
        let session = session.clone();
        load_budgets(session, month_start, month_end).instrument(tracing::info_span!("Budgets"))
    });

    let (bills, budgets) = runtime
        .block_on(async move { tokio::try_join!(bills_handle, budgets_handle) })
        .unwrap();

    let mut tt = tinytemplate::TinyTemplate::new();
    tt.add_template("report", TEMPLATE).unwrap();

    let context = TemplateContext { bills, budgets };

    let rendered = tt.render("report", &context).unwrap();
    std::fs::write(args.output_file, rendered).unwrap();
}
