use crate::shell_command;

pub fn uninstall_m() {
    shell_command("rm", vec!["-rf", "$HOME/.oxido/oxido"]);
}
