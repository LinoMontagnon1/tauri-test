document.getElementById('loginBtn').addEventListener('click', function() {
  const username = document.getElementById('username').value;
  const password = document.getElementById('password').value;

  window.__TAURI__.invoke('login', { username, password })
    .then((response) => {
      if (response) {
        // Login bem-sucedido
        window.location.href = 'dashboard.html';
      } else {
        // Falha no login
        alert('Login failed. Please check your username and password.');
      }
    })
    .catch((error) => {
      console.error('Error during login:', error);
      alert('An error occurred while trying to log in.');
    });
});



// Adiciona evento de clique ao botão de registro
document.getElementById('signupBtn').addEventListener('click', function() {
  // Obtenha o valor do usuário e da senha dos elementos do formulário de registro
  const username = document.getElementById('signupUsername').value;
  const password = document.getElementById('signupPassword').value;
  const confirmPassword = document.getElementById('confirmPassword').value;

  // Verifica se as senhas coincidem
  if (password !== confirmPassword) {
    alert('Passwords do not match.');
    return; // Interrompe a execução se as senhas não coincidirem
  }

  // Chama o comando 'register' do backend Rust
  window.__TAURI__.invoke('register', { username, password })
    .then(() => {
      // Se o registro for bem-sucedido, possivelmente redirecione para a página de login ou confirme o registro
      alert('Registration successful. You can now log in.');
      window.location.href = 'index.html'; // Ajuste conforme o seu fluxo de usuário
    })
    .catch((error) => {
      // Trate o caso de erro na invocação do comando ou na lógica Rust
      console.error('Error during registration:', error);
      alert('An error occurred while trying to register.');
    });
});