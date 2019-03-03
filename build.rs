use bindgen;
use cc;
use std::env;
use std::path::PathBuf;

fn main() {
    let bindings = bindgen::Builder::default()
        .clang_arg("-Iccat")
        .unstable_rust(false)
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");
    cc::Build::new()
        .flag("-std=gnu99")
        .flag("-w")
        .include("ccat")
        .file("ccat/lib/cat_ae.c")
        .file("ccat/lib/cat_anet.c")
        .file("ccat/lib/cat_ccmap.c")
        .file("ccat/lib/cat_clog.c")
        .file("ccat/lib/cat_dict.c")
        .file("ccat/lib/cat_ezxml.c")
        .file("ccat/lib/cat_json.c")
        .file("ccat/lib/cat_mpsc_queue.c")
        .file("ccat/lib/cat_network_util.c")
        .file("ccat/lib/cat_sds.c")
        .file("ccat/lib/cat_stack.c")
        .file("ccat/lib/cat_static_queue.c")
        .file("ccat/lib/cat_time_util.c")
        .file("ccat/ccat/client.c")
        .file("ccat/ccat/client_config.c")
        .file("ccat/ccat/context.c")
        .file("ccat/ccat/encoder.c")
        .file("ccat/ccat/encoder_binary.c")
        .file("ccat/ccat/encoder_text.c")
        .file("ccat/ccat/functions.c")
        .file("ccat/ccat/message.c")
        .file("ccat/ccat/message_aggregator.c")
        .file("ccat/ccat/message_aggregator_event.c")
        .file("ccat/ccat/message_aggregator_metric.c")
        .file("ccat/ccat/message_aggregator_trans.c")
        .file("ccat/ccat/message_helper.c")
        .file("ccat/ccat/message_id.c")
        .file("ccat/ccat/message_manager.c")
        .file("ccat/ccat/message_sender.c")
        .file("ccat/ccat/message_tree.c")
        .file("ccat/ccat/monitor.c")
        .file("ccat/ccat/monitor_collector.c")
        .file("ccat/ccat/router_json_parser.c")
        .file("ccat/ccat/server_connection_manager.c")
        .file("ccat/ccat/transaction.c")
        .compile("ccat");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Could't write bindings");
}
