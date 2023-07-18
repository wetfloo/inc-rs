use std::{borrow::Cow, env};

fn main() {
    let arg = get_conf_arg()
        .or_else(get_env_arg)
        .unwrap_or_else(|| Cow::Borrowed("/etc/app/app.conf"));

    println!("{}", arg);
}

fn get_conf_arg() -> Option<ArgCow> {
    let mut args = env::args().skip(1);
    let conf_key = args.next()?;
    let conf_value = args.next()?;

    match conf_key.as_str() {
        "--conf" => Some(Cow::Owned(conf_value)),
        _ => None,
    }
}

fn get_env_arg() -> Option<ArgCow> {
    let res = env::var("APP_CONF").ok()?;
    Some(Cow::Owned(res))
}

type ArgCow = Cow<'static, str>;
