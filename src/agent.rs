use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

pub async fn run_agent(prompt: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let api_key = crate::get_api_key()?;
    let model = crate::config::get_model();

    let client = Client::new();
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent",
        model
    );

    let user_prompt = prompt.unwrap_or_else(|| {
        "You are a senior headhunter. Provide a fictional remote frontend job from Japan in JSON format. Ensure output contains only JSON.".to_string()
    });

    let response = client
        .post(&url)
        .query(&[("key", &api_key)])
        .header("Content-Type", "application/json")
        .json(&json!({
            "contents": [{
                "parts": [{
                    "text": user_prompt
                }]
            }]
        }))
        .send()
        .await?;

    let status = response.status();
    if !status.is_success() {
        let err_text = response.text().await?;
        eprintln!("API request failed. Status: {}\nDetails: {}", status, err_text);
        return Ok(());
    }

    let response_text = response.text().await?;

    #[derive(Deserialize)]
    struct GeminiResponse {
        candidates: Vec<Candidate>,
    }

    #[derive(Deserialize)]
    struct Candidate {
        content: Content,
    }

    #[derive(Deserialize)]
    struct Content {
        parts: Vec<Part>,
    }

    #[derive(Deserialize)]
    struct Part {
        text: String,
    }

    let parsed: GeminiResponse = serde_json::from_str(&response_text)?;

    fn get_str<'a>(obj: &'a serde_json::Value, paths: &[&str]) -> &'a str {
        for path in paths {
            let parts: Vec<&str> = path.split('/').collect();
            let mut current = obj;
            for part in &parts {
                if current.is_null() {
                    break;
                }
                current = &current[*part];
            }
            if let Some(s) = current.as_str() {
                return s;
            }
        }
        "N/A"
    }

    fn get_i64(obj: &serde_json::Value, paths: &[&str]) -> i64 {
        for path in paths {
            let parts: Vec<&str> = path.split('/').collect();
            let mut current = obj;
            for part in &parts {
                if current.is_null() {
                    break;
                }
                current = &current[*part];
            }
            if let Some(n) = current.as_i64() {
                return n;
            }
        }
        0
    }

    if let Some(candidate) = parsed.candidates.first() {
        if let Some(part) = candidate.content.parts.first() {
            println!("{}", part.text);

            if let Ok(job_json) = serde_json::from_str::<serde_json::Value>(&part.text) {
                let root = job_json.get("job_listing").unwrap_or(&job_json);

                let job_title = get_str(root, &["job_title", "position/title", "position_title", "title"]);
                let company_name = get_str(root, &["company/name", "company_name"]);
                let hq = get_str(root, &["company/location", "company/headquarters", "company/hq"]);

                let salary_min = get_i64(root, &[
                    "salary/min",
                    "salary_range/min",
                    "compensation/salary_range/min",
                    "compensation/annual_salary_range/min",
                    "position/salary_range/min",
                    "position/min_salary",
                    "compensation/min_salary"
                ]);
                let salary_max = get_i64(root, &[
                    "salary/max",
                    "salary_range/max",
                    "compensation/salary_range/max",
                    "compensation/annual_salary_range/max",
                    "position/salary_range/max",
                    "position/max_salary",
                    "compensation/max_salary"
                ]);
                let salary_currency = get_str(root, &[
                    "salary/currency",
                    "salary_range/currency",
                    "compensation/salary_range/currency",
                    "position/salary_range/currency",
                    "position/currency",
                    "compensation/currency"
                ]);

                let posted_date = get_str(root, &["posting_date", "post_date", "posted_date", "position/posted_date"]);

                let mut tech_items: Vec<String> = Vec::new();
                if let Some(tech_val) = root.get("tech_stack") {
                    if let Some(arr) = tech_val.as_array() {
                        for item in arr {
                            if let Some(s) = item.as_str() {
                                tech_items.push(s.to_string());
                            }
                        }
                    } else if let Some(obj) = tech_val.as_object() {
                        for (_, value) in obj.iter() {
                            if let Some(arr) = value.as_array() {
                                for item in arr {
                                    if let Some(s) = item.as_str() {
                                        tech_items.push(s.to_string());
                                    }
                                }
                            }
                        }
                    }
                }

                let today = chrono::Local::now().format("%Y-%m-%d").to_string();
                let safe_company_name = company_name.replace("/", "-").replace(" ", "_");
                let filename = format!("{}-{}.md", today, safe_company_name);

                let jobs_dir = std::path::Path::new("jobs");
                std::fs::create_dir_all(jobs_dir)?;

                let is_high_salary = salary_max >= 15000000;
                let salary_emoji = if is_high_salary { " 🔥" } else { "" };

                let mut markdown = format!(
                    "# {}\n\n\
                    ## Company\n{} ({})\n\n\
                    ## Salary\n{} {}-{} {} / year{}\n\n\
                    ## Tech Stack\n",
                    job_title,
                    company_name,
                    hq,
                    salary_emoji,
                    salary_min,
                    salary_max,
                    salary_currency,
                    salary_emoji
                );

                for tech in &tech_items {
                    markdown.push_str(&format!("- {}\n", tech));
                }

                markdown.push_str(&format!("\n## Info\n- Posted: {}\n", posted_date));

                markdown.push_str("\n## Original JSON\n\n```json\n");
                markdown.push_str(&part.text);
                markdown.push_str("\n```\n");

                let file_path = jobs_dir.join(&filename);
                std::fs::write(&file_path, &markdown)?;

                println!("\n[Saved to jobs/{}]", filename);
            }
        }
    }

    Ok(())
}
