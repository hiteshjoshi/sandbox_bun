use hakoniwa::{Error, Sandbox, SandboxPolicy, Stdio};

fn main() -> Result<(), Error> {
    let policy = SandboxPolicy::from_str(
        r#"
share_net = true
share_uts = true
mounts = [
  { source = "/bin"        , target = "/bin"         },
  { source = "/lib"        , target = "/lib"         },
  { source = "/lib64"      , target = "/lib64"       },
  { source = "/usr"        , target = "/usr"         },
  { source = "/dev/urandom", target = "/dev/urandom" },
  { source = "/etc/resolv.conf", target = "/etc/resolv.conf" ,rw = true},
  { source = "/home/hitesh/Desktop/jail/src/home"   , target = "/home", rw = true},
]

[env]
HOME = "/home"
    "#,
    )?;

    let mut sandbox = Sandbox::new();
    sandbox.with_policy(policy);

    let prog = "/usr/bin/bun";
    let argv = vec![prog,"run","-i","--prefer-offline","index.ts"];
    let mut executor = sandbox.command(&prog, &argv);
    let result = executor // 2 seconds
        .current_dir("/home")? 
        .limit_cpu(Some(1)) 
        .limit_walltime(Some(20)) // --limit-walltime 5
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .run();

    dbg!(result.stdout);
    Ok(())
}