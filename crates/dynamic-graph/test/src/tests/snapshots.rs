use glob::glob;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

use serde_json::Value;
use similar::ChangeTag;

use colored::Colorize;

use reactive_graph_runtime_impl::RuntimeBuilder;
use reactive_graph_utils_test::error_verb;
use reactive_graph_utils_test::status_failed;
use reactive_graph_utils_test::status_ok;
use reactive_graph_utils_test::verb;

#[ignore]
#[tokio::test(flavor = "multi_thread")]
async fn test_dynamic_graph_snapshots() {
    reactive_graph_utils_test::init_logger();
    // TODO: compile runtime with dynamic graph, but without plugin system, graphql, ...!
    let runtime = RuntimeBuilder::new()
        .ignore_config_files()
        .disable_all_plugins(true)
        .instance_name("dynamic_graph_snapshots")
        .instance_description("Test the dynamic graph snapshots")
        .hostname("example.com")
        .port(12345)
        .secure(false)
        .init()
        .await
        .post_init()
        .await
        .get();
    let query_service = runtime.get_dynamic_graph_query_service();

    let mut failed_tests = 0;

    let test_dir = PathBuf::from("dynamic_graph/snapshots");
    let test_dir = fs::canonicalize(&test_dir).expect(&format!("Failed to canonicalize {test_dir:?}"));
    let pattern = format!("{}{}**/*.json", test_dir.to_str().expect(""), std::path::MAIN_SEPARATOR);
    for snapshot_json_file in glob(&pattern).expect(&format!("Cannot enter directory {test_dir:?}")) {
        let snapshot_json_file = snapshot_json_file.expect("Missing snapshot_json_file");
        let short_name = snapshot_json_file
            .file_stem()
            .map(OsStr::to_string_lossy)
            .map(|stem| stem.to_string())
            .expect("Failed to get short name of the dynamic graph test case");
        if short_name.ends_with("__setup") || short_name.ends_with("__verify") {
            continue;
        }
        let mut graphql_query_file = snapshot_json_file.clone();
        graphql_query_file.set_extension("graphql");
        let mut graphql_setup_mutation_file = graphql_query_file.clone();
        // Setup
        if let Some(setup_mutation_file_name) = graphql_setup_mutation_file
            .file_stem()
            .map(|stem| format!("{}__setup.graphql", stem.to_string_lossy()))
        {
            graphql_setup_mutation_file.set_file_name(setup_mutation_file_name);
            match fs::read_to_string(&graphql_setup_mutation_file) {
                Ok(graphql_setup_mutation_str) => {
                    println!("{} {short_name:<30}", verb("Setup"));
                    let _ = query_service.query_response(&graphql_setup_mutation_str).await;
                }
                Err(_) => {
                    // println!("[ {short_name:<30} ] [{}] No setup mutation found");
                }
            }
        }
        let snapshot_json_str = fs::read_to_string(&snapshot_json_file).expect("Failed to read snapshot_json_file");
        let expected = serde_json::from_str::<Value>(&snapshot_json_str).expect("Failed parse snapshot json value");
        let snapshot_pretty = serde_json::to_string_pretty(&expected).expect("Failed to convert snapshot_json to pretty json string");
        let graphql_query_str = fs::read_to_string(&graphql_query_file).expect("Failed to read graphql_query_file");
        println!("{} {short_name:<30}", verb("Executing"));
        // println!("{graphql_query_str}");
        let response = query_service
            .query_response(&graphql_query_str)
            .await
            .expect("Failed to generate the dynamic schema");
        let response_pretty = serde_json::to_string_pretty(&response.data).expect("Failed to convert response to pretty json string");
        // println!("[ {short_name:<30} ] [{}] Checking json result");
        // println!("{}", serde_json::to_string_pretty(&response.data).expect("Failed to convert to string"));
        if !response.errors.is_empty() {
            eprintln!(
                "{} {short_name:<30} {} {}",
                error_verb("Failed"),
                status_failed(),
                response.errors.iter().fold(String::new(), |e1, e2| format!("{e1}\n{e2:?}"))
            );
        }
        let actual = response.data.into_json().expect("Failed parse graphql response data");
        if expected != actual {
            failed_tests = failed_tests + 1;
            let diff = similar::TextDiff::from_lines(&snapshot_pretty, &response_pretty);
            for change in diff.iter_all_changes() {
                let sign = match change.tag() {
                    ChangeTag::Delete => "-".red(),
                    ChangeTag::Insert => "+".green(),
                    ChangeTag::Equal => " ".white(),
                };
                eprint!("{} {short_name:<30} {} {}", error_verb("Failed"), sign, change);
            }
        } else {
            println!("{} {short_name:<30} {}", verb("Succeeded"), status_ok());
        }

        // Verification Query
        let mut graphql_verification_query_file = graphql_query_file.clone();
        if let Some(verification_query_file_name) = graphql_verification_query_file
            .file_stem()
            .map(|stem| format!("{}__verify.graphql", stem.to_string_lossy()))
        {
            graphql_verification_query_file.set_file_name(verification_query_file_name);
            let mut snapshot_verification_json_file = snapshot_json_file.clone();
            if let Some(snapshot_verification_file_name) = snapshot_verification_json_file
                .file_stem()
                .map(|stem| format!("{}__verify.json", stem.to_string_lossy()))
            {
                snapshot_verification_json_file.set_file_name(snapshot_verification_file_name);

                match fs::read_to_string(&graphql_verification_query_file) {
                    Ok(graphql_verification_query_str) => match fs::read_to_string(&snapshot_verification_json_file) {
                        Ok(snapshot_verification_json_str) => {
                            let expected_verification =
                                serde_json::from_str::<Value>(&snapshot_verification_json_str).expect("Failed parse snapshot verification json value");
                            let snapshot_verification_pretty = serde_json::to_string_pretty(&expected_verification)
                                .expect("Failed to convert snapshot_verification_json to pretty json string");

                            println!("{} {short_name:<30}", verb("Verifying"));
                            let response_verification = query_service
                                .query_response(&graphql_verification_query_str)
                                .await
                                .expect("Failed to generate the dynamic schema");
                            let response_verification_pretty = serde_json::to_string_pretty(&response_verification.data)
                                .expect("Failed to convert verification response to pretty json string");

                            let actual_verification = response_verification.data.into_json().expect("Failed parse graphql response data");
                            if expected_verification != actual_verification {
                                failed_tests = failed_tests + 1;
                                let verification_diff = similar::TextDiff::from_lines(&snapshot_verification_pretty, &response_verification_pretty);
                                for change in verification_diff.iter_all_changes() {
                                    let sign = match change.tag() {
                                        ChangeTag::Delete => "-".red(),
                                        ChangeTag::Insert => "+".green(),
                                        ChangeTag::Equal => " ".white(),
                                    };
                                    // eprint!("{} {short_name:<30} | {} | {}", verb("Diff"), sign, change);
                                    eprint!("{} {short_name:<30} {} {}", error_verb("Verification"), sign, change);
                                }
                            } else {
                                println!("{} {short_name:<30} {}", verb("Verified"), status_ok());
                            }
                        }
                        Err(_) => {}
                    },
                    Err(_) => {
                        // println!("[ {short_name:<30} ] [{}] No verification query found");
                    }
                }
            }
        }

        // Cleanup Mutation
        let mut graphql_cleanup_mutation_file = graphql_query_file.clone();
        if let Some(cleanup_mutation_file_name) = graphql_cleanup_mutation_file
            .file_stem()
            .map(|stem| format!("{}__cleanup.graphql", stem.to_string_lossy()))
        {
            graphql_cleanup_mutation_file.set_file_name(cleanup_mutation_file_name);
            match fs::read_to_string(&graphql_cleanup_mutation_file) {
                Ok(graphql_cleanup_mutation_str) => {
                    println!("{} {short_name:<30}", verb("Cleanup"));
                    let _ = query_service.query_response(&graphql_cleanup_mutation_str).await;
                }
                Err(_) => {
                    // println!("[ {short_name:<30} ] [{}] No cleanup mutation found");
                }
            }
        }
    }
    runtime.pre_shutdown().await;
    runtime.shutdown().await;
    assert_eq!(failed_tests, 0);
}
