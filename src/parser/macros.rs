macro_rules! next_token {
    (in $tokens: expr) => (
        match $tokens.first() {
            Some(token) => Some(token.clone()),
            None => None,
        }
    );
    ($token: pat in $tokens: expr) => (
        match $tokens.first() {
            token @ Some(&$token) => Some(token.unwrap().clone()),
            _ => None,
        }
    )
}

macro_rules! eat_token {
    (in $tokens: expr) => {{
        let token = match $tokens.first() {
            Some(token) => {
                Some(token.clone())
            },
            _ => None,
        };
        if let Some(_) = token {
            $tokens.remove(0);
        }
        token
    }};
    ($token: pat in $tokens: expr) => {{
        let token = match $tokens.first() {
            token @ Some(&$token) => {
                Some(token.unwrap().clone())
            },
            _ => None,
        };
        if let Some(_) = token {
            $tokens.remove(0);
        }
        token
    }};
}

macro_rules! eat_tokens_until {
    ($token: pat in $tokens: expr) => {{
        let mut ret = None;
        loop {
            let found = match $tokens.first() {
                token @ Some(&$token) => {
                    ret = Some(token.unwrap().clone());
                    true
                },
                Some(..) => true,
                None => false,
            };
            if found {
                $tokens.remove(0);
            }
            if let Some(..) = ret {
                break;
            }
        };
        ret
    }}
}

macro_rules! try_parse {
    ($($x: expr),+) => ({{
        let mut f = || {
            $(
                let res = $x;
                if let Ok(ok) = res {
                    return Ok(ok.into());
                }
            )+
            match res {
                Ok(ok) => Ok(ok.into()),
                Err(err) => Err(err),
            }
        };
        f()
    }})
}