use vergen::*;

fn main() {
    let mut conf = Config::default();
    *conf.git_mut().rerun_on_head_change_mut() = true;
    *conf.git_mut().commit_timestamp_kind_mut() = TimestampKind::DateOnly;
    *conf.git_mut().sha_kind_mut() = ShaKind::Short;
    *conf.git_mut().semver_kind_mut() = SemverKind::Lightweight;
    *conf.git_mut().semver_dirty_mut() = Some("-dirty");

    vergen(conf).expect("vergen failed")
}
