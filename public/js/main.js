// Mobile Navigation Toggle
document.addEventListener('DOMContentLoaded', function() {
    const navToggle = document.getElementById('nav-toggle');
    const navMenu = document.getElementById('nav-menu');
    
    navToggle.addEventListener('click', function() {
        navMenu.classList.toggle('active');
        navToggle.classList.toggle('active');
    });

    // Close mobile menu when clicking on a link
    document.querySelectorAll('.nav-link').forEach(link => {
        link.addEventListener('click', () => {
            navMenu.classList.remove('active');
            navToggle.classList.remove('active');
        });
    });

    // Close mobile menu when clicking outside
    document.addEventListener('click', function(event) {
        const isClickInsideNav = navMenu.contains(event.target) || navToggle.contains(event.target);
        
        if (!isClickInsideNav && navMenu.classList.contains('active')) {
            navMenu.classList.remove('active');
            navToggle.classList.remove('active');
        }
    });
});

// Smooth scrolling for navigation links
document.querySelectorAll('a[href^="#"]').forEach(anchor => {
    anchor.addEventListener('click', function (e) {
        e.preventDefault();
        const target = document.querySelector(this.getAttribute('href'));
        
        if (target) {
            const offsetTop = target.getBoundingClientRect().top + window.pageYOffset - 80;
            
            window.scrollTo({
                top: offsetTop,
                behavior: 'smooth'
            });
        }
    });
});

// Navbar scroll effect
window.addEventListener('scroll', function() {
    const navbar = document.querySelector('.navbar');
    
    if (window.scrollY > 100) {
        navbar.style.backgroundColor = 'rgba(255, 255, 255, 0.98)';
        navbar.style.backdropFilter = 'blur(20px)';
    } else {
        navbar.style.backgroundColor = 'rgba(255, 255, 255, 0.95)';
        navbar.style.backdropFilter = 'blur(10px)';
    }
});

// Intersection Observer for animations
const observerOptions = {
    threshold: 0.1,
    rootMargin: '0px 0px -50px 0px'
};

const observer = new IntersectionObserver(function(entries) {
    entries.forEach(entry => {
        if (entry.isIntersecting) {
            entry.target.style.opacity = '1';
            entry.target.style.transform = 'translateY(0)';
        }
    });
}, observerOptions);

// Observe elements for animation
document.addEventListener('DOMContentLoaded', function() {
    const animatedElements = document.querySelectorAll('.feature-card, .pricing-card, .security-feature');
    
    animatedElements.forEach(el => {
        el.style.opacity = '0';
        el.style.transform = 'translateY(30px)';
        el.style.transition = 'opacity 0.6s ease-out, transform 0.6s ease-out';
        observer.observe(el);
    });
});

// Counter animation for hero stats
function animateCounter(element, target, duration = 2000) {
    const start = 0;
    const increment = target / (duration / 16);
    let current = start;
    
    const timer = setInterval(() => {
        current += increment;
        
        if (current >= target) {
            current = target;
            clearInterval(timer);
        }
        
        if (element.textContent.includes('%')) {
            element.textContent = Math.floor(current) + '%';
        } else if (element.textContent.includes('+')) {
            element.textContent = Math.floor(current) + '+';
        } else {
            element.textContent = Math.floor(current);
        }
    }, 16);
}

// Animate counters when they come into view
const statsObserver = new IntersectionObserver(function(entries) {
    entries.forEach(entry => {
        if (entry.isIntersecting) {
            const statNumber = entry.target.querySelector('.stat-number');
            const text = statNumber.textContent;
            
            if (text.includes('99.9%')) {
                animateCounter(statNumber, 99.9);
            } else if (text.includes('500+')) {
                animateCounter(statNumber, 500);
            }
            
            statsObserver.unobserve(entry.target);
        }
    });
}, { threshold: 0.5 });

document.addEventListener('DOMContentLoaded', function() {
    const heroStats = document.querySelector('.hero-stats');
    if (heroStats) {
        statsObserver.observe(heroStats);
    }
});

// Form handling for CTAs (placeholder functionality)
document.querySelectorAll('.btn-primary').forEach(button => {
    if (button.textContent.includes('Start Free Trial') || button.textContent.includes('Get Started')) {
        button.addEventListener('click', function(e) {
            e.preventDefault();
            
            // Create modal or redirect to signup page
            showSignupModal();
        });
    }
});

document.querySelectorAll('.btn-secondary').forEach(button => {
    if (button.textContent.includes('Schedule Demo')) {
        button.addEventListener('click', function(e) {
            e.preventDefault();
            
            // Create modal or redirect to demo scheduling page
            showDemoModal();
        });
    }
});

// Modal functions (placeholder implementations)
function showSignupModal() {
    // This would typically open a signup modal or redirect to a signup page
    alert('Signup functionality would be implemented here. This would typically redirect to a signup form or open a modal.');
    console.log('Signup button clicked - redirect to signup page');
}

function showDemoModal() {
    // This would typically open a demo scheduling modal or redirect to a scheduling page
    alert('Demo scheduling functionality would be implemented here. This would typically redirect to a calendar booking system.');
    console.log('Demo button clicked - redirect to demo scheduling page');
}

// Contact sales functionality
document.querySelectorAll('.btn').forEach(button => {
    if (button.textContent.includes('Contact Sales')) {
        button.addEventListener('click', function(e) {
            e.preventDefault();
            
            // Create contact form or redirect to contact page
            showContactModal();
        });
    }
});

function showContactModal() {
    alert('Contact sales functionality would be implemented here. This would typically open a contact form or redirect to a sales contact page.');
    console.log('Contact sales button clicked - redirect to contact page');
}

// Keyboard navigation improvements
document.addEventListener('keydown', function(e) {
    // Escape key closes mobile menu
    if (e.key === 'Escape') {
        const navMenu = document.getElementById('nav-menu');
        const navToggle = document.getElementById('nav-toggle');
        
        if (navMenu.classList.contains('active')) {
            navMenu.classList.remove('active');
            navToggle.classList.remove('active');
        }
    }
});

// Loading state management for buttons
function setButtonLoading(button, isLoading) {
    if (isLoading) {
        button.disabled = true;
        button.style.cursor = 'not-allowed';
        button.style.opacity = '0.6';
    } else {
        button.disabled = false;
        button.style.cursor = 'pointer';
        button.style.opacity = '1';
    }
}

// Utility function for smooth animations
function animateElement(element, animation, duration = 300) {
    element.style.animation = `${animation} ${duration}ms ease-out`;
    
    setTimeout(() => {
        element.style.animation = '';
    }, duration);
}

// Performance optimization - lazy loading for images when added
function setupLazyLoading() {
    const images = document.querySelectorAll('img[data-src]');
    
    const imageObserver = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                const img = entry.target;
                img.src = img.dataset.src;
                img.classList.remove('lazy');
                imageObserver.unobserve(img);
            }
        });
    });
    
    images.forEach(img => imageObserver.observe(img));
}

// Initialize lazy loading when DOM is ready
document.addEventListener('DOMContentLoaded', setupLazyLoading);

// Analytics tracking (placeholder for Google Analytics, etc.)
function trackEvent(action, category, label) {
    // This would integrate with your analytics platform
    console.log(`Analytics Event: ${category} - ${action} - ${label}`);
    
    // Example: gtag('event', action, { event_category: category, event_label: label });
}

// Track button clicks for analytics
document.addEventListener('click', function(e) {
    if (e.target.matches('.btn-primary')) {
        trackEvent('click', 'CTA', 'Primary Button');
    } else if (e.target.matches('.btn-secondary')) {
        trackEvent('click', 'CTA', 'Secondary Button');
    } else if (e.target.matches('.nav-link')) {
        trackEvent('click', 'Navigation', e.target.textContent.trim());
    }
});

// Scroll progress indicator (optional enhancement)
function createScrollProgress() {
    const progressBar = document.createElement('div');
    progressBar.style.cssText = `
        position: fixed;
        top: 0;
        left: 0;
        width: 0%;
        height: 3px;
        background: linear-gradient(90deg, var(--fern-green), var(--lapis-lazuli));
        z-index: 9999;
        transition: width 0.1s ease;
    `;
    
    document.body.appendChild(progressBar);
    
    window.addEventListener('scroll', () => {
        const scrollPercent = (window.scrollY / (document.documentElement.scrollHeight - window.innerHeight)) * 100;
        progressBar.style.width = scrollPercent + '%';
    });
}

// Initialize scroll progress (uncomment if desired)
// document.addEventListener('DOMContentLoaded', createScrollProgress);