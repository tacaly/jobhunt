fn main () {
    // starts the job gen work, to get jobs from the d

    // Stores the security token for the API service
    Token_index = "";
    Token_net = "";

}

//Jobnet auth api 
fn job_net_token {
    AuthenticationService authenticationService = new AuthenticationService();
    UserCredential userCredential = new UserCredential();
    userCredential.Username = Username;
    userCredential.Password = Password;
    SecurityToken securityToken = new SecurityToken();
    securityToken = authenticationService.RequestAuthentication(userCredential);
}

//JobIndex auth api
fn job_index_token {
    AuthenticationService authenticationService = new AuthenticationService();
    UserCredential userCredential = new UserCredential ();
    userCredential.UsernameIndex = UsernameIndex;
    userCredential.PasswordIndex = PasswordIndex;
    SecurityToken securityToken = new SecurityToken();
    securityToken = authenticationService.RequestAuthentication(userCredential);
}