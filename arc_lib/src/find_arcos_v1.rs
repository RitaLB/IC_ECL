pub mod find_arcos_v1 {
    
    pub fn find_arcos_v1(saidas:  &Vec<Vec<u8>>, entradas: &Vec<Vec<u8>>) -> Vec<(u8, u8)> {
        let linhas: usize = saidas[0].len();
        let mut arc: Vec<(u8, u8)> = criar_arcos(linhas);
        check_mudanca_s(&mut arc,  &saidas);
        check_entradas(&mut arc, &entradas);
        return arc;
    }   

    // criar vetor de tuplas com todos os arcos possíveis
    fn criar_arcos(linhas: usize) -> Vec<(u8, u8)>{
        let mut arc: Vec<(u8, u8)> = Vec::new();

        for k in 0..(linhas - 1) {
            let mut l = k + 1;
            while l < linhas {
                arc.push((k as u8, l as u8));
                l+= 1;
            }
        }

        return arc;
    }

    // selecionar arcos de mudança por arcos de s
    // colocando o & eu ocnsigo modificar diretamente o objeto parâmetro ?
    fn check_mudanca_s(arc: &mut Vec<(u8, u8)>, saidas:  &Vec<Vec<u8>> ){
        let mut i = 0;
        while i< (arc.len() *  saidas.len()){
            let(l, h) = arc[i];
            for saida in saidas{
                if saida[l as usize] == saida[h as usize]{
                    arc.remove(i);
                }
                else {
                    i += 1;
                }   
            } 
        }
    }

    // selecionando arcos com mudança em apenas 1 entrada
    fn check_entradas(arc: &mut Vec<(u8, u8)>,  entradas: &Vec<Vec<u8>>){
        let mut i = 0;

        while i < arc.len() { // analiza cada arco
            let mut resultado_entradas: Vec<u8> = Vec::new();
            let l = arc[i as usize].0;
            let h = arc[i as usize].1;
    
            // analisa cada entrada
            let mut j = 0;
            while j < entradas.len() {
                let entrada = &entradas[j];
                resultado_entradas.push(entrada[l as usize] ^ entrada[h as usize]);
                j += 1;
            }
            // resultado do for, por ex: arco (0,30 a resposta vai ser [ 1 1 ] caso as duas entradas A e B mudarem)
    
            // verificando se há apenas 1 ntrada com mudança
            let mut soma: u8= 0;
            let mut k = 0;
    
            while k <= resultado_entradas.len(){
                soma += resultado_entradas.pop().unwrap_or(0);
                k +=1;
            }
    
            if soma != 1 {
                arc.remove(i);
            } else {
                i += 1;
            }
        }
    }
}

/* 
fn main() {
    let saida: Vec<u8>  =  vec![ 1, 
    1,
    1,
    0] ;

    let entradas: Vec<Vec<u8>> = vec![vec![0, 0, 1, 1], vec![0, 1, 0, 1]];

    let arc = find_arcos::find_arcos(saida, 2, entradas);

     Imprimir 'arc' após as remoções
    for tupla in &arc {
        println!("{:?}", tupla);
        }
}
*/
