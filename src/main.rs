// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{error::Error};

slint::include_modules!();

const TAXPER: f64 = 0.30;
const OWNERPER: f64 = 0.55;
const PROFITPER: f64 = 0.05;
const OPEXER: f64 = 0.10;


fn main() -> Result<(), Box<dyn Error>> {
    let window_ui = AppWindow::new()?;

    let window_ui_handle = window_ui.as_weak();

    window_ui.on_divide_income(move |string| {
        let ui = window_ui_handle.unwrap();
        let num: f64 = string.trim().parse().unwrap();
        let tax: f64 = num * TAXPER;
        let owner: f64 = num * OWNERPER;
        let profit: f64 = num * PROFITPER;
        let opex: f64 = num * OPEXER;
        let result = format!("Tax: {:.2}\nOwner: {:.2}\nProfit: {:.2}\nOpEx: {:.2}", tax, owner, profit, opex);
        ui.set_results(result.into());
    });

    window_ui.run()?;

    Ok(())
}
