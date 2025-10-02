// Dashboard JavaScript
document.addEventListener('DOMContentLoaded', async function() {
    // Check authentication first
    await checkAuthentication();
    
    // User dropdown toggle
    const userMenu = document.querySelector('.user-menu');
    const userDropdownBtn = document.getElementById('user-dropdown-btn');
    const userDropdownMenu = document.getElementById('user-dropdown-menu');
    const signoutBtn = document.getElementById('signout-btn');
    
    // Ensure dropdown is hidden initially
    userDropdownMenu.classList.remove('show');
    
    // Toggle dropdown when clicking anywhere on the user menu
    userMenu.addEventListener('click', function(e) {
        e.stopPropagation();
        userDropdownMenu.classList.toggle('show');
    });
    
    // Close dropdown when clicking outside
    document.addEventListener('click', function(e) {
        if (!userMenu.contains(e.target)) {
            userDropdownMenu.classList.remove('show');
        }
    });
    
    // Close dropdown when pressing escape
    document.addEventListener('keydown', function(e) {
        if (e.key === 'Escape') {
            userDropdownMenu.classList.remove('show');
        }
    });
    
    // Handle signout
    signoutBtn.addEventListener('click', async function(e) {
        e.preventDefault();
        
        // Always clear local data first
        const token = localStorage.getItem('authToken') || sessionStorage.getItem('authToken');
        
        // Clear stored authentication data immediately
        localStorage.removeItem('authToken');
        sessionStorage.removeItem('authToken');
        localStorage.removeItem('userEmail');
        sessionStorage.removeItem('userEmail');
        
        try {
            if (token) {
                // Try to notify the server, but don't wait for success
                fetch('/api/auth/signout', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                        'Authorization': `Bearer ${token}`
                    }
                }).catch(error => {
                    console.log('Server signout failed, but local signout completed:', error);
                });
            }
            
            // Show success message
            showNotification('Signed out successfully', 'success');
            
            // Redirect to signin page
            setTimeout(() => {
                window.location.href = '/signin.html';
            }, 500);
            
        } catch (error) {
            console.log('Signout process completed with local cleanup:', error);
        }
    });
    
    // Authentication check function
    async function checkAuthentication() {
        const token = localStorage.getItem('authToken') || sessionStorage.getItem('authToken');
        
        if (!token) {
            // No token found, redirect to signin
            console.log('No token found, redirecting to signin');
            window.location.href = '/signin.html';
            return;
        }
        
        // Basic JWT token format check (should have 3 parts separated by dots)
        const tokenParts = token.split('.');
        if (tokenParts.length !== 3) {
            console.log('Invalid token format, redirecting to signin');
            clearAuthData();
            return;
        }
        
        try {
            // Decode the JWT payload to check expiration (basic check)
            const payload = JSON.parse(atob(tokenParts[1]));
            const currentTime = Math.floor(Date.now() / 1000);
            
            if (payload.exp && payload.exp < currentTime) {
                console.log('Token expired, redirecting to signin');
                clearAuthData();
                return;
            }
            
            console.log('Token appears valid and not expired');
            
        } catch (error) {
            console.error('Token validation error:', error);
            clearAuthData();
        }
    }
    
    // Helper function to clear auth data and redirect
    function clearAuthData() {
        localStorage.removeItem('authToken');
        sessionStorage.removeItem('authToken');
        localStorage.removeItem('userEmail');
        sessionStorage.removeItem('userEmail');
        window.location.href = '/signin.html';
    }
    
    // Simple notification function
    function showNotification(message, type = 'info') {
        const notification = document.createElement('div');
        notification.className = `notification notification-${type}`;
        notification.textContent = message;
        notification.style.cssText = `
            position: fixed;
            top: 20px;
            right: 20px;
            padding: 12px 20px;
            border-radius: 8px;
            color: white;
            font-weight: 500;
            z-index: 9999;
            animation: slideIn 0.3s ease;
        `;
        
        // Set background color based on type
        switch(type) {
            case 'success':
                notification.style.backgroundColor = '#C38D94';
                notification.style.color = 'white';
                break;
            case 'error':
                notification.style.backgroundColor = '#ef4444';
                break;
            case 'warning':
                notification.style.backgroundColor = '#f59e0b';
                break;
            default:
                notification.style.backgroundColor = '#3b82f6';
        }
        
        document.body.appendChild(notification);
        
        // Remove after 3 seconds
        setTimeout(() => {
            notification.style.animation = 'slideOut 0.3s ease';
            setTimeout(() => {
                if (notification.parentNode) {
                    notification.parentNode.removeChild(notification);
                }
            }, 300);
        }, 3000);
    }
});

// Add CSS animations for notifications
const style = document.createElement('style');
style.textContent = `
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
    
    @keyframes slideOut {
        from {
            transform: translateX(0);
            opacity: 1;
        }
        to {
            transform: translateX(100%);
            opacity: 0;
        }
    }
`;
document.head.appendChild(style);