use std::fs;

// Função que faz a compactação RLE (Run-Length Encoding)
fn comprimir_texto(texto: &str) -> String {
    let chars: Vec<char> = texto.chars().collect();
    if chars.is_empty() {
        return String::new();
    }

    let mut resultado = String::new();
    let mut current_char = chars[0];
    let mut count = 1;

    for &c in chars.iter().skip(1) {
        if c == current_char {
            count += 1;
        } else {
            // Se repetiu mais de 1 vez, adiciona o número antes do caractere
            if count > 1 {
                resultado.push_str(&count.to_string());
            }
            resultado.push(current_char);
            
            // Reinicia a contagem para o novo caractere
            current_char = c;
            count = 1;
        }
    }

    // Não se esqueça do último grupo de caracteres do arquivo
    if count > 1 {
        resultado.push_str(&count.to_string());
    }
    resultado.push(current_char);

    resultado
}

fn main() {
    let caminho_entrada = "file.txt";
    let caminho_saida = "file_comprimido.txt";

    // Lê o arquivo inteiro
    match fs::read_to_string(caminho_entrada) {
        Ok(texto) => {
            // Remove quebras de linha finais indesejadas para evitar ruído no teste
            let texto_limpo = texto.trim_end_matches(&['\r', '\n'][..]);
            
            // Processa a compactação
            let texto_comprimido = comprimir_texto(texto_limpo);

            // Escreve o resultado em um novo arquivo
            match fs::write(caminho_saida, &texto_comprimido) {
                Ok(_) => {
                    println!("Sucesso! Arquivo compactado salvo em '{}'", caminho_saida);
                    println!("Conteúdo original: {}", texto_limpo);
                    println!("Conteúdo gerado:   {}", texto_comprimido);
                }
                Err(e) => {
                    eprintln!("Erro ao escrever o novo arquivo: {}", e);
                }
            }
        }
        Err(erro) => {
            eprintln!("Erro ao ler o arquivo '{}': {}", caminho_entrada, erro);
        }
    }
}
