use chrono::{Datelike, Duration, Local};
use std::io::{self, Write};

fn main() {
    // 現在の日付を取得
    let today = Local::now();

    // 曜日の日本語表記のリスト
    let day_names = ["月", "火", "水", "木", "金"];

    // 今日が金曜日以降かどうかを判定（num_days_from_monday()が4以上なら金曜日から日曜日）
    let is_end_of_week = today.weekday().num_days_from_monday() >= 4;

    // 月曜日の日付を計算
    let monday = if is_end_of_week {
        // 金曜日から日曜日の場合、翌週の月曜日を取得
        let days_until_next_monday = 7 - today.weekday().num_days_from_monday();
        today + Duration::days(days_until_next_monday as i64)
    } else {
        // それ以外の場合、今週の月曜日を取得
        today - Duration::days(today.weekday().num_days_from_monday() as i64)
    };

    let mut schedule = Vec::new();

    // 月曜日から金曜日までループ
    for i in 0..5 {
        let day = monday + Duration::days(i);
        let date_str = day.format("%m/%d").to_string();
        let day_of_week = day_names[i as usize];
        // ユーザーから予定を入力
        print!("{} ({}) の予定を入力してください（空白の場合は出社）: ", date_str, day_of_week);
        io::stdout().flush().unwrap();

        let mut plan = String::new();
        io::stdin().read_line(&mut plan).expect("入力エラー");
        let plan = plan.trim().to_string();

        // 予定をベクターに追加
        schedule.push(format!("{} ({}) {}", date_str, day_of_week, if plan.is_empty() { "出社" } else { &plan }));
    }

    println!("■ 今週の予定");

    // 予定を出力
    for entry in schedule {
        println!("{}", entry);
    }
}
