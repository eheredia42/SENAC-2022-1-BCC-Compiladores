// programa  ::= asterisco+
// asterisco ::= "*"

pub struct Lexer {
    pub cursor: usize,
    pub codigo: Vec<char>,
}

pub fn programa(fonte: &str) -> bool {
    let mut lexer = Lexer {
        cursor: 0,
        codigo: fonte.chars().collect::<Vec<_>>(),
    };

    loop {
        if !asterisco(&mut lexer) {
            break;
        }
    }

    lexer.cursor > 0 && lexer.cursor == lexer.codigo.len()
}

pub fn asterisco(lexer: &mut Lexer) -> bool {
    match lexer.codigo.get(lexer.cursor) {
        Some(c) => {
            if c == &'*' {
                lexer.cursor += 1;
                true
            } else {
                false
            }
        }
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checa_que_um_asterisco_eh_valido() {
        let codigo = "*";
        assert!(programa(codigo));
    }
    #[test]
    fn checa_que_vários_asteriscos_sao_validos() {
        let codigo = "****";
        assert!(programa(codigo));
    }
    #[test]
    fn checa_que_codigo_vazio_eh_invalido() {
        let codigo = "";
        assert!(!programa(codigo));
    }
    #[test]
    fn checa_que_qualquer_outro_caracter_eh_invalido() {
        let codigo = "**+**";
        assert!(!programa(codigo));
    }
}
