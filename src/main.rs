//! Kelly Criterion Calculator
//! f* = (bp - q) / b
//! where b = odds - 1, p = win rate, q = 1-p

use std::io::{self, Write};

/// Language setting
#[derive(Clone, Copy, PartialEq)]
enum Language {
    English,
    Chinese,
}

/// Kelly calculation result
struct KellyResult {
    /// Optimal fraction (0-1)
    optimal_fraction: f64,
    /// Positive expected value
    positive_ev: bool,
    /// Expected value
    expected_value: f64,
}

/// Calculate Kelly Criterion
///
/// # Arguments
/// * `odds` - Decimal odds (e.g., 2.0 means even money, net odds = 1)
/// * `win_rate` - Probability of winning (0-1)
fn kelly_criterion(odds: f64, win_rate: f64) -> KellyResult {
    let b = odds - 1.0;
    let p = win_rate;
    let q = 1.0 - p;

    let optimal_fraction = (b * p - q) / b;
    let expected_value = p * b - q;

    KellyResult {
        optimal_fraction,
        positive_ev: expected_value > 0.0,
        expected_value,
    }
}

/// Polymarket Kelly calculation
///
/// # Arguments
/// * `market_price` - Market price (0-1), e.g., 0.60 for 60c
/// * `your_probability` - Your estimated true probability (0-1)
fn kelly_polymarket(market_price: f64, your_probability: f64) -> KellyResult {
    let p_market = market_price;
    let p_your = your_probability;

    let b = (1.0 - p_market) / p_market;
    let q = 1.0 - p_your;

    let optimal_fraction = (b * p_your - q) / b;
    let expected_value = p_your * b - q;

    KellyResult {
        optimal_fraction,
        positive_ev: expected_value > 0.0,
        expected_value,
    }
}

/// Format as percentage
fn format_pct(value: f64) -> String {
    format!("{:.2}%", value * 100.0)
}

/// Print separator
fn separator() {
    println!("{}", "─".repeat(50));
}

/// Print title
fn print_title(lang: Language) {
    separator();
    match lang {
        Language::English => {
            println!("                    Kelly Calculator");
            println!("              Kelly Criterion Calculator");
        }
        Language::Chinese => {
            println!("                    凯利公式计算器");
            println!("              Kelly Criterion Calculator");
        }
    }
    separator();
    println!();
}

/// Print Polymarket title
fn print_title_polymarket(lang: Language) {
    separator();
    match lang {
        Language::English => {
            println!("                Polymarket Kelly Calculator");
            println!("            Kelly Criterion for Polymarket");
        }
        Language::Chinese => {
            println!("                Polymarket 凯利计算器");
            println!("            Kelly Criterion for Polymarket");
        }
    }
    separator();
    println!();
}

/// Print result (standard mode)
fn print_result(odds: f64, win_rate: f64, result: &KellyResult, capital: Option<f64>, lang: Language) {
    println!();
    separator();
    match lang {
        Language::English => println!("                        Kelly Result"),
        Language::Chinese => println!("                        计算结果"),
    }
    separator();
    println!();

    match lang {
        Language::English => {
            println!("  Input:");
            println!("    ├─ Odds: {:.2}", odds);
            println!("    ├─ Net odds (b): {:.2}", odds - 1.0);
            println!("    └─ Win rate (p): {}", format_pct(win_rate));
            println!();
            println!("  Analysis:");
            println!("    ├─ Expected Value (EV): {:.2}%", result.expected_value * 100.0);

            if result.positive_ev {
                println!("    ├─ Status: ✓ Positive EV (Bet recommended)");
            } else {
                println!("    ├─ Status: ✗ Negative EV (Not recommended)");
            }

            if result.optimal_fraction <= 0.0 {
                println!("    └─ Position size: 0% (No bet)");
            } else if result.optimal_fraction > 1.0 {
                println!("    └─ Position size: 100%+ (Full Kelly or more - High risk!)");
            } else {
                println!("    └─ Position size: {}", format_pct(result.optimal_fraction));
            }
            println!();

            if let Some(cap) = capital {
                println!("  Position based on capital {:.2}:", cap);
                if result.optimal_fraction > 0.0 {
                    println!("    ├─ Full Kelly: {:.2}", cap * result.optimal_fraction);
                    println!("    ├─ Half Kelly: {:.2}", cap * result.optimal_fraction * 0.5);
                    println!("    └─ Quarter Kelly: {:.2}", cap * result.optimal_fraction * 0.25);
                } else {
                    println!("    └─ Recommendation: No bet");
                }
                println!();
            }
        }
        Language::Chinese => {
            println!("  输入参数:");
            println!("    ├─ 赔率: {:.2}", odds);
            println!("    ├─ 净赔率 (b): {:.2}", odds - 1.0);
            println!("    └─ 胜率 (p): {}", format_pct(win_rate));
            println!();
            println!("  分析:");
            println!("    ├─ 期望收益 (EV): {:.2}%", result.expected_value * 100.0);

            if result.positive_ev {
                println!("    ├─ 状态: ✓ 正期望值 (值得下注)");
            } else {
                println!("    ├─ 状态: ✗ 负期望值 (不建议下注)");
            }

            if result.optimal_fraction <= 0.0 {
                println!("    └─ 仓位建议: 0% (不下注)");
            } else if result.optimal_fraction > 1.0 {
                println!("    └─ 仓位建议: 100%+ (全仓甚至加杠杆，高风险！)");
            } else {
                println!("    └─ 仓位建议: {}", format_pct(result.optimal_fraction));
            }
            println!();

            if let Some(cap) = capital {
                println!("  基于本金 {:.2} 的投注金额:", cap);
                if result.optimal_fraction > 0.0 {
                    println!("    ├─ 全凯利: {:.2}", cap * result.optimal_fraction);
                    println!("    ├─ 半凯利: {:.2}", cap * result.optimal_fraction * 0.5);
                    println!("    └─ 1/4凯利: {:.2}", cap * result.optimal_fraction * 0.25);
                } else {
                    println!("    └─ 建议: 不下注");
                }
                println!();
            }
        }
    }

    separator();
}

/// Print Polymarket result
fn print_result_polymarket(market_price: f64, your_probability: f64, result: &KellyResult, capital: Option<f64>, lang: Language) {
    println!();
    separator();
    match lang {
        Language::English => println!("                    Polymarket Result"),
        Language::Chinese => println!("                    Polymarket 计算结果"),
    }
    separator();
    println!();

    match lang {
        Language::English => {
            println!("  Input:");
            println!("    ├─ Market price: {} (implied probability)", format_pct(market_price));
            println!("    ├─ Your probability: {} (your estimate)", format_pct(your_probability));
            println!("    └─ Implied odds: {:.2}", 1.0 / market_price);
            println!();
            println!("  Analysis:");
            println!("    ├─ Expected Value (EV): {:.2}%", result.expected_value * 100.0);

            if result.positive_ev {
                println!("    ├─ Status: ✓ Positive EV (Bet recommended)");
            } else {
                println!("    ├─ Status: ✗ Negative EV (Not recommended)");
            }

            if result.optimal_fraction <= 0.0 {
                println!("    └─ Position size: 0% (No bet)");
            } else if result.optimal_fraction > 1.0 {
                println!("    └─ Position size: 100%+ (Full Kelly or more - High risk!)");
            } else {
                println!("    └─ Position size: {}", format_pct(result.optimal_fraction));
            }
            println!();

            if let Some(cap) = capital {
                println!("  Position based on capital {:.2}:", cap);
                if result.optimal_fraction > 0.0 {
                    println!("    ├─ Full Kelly: {:.2}", cap * result.optimal_fraction);
                    println!("    ├─ Half Kelly: {:.2}", cap * result.optimal_fraction * 0.5);
                    println!("    └─ Quarter Kelly: {:.2}", cap * result.optimal_fraction * 0.25);
                } else {
                    println!("    └─ Recommendation: No bet");
                }
                println!();
            }
        }
        Language::Chinese => {
            println!("  输入参数:");
            println!("    ├─ 市场价格: {} (市场隐含概率)", format_pct(market_price));
            println!("    ├─ 你的概率: {} (你估计的真实概率)", format_pct(your_probability));
            println!("    └─ 隐含赔率: {:.2}", 1.0 / market_price);
            println!();
            println!("  分析:");
            println!("    ├─ 期望收益 (EV): {:.2}%", result.expected_value * 100.0);

            if result.positive_ev {
                println!("    ├─ 状态: ✓ 正期望值 (值得下注)");
            } else {
                println!("    ├─ 状态: ✗ 负期望值 (不建议下注)");
            }

            if result.optimal_fraction <= 0.0 {
                println!("    └─ 仓位建议: 0% (不下注)");
            } else if result.optimal_fraction > 1.0 {
                println!("    └─ 仓位建议: 100%+ (全仓甚至加杠杆，高风险！)");
            } else {
                println!("    └─ 仓位建议: {}", format_pct(result.optimal_fraction));
            }
            println!();

            if let Some(cap) = capital {
                println!("  基于本金 {:.2} 的投注金额:", cap);
                if result.optimal_fraction > 0.0 {
                    println!("    ├─ 全凯利: {:.2}", cap * result.optimal_fraction);
                    println!("    ├─ 半凯利: {:.2}", cap * result.optimal_fraction * 0.5);
                    println!("    └─ 1/4凯利: {:.2}", cap * result.optimal_fraction * 0.25);
                } else {
                    println!("    └─ 建议: 不下注");
                }
                println!();
            }
        }
    }

    separator();
}

/// Interactive mode
fn interactive(lang: Language) {
    print_title(lang);

    loop {
        let (prompt_odds, prompt_win_rate, prompt_capital, msg_quit, msg_odds_error, msg_win_error, msg_cap_error) = match lang {
            Language::English => (
                "Enter odds (e.g., 2.0 for 1:1, 'q' to quit):",
                "Enter win rate (0-100, e.g., 60 for 60%):",
                "Enter capital (optional, press Enter to skip):",
                "Goodbye!",
                "Odds must be greater than 1.0",
                "Win rate must be between 0-100",
                "Capital must be positive, skipped"
            ),
            Language::Chinese => (
                "请输入赔率 (如 2.0 表示 1赔1，输入 q 退出):",
                "请输入胜率 (0-100，如 60 表示 60%):",
                "请输入本金 (可选，直接回车跳过):",
                "再见！",
                "赔率必须大于 1.0",
                "胜率必须在 0-100 之间",
                "本金必须为正数，已跳过"
            )
        };

        print!("{} ", prompt_odds);
        io::stdout().flush().unwrap();

        let mut odds_input = String::new();
        io::stdin().read_line(&mut odds_input).unwrap();

        if odds_input.trim().to_lowercase() == "q" {
            println!("{}", msg_quit);
            break;
        }

        let odds: f64 = match odds_input.trim().parse() {
            Ok(n) if n > 1.0 => n,
            Ok(_) => {
                println!("✗ {}\n", msg_odds_error);
                continue;
            }
            Err(_) => {
                println!("✗ Invalid input\n");
                continue;
            }
        };

        println!("{} ", prompt_win_rate);
        io::stdout().flush().unwrap();

        let mut win_rate_input = String::new();
        io::stdin().read_line(&mut win_rate_input).unwrap();

        let win_rate_percent: f64 = match win_rate_input.trim().parse() {
            Ok(n) if n >= 0.0 && n <= 100.0 => n,
            Ok(_) => {
                println!("✗ {}\n", msg_win_error);
                continue;
            }
            Err(_) => {
                println!("✗ Invalid input\n");
                continue;
            }
        };

        let win_rate = win_rate_percent / 100.0;

        println!("{} ", prompt_capital);
        io::stdout().flush().unwrap();

        let mut capital_input = String::new();
        io::stdin().read_line(&mut capital_input).unwrap();

        let capital: Option<f64> = if capital_input.trim().is_empty() {
            None
        } else {
            match capital_input.trim().parse() {
                Ok(n) if n > 0.0 => Some(n),
                _ => {
                    println!("✗ {}\n", msg_cap_error);
                    None
                }
            }
        };

        let result = kelly_criterion(odds, win_rate);
        print_result(odds, win_rate, &result, capital, lang);
        println!();
    }
}

/// Polymarket interactive mode
fn interactive_polymarket(lang: Language) {
    print_title_polymarket(lang);

    loop {
        let (prompt_price, prompt_prob, prompt_capital, msg_quit, msg_price_error, msg_prob_error, msg_cap_error) = match lang {
            Language::English => (
                "Enter Polymarket market price (0-100, e.g., 60 for 60c, 'q' to quit):",
                "Enter your estimated probability (0-100):",
                "Enter capital (optional, press Enter to skip):",
                "Goodbye!",
                "Price must be between 0-100",
                "Probability must be between 0-100",
                "Capital must be positive, skipped"
            ),
            Language::Chinese => (
                "请输入 Polymarket 市场价格 (0-100，如 60 表示 60c，输入 q 退出):",
                "请输入你估计的真实概率 (0-100):",
                "请输入本金 (可选，直接回车跳过):",
                "再见！",
                "价格必须在 0-100 之间",
                "概率必须在 0-100 之间",
                "本金必须为正数，已跳过"
            )
        };

        print!("{} ", prompt_price);
        io::stdout().flush().unwrap();

        let mut price_input = String::new();
        io::stdin().read_line(&mut price_input).unwrap();

        if price_input.trim().to_lowercase() == "q" {
            println!("{}", msg_quit);
            break;
        }

        let market_price: f64 = match price_input.trim().parse::<f64>() {
            Ok(n) if n > 0.0 && n <= 100.0 => n / 100.0,
            Ok(_) => {
                println!("✗ {}\n", msg_price_error);
                continue;
            }
            Err(_) => {
                println!("✗ Invalid input\n");
                continue;
            }
        };

        println!("{} ", prompt_prob);
        io::stdout().flush().unwrap();

        let mut prob_input = String::new();
        io::stdin().read_line(&mut prob_input).unwrap();

        let your_probability: f64 = match prob_input.trim().parse::<f64>() {
            Ok(n) if n >= 0.0 && n <= 100.0 => n / 100.0,
            Ok(_) => {
                println!("✗ {}\n", msg_prob_error);
                continue;
            }
            Err(_) => {
                println!("✗ Invalid input\n");
                continue;
            }
        };

        println!("{} ", prompt_capital);
        io::stdout().flush().unwrap();

        let mut capital_input = String::new();
        io::stdin().read_line(&mut capital_input).unwrap();

        let capital: Option<f64> = if capital_input.trim().is_empty() {
            None
        } else {
            match capital_input.trim().parse() {
                Ok(n) if n > 0.0 => Some(n),
                _ => {
                    println!("✗ {}\n", msg_cap_error);
                    None
                }
            }
        };

        let result = kelly_polymarket(market_price, your_probability);
        print_result_polymarket(market_price, your_probability, &result, capital, lang);
        println!();
    }
}

/// CLI mode
fn cli_mode(odds: f64, win_rate: f64, capital: Option<f64>, lang: Language) {
    let result = kelly_criterion(odds, win_rate);
    print_result(odds, win_rate, &result, capital, lang);
}

/// Polymarket CLI mode
fn cli_mode_polymarket(market_price: f64, your_probability: f64, capital: Option<f64>, lang: Language) {
    let result = kelly_polymarket(market_price, your_probability);
    print_result_polymarket(market_price, your_probability, &result, capital, lang);
}

/// Print usage
fn print_usage(lang: Language) {
    match lang {
        Language::English => {
            println!("Usage:");
            println!("  kelly                           # Interactive mode");
            println!("  kelly <odds> <win_rate>          # CLI mode");
            println!("  kelly <odds> <win_rate> <capital> # With capital");
            println!();
            println!("  kelly -p                         # Polymarket interactive");
            println!("  kelly -p <price> <prob>          # Polymarket CLI");
            println!("  kelly -p <price> <prob> <capital>");
            println!();
            println!("  -z, --zh                        # Chinese output");
            println!();
            println!("Examples:");
            println!("  kelly 2.0 60                    # Odds 2.0, 60% win rate");
            println!("  kelly 2.0 60 10000              # With 10000 capital");
            println!();
            println!("  kelly -p 60 75                  # Market 60c, you think 75%");
            println!("  kelly -p 60 75 1000             # With 1000 capital");
            println!();
            println!("  kelly -z 2.0 60                 # Chinese output");
        }
        Language::Chinese => {
            println!("用法:");
            println!("  kelly                           # 交互式模式");
            println!("  kelly <赔率> <胜率>              # 命令行模式");
            println!("  kelly <赔率> <胜率> <本金>        # 指定本金");
            println!();
            println!("  kelly -p                         # Polymarket 交互式");
            println!("  kelly -p <价格> <概率>           # Polymarket 命令行");
            println!("  kelly -p <价格> <概率> <本金>");
            println!();
            println!("  -z, --zh                        # 中文输出");
            println!();
            println!("示例:");
            println!("  kelly 2.0 60                    # 赔率2.0，胜率60%");
            println!("  kelly 2.0 60 10000              # 本金10000");
            println!();
            println!("  kelly -p 60 75                  # 市场价格60c，你认为75%");
            println!("  kelly -p 60 75 1000             # 本金1000");
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Parse language flag
    let mut lang = Language::English;
    let has_zh_flag = args.iter().any(|a| a == "-z" || a == "--zh");
    if has_zh_flag {
        lang = Language::Chinese;
    }

    // Check for Polymarket mode
    let is_polymarket = args.iter().any(|a| a == "-p");

    if is_polymarket {
        let pm_args: Vec<&String> = args.iter().filter(|&a| {
            a != "-p" && a != "-z" && a != "--zh"
        }).collect();

        match pm_args.len() {
            0 => interactive_polymarket(lang),
            2 => {
                let market_price: f64 = pm_args[0].parse::<f64>().expect("Market price must be a number") / 100.0;
                let your_prob: f64 = pm_args[1].parse::<f64>().expect("Your probability must be a number") / 100.0;
                cli_mode_polymarket(market_price, your_prob, None, lang);
            }
            3 => {
                let market_price: f64 = pm_args[0].parse::<f64>().expect("Market price must be a number") / 100.0;
                let your_prob: f64 = pm_args[1].parse::<f64>().expect("Your probability must be a number") / 100.0;
                let capital: f64 = pm_args[2].parse().expect("Capital must be a number");
                cli_mode_polymarket(market_price, your_prob, Some(capital), lang);
            }
            _ => {
                println!("✗ Invalid Polymarket mode arguments");
                println!();
                println!("Usage: kelly -p <market_price> <your_probability> [capital]");
                println!("Example: kelly -p 60 75    # Market 60c, you think 75%");
            }
        }
    } else {
        // Filter out language flag
        let args: Vec<&String> = args.iter().filter(|&a| {
            a != "-z" && a != "--zh"
        }).collect();

        match args.len() {
            1 => interactive(lang),
            2 => {
                if args[1] == "-h" || args[1] == "--help" {
                    print_usage(lang);
                } else {
                    println!("✗ Insufficient arguments");
                    print_usage(lang);
                }
            }
            3 => {
                let odds: f64 = args[1].parse().expect("Odds must be a number");
                let win_rate: f64 = args[2].parse::<f64>().expect("Win rate must be a number") / 100.0;
                cli_mode(odds, win_rate, None, lang);
            }
            4 => {
                let odds: f64 = args[1].parse().expect("Odds must be a number");
                let win_rate: f64 = args[2].parse::<f64>().expect("Win rate must be a number") / 100.0;
                let capital: f64 = args[3].parse().expect("Capital must be a number");
                cli_mode(odds, win_rate, Some(capital), lang);
            }
            _ => {
                println!("✗ Too many arguments");
                print_usage(lang);
            }
        }
    }
}
