// Sign In Page JavaScript
document.addEventListener('DOMContentLoaded', function() {
    // Get form elements
    const signinForm = document.getElementById('signin-form');
    const emailInput = document.getElementById('email');
    const passwordInput = document.getElementById('password');
    const passwordToggle = document.getElementById('password-toggle');
    const signinBtn = document.getElementById('signin-btn');
    const googleSigninBtn = document.getElementById('google-signin');
    const signupLink = document.getElementById('signup-link');
    const forgotPasswordLink = document.querySelector('.forgot-password');
    
    // Password visibility toggle
    passwordToggle.addEventListener('click', function() {
        const isPassword = passwordInput.type === 'password';
        const eyeOpen = passwordToggle.querySelector('.eye-open');
        const eyeClosed = passwordToggle.querySelector('.eye-closed');
        
        passwordInput.type = isPassword ? 'text' : 'password';
        
        if (isPassword) {
            eyeOpen.style.display = 'none';
            eyeClosed.style.display = 'block';
        } else {
            eyeOpen.style.display = 'block';
            eyeClosed.style.display = 'none';
        }
    });
    
    // Form validation
    function validateEmail(email) {
        const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
        return emailRegex.test(email);
    }
    
    function validatePassword(password) {
        return password.length >= 1; // Relaxed for demo
    }
    
    function showError(input, message) {
        const formGroup = input.parentElement;
        formGroup.classList.add('error');
        formGroup.classList.remove('success');
        
        // Remove existing error message
        const existingError = formGroup.querySelector('.error-message');
        if (existingError) {
            existingError.remove();
        }
        
        // Add new error message
        const errorDiv = document.createElement('div');
        errorDiv.className = 'error-message';
        errorDiv.innerHTML = `
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="12" cy="12" r="10"/>
                <line x1="15" y1="9" x2="9" y2="15"/>
                <line x1="9" y1="9" x2="15" y2="15"/>
            </svg>
            ${message}
        `;
        formGroup.appendChild(errorDiv);
    }
    
    function showSuccess(input) {
        const formGroup = input.parentElement;
        formGroup.classList.add('success');
        formGroup.classList.remove('error');
        
        // Remove error message
        const existingError = formGroup.querySelector('.error-message');
        if (existingError) {
            existingError.remove();
        }
    }
    
    function clearValidation(input) {
        const formGroup = input.parentElement;
        formGroup.classList.remove('error', 'success');
        
        const existingError = formGroup.querySelector('.error-message');
        if (existingError) {
            existingError.remove();
        }
    }
    
    // Real-time validation
    emailInput.addEventListener('blur', function() {
        const email = emailInput.value.trim();
        
        if (!email) {
            showError(emailInput, 'Email address is required');
        } else if (!validateEmail(email)) {
            showError(emailInput, 'Please enter a valid email address');
        } else {
            showSuccess(emailInput);
        }
    });
    
    emailInput.addEventListener('input', function() {
        if (emailInput.parentElement.classList.contains('error')) {
            clearValidation(emailInput);
        }
    });
    
    passwordInput.addEventListener('blur', function() {
        const password = passwordInput.value;
        
        if (!password) {
            showError(passwordInput, 'Password is required');
        } else if (!validatePassword(password)) {
            showError(passwordInput, 'Password must be at least 8 characters');
        } else {
            showSuccess(passwordInput);
        }
    });
    
    passwordInput.addEventListener('input', function() {
        if (passwordInput.parentElement.classList.contains('error')) {
            clearValidation(passwordInput);
        }
    });
    
    // Form submission
    signinForm.addEventListener('submit', function(e) {
        e.preventDefault();
        
        const email = emailInput.value.trim();
        const password = passwordInput.value;
        let isValid = true;
        
        // Validate email
        if (!email) {
            showError(emailInput, 'Email address is required');
            isValid = false;
        } else if (!validateEmail(email)) {
            showError(emailInput, 'Please enter a valid email address');
            isValid = false;
        }
        
        // Validate password
        if (!password) {
            showError(passwordInput, 'Password is required');
            isValid = false;
        } else if (!validatePassword(password)) {
            showError(passwordInput, 'Password must be at least 8 characters');
            isValid = false;
        }
        
        if (!isValid) {
            return;
        }
        
        // Show loading state
        setLoadingState(true);
        
        // Call actual authentication API
        handleSignIn(email, password);
    });
    
    async function handleSignIn(email, password) {
        try {
            const response = await fetch('/api/auth/signin', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ email, password })
            });

            if (response.ok) {
                const data = await response.json();
                
                // Store the token
                if (data.token) {
                    localStorage.setItem('authToken', data.token);
                    
                    // Show success message
                    showGlobalSuccess('Sign in successful! Redirecting to dashboard...');
                    
                    // Redirect to dashboard after a brief delay
                    setTimeout(() => {
                        window.location.href = '/dashboard.html';
                    }, 1000);
                } else {
                    setLoadingState(false);
                    showGlobalError('Authentication failed: No token received');
                }
            } else {
                // Handle HTTP error responses
                setLoadingState(false);
                let errorMessage = 'Sign in failed';
                
                try {
                    const errorData = await response.json();
                    errorMessage = errorData.error || errorData.message || errorMessage;
                } catch (e) {
                    // If response is not JSON, use status text
                    errorMessage = response.statusText || errorMessage;
                }
                
                showGlobalError(errorMessage);
                console.error('Sign in failed:', response.status, errorMessage);
            }
        } catch (error) {
            setLoadingState(false);
            showGlobalError('Network error: Please check your connection and try again');
            console.error('Sign in network error:', error);
        }
    }
    
    // Loading state management
    function setLoadingState(loading) {
        const btnText = signinBtn.querySelector('.btn-text');
        const spinner = signinBtn.querySelector('.loading-spinner');
        
        if (loading) {
            signinBtn.disabled = true;
            btnText.textContent = 'Signing in...';
            spinner.style.display = 'block';
        } else {
            signinBtn.disabled = false;
            btnText.textContent = 'Sign In';
            spinner.style.display = 'none';
        }
    }
    
    // Google Sign In (placeholder)
    googleSigninBtn.addEventListener('click', function() {
        // This would integrate with Google OAuth
        // For demo purposes, redirect to dashboard
        console.log('Google sign in clicked');
        alert('Google Sign-In would be integrated here with your Supabase authentication.');
        
        // In production, this would trigger Google OAuth flow
        // Example with Supabase:
        // supabase.auth.signInWithOAuth({ provider: 'google' })
    });
    
    // Sign up link
    signupLink.addEventListener('click', function(e) {
        e.preventDefault();
        // Redirect to sign up page (to be created)
        alert('Sign-up page would be created here. For demo, you can use any credentials to sign in.');
    });
    
    // Forgot password link
    forgotPasswordLink.addEventListener('click', function(e) {
        e.preventDefault();
        // Handle forgot password
        alert('Password reset functionality would be implemented here with Supabase auth.');
    });
    
    // Demo functionality - auto-fill for easier testing
    function fillDemoCredentials() {
        emailInput.value = 'demo@breezeehr.com';
        passwordInput.value = 'demo123';
    }
    
    // Add demo button for easier testing (remove in production)
    const demoAlert = document.querySelector('.demo-alert');
    if (demoAlert) {
        demoAlert.style.cursor = 'pointer';
        demoAlert.addEventListener('click', fillDemoCredentials);
        demoAlert.title = 'Click to auto-fill demo credentials';
    }
    
    // Keyboard shortcuts
    document.addEventListener('keydown', function(e) {
        // Alt + D for demo credentials (remove in production)
        if (e.altKey && e.key === 'd') {
            e.preventDefault();
            fillDemoCredentials();
        }
    });
    
    // Analytics tracking (placeholder)
    function trackEvent(action, details = {}) {
        console.log('Analytics Event:', action, details);
        // In production, integrate with your analytics platform
        // Example: gtag('event', action, details);
    }
    
    // Track form submission attempt
    signinForm.addEventListener('submit', () => trackEvent('sign_in_attempt'));
    
    // Global notification functions
    function showGlobalError(message) {
        showGlobalNotification(message, 'error');
    }
    
    function showGlobalSuccess(message) {
        showGlobalNotification(message, 'success');
    }
    
    function showGlobalNotification(message, type) {
        // Remove existing notifications
        const existing = document.querySelector('.global-notification');
        if (existing) {
            existing.remove();
        }
        
        const notification = document.createElement('div');
        notification.className = `global-notification ${type}`;
        notification.innerHTML = `
            <div class="notification-content">
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    ${type === 'error' ? 
                        '<circle cx="12" cy="12" r="10"/><line x1="15" y1="9" x2="9" y2="15"/><line x1="9" y1="9" x2="15" y2="15"/>' :
                        '<path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22,4 12,14.01 9,11.01"/>'
                    }
                </svg>
                <span>${message}</span>
                <button class="notification-close">×</button>
            </div>
        `;
        
        document.body.appendChild(notification);
        
        // Auto remove after 5 seconds
        setTimeout(() => {
            if (notification.parentNode) {
                notification.remove();
            }
        }, 5000);
        
        // Manual close
        notification.querySelector('.notification-close').addEventListener('click', () => {
            notification.remove();
        });
    }
    
    // Add event tracking
    googleSigninBtn.addEventListener('click', () => trackEvent('google_sign_in_attempt'));
    signupLink.addEventListener('click', () => trackEvent('signup_link_click'));
    forgotPasswordLink.addEventListener('click', () => trackEvent('forgot_password_click'));
});

// Add global notification styles
const notificationStyles = document.createElement('style');
notificationStyles.textContent = `
    .global-notification {
        position: fixed;
        top: 20px;
        right: 20px;
        z-index: 1000;
        max-width: 400px;
        padding: 16px;
        border-radius: 8px;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
        animation: slideIn 0.3s ease-out;
    }
    
    .global-notification.error {
        background-color: #fef2f2;
        border: 1px solid #fecaca;
        color: #dc2626;
    }
    
    .global-notification.success {
        background-color: #faf5f6;
        border: 1px solid var(--rosy-brown);
        color: var(--rosy-brown);
    }
    
    .notification-content {
        display: flex;
        align-items: center;
        gap: 12px;
    }
    
    .notification-content svg {
        flex-shrink: 0;
    }
    
    .notification-close {
        background: none;
        border: none;
        font-size: 18px;
        cursor: pointer;
        margin-left: auto;
        padding: 0;
        width: 20px;
        height: 20px;
        display: flex;
        align-items: center;
        justify-content: center;
        border-radius: 50%;
        transition: background-color 0.2s;
    }
    
    .notification-close:hover {
        background-color: rgba(0, 0, 0, 0.1);
    }
    
    @keyframes slideIn {
        from {
            transform: translateX(100%);
            opacity: 0;
        }
        to {
            transform: translateX(0);
            opacity: 1;
        }
    }
`;
document.head.appendChild(notificationStyles);