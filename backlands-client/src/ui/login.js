import { state } from '../state.js';
import { connect, send, on } from '../network/socket.js';

export function showLogin(container) {
  const card = document.createElement('div');
  card.className = 'card';
  card.innerHTML = `
    <h2>Backlands MMORPG</h2>
    <div class="input-group">
      <label for="username">Usuário</label>
      <input type="text" id="username" placeholder="Digite seu usuário">
    </div>
    <div class="input-group">
      <label for="password">Senha</label>
      <input type="password" id="password" placeholder="Digite sua senha">
    </div>
    <button id="btn-login">Entrar</button>
    <div id="login-error" class="error-msg"></div>
  `;
  container.appendChild(card);

  document.getElementById('btn-login').addEventListener('click', async () => {
    const user = document.getElementById('username').value.trim();
    const pass = document.getElementById('password').value.trim();
    if (!user || !pass) { showError('Preencha usuário e senha'); return; }

    const btn = document.getElementById('btn-login');
    btn.disabled = true;
    btn.textContent = 'Conectando...';

    try {
      await connect();
    } catch {
      showError('Servidor indisponível. Verifique se o servidor está rodando.');
      btn.disabled = false;
      btn.textContent = 'Entrar';
      return;
    }

    on('char_list', (data) => {
      state.set('currentUser', user);
      state.set('charList', data.players);
      location.hash = '#select-char';
    });

    on('login_fail', (data) => {
      showError(data.reason);
      btn.disabled = false;
      btn.textContent = 'Entrar';
    });

    send('login', { username: user, password: pass });
  });
}

function showError(msg) {
  document.getElementById('login-error').innerText = msg;
}
