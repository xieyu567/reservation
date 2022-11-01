use std::{fs, process::Command};

fn main() {
    tonic_build::configure()
        .out_dir("src/pb")
        .compile(&["protos/reservation.proto"], &["protos"])
        .unwrap();

    Command::new("cargo").args(["fmt"]).output().unwrap();

    fs::remove_file("src/pb/google.protobuf.rs").unwrap();
}
