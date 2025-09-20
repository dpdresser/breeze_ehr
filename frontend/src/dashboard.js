const tokenKey = 'sovaehr:auth-token';
const emailKey = 'sovaehr:last-signin-email';

const state = {
  appointments: [
    {
      id: 1,
      start: '2025-01-01T09:00:00',
      patient: 'Jordan Alvarez',
      type: 'Initial consult',
      status: 'Confirmed',
    },
    {
      id: 2,
      start: '2025-01-01T10:30:00',
      patient: 'Priya Natarajan',
      type: 'Follow-up',
      status: 'Telehealth',
    },
    {
      id: 3,
      start: '2025-01-01T13:00:00',
      patient: 'Marcus Lee',
      type: 'Billing review',
      status: 'In office',
    },
  ],
  tasks: [
    {
      id: 't-1',
      title: 'Review intake questionnaire for Jordan Alvarez',
      due: 'Today • 8:45 AM',
    },
    {
      id: 't-2',
      title: 'Approve treatment plan updates for Priya Natarajan',
      due: 'Today • 12:00 PM',
    },
  ],
};

const timeFormatter = new Intl.DateTimeFormat(undefined, {
  hour: 'numeric',
  minute: 'numeric',
});

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

function hydrateGreeting() {
  const greeting = document.querySelector('[data-user-greeting]');
  if (!greeting) return;

  const email = localStorage.getItem(emailKey);
  const nameFromEmail = email ? email.split('@')[0] : null;
  const capitalized = nameFromEmail
    ? nameFromEmail
        .split(/[._-]/)
        .map((segment) => segment.charAt(0).toUpperCase() + segment.slice(1))
        .join(' ')
    : null;

  greeting.textContent = capitalized ? `Welcome back, ${capitalized}!` : 'Welcome back!';
}

function renderMetrics() {
  const appointmentsCount = document.querySelector('[data-metric="appointments"]');
  const tasksCount = document.querySelector('[data-metric="tasks"]');

  if (appointmentsCount) {
    appointmentsCount.textContent = state.appointments.length.toString();
  }

  if (tasksCount) {
    tasksCount.textContent = state.tasks.length.toString();
  }
}

function renderActionItems() {
  const list = document.querySelector('[data-action-items]');
  if (!list) return;

  if (!state.tasks.length) {
    list.innerHTML = `
      <li class="rounded-xl border border-dashed border-terra-dark/30 bg-white/70 px-5 py-4 text-sm text-terra-light">
        All caught up! New tasks will show here.
      </li>
    `;
    return;
  }

  list.innerHTML = state.tasks
    .map(
      (task) => `
        <li class="rounded-xl border border-seafoam-dark/40 bg-white px-5 py-4 shadow-sm">
          <p class="text-sm font-medium text-terra-dark">${task.title}</p>
          <p class="mt-1 text-xs text-terra-light">${task.due}</p>
        </li>
      `,
    )
    .join('');
}

function renderTimeline() {
  const container = document.querySelector('[data-timeline]');
  if (!container) return;

  if (!state.appointments.length) {
    container.innerHTML = `
      <li class="rounded-xl border border-dashed border-terra-dark/30 bg-white/70 px-5 py-4 text-sm text-terra-light">
        No appointments scheduled for today.
      </li>
    `;
    return;
  }

  container.innerHTML = state.appointments
    .map((appointment) => {
      const label = timeFormatter.format(new Date(appointment.start));
      return `
        <li class="rounded-2xl border border-seafoam-dark/30 bg-seafoam-light/40 px-5 py-4 shadow-sm transition hover:-translate-y-0.5 hover:bg-seafoam-light/60">
          <div class="flex items-center justify-between">
            <p class="text-sm font-semibold text-terra-dark">${appointment.patient}</p>
            <span class="text-xs text-terra-light">${label}</span>
          </div>
          <p class="mt-1 text-xs uppercase tracking-wide text-terra-light">${appointment.type}</p>
          <p class="mt-2 inline-flex rounded-full bg-terra-dark px-3 py-1 text-xs font-semibold text-seafoam-light">${appointment.status}</p>
        </li>
      `;
    })
    .join('');
}

function attachSignout() {
  const button = document.querySelector('[data-action="signout"]');
  if (!button) return;

  button.addEventListener('click', () => {
    localStorage.removeItem(tokenKey);
    showToast('Signed out. See you soon!', 'success');
    setTimeout(() => {
      window.location.assign('/signin');
    }, 800);
  });
}

(function init() {
  const token = localStorage.getItem(tokenKey);
  if (!token) {
    window.location.replace('/signin');
    return;
  }

  hydrateGreeting();
  renderMetrics();
  renderActionItems();
  renderTimeline();
  attachSignout();
})();
