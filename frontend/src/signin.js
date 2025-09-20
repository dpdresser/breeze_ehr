const form = document.querySelector('#signin-form');

const errorMessages = {
  email: 'Enter a valid work email.',
  password:
    'Password needs 8+ chars with 1 uppercase letter, 1 number, and 1 special character.',
};

const emailKey = 'sovaehr:last-signin-email';

function validateEmail(value) {
  return /.+@.+\..+/.test(value);
}

function hasDigit(value = '') {
  return /\d/.test(value);
}

function hasUppercase(value = '') {
  return /[A-Z]/.test(value);
}

function hasSpecialCharacter(value = '') {
  return /[^\w\s]/.test(value);
}

function isStrongPassword(value = '') {
  return (
    value.length >= 8 &&
    hasDigit(value) &&
    hasUppercase(value) &&
    hasSpecialCharacter(value)
  );
}

function showError(field, message) {
  const error = form?.querySelector(`[data-error="${field}"]`);
  if (!error) return;
  error.textContent = message;
  error.classList.remove('hidden');
}

function clearErrors() {
  form?.querySelectorAll('[data-error]').forEach((element) => {
    element.textContent = '';
    element.classList.add('hidden');
  });
}

function showToast(message, tone = 'success') {
  const palette = tone === 'error'
    ? 'bg-red-600 text-white shadow-red-900/20'
    : 'bg-terra-dark text-seafoam-light shadow-terra-dark/30';

  const toast = document.createElement('div');
  toast.setAttribute('role', 'status');
  toast.className = `fixed bottom-6 right-6 z-50 rounded-lg px-4 py-3 text-sm font-semibold shadow-lg transition-opacity duration-300 opacity-0 ${palette}`;
  toast.textContent = message;

  document.body.appendChild(toast);
  requestAnimationFrame(() => toast.classList.remove('opacity-0'));
  setTimeout(() => {
    toast.classList.add('opacity-0');
    setTimeout(() => toast.remove(), 300);
  }, 2600);
}

async function requestSignin(payload) {
  const response = await fetch('/api/auth/signin', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      Accept: 'application/json',
    },
    body: JSON.stringify({ email: payload.email, password: payload.password }),
  });

  let result;
  try {
    result = await response.json();
  } catch (error) {
    result = null;
  }

  if (!response.ok) {
    const message =
      result?.error?.message ||
      result?.message ||
      'Unable to sign you in right now. Please verify your credentials.';
    throw new Error(message);
  }

  if (result?.token) {
    localStorage.setItem('sovaehr:auth-token', result.token);
  }
  localStorage.setItem(emailKey, payload.email);

  return result;
}

form?.addEventListener('submit', async (event) => {
  event.preventDefault();
  clearErrors();

  const formData = new FormData(form);
  const values = Object.fromEntries(formData.entries());

  let hasError = false;

  if (!validateEmail(values.email ?? '')) {
    showError('email', errorMessages.email);
    hasError = true;
  }

  if (!values.password || !isStrongPassword(values.password)) {
    showError('password', errorMessages.password);
    hasError = true;
  }

  if (hasError) {
    showToast('Fix the highlighted fields and try again.', 'error');
    return;
  }

  const submitButton = form.querySelector('button[type="submit"]');
  submitButton?.setAttribute('disabled', 'disabled');
  submitButton?.classList.add('opacity-70');

  try {
    await requestSignin(values);
    showToast('Signed in! Redirecting to your dashboard...');
    setTimeout(() => {
      window.location.assign('/dashboard');
    }, 1200);
  } catch (error) {
    console.error('Signin failed', error);
    showToast(error.message ?? 'Something went wrong. Try again in a moment.', 'error');
  } finally {
    submitButton?.removeAttribute('disabled');
    submitButton?.classList.remove('opacity-70');
  }
});

(function greetReturningUser() {
  const token = localStorage.getItem('sovaehr:auth-token');
  if (!token) return;
  showToast('Welcome back!');
})();
