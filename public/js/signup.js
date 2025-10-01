// Sign Up Page JavaScript
document.addEventListener('DOMContentLoaded', function() {
    // Get form elements
    const signupForm = document.getElementById('signup-form');
    const firstNameInput = document.getElementById('firstName');
    const lastNameInput = document.getElementById('lastName');
    const emailInput = document.getElementById('email');
    const passwordInput = document.getElementById('password');
    const confirmPasswordInput = document.getElementById('confirmPassword');
    const passwordToggle = document.getElementById('password-toggle');
    const confirmPasswordToggle = document.getElementById('confirm-password-toggle');
    const signupBtn = document.getElementById('signup-btn');
    const googleSignupBtn = document.getElementById('google-signup');
    const signinLink = document.getElementById('signin-link');
    const termsCheckbox = document.getElementById('terms');
    const strengthFill = document.getElementById('strength-fill');
    const strengthText = document.getElementById('strength-text');
    
    // Password visibility toggle functions
    function setupPasswordToggle(toggleBtn, passwordField) {
        toggleBtn.addEventListener('click', function() {
            const isPassword = passwordField.type === 'password';
            const eyeOpen = toggleBtn.querySelector('.eye-open');
            const eyeClosed = toggleBtn.querySelector('.eye-closed');
            
            passwordField.type = isPassword ? 'text' : 'password';
            
            if (isPassword) {
                eyeOpen.style.display = 'none';
                eyeClosed.style.display = 'block';
            } else {
                eyeOpen.style.display = 'block';
                eyeClosed.style.display = 'none';
            }
        });
    }
    
    // Setup password toggles
    setupPasswordToggle(passwordToggle, passwordInput);
    setupPasswordToggle(confirmPasswordToggle, confirmPasswordInput);
    
    // Password strength checker
    function checkPasswordStrength(password) {
        let strength = 0;
        let feedback = 'Password strength';
        
        if (password.length >= 8) strength += 1;
        if (password.match(/[a-z]/)) strength += 1;
        if (password.match(/[A-Z]/)) strength += 1;
        if (password.match(/[0-9]/)) strength += 1;
        if (password.match(/[^a-zA-Z0-9]/)) strength += 1;
        
        // Reset classes
        strengthFill.className = 'strength-fill';
        strengthText.className = 'strength-text';
        
        if (password.length === 0) {
            strengthFill.style.width = '0%';
            strengthText.textContent = 'Password strength';
            return;
        }
        
        switch (strength) {
            case 0:
            case 1:
                strengthFill.classList.add('weak');
                strengthText.classList.add('weak');
                strengthText.textContent = 'Weak password';
                break;
            case 2:
            case 3:
                strengthFill.classList.add('fair');
                strengthText.classList.add('fair');
                strengthText.textContent = 'Fair password';
                break;
            case 4:
                strengthFill.classList.add('good');
                strengthText.classList.add('good');
                strengthText.textContent = 'Good password';
                break;
            case 5:
                strengthFill.classList.add('strong');
                strengthText.classList.add('strong');
                strengthText.textContent = 'Strong password';
                break;
        }
        
        return strength;
    }
    
    // Password strength monitoring
    passwordInput.addEventListener('input', function() {
        checkPasswordStrength(passwordInput.value);
    });
    
    // Form validation functions
    function validateEmail(email) {
        const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
        return emailRegex.test(email);
    }
    
    function validatePassword(password) {
        return password.length >= 8;
    }
    
    function validatePasswordMatch(password, confirmPassword) {
        return password === confirmPassword && password.length > 0;
    }
    
    function validateName(name) {
        return name.trim().length >= 2;
    }
    
    function showError(input, message) {
        const formGroup = input.closest('.form-group');
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
        
        // Insert after the input or password container
        const inputContainer = formGroup.querySelector('.password-input-container') || input;
        inputContainer.parentNode.insertBefore(errorDiv, inputContainer.nextSibling);
    }
    
    function showSuccess(input) {
        const formGroup = input.closest('.form-group');
        formGroup.classList.add('success');
        formGroup.classList.remove('error');
        
        // Remove existing error message
        const existingError = formGroup.querySelector('.error-message');
        if (existingError) {
            existingError.remove();
        }
    }
    
    function clearValidation(input) {
        const formGroup = input.closest('.form-group');
        formGroup.classList.remove('error', 'success');
        
        const existingError = formGroup.querySelector('.error-message');
        if (existingError) {
            existingError.remove();
        }
    }
    
    // Real-time validation
    firstNameInput.addEventListener('blur', function() {
        if (!validateName(firstNameInput.value)) {
            showError(firstNameInput, 'First name must be at least 2 characters');
        } else {
            showSuccess(firstNameInput);
        }
    });
    
    lastNameInput.addEventListener('blur', function() {
        if (!validateName(lastNameInput.value)) {
            showError(lastNameInput, 'Last name must be at least 2 characters');
        } else {
            showSuccess(lastNameInput);
        }
    });
    
    emailInput.addEventListener('blur', function() {
        if (!validateEmail(emailInput.value)) {
            showError(emailInput, 'Please enter a valid email address');
        } else {
            showSuccess(emailInput);
        }
    });
    
    passwordInput.addEventListener('blur', function() {
        if (!validatePassword(passwordInput.value)) {
            showError(passwordInput, 'Password must be at least 8 characters long');
        } else {
            showSuccess(passwordInput);
        }
    });
    
    confirmPasswordInput.addEventListener('blur', function() {
        if (!validatePasswordMatch(passwordInput.value, confirmPasswordInput.value)) {
            showError(confirmPasswordInput, 'Passwords do not match');
        } else {
            showSuccess(confirmPasswordInput);
        }
    });
    
    // Clear validation on input
    [firstNameInput, lastNameInput, emailInput, passwordInput, confirmPasswordInput].forEach(input => {
        input.addEventListener('input', function() {
            clearValidation(input);
        });
    });
    
    // Form submission
    signupForm.addEventListener('submit', async function(e) {
        e.preventDefault();
        
        // Validate all fields
        let isValid = true;
        
        if (!validateName(firstNameInput.value)) {
            showError(firstNameInput, 'First name must be at least 2 characters');
            isValid = false;
        }
        
        if (!validateName(lastNameInput.value)) {
            showError(lastNameInput, 'Last name must be at least 2 characters');
            isValid = false;
        }
        
        if (!validateEmail(emailInput.value)) {
            showError(emailInput, 'Please enter a valid email address');
            isValid = false;
        }
        
        if (!validatePassword(passwordInput.value)) {
            showError(passwordInput, 'Password must be at least 8 characters long');
            isValid = false;
        }
        
        if (!validatePasswordMatch(passwordInput.value, confirmPasswordInput.value)) {
            showError(confirmPasswordInput, 'Passwords do not match');
            isValid = false;
        }
        
        if (!termsCheckbox.checked) {
            showError(termsCheckbox, 'You must agree to the Terms of Service');
            isValid = false;
        }
        
        if (!isValid) {
            return;
        }
        
        // Show loading state
        const btnText = signupBtn.querySelector('.btn-text');
        const loadingSpinner = signupBtn.querySelector('.loading-spinner');
        
        btnText.textContent = 'Creating Account...';
        loadingSpinner.style.display = 'block';
        signupBtn.disabled = true;
        
        try {
            const response = await fetch('/api/auth/signup', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    firstName: firstNameInput.value.trim(),
                    lastName: lastNameInput.value.trim(),
                    email: emailInput.value.trim(),
                    password: passwordInput.value
                })
            });
            
            const data = await response.json();
            
            if (response.ok) {
                // Success - redirect to verification page or signin
                showGlobalSuccess('Account created successfully! Please check your email to verify your account.');
                setTimeout(() => {
                    window.location.href = '/signin.html';
                }, 2000);
            } else {
                // Handle specific error cases
                if (data.message) {
                    showGlobalError(data.message);
                } else {
                    showGlobalError('Failed to create account. Please try again.');
                }
            }
        } catch (error) {
            console.error('Signup error:', error);
            showGlobalError('Network error. Please check your connection and try again.');
        } finally {
            // Reset loading state
            btnText.textContent = 'Create Account';
            loadingSpinner.style.display = 'none';
            signupBtn.disabled = false;
        }
    });
    
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
    
    // Google signup (placeholder)
    googleSignupBtn.addEventListener('click', function() {
        showGlobalError('Google signup is not available in demo mode');
    });
    
    // Navigation
    signinLink.addEventListener('click', function(e) {
        e.preventDefault();
        window.location.href = '/signin.html';
    });
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
        background-color: #f0fdf4;
        border: 1px solid #bbf7d0;
        color: #16a34a;
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