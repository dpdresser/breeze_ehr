const form = document.querySelector('#signup-form');

const errorMessages = {
  fullName: 'Please enter your full name.',
  email: 'Enter a valid work email.',
  password:
    'Password needs 8+ chars with 1 uppercase letter, 1 number, and 1 special character.',
  confirmPassword: 'Your passwords must match.',
};

function validateEmail(value) {
  return /.+@.+\..+/.test(value);
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
  }, 2800);
}

function serializeFormData(formData) {
  return Object.fromEntries(formData.entries());
}

async function requestSignup(payload) {
  const body = {
    email: payload.email,
    password: payload.password,
    redirect_to: window.location.origin.concat('/signin'),
  };

  const response = await fetch('/api/auth/signup', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      Accept: 'application/json',
    },
    body: JSON.stringify(body),
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
      'We could not create your account. Please try again.';
    throw new Error(message);
  }

  localStorage.setItem('sovaehr:last-signup-email', payload.email);
  return result;
}

form?.addEventListener('submit', async (event) => {
  event.preventDefault();
  clearErrors();

  const formData = new FormData(form);
  const values = serializeFormData(formData);

  let hasError = false;

  if (!values.fullName?.trim()) {
    showError('fullName', errorMessages.fullName);
    hasError = true;
  }

  if (!validateEmail(values.email ?? '')) {
    showError('email', errorMessages.email);
    hasError = true;
  }

  if (!values.password || !isStrongPassword(values.password)) {
    showError('password', errorMessages.password);
    hasError = true;
  }

  if (values.password !== values.confirmPassword) {
    showError('confirmPassword', errorMessages.confirmPassword);
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
    await requestSignup(values);
    form.reset();
    showToast('Account created! Check your email to confirm access.');
  } catch (error) {
    console.error('Signup failed', error);
    showToast(error.message ?? 'Something went wrong. Try again in a moment.', 'error');
  } finally {
    submitButton?.removeAttribute('disabled');
    submitButton?.classList.remove('opacity-70');
  }
});

(function greetReturningUser() {
  const lastEmail = localStorage.getItem('sovaehr:last-signup-email');
  if (!lastEmail) return;

  showToast(`Welcome back, ${lastEmail.split('@')[0]}!`, 'success');
})();
