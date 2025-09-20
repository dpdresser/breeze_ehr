const appointments = [
  {
    id: 1,
    start: '2025-01-01T09:00:00',
    summary: 'Initial consult',
    patient: 'Jordan Alvarez',
    modality: 'Telehealth',
  },
  {
    id: 2,
    start: '2025-01-01T10:30:00',
    summary: 'Follow-up',
    patient: 'Priya Natarajan',
    modality: 'In person',
  },
  {
    id: 3,
    start: '2025-01-01T13:00:00',
    summary: 'Billing review',
    patient: 'Marcus Lee',
    modality: 'Admin',
  },
];

const modalityPills = {
  Telehealth: 'bg-terra-dark text-seafoam-light',
  'In person': 'bg-seafoam-dark/70 text-terra-dark',
  Admin: 'bg-white border border-terra-dark/20 text-terra-dark',
};

const timeFormatter = new Intl.DateTimeFormat(undefined, {
  hour: 'numeric',
  minute: 'numeric',
});

function formatAppointmentTime(value) {
  try {
    return timeFormatter.format(new Date(value));
  } catch (error) {
    console.warn('Unable to format appointment time', { value, error });
    return value;
  }
}

function renderAppointments(list) {
  const container = document.querySelector('[data-appointments]');
  if (!container) {
    return;
  }

  if (!list.length) {
    container.innerHTML = `
      <li class="rounded-lg border border-dashed border-terra-dark/30 bg-white/60 px-4 py-6 text-center text-sm text-terra-light">
        No appointments scheduled for today.
      </li>
    `;
    return;
  }

  container.innerHTML = list
    .map((appointment) => {
      const label = formatAppointmentTime(appointment.start);
      const pillClasses = modalityPills[appointment.modality] ?? 'bg-terra-dark text-seafoam-light';
      return `
        <li class="flex items-center justify-between rounded-lg border border-seafoam-dark/30 bg-seafoam-light/40 px-4 py-3 shadow-sm">
          <div>
            <p class="text-sm font-semibold">${appointment.patient}</p>
            <p class="text-xs text-terra-light">${label} • ${appointment.summary}</p>
          </div>
          <span class="rounded-full px-3 py-1 text-xs font-semibold ${pillClasses}">${appointment.modality}</span>
        </li>
      `;
    })
    .join('');
}

function showToast(message) {
  const toast = document.createElement('div');
  toast.setAttribute('role', 'status');
  toast.className = 'fixed bottom-6 right-6 z-50 rounded-lg bg-terra-dark px-4 py-3 text-sm font-semibold text-seafoam-light shadow-lg shadow-terra-dark/30 transition-opacity duration-300 opacity-0';
  toast.textContent = message;

  document.body.appendChild(toast);

  requestAnimationFrame(() => {
    toast.classList.remove('opacity-0');
  });

  setTimeout(() => {
    toast.classList.add('opacity-0');
    setTimeout(() => toast.remove(), 300);
  }, 3000);
}

function attachInteractions() {
  const requestDemoButton = document.querySelector('[data-action="request-demo"]');
  const exploreButton = document.querySelector('[data-action="explore"]');

  if (requestDemoButton) {
    requestDemoButton.addEventListener('click', () => {
      const timestamp = new Date().toISOString();
      localStorage.setItem('sovaehr:last-demo-request', timestamp);
      showToast('Thanks! We’ll reach out soon to schedule your walkthrough.');
    });
  }

  if (exploreButton) {
    exploreButton.addEventListener('click', () => {
      const featuresSection = document.querySelector('#features');
      if (featuresSection) {
        featuresSection.scrollIntoView({ behavior: 'smooth', block: 'start' });
      }
    });
  }

  const lastRequestAt = localStorage.getItem('sovaehr:last-demo-request');
  if (lastRequestAt) {
    const lastRequestDate = new Date(lastRequestAt);
    if (!Number.isNaN(lastRequestDate.valueOf())) {
      const relativeTimeFormatter = new Intl.RelativeTimeFormat(undefined, { numeric: 'auto' });
      const diffInHours = Math.round((Date.now() - lastRequestDate.valueOf()) / (1000 * 60 * 60));
      const relativeLabel = diffInHours
        ? relativeTimeFormatter.format(-diffInHours, 'hour')
        : 'just now';
      showToast(`Hi again! Your last demo request was ${relativeLabel}.`);
    }
  }
}

renderAppointments(appointments);
attachInteractions();
