use crate::security::auth_key::ApiKey;

#[get("/")]
pub async fn index() -> &'static str {
    "Está api tem a intenção de armazenar validações criptograficas genericas para as rotas
    Lembrando que a base tem a intenção de ser fracamente associada tornando praticamente 
    impossivel a extração dos dados associados por conta de não sabermos o que está sendo validado
    nem de sabermos qual a chave associada a categoria criada
    /category/<key> - irá armazenar as categorias e gerencia-las
    /data/<category>/<key> - Crud para armazenar as validações
    
    Mais tarde implementar rotas de storage(seguro)
    /storage/<category>/<key> - Crud para armazenar dados de forma segura
    
    A intenção de uso é para armazenar senhas ou dados importantes sem a associação com o uso delas tornando
    um vazamento de dados bem menos perigoso, uma vez que sem a categoria/chave(armazenada somente do lado do cliente)
    os dados são só um hash ou no caso do storage um blob de dados sem significado."
}

#[get("/protected")]
pub fn protected(_key: ApiKey) -> &'static str {
    "This is a protected route"
}
